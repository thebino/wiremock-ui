use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScenarioResponse {
    pub scenarios: Vec<Scenario>,
}

#[derive(Serialize, Deserialize)]
pub struct Scenario {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) state: Option<String>,
    #[serde(rename = "possibleStates")]
    pub(crate) possible_states: Option<Vec<String>>,
}
