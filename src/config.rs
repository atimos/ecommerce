mod validation;

use crate::identifier::{Module as ModuleId, Region as RegionId, Store as StoreId};
use indexmap::IndexMap;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Store {
    structure: HashMap<ModuleId, Structure>,
    base: HashMap<ModuleId, Value>,
    region: HashMap<RegionId, HashMap<ModuleId, Value>>,
    store: HashMap<StoreId, HashMap<ModuleId, Value>>,
    calculated: HashMap<(RegionId, StoreId), HashMap<ModuleId, Value>>,
}

impl Store {
    pub fn set_base(&mut self, id: &ModuleId, value: Value) -> Result<(), validation::Error> {
        if let Some(structure) = self.structure.get(id) {
            validation::validate(&value, structure, id)?;
            self.base.insert(id.clone(), value.clone());

            self.calculated.values_mut().for_each(|item| {
                item.insert(id.clone(), value.clone());
            });
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Structure {
    Section(IndexMap<String, Structure>),
    List(Box<Structure>),
    MultiValueList(Box<Structure>),
    Optional(Box<Structure>),
    String {
        default: Option<String>,
        #[serde(default)]
        secret: bool,
    },
    Bool {
        default: Option<bool>,
        #[serde(default)]
        secret: bool,
    },
    Integer {
        default: Option<Decimal>,
        #[serde(default)]
        secret: bool,
    },
    Number {
        default: Option<Decimal>,
        #[serde(default)]
        secret: bool,
    },
}
