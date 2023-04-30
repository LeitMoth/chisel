use std::collections::HashMap;

#[derive(Debug)]
pub struct TextTree<'a> {
    pub key_value_pairs: HashMap<&'a str, Vec<&'a str>>,
    pub children_nodes: HashMap<&'a str, Vec<TextTree<'a>>>,
}

impl TextTree<'_> {
    pub fn is_empty(&self) -> bool {
        self.key_value_pairs.is_empty() && self.children_nodes.is_empty()
    }
}
pub struct BasicParser<'a> {
    pub input: &'a str,
}

impl<'a> BasicParser<'a> {
    pub fn read_tree(&mut self) -> Result<TextTree<'a>, String> {
        let mut node = TextTree {
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
            // println!("{}", n);

            match c? {
                '"' => {
                    let split = self.input.splitn(5, "\"");

                    let split: Vec<&str> = split.collect();

                    let key = split[1];
                    let value = split[3];

                    if !node.key_value_pairs.contains_key(key) {
                        node.key_value_pairs.insert(key.clone(), Vec::new());
                    }
                    node.key_value_pairs
                        .get_mut(key)
                        .expect("Didn't we just make one?")
                        .push(value);

                    self.input = split[4];
                }
                'a'..='z' | 'A'..='Z' => {
                    let (name, rest) = self.input.split_once("{").ok_or("Expected {")?;
                    self.input = rest;
                    let name = name.trim();
                    if !node.children_nodes.contains_key(name) {
                        node.children_nodes.insert(name, Vec::new());
                    }
                    node.children_nodes
                        .get_mut(&name)
                        .ok_or("bruh")?
                        .push(self.read_tree()?);
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
