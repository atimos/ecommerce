use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct Module(uuid::Uuid);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct Region(uuid::Uuid);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct Store(uuid::Uuid);

impl Default for Module {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
