use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct GenericNode {
    pub key_value_pairs: HashMap<String, Vec<String>>,
    pub children_nodes: HashMap<String, Vec<GenericNode>>,
}

impl GenericNode {
    pub fn get_value(&self, key: &str) -> &String {
        self.key_value_pairs.get(key).unwrap().get(0).unwrap()
    }

    pub fn set_value(&mut self, key: impl ToString, value: impl ToString) {
        self.key_value_pairs
            .insert(key.to_string(), vec![value.to_string()]);
    }

    pub fn set_child(&mut self, name: impl ToString, child: GenericNode) {
        self.set_children(name, vec![child]);
    }

    pub fn set_children(&mut self, name: impl ToString, children: Vec<GenericNode>) {
        self.children_nodes.insert(name.to_string(), children);
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
                buf += &node.to_text(indent_level + 1);
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

    pub fn parse(input: &str) -> Result<GenericNode, String> {
        let tmp = read_tree(input)?;

        if tmp.leftover.len() != 0 {
            Err("Leftover input: ".to_owned() + tmp.leftover)
        } else {
            Ok(tmp.subtree)
        }
    }
}

struct ParseStep<'a> {
    subtree: GenericNode,
    leftover: &'a str,
}

fn read_tree(mut input: &str) -> Result<ParseStep, String> {
    let mut node = GenericNode::new();

    loop {
        input = input.trim();

        match input.chars().next() {
            None => break,
            Some('"') => {
                // Not extremely efficient. I think I could unpack into a tuple with a crate called "itertools"
                let split: Vec<&str> = input.splitn(5, "\"").collect();

                let key = split[1].to_owned();
                let value = split[3].to_owned();

                match node.key_value_pairs.get_mut(&key) {
                    Some(values) => values.push(value),
                    None => {
                        node.key_value_pairs.insert(key, vec![value]);
                    }
                }

                input = split[4];
            }
            Some('a'..='z' | 'A'..='Z') => {
                let (name, rest) = input.split_once("{").ok_or("Expected {")?;
                let name = name.trim();

                let ParseStep { subtree, leftover } = read_tree(rest)?;
                input = leftover;

                match node.children_nodes.get_mut(name) {
                    Some(children) => children.push(subtree),
                    None => {
                        node.children_nodes.insert(name.to_owned(), vec![subtree]);
                    }
                }
            }
            Some('}') => {
                input = &input[1..];
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(ParseStep {
        subtree: node,
        leftover: input,
    })
}
