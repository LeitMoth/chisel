use serde::{de, Deserialize};

use crate::vmf::error::{Error, Result};

use super::basic::{BasicParser, TextTree};

pub struct Deserializer<'de> {
    parent_stack: Vec<TextTree<'de>>,
    current_values: Option<Vec<&'de str>>,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        let inp = BasicParser { input }.read_tree().unwrap();
        // println!("{inp:#?}");

        // let mut out = File::create("test").unwrap();

        // write!(out,"{inp:#?}").unwrap();

        // drop(out);

        Deserializer {
            parent_stack: vec![inp],
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

    println!(
        "ZZZZZZZ\n{:#?}\nZZZZZZZ {}",
        deserializer.parent_stack,
        deserializer.parent_stack.len()
    );

    Ok(t)
}

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
        // println!("{:#?}", self.parent_stack);

        let next = self
            .parent_stack
            .last_mut()
            .expect("whoops, Parent stack is empty")
            .children_nodes
            .get_mut(name)
            .and_then(|children| children.pop())
            .ok_or(Error::EndOfSequence)?;

        self.parent_stack.push(next);

        // Looked at Postcard, thanks
        let temp = visitor.visit_seq(StructAccess {
            de: self,
            fields,
            index: 0,
        });

        // If we completely parsed a struct, clean it up from our TextTree
        let t = self
            .parent_stack
            .pop()
            .expect("Failed to remove the parent we just added I guess");
        if t.is_empty() {
            self.parent_stack
                .last_mut()
                .expect("parent_stack_empty")
                .children_nodes
                .remove(name);
        } else {
            println!("OOOOOOOOO\n{t:#?}\nOOOOOOOOOOO");
            // panic!()
            // self.parent_stack.last_mut().unwrap().children_nodes.get_mut(name).unwrap().push(t);
        }

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
        let temp = seed.deserialize(&mut *self.de);

        // I don't know if this is good practice but is the best I could think of
        if let Err(Error::EndOfSequence) = temp {
            return Ok(None);
        }

        temp.map(Some)
    }
}

struct StructAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    fields: &'static [&'static str],
    index: usize,
}

impl<'de, 'a> de::SeqAccess<'de> for StructAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
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

        self.index += 1;

        return seed.deserialize(&mut *self.de).map(Some);
    }
}
