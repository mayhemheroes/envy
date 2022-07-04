#![no_main]
use libfuzzer_sys::fuzz_target;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PlainEnum {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Enum {
    A(u8),
    B(()),
    C(Vec<PlainEnum>),
    D(i128),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum FloatEnum {
    A(Enum),
    E(Option<f32>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Struct {
    _a: (),
    _b: u8,
    _c: Vec<Enum>,
    _d: (u128, i8, (), PlainEnum, String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FloatStruct {
    _a: Struct,
    _b: f64,
}

macro_rules! fromiter {
    ($ty:ty, $data:expr) => {{
        #[cfg(feature = "debug")]
        println!("deserializing to {}", stringify!($ty));

        let _: Result<$ty, _> = ::envy::from_iter($data);
    }};
}

macro_rules! from_bytes {
    ($ty:ty, $data:expr) => {{
        fromiter!($ty, $data);
        fromiter!(Vec<$ty>, $data);
        fromiter!(Option<$ty>, $data);
    }};
}

fuzz_target!(|data: Vec<(String, String)>| {
    from_bytes!(bool, data.clone());
    from_bytes!(i8, data.clone());
    from_bytes!(i16, data.clone());
    from_bytes!(i32, data.clone());
    from_bytes!(i64, data.clone());
    from_bytes!(i128, data.clone());
    from_bytes!(u8, data.clone());
    from_bytes!(u16, data.clone());
    from_bytes!(u32, data.clone());
    from_bytes!(u64, data.clone());
    from_bytes!(u128, data.clone());
    from_bytes!(f32, data.clone());
    from_bytes!(f64, data.clone());
    from_bytes!(char, data.clone());
    from_bytes!(String, data.clone());
    from_bytes!((), data.clone());
    from_bytes!(PlainEnum, data.clone());
    from_bytes!(Enum, data.clone());
    from_bytes!(FloatEnum, data.clone());
    from_bytes!(Struct, data.clone());
    from_bytes!(FloatStruct, data.clone());
});
