use crate::responses::request::MappingRequest;
use crate::responses::response::MappingResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Mapping {
    id: Option<String>,
    uuid: Option<String>,
    name: Option<String>,
    requests: Option<MappingRequest>,
    responses: Option<MappingResponse>,
    persistent: Option<bool>,
    priority: Option<i32>,
    #[serde(rename = "scenarioName")]
    scenario_name: Option<String>,
    #[serde(rename = "requiredScenarioState")]
    required_scenario_state: Option<String>,
    #[serde(rename = "newScenarioState")]
    new_scenario_state: Option<String>,
    #[serde(rename = "postServeActions")]
    post_serve_actions: Option<String>,
    #[allow(dead_code)]
    #[serde(skip)]
    metadata: String,
}
