use std::str::FromStr;

use serde::{de, Deserialize};

use crate::vmf::error::{Error, Result};

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    // We don't care if we only have whitespace left over
    if deserializer.input.trim().is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de> Deserializer<'de> {
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

macro_rules! deserialize {
    ($de: ident, $vis: ident) => {
        fn $de<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            visitor.$vis(self.parse()?)
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
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
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
        self.jump_past("}")?;

        visitor.visit_unit()
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
        // Looked at Postcard, thanks
        visitor.visit_seq(StructAccess {
            de: self,
            fields,
            name,
            index: 0,
        })
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
        let opened = self.de.input.find("{");
        let closed = self.de.input.find("}");

        match (opened, closed) {
            // (None,None) => Err(Error::Eof),
            (None, _) => Ok(None),
            (Some(opened), Some(closed)) if closed < opened => Ok(None),
            _ => {
                let thing = seed.deserialize(&mut *self.de);

                if let Err(Error::StructNameChanged) = thing {
                    // If we went one element too far and went into a different list
                    // we stop the list there
                    Ok(None)
                } else {
                    // Otherwise we use the value and continue
                    thing.map(Some)
                }
            }
        }
    }
}

struct StructAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    fields: &'static [&'static str],
    name: &'static str,
    index: usize,
}

impl<'de, 'a> de::SeqAccess<'de> for StructAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        let result;

        // Special things to do before we get to the first element
        if self.index == 0 {
            // trim the start: to keep from trimming everywhere, I have decided
            // that if you are going to do something other than a find on the string,
            // just do the trim directly before it. The reader is responsible for the trimming.
            self.de.input = self.de.input.trim_start();
            if !self.de.try_remove_from_start(self.name) {
                // This error is actually caught and used for logic, which isn't great.
                // I don't know a better solution at the moment.
                // Lists of things are very strange, and we really only know we want to
                // switch to a new list when the name of the struct is changed.
                // and we have to be inside this function, already processing the element,
                // in order to see that the name has changed
                return Err(Error::StructNameChanged);
            }
            // Move past the first bracket
            self.de.jump_past("{")?;
        }

        if self.fields[self.index] == "" {
            // An empty field means a list or structure is here. We can just deserialize it
            result = seed.deserialize(&mut *self.de);
        } else {
            // Non empty field means it is a simple "key" "value" setup
            self.de.jump_past("\"")?;
            self.de
                .remove_from_start(&self.fields[self.index])
                .ok_or(Error::ExpectedFieldName)?;
            self.de
                .remove_from_start("\"")
                .ok_or(Error::ExpectedClosingQuote)?;

            self.de.jump_past("\"")?;

            result = seed.deserialize(&mut *self.de);

            self.de.jump_past("\"")?;
        }

        self.index += 1;

        // If we just finished with the last element, clean up the ending curly brace
        if self.index >= self.fields.len() {
            self.de.jump_past("}")?;
        }

        return result.map(Some);
    }
}
