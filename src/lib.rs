extern crate serde;
extern crate rand;
#[macro_use]
extern crate error_chain;

pub mod errors;

use std::marker::PhantomData;
use std::fmt;
use std::result;

use serde::de::{self, Visitor};

use errors::*;


pub trait Checkable: Sized {
    const NAME: &'static str;
    fn check(key: &str) -> Result<Self>;
}


#[macro_export]
macro_rules! byte_seq {
    ($name:ident; $count:expr) => {
        use serde::{Serialize, Serializer, Deserialize, Deserializer};

        #[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
        pub struct $name([u8; $count]);

        impl Checkable for $name {
            const NAME: &'static str = stringify!($name);
            fn check(key: &str) -> Result<$name> {
                ensure!(key.chars().count() == ($count * 2),
                        ErrorKind::InvalidKeyLen(key.to_string(), stringify!($name)));

                let mut bytes: [u8; $count] = [0u8; $count];
                for i in 0..$count {
                    if let Ok(byte) = u8::from_str_radix(&key[i*2..i*2+2], 16) {
                        bytes[i] = byte;
                    } else {
                        bail!(ErrorKind::InvalidKeyChars(key.to_string(), stringify!($name)))
                    }
                }
                Ok($name(bytes))
            }

        }

        impl $name {

            pub fn generate_new() -> $name {
                use rand::thread_rng;
                use rand::Rng;

                let mut bytes: [u8; $count] = [0u8; $count];
                thread_rng().fill_bytes(&mut bytes);
                $name(bytes)
            }

            pub fn to_string(&self) -> String {
                let mut result = String::with_capacity($count * 2);
                for byte in &self.0 {
                    result.push_str(&format!("{:0width$X}", byte, width=2));
                }
                result
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.to_string())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self)
            }
        }


        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
                where S: Serializer
            {
                serializer.serialize_str(&self.to_string())
            }
        }


        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                deserializer.deserialize_str(CheckableVisitor{_type:PhantomData})
            }
        }
    }
}


pub struct CheckableVisitor<T: Checkable> {
    pub _type: PhantomData<T>,
}

impl<'de, Checked: Checkable> Visitor<'de> for CheckableVisitor<Checked> {
    type Value = Checked;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("An $name in String format!")
    }

    fn visit_str<E>(self, value: &str) -> result::Result<Checked, E>
        where E: de::Error
    {
        Checked::check(value).map_err(|e| {
                                          E::custom(format!("Failed to deserialize {} '{:?}'",
                                                            Checked::NAME,
                                                            e))
                                      })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    byte_seq!(Test; 10);

    #[test]
    fn serialize_deserialize() {
        let a = Test::generate_new();
        assert_eq!(a, Test::check(&a.to_string()).unwrap());
    }
}
