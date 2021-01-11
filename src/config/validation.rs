use super::{Structure, Value};
use crate::identifier::Module as ModuleId;
use boolinator::Boolinator;

#[derive(thiserror::Error, displaydoc::Display, Eq, PartialEq, Debug)]
pub enum Error {
    /// Missing field {field} in {path}
    MissingField { field: String, path: String },
    /// Unknown field {field} in {path}
    InvalidField { field: String, path: String },
    /// Required value is missing in {path}
    Null { path: String },
    /// Value is not valid in {path}
    InvalidKind { path: String },
}

pub fn validate(value: &Value, kind: &Structure, module: &ModuleId) -> Result<(), Error> {
    validate_with_path(value, kind, vec![])
}

fn validate_with_path(value: &Value, kind: &Structure, path: Vec<String>) -> Result<(), Error> {
    match (value, kind) {
        (Value::Null, kind) => {
            matches!(kind, Structure::Optional(_)).ok_or(Error::Null { path: join(&path) })
        }
        (Value::Object(fields), Structure::Section(struct_fields)) => {
            struct_fields.iter().try_for_each(|(field, kind)| {
                (fields.contains_key(field) || matches!(kind, Structure::Optional(_)))
                    .ok_or_else(|| Error::MissingField { field: field.clone(), path: join(&path) })
            })?;

            fields.iter().try_for_each(|(field, value)| match struct_fields.get(field) {
                Some(kind) => validate_with_path(value, kind, append(path.clone(), field)),
                None => Err(Error::InvalidField { field: field.clone(), path: join(&path) }),
            })
        }
        (Value::Array(items), Structure::MultiValueList(kind)) => {
            if items.is_empty() && !matches!(**kind, Structure::Optional(_)) {
                return Err(Error::Null { path: join(&path) });
            }
            items.iter().enumerate().try_for_each(|(idx, item)| match item {
                Value::Null => Err(Error::InvalidKind { path: join(&append(path.clone(), idx)) }),
                item => validate_with_path(item, kind, path.clone()),
            })
        }
        (value, Structure::List(kind) | Structure::Optional(kind)) => validate_with_path(value, kind, path),
        (value @ Value::Number(_), Structure::Integer { .. }) if !value.is_f64() => Ok(()),
        (Value::Number(_), Structure::Number { .. })
        | (Value::Bool(_), Structure::Bool { .. })
        | (Value::String(_), Structure::String { .. }) => Ok(()),
        _ => Err(Error::InvalidKind { path: join(&path) }),
    }
}

fn join(parts: &[String]) -> String {
    parts.join(".")
}

fn append<S: std::fmt::Display>(mut parts: Vec<String>, next: S) -> Vec<String> {
    parts.push(next.to_string());
    parts
}

#[cfg(test)]
mod tests {
    use super::{validate, Structure, ModuleId, Value};

    #[test]
    fn optional_field_not_present_should_match() {
        let structure: Structure = ron::from_str(
            r#"
        Section({
            "merchant_id": String(),
            "secret": Optional(String(secret: true)),
            "password": String(secret: true),
            "number": Number(),
            "integer": Integer(),
            "multivalue": MultiValueList(String()),
        })
        "#,
        )
        .unwrap();
        let value: Value = serde_json::from_str(
            r#"
        {
            "merchant_id": "test",
            "password": "test",
            "number": 100000000000000000000000000000000000000000000000000000000000000000.1,
            "integer": 10000000000000000000000000000000000000000000000000000000000000000000,
            "multivalue": [""]
        }
        "#,
        )
        .unwrap();
        assert_eq!(Ok(()), validate(&value, &structure, &ModuleId::default()));
    }
}
