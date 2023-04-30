use super::{
    entity::{Entity, HiddenEntity},
    world::World, de::basic::TextTree,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vmf(
    pub VersionInfo,
    pub VisGroups,
    pub ViewSettings,
    pub World,
    pub Vec<Entity>,
    pub Vec<HiddenEntity>,
    pub Cameras,
    pub Cordons,
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
    pub groups: Vec<VisGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "visgroup")]
pub struct VisGroup {
    pub name: String,
    pub visgroupid: u32,
    // pub groupid: u32,
    pub color: String,
    #[serde(rename = "visgroup")]
    pub sub_groups: VisGroups,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "editor")]
pub struct EditorProperties {
    pub color: String,
    #[serde(rename = "visgroupshown")]
    pub visgroup_shown: u32,
    #[serde(rename = "visgroupautoshown")]
    pub visgroup_auto_shown: u32,
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
    pub cameras: Vec<Camera>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "camera")]
pub struct Camera {
    pub position: String,
    pub look: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "cordons")]
pub struct Cordons {
    pub active: u32,
    #[serde(rename = "cordon")]
    pub cordons: Vec<Cordon>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "cordon")]
pub struct Cordon {
    pub name: String,
    pub active: u32,
    #[serde(rename = "box")]
    pub cordon_box: CordonBox,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "box")]
pub struct CordonBox {
    pub mins: String,
    pub maxs: String,
}
