use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MappingRequest {
    limit: i64,
    offset: i64,
}
