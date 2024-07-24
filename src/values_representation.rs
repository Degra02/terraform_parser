use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesRepresentation {
    pub outputs: Option<HashMap<String, Output>>,
    pub root_module: Module
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub value: serde_json::Value,
    pub sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub address: Option<String>,
    pub resources: Vec<Resource>,
    pub child_modules: Option<Vec<Module>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Managed,
    Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub address: String,
    pub mode: Mode,

    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub index: Option<usize>,
    pub provider_name: String,
    pub schema_version: u64,
    pub values: HashMap<String, serde_json::Value>,
    pub sensitive_values: HashMap<String, serde_json::Value>,
}
