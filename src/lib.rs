//! # Serde Serialization/deserialization to and from the file system
//!
extern crate serde;
#[cfg(test)] #[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate serde_bytes;
#[macro_use] extern crate derive_error;

#[cfg(test)] extern crate tempdir;
#[cfg(test)] #[macro_use] extern crate assert_matches;

use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

mod ser;
pub use ser::FilesystemSerializer;
pub use ser::Error as SerializerError;

/// Serializes a value to a file system
///
/// ```
/// extern crate tempdir;
/// use tempdir::TempDir;
/// #[macro_use] extern crate serde_derive;
///
/// #[derive(Serialize, Deserialize, Debug, PartialEq)]
/// struct MyStruct {
///   value: String,
/// }
///
/// extern crate serde_fs;
/// use serde_fs::{from_fs, to_fs};
///
/// fn main() {
///   let val = MyStruct { value: "Hello".into() };
///   let tmp = TempDir::new("serde_fs").unwrap();
///   to_fs(tmp.path(), &val).unwrap();
///   let val1: MyStruct = from_fs(tmp.path()).unwrap();
///   assert_eq!(val1, val);
/// }
///
/// ```
pub fn to_fs<T: ?Sized, P>(path: P, value: &T) -> Result<(), SerializerError>
    where T: Serialize, P: AsRef<Path> {
    let serializer = FilesystemSerializer::new(path);
    value.serialize(serializer)
}

mod de;
pub use de::FilesystemDeserializer;
pub use de::Error as DeserializerError;

/// Deserializes a value from a file system
///
/// ```
/// extern crate tempdir;
/// use tempdir::TempDir;
/// #[macro_use] extern crate serde_derive;
///
/// #[derive(Serialize, Deserialize, Debug, PartialEq)]
/// struct MyStruct {
///   value: String,
/// }
///
/// extern crate serde_fs;
/// use serde_fs::{from_fs, to_fs};
///
/// fn main() {
///   let val = MyStruct { value: "Hello".into() };
///   let tmp = TempDir::new("serde_fs").unwrap();
///   to_fs(tmp.path(), &val).unwrap();
///   let val1: MyStruct = from_fs(tmp.path()).unwrap();
///   assert_eq!(val1, val);
/// }
///
/// ```
pub fn from_fs<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T, DeserializerError> {
    let deserializer = FilesystemDeserializer::new(path);
    T::deserialize(deserializer)
}

/// Deserializes a value from a file system in place
///
/// ```
/// extern crate tempdir;
/// use tempdir::TempDir;
/// #[macro_use] extern crate serde_derive;
///
/// #[derive(Serialize, Deserialize, Debug, PartialEq)]
/// struct MyStruct {
///   value: String,
/// }
///
/// extern crate serde_fs;
/// use serde_fs::{from_fs_in_place, to_fs};
///
/// fn main() {
///   let val = MyStruct { value: "Hello".into() };
///   let tmp = TempDir::new("serde_fs").unwrap();
///   to_fs(tmp.path(), &val).unwrap();
///   let mut val1 = MyStruct { value: "Nothing here".into() };
///   from_fs_in_place(tmp.path(), &mut val1).unwrap();
///   assert_eq!(val1, val);
/// }
///
/// ```
pub fn from_fs_in_place<P: AsRef<Path>, T: DeserializeOwned>(path: P, place: &mut T) -> Result<(), DeserializerError> {
    let deserializer = FilesystemDeserializer::new(path);
    T::deserialize_in_place(deserializer, place)
}
