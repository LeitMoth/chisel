use std::fmt::Debug;

use bevy::prelude::Vec3;

use super::generic::GenericNode;

/*
TODO:

Hidden Solids
Entities
Hidden Entities

*/

#[derive(Debug)]
pub struct Vmf {
    pub version_info: VersionInfo,
    pub world: World,
    pub rest: GenericNode,
}

impl Vmf {
    pub fn parse(mut g: GenericNode) -> Self {
        let version_info = g
            .children_nodes
            .remove("versioninfo")
            .unwrap()
            .pop()
            .unwrap();
        let world = g.children_nodes.remove("world").unwrap().pop().unwrap();

        let version_info = VersionInfo::parse(version_info);
        let world = World::parse(world);

        Self {
            version_info,
            world,
            rest: g,
        }
    }

    pub fn as_generic(&self) -> GenericNode {
        let mut g = self.rest.clone();

        g.set_child("versioninfo", self.version_info.as_generic());
        g.set_child("world", self.world.as_generic());

        g
    }
}

#[derive(Debug)]
pub struct VersionInfo {
    pub editor_version: u32,
    pub editor_build: u32,
    pub map_version: u32,
    pub format_version: u32,
    pub prefab: u32,
}

/*
Maybe I can figure out some sort of macro that maps these easier?
the parse and as_generic look very similar.

I need to rename a lot of things, and find out if I can do the macro thing
*/

impl VersionInfo {
    fn parse(g: GenericNode) -> Self {
        Self {
            editor_version: g.get_value("editorversion").parse().unwrap(),
            editor_build: g.get_value("editorbuild").parse().unwrap(),
            map_version: g.get_value("mapversion").parse().unwrap(),
            format_version: g.get_value("formatversion").parse().unwrap(),
            prefab: g.get_value("prefab").parse().unwrap(),
        }
    }

    fn as_generic(&self) -> GenericNode {
        let mut g = GenericNode::new();

        g.set_value("editorversion", self.editor_version);
        g.set_value("editorbuild", self.editor_build);
        g.set_value("mapversion", self.map_version);
        g.set_value("formatversion", self.format_version);
        g.set_value("prefab", self.prefab);

        g
    }
}

#[derive(Debug)]
pub struct World {
    pub solids: Vec<Solid>,
    pub rest: GenericNode,
}

impl World {
    fn parse(mut g: GenericNode) -> Self {
        let solids = g.children_nodes.remove("solid").unwrap();
        let solids = solids.into_iter().map(Solid::parse).collect();
        Self { solids, rest: g }
    }
    fn as_generic(&self) -> GenericNode {
        let mut g = self.rest.clone();

        g.set_children(
            "solid",
            self.solids.iter().map(|s| s.as_generic()).collect(),
        );

        g
    }
}

#[derive(Debug)]
pub struct Solid {
    pub id: u32,
    pub sides: Vec<Side>,
    pub rest: GenericNode,
}

impl Solid {
    fn parse(mut g: GenericNode) -> Self {
        let sides = g
            .children_nodes
            .remove("side")
            .unwrap()
            .into_iter()
            .map(Side::parse)
            .collect();
        let id = g
            .key_value_pairs
            .remove("id")
            .unwrap()
            .pop()
            .unwrap()
            .parse()
            .unwrap();
        Self { id, sides, rest: g }
    }

    fn as_generic(&self) -> GenericNode {
        let mut g = self.rest.clone();

        g.set_value("id", self.id);
        g.set_children("side", self.sides.iter().map(|s| s.as_generic()).collect());

        g
    }
}

#[derive(Debug)]
pub struct Side {
    pub id: u32,
    pub plane: Plane,
    pub material: String,
    pub u_axis: UV,
    pub v_axis: UV,
    pub rotation: f32,
    pub lightmap_scale: u32,
    pub smoothing_groups: u32,
    pub rest: GenericNode,
}

impl Side {
    fn parse(mut g: GenericNode) -> Self {
        let id = g
            .key_value_pairs
            .remove("id")
            .unwrap()
            .pop()
            .unwrap()
            .parse()
            .unwrap();
        let plane = g.key_value_pairs.remove("plane").unwrap().pop().unwrap();
        let material = g.key_value_pairs.remove("material").unwrap().pop().unwrap();
        let u_axis = g.key_value_pairs.remove("uaxis").unwrap().pop().unwrap();
        let v_axis = g.key_value_pairs.remove("vaxis").unwrap().pop().unwrap();
        let rotation = g.key_value_pairs.remove("rotation").unwrap().pop().unwrap();
        let lightmap_scale = g
            .key_value_pairs
            .remove("lightmapscale")
            .unwrap()
            .pop()
            .unwrap();
        let smoothing_groups = g
            .key_value_pairs
            .remove("smoothing_groups")
            .unwrap()
            .pop()
            .unwrap();

        let plane = Plane::parse(&plane);
        let u_axis = UV::parse(&u_axis);
        let v_axis = UV::parse(&v_axis);
        let rotation = rotation.parse().unwrap();
        let lightmap_scale = lightmap_scale.parse().unwrap();
        let smoothing_groups = smoothing_groups.parse().unwrap();

        Self {
            id,
            plane,
            material,
            u_axis,
            v_axis,
            rotation,
            lightmap_scale,
            smoothing_groups,
            rest: g,
        }
    }

    fn as_generic(&self) -> GenericNode {
        let mut g = self.rest.clone();

        g.set_value("id", self.id);
        g.set_value("plane", self.plane.to_string());
        g.set_value("material", &self.material);
        g.set_value("uaxis", self.u_axis.to_string());
        g.set_value("vaxis", self.v_axis.to_string());
        g.set_value("rotation", self.rotation);
        g.set_value("lightmapscale", self.lightmap_scale);
        g.set_value("smoothing_groups", self.smoothing_groups);

        g
    }
}

#[derive(Debug)]
pub struct UV([f32; 4], f32);

impl UV {
    fn parse(mut s: &str) -> Self {
        let mut tmp = Self([0.0, 0.0, 0.0, 0.0], 0.0);
        s = &s[1..];
        let mut lr = s.split("]");
        let coords = lr.next().unwrap();
        let mut coords = coords.split(" ");
        for i in 0..4 {
            tmp.0[i] = coords.next().unwrap().parse().unwrap();
        }
        tmp.1 = lr.next().unwrap().trim().parse().unwrap();

        tmp
    }

    fn to_string(&self) -> String {
        let mut tmp = String::new();
        tmp += "[";
        tmp += &self.0[0].to_string();
        tmp += " ";
        tmp += &self.0[1].to_string();
        tmp += " ";
        tmp += &self.0[2].to_string();
        tmp += " ";
        tmp += &self.0[3].to_string();
        tmp += "] ";
        tmp += &self.1.to_string();

        tmp
    }
}

#[derive(Debug)]
pub struct Plane {
    pub points: [Point; 3],
}

impl Plane {
    fn parse(mut input: &str) -> Self {
        let mut jump_past = |pattern: &str| {
            input.find(pattern).map(|pos| {
                let d = &input[0..pos];
                input = &input[pos + pattern.len()..];
                d
            })
        };

        let mut p: Plane = Plane {
            points: [
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            ],
        };

        for pi in 0..3 {
            jump_past("(").unwrap();

            let x = jump_past(" ").unwrap().parse().unwrap();
            let y = jump_past(" ").unwrap().parse().unwrap();
            let z = jump_past(")").unwrap().parse().unwrap();

            p.points[pi] = Point { x, y, z };
        }

        p
    }

    fn to_string(&self) -> String {
        let mut tmp = String::new();
        for (i, point) in self.points.iter().enumerate() {
            tmp += "(";
            tmp += &point.x.to_string();
            tmp += " ";
            tmp += &point.y.to_string();
            tmp += " ";
            tmp += &point.z.to_string();
            tmp += ")";

            if i < self.points.len() - 1 {
                tmp += " "
            }
        }

        tmp
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    // IMPORTANT: Hammer stores coordinates with Z as up, where here we use Y
    pub fn new_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.z, self.y)
    }
}
