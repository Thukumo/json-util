use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
#[allow(dead_code)]
pub struct TypeUnmatchedError(String);

#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

macro_rules! impl_try_into {
    ($enum_type:ty, $variant:ident, $output:ty) => {
        impl TryInto<$output> for $enum_type {
            type Error = TypeUnmatchedError;
            fn try_into(self) -> Result<$output, Self::Error> {
                if let Self::$variant(value) = self {
                    Ok(value)
                } else {
                    Err(TypeUnmatchedError(format!("Expected {}, but found another variant", stringify!($variant))))
                }
            }
        }
    };
}

impl_try_into!(Number, Float, f64);
impl_try_into!(Number, Int, i64);

#[derive(Debug, Clone)]
pub enum JsonValue {
    String(String),
    Number(Number),
    Bool(bool),
    Null,
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

impl_try_into!(JsonValue, String, String);
impl_try_into!(JsonValue, Number, Number);
impl_try_into!(JsonValue, Bool, bool);
impl_try_into!(JsonValue, Object, HashMap<String, JsonValue>);
impl_try_into!(JsonValue, Array, Vec<JsonValue>);

impl Index<&str> for JsonValue {
    type Output = Self;
    fn index(&self, index: &str) -> &Self::Output {
        if let Self::Object(map) = self {
            map.get(index).expect(&format!("Key '{}' not found in object", index))
        } else {
            panic!("Attempted to index a non-object JsonValue");
        }
    }
}

impl Index<String> for JsonValue {
    type Output = Self;
    fn index(&self, index: String) -> &Self::Output {
        self.index(index.as_str())
    }
}
