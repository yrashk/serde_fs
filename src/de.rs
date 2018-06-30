use std::io::{self, Read};
use std::fs;
use std::path::Path;
use std::fmt::Display;

use serde::Deserializer;
use serde::de::{self, Visitor, Error as SerdeError};

#[derive(Debug, Error)]
pub enum Error {
    IoError(io::Error),
    ParseBoolError(::std::str::ParseBoolError),
    ParseIntError(::std::num::ParseIntError),
    ParseFloatError(::std::num::ParseFloatError),
    Empty,
    FileNotFound,
    Unsupported,
    #[error(non_std, no_from)]
    InvalidLen {
        expected: usize,
        got: usize,
    },
    #[error(non_std, no_from)]
    InvalidEnum(String),
    #[error(non_std, no_from)]
    Custom(String),
}

impl SerdeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg))
    }
}

#[derive(Clone)]
pub struct FilesystemDeserializer<P: AsRef<Path>> {
    path: P,
}

impl<P: AsRef<Path>> FilesystemDeserializer<P> {
    pub fn new(path: P) -> Self {
        FilesystemDeserializer { path }
    }
}

fn string_from_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn bytes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut b = vec![];
    file.read_to_end(&mut b)?;
    Ok(b)
}


impl<'de, P: AsRef<Path>> Deserializer<'de> for FilesystemDeserializer<P> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        if self.path.as_ref().is_dir() {
            self.deserialize_map(visitor)
        } else if self.path.as_ref().is_file() {
            visitor.visit_str(&string_from_file(self.path)?)
        } else {
            Err(Error::FileNotFound)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_bool(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_i8(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_i16(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_i32(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_i64(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_u8(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_u16(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_u32(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_u64(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_f32(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_f64(string_from_file(self.path)?.trim().parse()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        let file = fs::File::open(self.path)?;
        // FIXME: replace with .chars() if it will become available:
        // https://github.com/rust-lang/rust/issues/27802
        // or an alternative solution
        let mut bytes = file.bytes();
        match bytes.next() {
            None => Err(Error::Empty),
            Some(c) => visitor.visit_char(c?.into()),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_str(&string_from_file(self.path)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_string(string_from_file(self.path)?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_bytes(&bytes_from_file(self.path)?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_byte_buf(bytes_from_file(self.path)?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        if !self.path.as_ref().exists() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        if self.path.as_ref().is_file() {
            visitor.visit_unit()
        } else {
            Err(Error::FileNotFound)
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        let seq_access = SeqAccess { path: self.path, counter: 0, len: None };
        visitor.visit_seq(seq_access)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        let seq_access = SeqAccess { path: self.path, counter: 0, len: Some(len) };
        visitor.visit_seq(seq_access)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_map(MapAccess::new(self.path)?)
    }

    fn deserialize_struct<V>(self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_map(MapAccess::new_for(self.path, fields.into_iter().map(|f| Ok(String::from(*f))))?)
    }

    fn deserialize_enum<V>(self, _name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        use serde::de::IntoDeserializer;
        let variant = if self.path.as_ref().is_dir() {
            string_from_file(self.path.as_ref().join("variant"))?
        } else {
            string_from_file(self.path.as_ref())?
        };
        for available_variant in variants {
           if available_variant == &&variant {
               if self.path.as_ref().is_dir() {
                   // not a unit enum
                   return visitor.visit_enum(VariantAccess { path: self.path, variant });
               } else {
                   return visitor.visit_enum(variant.into_deserializer());
               }
           }
        }
        Err(Error::InvalidEnum(variant))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_str(&string_from_file(self.path)?)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_unit()
    }
}

struct SeqAccess<P: AsRef<Path>> {
    path: P,
    counter: usize,
    len: Option<usize>,
}

impl<'de, P: AsRef<Path>> de::SeqAccess<'de> for SeqAccess<P> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where
        T: de::DeserializeSeed<'de> {
        let path = self.path.as_ref().join(format!("{}", self.counter));
        if path.exists() {
            self.counter += 1;
            Ok(Some(seed.deserialize(FilesystemDeserializer::new(path))?))
        } else {
            if self.len.is_some() && self.len.clone().unwrap() != self.counter {
                Err(Error::InvalidLen {
                    expected: self.len.clone().unwrap(),
                    got: self.counter,
                })
            } else {
                Ok(None)
            }
        }
    }
}


struct VariantAccess<P: AsRef<Path>> {
    path: P,
    variant: String,
}

impl<'a, 'de, P: AsRef<Path>> de::VariantAccess<'de> for VariantAccess<P> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error> where
        T: de::DeserializeSeed<'de> {
       seed.deserialize(FilesystemDeserializer::new(self.path.as_ref().join("value")))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        FilesystemDeserializer::new(self.path.as_ref()).deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        FilesystemDeserializer::new(self.path.as_ref()).deserialize_struct("", fields, visitor)
    }
}


impl<'a, 'de, P: AsRef<Path>> de::EnumAccess<'de> for VariantAccess<P> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> where
        V: de::DeserializeSeed<'de> {
        use serde::de::IntoDeserializer;
        use serde::de::value;
        let deserializer: value::StringDeserializer<Self::Error> = self.variant.clone().into_deserializer();
        Ok((seed.deserialize(deserializer)?, self))
    }
}

struct MapAccess<P: AsRef<Path>> {
    path: P,
    dir: Box<Iterator<Item = Result<String, io::Error>>>,
    key: Option<String>,
}

impl<P: AsRef<Path>> MapAccess<P> {
    fn new(path: P) -> Result<Self, Error> {
        let dir = Box::new(fs::read_dir(path.as_ref())?
            .map(|res| match res {
                Ok(file) => Ok(file.file_name().into_string().unwrap()),
                Err(err) => Err(err),
            }));
        Ok(MapAccess {
            path,
            dir,
            key: None,
        })
    }
    fn new_for<I: Iterator<Item = Result<String, io::Error>> + 'static>(path: P, iter: I) -> Result<Self, Error>
    {
        Ok(MapAccess {
            path,
            dir: Box::new(iter),
            key: None,
        })
    }
}

impl<'de, P: AsRef<Path>> de::MapAccess<'de> for MapAccess<P> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> where
        K: de::DeserializeSeed<'de> {
        use serde::de::IntoDeserializer;
        use serde::de::value;
        match self.dir.next() {
            Some(Ok(name)) => {
                self.key = Some(name.clone());
                let deserializer: value::StringDeserializer<Self::Error> = name.into_deserializer();
                Ok(Some(seed.deserialize(deserializer)?))
            },
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error> where
        V: de::DeserializeSeed<'de> {
        match self.key.take() {
            None => Err(Error::FileNotFound),
            Some(key) =>
              seed.deserialize(FilesystemDeserializer::new(self.path.as_ref().join(key))),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use super::super::FilesystemSerializer;
    use tempdir::TempDir;

    use serde::{Serialize, Deserialize};

    #[test]
    fn boolean() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("bool"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("bool"),
        };
        true.serialize(serializer.clone()).unwrap();
        assert_eq!(bool::deserialize(deserializer.clone()).unwrap(), true);
        false.serialize(serializer.clone()).unwrap();
        assert_eq!(bool::deserialize(deserializer.clone()).unwrap(), false);
    }

    #[test]
    fn boolean_extra_space() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("bool"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("bool"),
        };
        " true \n".serialize(serializer.clone()).unwrap();
        assert_eq!(bool::deserialize(deserializer.clone()).unwrap(), true);
        " false \n".serialize(serializer.clone()).unwrap();
        assert_eq!(bool::deserialize(deserializer.clone()).unwrap(), false);
    }

    #[test]
    fn numbers() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("n"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("n"),
        };
        (-1i8).serialize(serializer.clone()).unwrap();
        assert_eq!(i8::deserialize(deserializer.clone()).unwrap(), -1);
        (1u8).serialize(serializer.clone()).unwrap();
        assert_eq!(u8::deserialize(deserializer.clone()).unwrap(), 1);
        (-1i16).serialize(serializer.clone()).unwrap();
        assert_eq!(i16::deserialize(deserializer.clone()).unwrap(), -1);
        (1u16).serialize(serializer.clone()).unwrap();
        assert_eq!(u16::deserialize(deserializer.clone()).unwrap(), 1);
        (-1i32).serialize(serializer.clone()).unwrap();
        assert_eq!(i32::deserialize(deserializer.clone()).unwrap(), -1);
        (1u32).serialize(serializer.clone()).unwrap();
        assert_eq!(u32::deserialize(deserializer.clone()).unwrap(), 1);
        (-1i64).serialize(serializer.clone()).unwrap();
        assert_eq!(i64::deserialize(deserializer.clone()).unwrap(), -1);
        (1u64).serialize(serializer.clone()).unwrap();
        assert_eq!(u64::deserialize(deserializer.clone()).unwrap(), 1);
        (1.3f32).serialize(serializer.clone()).unwrap();
        assert_eq!(f32::deserialize(deserializer.clone()).unwrap(), 1.3);
        (1.31f64).serialize(serializer.clone()).unwrap();
        assert_eq!(f64::deserialize(deserializer.clone()).unwrap(), 1.31);
    }

    #[test]
    fn numbers_extra_space() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("n"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("n"),
        };
        " -1 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(i8::deserialize(deserializer.clone()).unwrap(), -1);
        " 1 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(u8::deserialize(deserializer.clone()).unwrap(), 1);
        " -1 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(i16::deserialize(deserializer.clone()).unwrap(), -1);
        " 1 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(u16::deserialize(deserializer.clone()).unwrap(), 1);
        " -1 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(i32::deserialize(deserializer.clone()).unwrap(), -1);
        " 1 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(u32::deserialize(deserializer.clone()).unwrap(), 1);
        " -1 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(i64::deserialize(deserializer.clone()).unwrap(), -1);
        " 1 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(u64::deserialize(deserializer.clone()).unwrap(), 1);
        " 1.3 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(f32::deserialize(deserializer.clone()).unwrap(), 1.3);
        " 1.31 \n".serialize(serializer.clone()).unwrap();
        assert_eq!(f64::deserialize(deserializer.clone()).unwrap(), 1.31);
    }


    #[test]
    fn char() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("c"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("c"),
        };
        '!'.serialize(serializer.clone()).unwrap();
        assert_eq!(char::deserialize(deserializer.clone()).unwrap(), '!');
    }

    #[test]
    fn char_empty() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("c"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("c"),
        };
        "".serialize(serializer.clone()).unwrap();
        let err = char::deserialize(deserializer.clone()).unwrap_err();
        assert_matches!(err, Error::Empty);
    }

    #[test]
    fn string() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("str"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("str"),
        };
        "hello".serialize(serializer.clone()).unwrap();
        assert_eq!(String::deserialize(deserializer.clone()).unwrap(), "hello");
    }

    #[test]
    fn bytes() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("bytes"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("bytes"),
        };
        use serde_bytes::{Bytes, ByteBuf};
        Bytes::new(b"hello").serialize(serializer.clone()).unwrap();
        let byte_buf = ByteBuf::deserialize(deserializer.clone()).unwrap();
        let buf: &[u8] = byte_buf.as_ref();
        assert_eq!(buf, b"hello");
    }

    #[test]
    fn option() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("option"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("option"),
        };
        Some("hello").serialize(serializer.clone()).unwrap();
        assert_eq!(Option::<String>::deserialize(deserializer.clone()).unwrap().unwrap(), "hello");
        None::<Option<&str>>.serialize(serializer.clone()).unwrap();
        assert!(Option::<String>::deserialize(deserializer.clone()).unwrap().is_none());
    }

    #[test]
    fn unit() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("unit"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("unit"),
        };
        ().serialize(serializer.clone()).unwrap();
        assert_eq!(<()>::deserialize(deserializer.clone()).unwrap(), ());
    }

    #[test]
    fn unit_not_found() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("unit"),
        };
        let err = <()>::deserialize(deserializer.clone()).unwrap_err();
        assert_matches!(err, Error::FileNotFound);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct UnitStruct;

    #[test]
    fn unit_struct() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("unit"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("unit"),
        };
        UnitStruct.serialize(serializer.clone()).unwrap();
        assert_eq!(UnitStruct::deserialize(deserializer.clone()).unwrap(), UnitStruct);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum UnitVariant {
        A,
        #[serde(rename = "b")]
        B,
        C,
    }

    #[test]
    fn unit_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("unit"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("unit"),
        };
        UnitVariant::B.serialize(serializer.clone()).unwrap();
        assert_eq!(UnitVariant::deserialize(deserializer.clone()).unwrap(), UnitVariant::B);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum UnitVariant1 {
        A,
        B,
        C,
    }

    #[test]
    fn unit_wrong_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("unit"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("unit"),
        };
        UnitVariant::B.serialize(serializer.clone()).unwrap();
        let err = UnitVariant1::deserialize(deserializer.clone()).unwrap_err();
        assert_matches!(err, Error::InvalidEnum(ref str) if str == "b");
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct NewTypeVariant(UnitVariant);

    #[test]
    fn newtype_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("newtype"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("newtype"),
        };
        NewTypeVariant(UnitVariant::B).serialize(serializer.clone()).unwrap();
        assert_eq!(NewTypeVariant::deserialize(deserializer.clone()).unwrap(), NewTypeVariant(UnitVariant::B));
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct NewTypeStruct(u8);

    #[test]
    fn newtype_struct() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("newtype"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("newtype"),
        };
        NewTypeStruct(100).serialize(serializer.clone()).unwrap();
        assert_eq!(NewTypeStruct::deserialize(deserializer.clone()).unwrap(), NewTypeStruct(100));
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct NewTypeTupleVariant(TupleVariant);

    #[test]
    fn newtype_tuple_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("newtype"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("newtype"),
        };
        let v = NewTypeTupleVariant(TupleVariant::V1(100, 100, 100));
        v.serialize(serializer.clone()).unwrap();
        assert_eq!(NewTypeTupleVariant::deserialize(deserializer.clone()).unwrap(), v);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum VariantNewType {
        C(u8),
    }

    #[test]
    fn tuple_newtype_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("var"));
         let deserializer = FilesystemDeserializer {
            path: tmp.path().join("var"),
        };
        VariantNewType::C(100).serialize(serializer.clone()).unwrap();
        assert_eq!(VariantNewType::deserialize(deserializer.clone()).unwrap(), VariantNewType::C(100));
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(tag = "type", content = "content")]
    enum VariantNewTypeTag {
        C(u8),
    }

    #[test]
    fn tuple_newtype_variant_tag() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("var"));
         let deserializer = FilesystemDeserializer {
            path: tmp.path().join("var"),
        };
        VariantNewTypeTag::C(100).serialize(serializer.clone()).unwrap();
        assert_eq!(VariantNewTypeTag::deserialize(deserializer.clone()).unwrap(), VariantNewTypeTag::C(100));
    }

    #[test]
    fn seq() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("seq"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("seq"),
        };
        vec![100,200,300].serialize(serializer.clone()).unwrap();
        assert_eq!(Vec::<u16>::deserialize(deserializer.clone()).unwrap(), vec![100, 200, 300]);
    }

    #[test]
    fn seq_complex_elem() {
        #[derive(Debug, Deserialize, PartialEq, Serialize)]
        struct Complex {
            something: u32,
            stuff: u32,
        }

        impl Complex {
            fn new(x: u32) -> Complex {
                Complex {
                    something: x,
                    stuff: x,
                }
            }
        }

        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("seq-complex-elem"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("seq-complex-elem"),
        };

        let value = vec![
            Complex::new(0),
            Complex::new(1),
            Complex::new(2),
        ];
        value.serialize(serializer.clone()).unwrap();
        assert_eq!(Vec::<Complex>::deserialize(deserializer.clone()).unwrap(), value);
    }

    #[test]
    fn tuple() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("tuple"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("tuple"),
        };
        (100,200,300).serialize(serializer.clone()).unwrap();
        assert_eq!(<(u8, u8, u16)>::deserialize(deserializer.clone()).unwrap(), (100, 200, 300));
    }

    #[test]
    fn tuple_size_mismatch() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("tuple"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("tuple"),
        };
        (100,200).serialize(serializer.clone()).unwrap();
        let err = <(u8, u8, u16)>::deserialize(deserializer.clone()).unwrap_err();
        assert_matches!(err, Error::InvalidLen { expected: 3, got: 2 });
    }

    #[test]
    fn tuple_type_mismatch() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("tuple"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("tuple"),
        };
        (100,"hello").serialize(serializer.clone()).unwrap();
        let err = <(u8, u8)>::deserialize(deserializer.clone()).unwrap_err();
        assert_matches!(err, Error::ParseIntError(_));
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TupleStruct(u8, u8, u8);

    #[test]
    fn tuple_struct() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("tuple"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("tuple"),
        };
        TupleStruct(100,200,100).serialize(serializer.clone()).unwrap();
        assert_eq!(TupleStruct::deserialize(deserializer.clone()).unwrap(), TupleStruct(100, 200, 100));
    }

    #[test]
    fn tuple_struct_type_mismatch() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("tuple"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("tuple"),
        };
        (100,"hello", 100).serialize(serializer.clone()).unwrap();
        let err = TupleStruct::deserialize(deserializer.clone()).unwrap_err();
        assert_matches!(err, Error::ParseIntError(_));
    }


    #[test]
    fn tuple_struct_size_mismatch() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("tuple"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("tuple"),
        };
        (100,200).serialize(serializer.clone()).unwrap();
        let err = TupleStruct::deserialize(deserializer.clone()).unwrap_err();
        assert_matches!(err, Error::InvalidLen { expected: 3, got: 2 });
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum TupleVariant {
        V1(u8, u8, u8),
    }

    #[test]
    fn tuple_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("tuple"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("tuple"),
        };
        TupleVariant::V1(100,200,100).serialize(serializer.clone()).unwrap();
        assert_eq!(TupleVariant::deserialize(deserializer.clone()).unwrap(), TupleVariant::V1(100, 200, 100));
    }

    #[test]
    fn map() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("map"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("map"),
        };
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("test".into(), 100);
        map.insert("passed".into(), 2100);
        map.serialize(serializer.clone()).unwrap();
        assert_eq!(HashMap::<String, i32>::deserialize(deserializer.clone()).unwrap(), map);
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Default, Debug)]
    struct Struct {
        test: u8,
        passed: u64,
    }

    #[test]
    fn structure() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("struct"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("struct"),
        };
        let s = Struct {
            test: 100,
            passed: 2100,
        };
        s.serialize(serializer.clone()).unwrap();
        assert_eq!(Struct::deserialize(deserializer.clone()).unwrap(), s);
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Default, Debug)]
    struct Struct1 {
        test: u8,
        passed: u64,
        extra: u8,
    }

    #[test]
    fn structure_extra() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("struct"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("struct"),
        };
        let s = Struct1 {
            test: 100,
            passed: 2100,
            extra: 10,
        };
        s.serialize(serializer.clone()).unwrap();
        assert_eq!(Struct::deserialize(deserializer.clone()).unwrap(), Struct { test: 100, passed: 2100 });
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum StructVariant {
        V1 {
            test: u8,
            passed: u64,
        }
    }

    #[test]
    fn struct_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("struct"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("struct"),
        };
        let s = StructVariant::V1 {
            test: 100,
            passed: 2100,
        };
        s.serialize(serializer.clone()).unwrap();
        assert_eq!(StructVariant::deserialize(deserializer.clone()).unwrap(), s);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(tag = "type", content = ".")]
    enum StructVariantTagInternal {
        V1 {
            test: u8,
            passed: u64,
        }
    }

    #[test]
    fn struct_variant_tag_internal() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("struct"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("struct"),
        };
        let s = StructVariantTagInternal::V1 {
            test: 100,
            passed: 2100,
        };
        s.serialize(serializer.clone()).unwrap();
        assert_eq!(StructVariantTagInternal::deserialize(deserializer.clone()).unwrap(), s);
    }

    // The test below is currently expected to fail as there
    // is no known solution to the issue. The test above (with content set to ".")
    // replicates the intended behaviour of this one and works. This one doesn't
    // See issue e8310019-9017-4ce8-9397-800ca44300ce
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(tag = "type")]
    enum StructVariantTag {
        V1 {
            test: u8,
            passed: u64,
        }
    }

    #[test]
    #[should_panic(expected = "invalid type")]
    fn struct_variant_tag() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("struct"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("struct"),
        };
        let s = StructVariantTag::V1 {
            test: 100,
            passed: 2100,
        };
        s.serialize(serializer.clone()).unwrap();
        assert_eq!(StructVariantTag::deserialize(deserializer.clone()).unwrap(), s);
    }



    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(tag = "type", content = "content")]
    enum StructVariantTagContent {
        V1 {
            test: u8,
            passed: u64,
        }
    }

    #[test]
    fn struct_variant_tag_content() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer::new(tmp.path().join("struct"));
        let deserializer = FilesystemDeserializer {
            path: tmp.path().join("struct"),
        };
        let s = StructVariantTagContent::V1 {
            test: 100,
            passed: 2100,
        };
        s.serialize(serializer.clone()).unwrap();
        assert_eq!(StructVariantTagContent::deserialize(deserializer.clone()).unwrap(), s);
    }

}