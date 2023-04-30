use serde::{Deserialize, Serialize};

use super::vmf::EditorProperties;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub classname: String,

    pub editor: EditorProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HiddenEntity {
    pub entity: Entity,
}
