use crate::responses::mapping::Mapping;
use crate::responses::object::MappingObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MappingResponse {
    pub mappings: Vec<Mapping>,
    pub meta: MappingObject,
}
