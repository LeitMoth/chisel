use std::{collections::HashMap, io::BufWriter, fmt::{Display, Debug}, sync::Mutex};

pub trait ToGeneric {
    fn as_generic(&self) -> &GenericNode;
}

#[derive(Clone)]
pub struct GenericNode {
    key_value_pairs: HashMap<String, Vec<String>>,
    children_nodes: HashMap<String, Vec<Box<dyn ToGeneric>>>,
}

impl Clone for Box<dyn ToGeneric> {
    fn clone(&self) -> Self {
        Box::new(GenericNode { 
            key_value_pairs: self.as_generic().key_value_pairs.clone(),
            children_nodes: self.as_generic().children_nodes.clone(),
        })
    }
}

impl ToGeneric for GenericNode {
    fn as_generic(&self) -> &GenericNode {
       self
    }
}

impl GenericNode {
    fn get_value(&self, key: &str) -> &String {
        self.key_value_pairs.get(key).unwrap().get(0).unwrap()
    }

    fn set_value(&mut self, key: impl ToString, value: impl ToString) {
        self.key_value_pairs.insert(key.to_string(), vec![value.to_string()]);
    }

    fn set_child(&mut self, name: impl ToString, child: &impl ToGeneric) {
        self.children_nodes.insert(name.to_string(), vec![Box::new(child.as_generic().clone())]);
    }

    pub fn to_string(&self) -> String {
        self.to_text(0)
    }

    fn to_text(&self, indent_level: u32) -> String {
        let mut buf = String::new();

        macro_rules! crlf {
            () => {
                buf += "\r\n";
            };
        }
        macro_rules! indent {
            () => {
                for _ in 0..indent_level {
                    buf += "\t"
                }
            };
        }

        for (key, values) in &self.key_value_pairs {
            for value in values {
                indent!();
                buf += "\"";
                buf += &key;
                buf += "\" \"";
                buf += &value;
                buf += "\"";
                crlf!();
            }

        }

        for (name, nodes) in &self.children_nodes {

            for node in nodes {
                indent!();
                buf += &name;
                crlf!();
                indent!();
                buf += "{";
                crlf!();
                buf += &node.as_generic().to_text(indent_level + 1);
                indent!();
                buf += "}";
                crlf!();
            }
        }

        buf
    }

    fn new() -> Self {
        Self {
            key_value_pairs: HashMap::new(),
            children_nodes: HashMap::new()
        }
    }

}

pub struct BasicParser<'a> {
   pub input: &'a str,
}

impl BasicParser<'_> {
    pub fn read_tree(&mut self) -> Result<GenericNode, String> {
        let mut node = GenericNode {
            key_value_pairs: HashMap::new(),
            children_nodes: HashMap::new(),
        };

        loop {
            self.input = self.input.trim();
            let c = self.input.chars().nth(0).ok_or("EOF");
            if let Err(e) = c {
                break;
            }

            let n = self.input.len();
            println!("{}", n);

            match c? {
                '"' => {
                    let split = self.input.splitn(5, "\"");

                    let split: Vec<&str> = split.collect();

                    let key = split[1].to_owned();
                    let value = split[3].to_owned();

                    if !node.key_value_pairs.contains_key(&key) {
                        node.key_value_pairs.insert(key.clone(), Vec::new());
                    }
                    node.key_value_pairs.get_mut(&key).expect("Didn't we just make one?").push(value);

                    self.input = split[4];
                }
                'a'..='z' | 'A'..='Z' => {
                    let (name, rest) = self.input.split_once("{").ok_or("Expected {")?;
                    self.input = rest;
                    let name = name.trim().to_owned();
                    if !node.children_nodes.contains_key(&name) {
                        node.children_nodes.insert(name.clone(), Vec::new());
                    }
                    node.children_nodes
                        .get_mut(&name)
                        .ok_or("bruh")?
                        .push(Box::new(self.read_tree()?));
                }
                '}' => {
                    self.input = &self.input[1..];
                    break
                }
                _ => unreachable!(),
            }
        }

        Ok(node)
    }
}


struct MaybeDirty<T> {
    dirty: bool,
    data: T,
}

impl <T> MaybeDirty<T> {
    fn get(&mut self, update: &mut impl FnMut(&mut T)) -> &T {
        if self.dirty {
            update(&mut self.data);
            &self.data
        } else {
            &self.data
        }
    }

    fn new(t: T) -> Self {
        Self {
            dirty: true,
            data: t,
        }
    }
}

pub struct Vmf {
    generic_rep: MaybeDirty<GenericNode>,
    rest: GenericNode,
    version_info: VersionInfo,
}

impl Vmf {
    fn update_generic(&self, g: &mut GenericNode) {
        g.set_child("versioninfo", &self.version_info);
    }

    pub fn parse(mut g: GenericNode) -> Self {
        let v = g.children_nodes.remove("version_info").unwrap();
        Self {
            generic_rep: MaybeDirty::new(GenericNode::new()),
            rest: g,
            version_info: VersionInfo::parse(v[0].as_generic())
        }
    }
}

impl ToGeneric for Vmf {
    fn as_generic(&self) -> &GenericNode {
        self.generic_rep.get(&mut |g| self.update_generic(g))
    }
}

pub struct VersionInfo {
    generic_rep: MaybeDirty<GenericNode>,

    pub editor_version: u32,
    pub editor_build: u32,
    pub map_version: u32,
    pub format_version: u32,
    pub prefab: u32,
}

impl ToGeneric for VersionInfo {
    fn as_generic(&self) -> &GenericNode {
        self.generic_rep.get(&mut |g| {
            g.set_value("editorversion", self.editor_version);
            g.set_value("editorbuild", self.editor_build);
            g.set_value("mapversion", self.map_version);
            g.set_value("formatversion", self.format_version);
            g.set_value("prefab", self.prefab);
        })
    }
}


impl VersionInfo {
    fn parse(g: &GenericNode) -> Self {
        Self {
            generic_rep: MaybeDirty::new(GenericNode::new()),
            editor_version: g.get_value("editorversion").parse().unwrap(),
            editor_build: g.get_value("editorbuild").parse().unwrap(),
            map_version: g.get_value("mapversion").parse().unwrap(),
            format_version: g.get_value("formatversion").parse().unwrap(),
            prefab: g.get_value("prefab").parse().unwrap(),
        }
    }
}


// struct Root {
//     world: World,
//     entities: Entities,
//     rest: GenericNode
// }

// struct World {
//     solids: Vec<Solid>,
//     rest: GenericNode,
// }

// struct Solid {
//     id: u32,
//     sides: Vec<Side>,
//     rest: GenericNode,
// }

// struct Side {
//     pub id: u32,
//     pub plane: Plane,
//     pub material: String,
//     pub u_axis: ([f32;4], f32),
//     pub v_axis: ([f32;4], f32),
//     pub rotation: f32,
//     pub lightmap_scale: u32,
//     pub smoothing_groups: u32,
// }
