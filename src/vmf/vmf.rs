use super::world::World;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vmf(
    pub VersionInfo,
    pub VisGroups,
    pub ViewSettings,
    pub World,
    pub Cameras,
    pub Cordons,
    // pub Hidden<Entity>
);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "versioninfo")]
pub struct VersionInfo {
    #[serde(rename = "editorversion")]
    pub editor_version: u32,
    #[serde(rename = "editorbuild")]
    pub editor_build: u32,
    #[serde(rename = "mapversion")]
    pub map_version: u32,
    #[serde(rename = "formatversion")]
    pub format_version: u32,
    pub prefab: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "visgroups")]
pub struct VisGroups {
    #[serde(rename = "")]
    pub groups: Vec<VisGroups>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "visgroup")]
pub struct VisGroup {
    pub name: String,
    pub visgroupid: u32,
    pub color: String,
    #[serde(rename = "")]
    pub sub_groups: Vec<VisGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "viewsettings")]
pub struct ViewSettings {
    #[serde(rename = "bSnapToGrid")]
    pub snap_to_grid: bool,
    #[serde(rename = "bShowGrid")]
    pub show_grid: bool,
    #[serde(rename = "bShowLogicalGrid")]
    pub show_logical_grid: bool,
    #[serde(rename = "nGridSpacing")]
    pub grid_spacing: u32,
    #[serde(rename = "bShow3DGrid")]
    pub show_3d_grid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "cameras")]
pub struct Cameras {
    #[serde(rename = "activecamera")]
    pub active_camera: i32,
    //cameras: Vec<Camera>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "cordons")]
pub struct Cordons {
    pub active: u32,
}
