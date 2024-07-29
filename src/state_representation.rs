use serde::{Deserialize, Serialize};

use crate::values_representation::ValuesRepresentation;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateRepresentation {
    pub terraform_version: String,
    pub values: ValuesRepresentation,
}
