use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MappingRequest {
    pub method: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "urlPath")]
    pub url_path: Option<String>,
    #[serde(rename = "urlPathPattern")]
    pub url_path_pattern: Option<String>,
    #[serde(rename = "queryParameters")]
    pub query_parameters: Option<String>,
    // pub headers: Option<String>,
    // pub basicAuthCredentials: Option<String>,
    // pub cookies: Option<String>,
    // pub bodyPatterns: Option<String>,
}
