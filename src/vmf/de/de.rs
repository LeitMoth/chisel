use std::{
    collections::HashMap,
    fs::File,
    iter::Map,
    str::{FromStr, Lines},
};

use serde::{de, Deserialize};

use std::io::Write;

use crate::vmf::error::{Error, Result};

use super::basic::{BasicParser, TextTree};

pub struct Deserializer<'de> {
    // input: &'de str,
    // input: TextTree<'de>,
    // current_parent: TextTree<'de>,
    current_children: Option<Vec<TextTree<'de>>>,
    current_values: Option<Vec<&'de str>>,

    // root: TextTree<'de>,
    parent_stack: Vec<TextTree<'de>>,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        let inp = BasicParser { input }.read_tree().unwrap();
        // println!("{inp:#?}");

        // let mut out = File::create("test").unwrap();

        // write!(out,"{inp:#?}").unwrap();

        // drop(out);

        Deserializer {
            // input:
            // root: inp,
            parent_stack: vec![inp],
            current_children: None,
            current_values: None,
        }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    // We don't care if we only have whitespace left over
    // if deserializer.input.trim().is_empty() {
    //     Ok(t)
    // } else {
    //     Err(Error::TrailingCharacters)
    // }
    Ok(t)
}

/*
impl Deserializer<'de> {
    // Parsing helper functions

    fn jump_past(&mut self, pattern: &str) -> Result<()> {
        match self.input.find(pattern) {
            Some(pos) => {
                self.input = &self.input[pos + pattern.len()..];
                Ok(())
            }
            None => Err(Error::Eof),
        }
    }

    fn try_remove_from_start(&mut self, pattern: &str) -> bool {
        if self.input.starts_with(pattern) {
            self.input = &self.input[pattern.len()..];
            true
        } else {
            false
        }
    }

    // Returns an option of () if it succeeds.
    // This looks a little strange, but works very well with .ok_or()
    fn remove_from_start(&mut self, pattern: &str) -> Option<()> {
        self.try_remove_from_start(pattern).then_some(())
    }

    fn parse_bool(&mut self) -> Result<bool> {
        if self.input.starts_with("1") {
            self.input = &self.input["1".len()..];
            Ok(true)
        } else if self.input.starts_with("0") {
            self.input = &self.input["0".len()..];
            Ok(false)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    fn parse<T>(&mut self) -> Result<T>
    where
        T: FromStr,
    {
        match self.input.find('"') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len..];
                s.parse::<T>().map_err(|_| Error::BadParse)
            }
            None => Err(Error::Eof),
        }
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        match self.input.find('"') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len..];
                Ok(s)
            }
            None => Err(Error::Eof),
        }
    }
}

*/

macro_rules! deserialize {
    ($de: ident, $vis: ident) => {
        fn $de<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            visitor.$vis(
                self.current_values
                    .as_mut()
                    .ok_or(Error::ExpectedInteger)?
                    .pop()
                    .ok_or(Error::ExpectedInteger)?
                    .parse()
                    .map_err(|_| Error::ExpectedInteger)?,
            )
        }
    };
}

macro_rules! dont {
    ($de: ident) => {
        fn $de<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            unimplemented!()
        }
    };
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    deserialize! {deserialize_i8 ,visit_i8 }
    deserialize! {deserialize_i16,visit_i16}
    deserialize! {deserialize_i32,visit_i32}
    deserialize! {deserialize_i64,visit_i64}

    deserialize! {deserialize_u8 ,visit_u8 }
    deserialize! {deserialize_u16,visit_u16}
    deserialize! {deserialize_u32,visit_u32}
    deserialize! {deserialize_u64,visit_u64}

    deserialize! {deserialize_f32,visit_f32}
    deserialize! {deserialize_f64,visit_f64}

    deserialize! {deserialize_char,visit_char}

    dont! {deserialize_any}

    dont! {deserialize_bytes}
    dont! {deserialize_byte_buf}

    dont! {deserialize_option}

    dont! {deserialize_unit}

    dont! {deserialize_map}

    dont! {deserialize_identifier}

    dont! {deserialize_ignored_any}

    // We don't implement newtype_struct or enum either, but the macro wouldn't work with them
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    // Start of actual implementation

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let s = self
            .current_values
            .as_mut()
            .ok_or(Error::ExpectedBoolean)?
            .pop()
            .ok_or(Error::ExpectedBoolean)?;
        if s.eq("1") {
            visitor.visit_bool(true)
        } else if s.eq("0") {
            visitor.visit_bool(false)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(
            self.current_values
                .as_mut()
                .ok_or(Error::ExpectedString)?
                .pop()
                .ok_or(Error::ExpectedString)?,
        )
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // self.jump_past("}")?;
        todo!();

        // visitor.visit_unit()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAcess { de: self })
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let next = self
            .parent_stack
            .last_mut()
            .unwrap()
            .children_nodes
            .get_mut(name);
        // if let None = next {
        //     return visitor.visit_seq(StructAccess {
        //         de: self,
        //         fields,
        //         index: 0,
        //         empty: true,
        //     });
        // }
        let asdf = next.unwrap().pop();

        if let None = asdf {
            return Err(Error::Test);
        }
        
        let bbbb = asdf.unwrap();
        self.parent_stack.push(bbbb);

        // Looked at Postcard, thanks
        let temp = visitor.visit_seq(StructAccess {
            de: self,
            fields,
            index: 0,
            empty: false,
        });

        self.parent_stack.pop();

        temp
    }
}

struct SeqAcess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAcess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        // match (
        //     self.de.current_values.as_ref(),
        //     self.de.current_children.as_ref(),
        // ) {
        //     // (Some(values), _) if values.len() == 0 => return Ok(None),
        //     // (_, Some(children)) if children.len() == 0 => return Ok(None),
        //     (None, None) => return Ok(None),
        //     _ => (),
        // }

        if self.de.parent_stack.last().unwrap().children_nodes.len() == 0 && self.de.parent_stack.last().unwrap().key_value_pairs.len() == 0 {
            return Ok(None)
        }
        let temp = seed.deserialize(&mut *self.de);

        if let Err(Error::Test) = temp {
            return Ok(None)
        }
        
        temp.map(Some)
    }
}

struct StructAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    fields: &'static [&'static str],
    index: usize,
    empty: bool,
}

impl<'de, 'a> de::SeqAccess<'de> for StructAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.empty {
            return Ok(None);
        }
        if self.index >= self.fields.len() {
            return Ok(None);
        }

        let current_field = self.fields[self.index];

        self.de.current_values = self
            .de
            .parent_stack
            .last_mut()
            .as_mut()
            .unwrap()
            .key_value_pairs
            .remove(current_field);
        self.de.current_children = self
            .de
            .parent_stack
            .last_mut()
            .as_mut()
            .unwrap()
            .children_nodes
            .remove(current_field);

        self.index += 1;

        return seed.deserialize(&mut *self.de).map(Some);
    }
}
