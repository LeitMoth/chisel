use serde::{de::Visitor, Deserialize, Serialize};

use super::vmf::EditorProperties;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "group")]
pub struct Group {
    pub id: u32,
    pub prop: EditorProperties,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "world")]
pub struct World {
    pub id: u32,
    #[serde(rename = "mapversion")]
    pub map_version: u32,
    #[serde(rename = "classname")]
    pub class_name: String,
    #[serde(rename = "skyname")]
    pub sky_name: String,
    #[serde(rename = "maxpropscreenwidth")]
    pub max_prop_screen_width: i32,
    #[serde(rename = "detailvbsp")]
    pub detail_vbsp: String,
    #[serde(rename = "detailmaterial")]
    pub detail_material: String,
    #[serde(rename = "maxblobcount")]
    pub max_blob_count: u32,
    #[serde(rename = "solid")]
    pub solids: Vec<Solid>,
    pub hidden: Vec<HiddenSolid>,
    pub group: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "hidden")]
pub struct HiddenSolid {
    pub solid: Solid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "solid")]
pub struct Solid {
    pub id: u32,
    #[serde(rename = "side")]
    pub sides: Vec<Side>,
    pub editor: EditorProperties,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "side")]
pub struct Side {
    pub id: u32,
    pub plane: Plane,
    pub material: String,
    // u_axis: ([f32;4], f32),
    // v_axis: ([f32;4], f32),
    pub uaxis: String,
    pub vaxis: String,
    pub rotation: f32,
    // pub rotation: String,
    #[serde(rename = "lightmapscale")]
    pub lightmap_scale: u32,
    pub smoothing_groups: u32,
}

#[derive(Debug)]
pub struct Plane {
    pub points: [Point; 3],
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

struct PlaneVisitor;

impl<'de> Visitor<'de> for PlaneVisitor {
    type Value = Plane;

    fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Plane, E>
    where
        E: serde::de::Error,
    {
        let mut input = v;

        let mut jump_past = |pattern: &str| match input.find(pattern) {
            Some(pos) => {
                let d = &input[0..pos];
                input = &input[pos + pattern.len()..];
                Ok(d)
            }
            None => Err(super::error::Error::Eof),
        };

        let mut p = Plane {
            points: [
                Point { x: 0.0, y: 0.0, z: 0.0 },
                Point { x: 0.0, y: 0.0, z: 0.0 },
                Point { x: 0.0, y: 0.0, z: 0.0 },
            ],
        };

        for pi in 0..3 {
            jump_past("(").unwrap();

            let x = jump_past(" ").unwrap().parse().unwrap();
            let y = jump_past(" ").unwrap().parse().unwrap();
            let z = jump_past(")").unwrap().parse().unwrap();

            p.points[pi] = Point { x, y, z };
        }

        Ok(p)
    }
}

impl<'de> Deserialize<'de> for Plane {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PlaneVisitor)
    }
}

impl Serialize for Plane {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v: String = format!(
            "({} {} {}) ({} {} {}) ({} {} {})",
            self.points[0].x,
            self.points[0].y,
            self.points[0].z,
            self.points[1].x,
            self.points[1].y,
            self.points[1].z,
            self.points[2].x,
            self.points[2].y,
            self.points[2].z
        );
        serializer.serialize_str(&v)
    }
}
