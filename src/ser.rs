use std::io::{self, Write};
use std::fs;
use std::path::{Path, PathBuf};

use std::fmt::Display;

use serde::{Serializer, Serialize};
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct,
                 SerializeTupleVariant, SerializeMap, SerializeStruct,
                 SerializeStructVariant, Impossible};
use serde::ser::Error as SerdeError;

#[derive(Debug, Error)]
pub enum Error {
    IoError(io::Error),
    KeyMustBeAString,
    #[error(non_std, no_from)]
    Custom(String),
}

impl SerdeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg))
    }
}

#[derive(Clone)]
pub struct FilesystemSerializer<P: AsRef<Path>> {
    path: P,
}

impl<P: AsRef<Path>> FilesystemSerializer<P> {
    pub fn new(path: P) -> Self {
        FilesystemSerializer { path }
    }
}

pub struct FilesystemSequenceSerializer<P: AsRef<Path>> {
    path: P,
    counter: usize,
}

impl<P: AsRef<Path>> FilesystemSequenceSerializer<P> {
    pub fn new(path: P) -> Result<Self, Error> {
        if path.as_ref().is_dir() {
            let files = fs::read_dir(path.as_ref())?;
            for file in files.filter(Result::is_ok).map(Result::unwrap) {
                if let Ok(_) = file.file_name().to_str().unwrap().parse::<usize>() {
                    fs::remove_file(file.path())?;
                }
            }
        } else if path.as_ref().is_file() {
            fs::remove_file(path.as_ref())?;
        }
        fs::create_dir_all(path.as_ref())?;
        Ok(FilesystemSequenceSerializer {
            path, counter: 0
        })
    }
}

pub struct FilesystemMapSerializer<P: AsRef<Path>> {
    path: P,
    key: Option<String>,
}

impl<P: AsRef<Path>> FilesystemMapSerializer<P> {
    pub fn new(path: P) -> Result<Self, Error> {
        if path.as_ref().is_file() {
            fs::remove_file(path.as_ref())?;
        }
        fs::create_dir_all(path.as_ref())?;
        Ok(FilesystemMapSerializer {
            path, key: None
        })
    }
}

impl<P: AsRef<Path>> FilesystemSequenceSerializer<P> {

     fn do_serialize_element<T: ? Sized>(&mut self, value: &T) -> Result<(), Error> where
        T: Serialize {
        let result = value.serialize(FilesystemSerializer { path: self.path.as_ref().join(format!("{}", self.counter)) });
        self.counter += 1;
        result
    }

    fn do_end(self) -> Result<(), Error> {
        Ok(())
    }
}


impl<P: AsRef<Path>> SerializeSeq for FilesystemSequenceSerializer<P> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.do_serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.do_end()
    }

}


impl<P: AsRef<Path>> SerializeTuple for FilesystemSequenceSerializer<P> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.do_serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.do_end()
    }

}

impl<P: AsRef<Path>> SerializeTupleStruct for FilesystemSequenceSerializer<P> {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.do_end()
    }

    fn serialize_field<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.do_serialize_element(value)
    }
}

impl<P: AsRef<Path>> SerializeTupleVariant for FilesystemSequenceSerializer<P> {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.do_end()
    }

    fn serialize_field<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.do_serialize_element(value)
    }
}

struct MapKeySerializer;

impl Serializer for MapKeySerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(String::from(v))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_some<T: ? Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_newtype_struct<T: ? Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_newtype_variant<T: ? Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::KeyMustBeAString)
    }

}

impl<P: AsRef<Path>> SerializeMap for FilesystemMapSerializer<P> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ? Sized>(&mut self, key: &T) -> Result<(), Self::Error> where
        T: Serialize {
        let key = key.serialize(MapKeySerializer)?;
        fs::create_dir_all(&self.path)?;
        fs::File::create(&key)?;
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        let key = match self.key.take() {
            None => return Err(Error::KeyMustBeAString),
            Some(key) => key,
        };
        value.serialize(FilesystemSerializer { path: self.path.as_ref().join(key) })
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize {
        let key = key.serialize(MapKeySerializer)?;
        value.serialize(FilesystemSerializer { path: self.path.as_ref().join(key) })
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

}

impl<P: AsRef<Path>> SerializeStruct for FilesystemMapSerializer<P> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ? Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(FilesystemSerializer { path: self.path.as_ref().join(key) })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<P: AsRef<Path>> SerializeStructVariant for FilesystemMapSerializer<P> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ? Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(FilesystemSerializer { path: self.path.as_ref().join(key) })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[inline]
fn write_to_file<P: AsRef<Path>, V: AsRef<[u8]>>(path: P, content: V) -> Result<(), io::Error> {
    if path.as_ref().is_dir() {
        fs::remove_dir_all(path.as_ref())?;
    }
    let mut dir = PathBuf::from(path.as_ref());
    dir.pop();
    fs::create_dir_all(&dir)?;

    let mut file = fs::File::create(path.as_ref())?;
    file.write(content.as_ref())?;
    Ok(())
}

impl<P: AsRef<Path>> Serializer for FilesystemSerializer<P> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = FilesystemSequenceSerializer<P>;
    type SerializeTuple = FilesystemSequenceSerializer<P>;
    type SerializeTupleStruct = FilesystemSequenceSerializer<P>;
    type SerializeTupleVariant = FilesystemSequenceSerializer<PathBuf>;
    type SerializeMap = FilesystemMapSerializer<P>;
    type SerializeStruct = FilesystemMapSerializer<P>;
    type SerializeStructVariant = FilesystemMapSerializer<PathBuf>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            write_to_file(self.path, "true")?;
        } else {
            write_to_file(self.path, "false")?;
        }
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, format!("{}", v))?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        if self.path.as_ref().is_file() {
            fs::remove_file(self.path)?;
        } else if self.path.as_ref().is_dir() {
            fs::remove_dir_all(self.path)?;
        }
        Ok(())
    }

    fn serialize_some<T: ? Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write_to_file(self.path, &[])?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ? Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ? Sized>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        if self.path.as_ref().is_dir() {
            if self.path.as_ref().join("variant").is_file() {
                fs::remove_file(self.path.as_ref().join("variant"))?;
            }
            if self.path.as_ref().join("value").is_file() {
                fs::remove_file(self.path.as_ref().join("value"))?;
            } else if self.path.as_ref().join("value").is_dir() {
                fs::remove_dir_all(self.path.as_ref().join("value"))?;
            }
        } else if self.path.as_ref().is_file() {
            fs::remove_file(self.path.as_ref())?;
        }
        fs::create_dir_all(self.path.as_ref())?;
        write_to_file(self.path.as_ref().join("variant"), variant)?;
        value.serialize(FilesystemSerializer::new(self.path.as_ref().join("value")))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        FilesystemSequenceSerializer::new(self.path)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        FilesystemSequenceSerializer::new(self.path)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        FilesystemSequenceSerializer::new(self.path)
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let serializer = FilesystemSequenceSerializer::new(PathBuf::from(self.path.as_ref()))?;
        write_to_file(self.path.as_ref().join("variant"), variant)?;
        Ok(serializer)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        FilesystemMapSerializer::new(self.path)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        FilesystemMapSerializer::new(self.path)
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        let serializer = FilesystemMapSerializer::new(PathBuf::from(self.path.as_ref()))?;
        write_to_file(self.path.as_ref().join("variant"), variant)?;
        Ok(serializer)
    }

}

#[cfg(test)]
#[allow(dead_code)]
mod tests {

    use std::fs;
    use std::io::Read;
    use std::path::Path;

    use super::*;
    use tempdir::TempDir;

    fn file_to_string<P: AsRef<Path>>(path: P) -> String {
        let mut file = fs::File::open(path).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        s
    }

    #[test]
    fn boolean() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("bool"),
        };
        true.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("bool")), "true");
        false.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("bool")), "false");
    }

    #[test]
    fn numbers() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("n"),
        };
        (-1i8).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "-1");
        (1u8).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "1");
        (-1i16).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "-1");
        (1u16).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "1");
        (-1i32).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "-1");
        (1u32).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "1");
        (-1i64).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "-1");
        (1u64).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "1");
        (1.3f32).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "1.3");
        (1.31f64).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("n")), "1.31");
    }

    #[test]
    fn char() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("c"),
        };
        '!'.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("c")), "!");
    }

    #[test]
    fn string() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("str"),
        };
        "hello".serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("str")), "hello");
    }

    #[test]
    fn bytes() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("bytes"),
        };
        use serde_bytes::Bytes;
        Bytes::new(b"hello").serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("bytes")), "hello");
    }

    #[test]
    fn option() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("option"),
        };
        Some("hello").serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("option")), "hello");
        None::<Option<&str>>.serialize(serializer.clone()).unwrap();
        assert!(!tmp.path().join("option").exists());
    }

    #[test]
    fn unit() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("unit"),
        };
        ().serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("unit")), "");
    }

    #[derive(Serialize)]
    struct UnitStruct;

    #[test]
    fn unit_struct() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("unit"),
        };
        UnitStruct.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("unit")), "");
    }

    #[derive(Serialize)]
    enum UnitVariant {
        A,
        #[serde(rename = "b")]
        B,
        C,
    }

    #[test]
    fn unit_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("unit"),
        };
        UnitVariant::B.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("unit")), "b");
    }

    #[derive(Serialize)]
    enum VariantNewType {
        C(u8),
    }

    #[test]
    fn tuple_newtype_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("var"),
        };
        VariantNewType::C(100).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("var").join("variant")), "C");
        assert_eq!(file_to_string(tmp.path().join("var").join("value")), "100");
    }

    #[test]
    fn tuple_newtype_variant_keep_aux_files() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("var"),
        };
        fs::create_dir_all(tmp.path().join("var")).unwrap();
        write_to_file(tmp.path().join("var").join("README.md"), "Hello!").unwrap();
        VariantNewType::C(100).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("var").join("variant")), "C");
        assert_eq!(file_to_string(tmp.path().join("var").join("value")), "100");
        assert_eq!(file_to_string(tmp.path().join("var").join("README.md")), "Hello!");
    }

    #[derive(Serialize)]
    #[serde(tag = "type", content = "content")]
    enum VariantNewTypeTag {
        C(u8),
    }

    #[test]
    fn tuple_newtype_variant_tag() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("var"),
        };
        VariantNewTypeTag::C(100).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("var").join("type")), "C");
        assert_eq!(file_to_string(tmp.path().join("var").join("content")), "100");
    }

    #[derive(Serialize)]
    struct NewTypeStruct(u8);

    #[test]
    fn newtype_struct() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("newtype"),
        };
        NewTypeStruct(100).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("newtype")), "100");
    }

    #[derive(Serialize)]
    struct NewTypeVariant(UnitVariant);

    #[test]
    fn newtype_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("newtype"),
        };
        NewTypeVariant(UnitVariant::B).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("newtype")), "b");
    }

    #[derive(Serialize)]
    struct NewTypeTupleVariant(TupleVariant);

    #[test]
    fn newtype_tuple_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("newtype"),
        };
        let v = NewTypeTupleVariant(TupleVariant::V1(100, 100, 100));
        v.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("newtype").join("variant")), "V1");
        assert_eq!(file_to_string(tmp.path().join("newtype").join("0")), "100");
        assert_eq!(file_to_string(tmp.path().join("newtype").join("1")), "100");
        assert_eq!(file_to_string(tmp.path().join("newtype").join("2")), "100");
    }

    #[test]
    fn seq() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("seq"),
        };
        vec![100,200,300].serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("seq").join("0")), "100");
        assert_eq!(file_to_string(tmp.path().join("seq").join("1")), "200");
        assert_eq!(file_to_string(tmp.path().join("seq").join("2")), "300");
    }

    #[test]
    fn seq_keep_aux_files() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("seq"),
        };
        fs::create_dir_all(tmp.path().join("seq")).unwrap();
        write_to_file(tmp.path().join("seq").join("README.md"), "Hello!").unwrap();
        vec![100,200,300].serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("seq").join("README.md")), "Hello!");
    }

    #[test]
    fn seq_empty() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("seq"),
        };
        Vec::<u8>::new().serialize(serializer.clone()).unwrap();
        assert!(tmp.path().join("seq").is_dir());
    }

    #[test]
    fn seq_shrink() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("seq"),
        };
        vec![100,200,300].serialize(serializer.clone()).unwrap();
        vec![100,200].serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("seq").join("0")), "100");
        assert_eq!(file_to_string(tmp.path().join("seq").join("1")), "200");
        assert!(!tmp.path().join("seq").join("2").exists());
    }


    #[test]
    fn tuple() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("tuple"),
        };
        (100,200,300).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("tuple").join("0")), "100");
        assert_eq!(file_to_string(tmp.path().join("tuple").join("1")), "200");
        assert_eq!(file_to_string(tmp.path().join("tuple").join("2")), "300");
    }


    #[derive(Serialize)]
    struct TupleStruct(u8, u8, u8);

    #[test]
    fn tuple_struct() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("tuple"),
        };
        TupleStruct(100,200,100).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("tuple").join("0")), "100");
        assert_eq!(file_to_string(tmp.path().join("tuple").join("1")), "200");
        assert_eq!(file_to_string(tmp.path().join("tuple").join("2")), "100");
    }

    #[derive(Serialize)]
    enum TupleVariant {
        V1(u8, u8, u8),
    }

    #[test]
    fn tuple_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("tuple"),
        };
        TupleVariant::V1(100,200,100).serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("tuple").join("variant")), "V1");
        assert_eq!(file_to_string(tmp.path().join("tuple").join("0")), "100");
        assert_eq!(file_to_string(tmp.path().join("tuple").join("1")), "200");
        assert_eq!(file_to_string(tmp.path().join("tuple").join("2")), "100");
    }

    #[test]
    fn map() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("map"),
        };
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("test", 100);
        map.insert("passed", 2100);
        map.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("map").join("test")), "100");
        assert_eq!(file_to_string(tmp.path().join("map").join("passed")), "2100");
    }

    #[test]
    fn map_keep_aux_files() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("map"),
        };
        fs::create_dir_all(tmp.path().join("map")).unwrap();
        write_to_file(tmp.path().join("map").join("README.md"), "Hello!").unwrap();
        vec![100,200,300].serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("map").join("README.md")), "Hello!");
    }


    #[test]
    fn map_extra() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("map"),
        };
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("test", 100);
        map.insert("passed", 2100);
        map.serialize(serializer.clone()).unwrap();
        let mut map = HashMap::new();
        map.insert("test", 100);
        map.serialize(serializer.clone()).unwrap();
        assert!(!tmp.path().join("passed").exists());
    }


    macro_rules! key_must_be_a_string_check {
        ($ty: ident, $serializer: expr) => {{
           use std::collections::HashMap;
           let mut map = HashMap::new();
           map.insert($ty::default(), $ty::default());
           let err = map.serialize($serializer.clone()).unwrap_err();
           assert_matches!(err, Error::KeyMustBeAString);
        }};
    }

    #[test]
    fn map_invalid_key() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("map"),
        };
        key_must_be_a_string_check!(i8, serializer);
        key_must_be_a_string_check!(u8, serializer);
        key_must_be_a_string_check!(i16, serializer);
        key_must_be_a_string_check!(u16, serializer);
        key_must_be_a_string_check!(i32, serializer);
        key_must_be_a_string_check!(u32, serializer);
        key_must_be_a_string_check!(i64, serializer);
        key_must_be_a_string_check!(u64, serializer);
        key_must_be_a_string_check!(bool, serializer);
        key_must_be_a_string_check!(Struct, serializer);
    }

    #[derive(Serialize, PartialEq, Eq, Hash, Default)]
    struct Struct {
        test: u8,
        passed: u64,
    }

    #[test]
    fn structure() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("struct"),
        };
        Struct {
            test: 100,
            passed: 2100,
        }.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("struct").join("test")), "100");
        assert_eq!(file_to_string(tmp.path().join("struct").join("passed")), "2100");
    }

    #[test]
    fn structure_keep_aux_files() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("struct"),
        };
        fs::create_dir_all(tmp.path().join("struct")).unwrap();
        write_to_file(tmp.path().join("struct").join("README.md"), "Hello!").unwrap();
        Struct {
            test: 100,
            passed: 2100,
        }.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("struct").join("README.md")), "Hello!");
    }

    #[derive(Serialize)]
    enum StructVariant {
        V1 {
            test: u8,
            passed: u64,
        }
    }

    #[test]
    fn struct_variant() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("struct"),
        };
        StructVariant::V1 {
            test: 100,
            passed: 2100,
        }.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("struct").join("variant")), "V1");
        assert_eq!(file_to_string(tmp.path().join("struct").join("test")), "100");
        assert_eq!(file_to_string(tmp.path().join("struct").join("passed")), "2100");
    }

    #[derive(Serialize)]
    #[serde(tag = "type")]
    enum StructVariantTag {
        V1 {
            test: u8,
            passed: u64,
        }
    }

    #[test]
    fn struct_variant_tag() {
        let tmp = TempDir::new("serde-fs").unwrap();
        let serializer = FilesystemSerializer {
            path: tmp.path().join("struct"),
        };
        StructVariantTag::V1 {
            test: 100,
            passed: 2100,
        }.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("struct").join("type")), "V1");
        assert_eq!(file_to_string(tmp.path().join("struct").join("test")), "100");
        assert_eq!(file_to_string(tmp.path().join("struct").join("passed")), "2100");
    }

    #[derive(Serialize)]
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
        let serializer = FilesystemSerializer {
            path: tmp.path().join("struct"),
        };
        StructVariantTagContent::V1 {
            test: 100,
            passed: 2100,
        }.serialize(serializer.clone()).unwrap();
        assert_eq!(file_to_string(tmp.path().join("struct").join("type")), "V1");
        assert_eq!(file_to_string(tmp.path().join("struct").join("content").join("test")), "100");
        assert_eq!(file_to_string(tmp.path().join("struct").join("content").join("passed")), "2100");
    }


}