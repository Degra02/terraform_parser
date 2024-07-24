pub mod state_reresentation;
pub mod values_representation;
#[cfg(test)]
pub mod tests;

#[derive(Default)]
pub struct TerraformParser {}

impl TerraformParser {
    pub fn parse_state(state: &str) -> Result<state_reresentation::StateRepresentation, serde_json::Error> {
        serde_json::from_str(state)
    }
}

