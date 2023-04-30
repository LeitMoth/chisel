use serde::{ser, Serialize};

use crate::vmf::error::{Error, Result};

pub struct Serializer {
    output: String,
    indent: u8,
    buffer: Vec<String>,
    last_field_was_struct: bool,
}

impl Serializer {
    fn add_padded_line(&mut self, text: &str) {
        self.pad();
        self.output += text;
        self.output += "\r\n";
    }
    fn pad(&mut self) {
        for _ in 0..self.indent {
            self.output += "\t";
        }
    }
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        indent: 0,
        buffer: Vec::new(),
        last_field_was_struct: false,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

macro_rules! serialize {
    ($ser: ident, $t: ty) => {
        fn $ser(self, v: $t) -> Result<()> {
            self.buffer.push(v.to_string());
            self.last_field_was_struct = false;
            // self.output += &v.to_string();
            Ok(())
        }
    };
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    // Unimplemented

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    // Implementation Starts here

    serialize! {serialize_i8 , i8 }
    serialize! {serialize_i16, i16}
    serialize! {serialize_i32, i32}
    serialize! {serialize_i64, i64}

    serialize! {serialize_u8 , u8 }
    serialize! {serialize_u16, u16}
    serialize! {serialize_u32, u32}
    serialize! {serialize_u64, u64}

    serialize! {serialize_f32, f32}
    serialize! {serialize_f64, f64}

    serialize! {serialize_char, char}

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.buffer.push(if v { "1" } else { "0" }.to_string());
        self.last_field_was_struct = false;
        // self.output += if v { "1" } else { "0" };
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.buffer.push(v.to_string());
        self.last_field_was_struct = false;
        // self.output += v;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        self.add_padded_line(name);
        self.add_padded_line("{");
        self.add_padded_line("}");
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.buffer.push("".to_string());
        self.last_field_was_struct = false;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.add_padded_line(name);
        self.add_padded_line("{");
        self.indent += 1;
        self.last_field_was_struct = false;
        Ok(self)
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        /*
         * We use an empty key to signify that we just want the value serialized, without any surrounding quotes
         * This is how the VMF format chooses to represent nested strucks, and arrays of nested structs.
         *
         * It is a bit hacky, but as long as I mark all up all Vec's with #[serde(rename="")] they will be serialized correctly
         * Deserialization was a bit tricky because of the ambiguity
         */
        // if key != "" {
        //     self.pad();
        //     self.output += "\"";
        //     key.serialize(&mut **self)?;

        //     self.output += "\" \"";

        //     value.serialize(&mut **self)?;
        //     self.output += "\"\r\n";
        // } else {
        //     value.serialize(&mut **self)?;
        // }

        /*
        Maybe push these two below things in to
        a vec: Vec<Pair<String,String>>.
        But hold off until we call the finialize method

        and maybe when we are ending a struct, we can set a boolean
        to true, that finalize can pick up on and display these correctly
         */

        // key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;

        todo!(); // Maybe I add like a do_next() method, that holds a buffer and lags behind. Then we can control easier and adjust after the fact.

        if self.last_field_was_struct {
            self.output += &self.buffer.pop().unwrap();
        } else {
            self.pad();
            self.output += "\"";
            self.output += &key;

            self.output += "\" \"";
            self.output += &self.buffer.pop().unwrap();

            value.serialize(&mut **self)?;
            self.output += "\"\r\n";
        }

        self.last_field_was_struct = false;

        Ok(())
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        // bool that_was_just_a_struct = true
        self.last_field_was_struct = true;
        self.indent -= 1;
        self.add_padded_line("}");
        Ok(())
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    /*
     * This is only intended for top-level use.
     * Trying to make a tuple of values instead of a tuple of structs just mushes all the values together, and is not what you want.
     */

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// Everything below this point is unimplemented

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

// May be needed for entities
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}
