# Byte Sequences

This crate provides some utility to create byte sequences like ApiKeys or SessionIds.
It contains a macro that lets you specify the name and length in bytes of the sequence.
The macro than creates a struct that has following methods

```rust

fn generate_new() -> Self

fn to_string() -> String

fn check(key: &str) -> Result<Self>;

```

It also implements `Display`, `Debug`, `serde::Serialize`, `serde::Deserialize`, `PartialOrd`, `Ord`, `PartialEq`, `Eq`, `Clone`, `Copy` and `Hash`;

Complete example:

```rust
#[macro_use]
extern crate byte_sequence;
use serde;
use std;
use byte_sequence::Checkable;

byte_seq!(ApiKey; 32);

#[test]
fn example() {
    // Creates a new ApiKey containing 32 random bytes using a thread_rng
    let key = ApiKey::generate_new();

    // The to_string method creates a hex encoded string:
    // i.e. 'BBC47F308F3D02C3C6C3D6C9555296A64407FE72AD92DE8C7344D610CFFABF67'
    assert_eq!(key.to_string().len(), 64);

    // you can also do it the other way around: Parse a string into an ApiKey
    let key = ApiKey::check("BBC47F308F3D02C3C6C3D6C9555296A64407FE72AD92DE8C7344D610CFFABF67").unwrap();
    assert_eq!(key.to_string(), "BBC47F308F3D02C3C6C3D6C9555296A64407FE72AD92DE8C7344D610CFFABF67");
}

```


You can also extend the generated structs.
For example add a public part of the ApiKey:

```rust
impl ApiKey {
    pub fn public_part(&self) -> PublicApiKey {
        let mut pub_data = [0u8; 8];
        pub_data.copy_from_slice(&self.0[0..8]);
        PublicApiKey(pub_data)
    }
}
```
