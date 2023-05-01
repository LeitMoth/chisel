use std::collections::HashMap;
use std::fmt::Debug;

pub trait ToGeneric {
    fn as_generic(&self) -> Box<GenericNode>;
}

#[derive(Clone, Debug)]
pub struct GenericNode {
    pub key_value_pairs: HashMap<String, Vec<String>>,
    pub children_nodes: HashMap<String, Vec<Box<dyn ToGeneric>>>,
}

impl Clone for Box<dyn ToGeneric> {
    fn clone(&self) -> Self {
        Box::new(GenericNode {
            key_value_pairs: self.as_generic().key_value_pairs.clone(),
            children_nodes: self.as_generic().children_nodes.clone(),
        })
    }
}

impl Debug for Box<dyn ToGeneric> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_generic().fmt(f)
    }
}

impl ToGeneric for GenericNode {
    fn as_generic(&self) -> Box<GenericNode> {
        Box::new(self.clone())
    }
}

impl GenericNode {
    pub fn get_value(&self, key: &str) -> &String {
        self.key_value_pairs.get(key).unwrap().get(0).unwrap()
    }

    pub fn set_value(&mut self, key: impl ToString, value: impl ToString) {
        self.key_value_pairs
            .insert(key.to_string(), vec![value.to_string()]);
    }

    pub fn set_child(&mut self, name: impl ToString, child: Box<dyn ToGeneric>) {
        self.set_children(name, vec![child]);
        // self.children_nodes
        //     .insert(name.to_string(), vec![child.as_generic()]);
    }

    pub fn set_children(&mut self, name: impl ToString, children: Vec<Box<dyn ToGeneric>>) {
        self.children_nodes
            .insert(name.to_string(), children);
    }

    pub fn to_string(&self) -> String {
        self.to_text(0)
    }

    pub fn to_text(&self, indent_level: u32) -> String {
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

    pub fn new() -> Self {
        Self {
            key_value_pairs: HashMap::new(),
            children_nodes: HashMap::new(),
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
                    node.key_value_pairs
                        .get_mut(&key)
                        .expect("Didn't we just make one?")
                        .push(value);

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
                    break;
                }
                _ => unreachable!(),
            }
        }

        Ok(node)
    }
}
