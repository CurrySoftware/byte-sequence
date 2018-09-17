extern crate rand;
extern crate serde;
#[macro_use]
extern crate failure;

use std::result;

use serde::de::{self, Visitor};

pub trait Checkable: Sized {
    const NAME: &'static str;
    fn check(key: &str) -> Result<Self, failure::Error>;
}

#[derive(Debug, Fail)]
pub enum ByteSequenceError {
    #[fail(
        display = "Failed to convert '{}' to '{}' because it had the wrong length",
        raw_key,
        typename
    )]
    InvalidKeyLen {
        raw_key: String,
        typename: &'static str,
    },
    #[fail(
        display = "Failed to convert '{}' to '{}' because it contained invalid characters",
        raw_key,
        typename
    )]
    InvalidKeyChars {
        raw_key: String,
        typename: &'static str,
    },
}

#[macro_export]
macro_rules! byte_seq {
    ($name:ident; $count:expr) => {
        #[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
        pub struct $name([u8; $count]);

        impl $crate::Checkable for $name {
            const NAME: &'static str = stringify!($name);
            fn check(key: &str) -> std::result::Result<$name, $crate::failure::Error> {
                ensure!(
                    key.chars().count() == ($count * 2),
                    $crate::ByteSequenceError::InvalidKeyLen {
                        raw_key: key.to_string(),
                        typename: stringify!($name)
                    }
                );

                let mut bytes: [u8; $count] = [0u8; $count];
                for i in 0..$count {
                    if let Ok(byte) = u8::from_str_radix(&key[i * 2..i * 2 + 2], 16) {
                        bytes[i] = byte;
                    } else {
                        bail!($crate::ByteSequenceError::InvalidKeyChars {
                            raw_key: key.to_string(),
                            typename: stringify!($name)
                        })
                    }
                }
                Ok($name(bytes))
            }
        }

        impl $name {
            pub fn generate_new() -> $name {
                use rand::thread_rng;
                use rand::RngCore;

                let mut bytes: [u8; $count] = [0u8; $count];
                thread_rng().fill_bytes(&mut bytes);
                $name(bytes)
            }

            pub fn to_string(&self) -> String {
                let mut result = String::with_capacity($count * 2);
                for byte in &self.0 {
                    result.push_str(&format!("{:0width$X}", byte, width = 2));
                }
                result
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.to_string())
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_str($crate::CheckableVisitor {
                    _type: std::marker::PhantomData,
                })
            }
        }
    };
}

pub struct CheckableVisitor<T: Checkable> {
    pub _type: std::marker::PhantomData<T>,
}

impl<'de, Checked: Checkable> Visitor<'de> for CheckableVisitor<Checked> {
    type Value = Checked;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("An $name in String format!")
    }

    fn visit_str<E>(self, value: &str) -> result::Result<Checked, E>
    where
        E: de::Error,
    {
        Checked::check(value)
            .map_err(|e| E::custom(format!("Failed to deserialize {} '{:?}'", Checked::NAME, e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    byte_seq!(Test; 10);

    #[test]
    fn serialize_deserialize() {
        let a = Test::generate_new();
        assert_eq!(a, Test::check(&a.to_string()).unwrap());
    }

    byte_seq!(ApiKey; 32);

    #[test]
    fn example() {
        // Creates a new ApiKey containing 32 random bytes using a thread_rng
        let key = ApiKey::generate_new();

        // The to_string method creates a hex encoded string:
        // i.e. 'BBC47F308F3D02C3C6C3D6C9555296A64407FE72AD92DE8C7344D610CFFABF67'
        assert_eq!(key.to_string().len(), 64);

        // you can also do it the other way around: Parse a string into an ApiKey
        let key = ApiKey::check("BBC47F308F3D02C3C6C3D6C9555296A64407FE72AD92DE8C7344D610CFFABF67")
            .unwrap();
        assert_eq!(
            key.to_string(),
            "BBC47F308F3D02C3C6C3D6C9555296A64407FE72AD92DE8C7344D610CFFABF67"
        );
    }

}
