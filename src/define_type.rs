use std::{collections::HashMap, ops::Index};

use crate::parse_value;

#[derive(Debug)]
pub enum ParseError {
    InvalidData(String),
    TypeUnmatchedError(String),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ParseError::ParseFloatError(err)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError::ParseIntError(err)
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

macro_rules! impl_try_into {
    ($enum_type:ty, $variant:ident, $output:ty) => {
        impl TryInto<$output> for $enum_type {
            type Error = crate::ParseError;
            fn try_into(self) -> Result<$output, Self::Error> {
                if let Self::$variant(value) = self {
                    Ok(value)
                } else {
                    Err(ParseError::TypeUnmatchedError(format!("Expected {}::{}", stringify!($enum_type), stringify!($variant))))
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
    InValidLocation,
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
            if let Some(val) = map.get(index) {
                val
            } else {
                &Self::InValidLocation
            }
        } else {
            &Self::InValidLocation
        }
    }
}

impl Index<String> for JsonValue {
    type Output = Self;
    fn index(&self, index: String) -> &Self::Output {
        self.index(index.as_str())
    }
}

#[derive(Debug)]
pub enum JsonValueLazy {
    Something(String),
    Object(HashMap<String, JsonValueLazy>),
    Array(Vec<JsonValueLazy>),
    InValidLocation,
}

impl Into<JsonValue> for JsonValueLazy {
    fn into(self) -> JsonValue {
        match self {
            JsonValueLazy::Something(val) => {
                parse_value(&val).unwrap()
            }
            JsonValueLazy::Array(val) => {
                JsonValue::Array(val.into_iter().map(|j| j.into()).collect())
            }
            JsonValueLazy::Object(val) => {
                JsonValue::Object(val.into_iter().map(|(key, val)| {
                    (key, val.into())
                }).collect::<HashMap<String, JsonValue>>())
            }
            JsonValueLazy::InValidLocation => {
                JsonValue::InValidLocation
            }
        }
    }
}

impl Index<&str> for JsonValueLazy {
    type Output = Self;
    fn index(&self, index: &str) -> &Self::Output {
        if let Self::Object(map) = self {
            map.get(index).unwrap_or_else(|| &Self::InValidLocation)
        } else {
            &Self::InValidLocation
        }
    }
}

impl Index<String> for JsonValueLazy {
    type Output = Self;
    fn index(&self, index: String) -> &Self::Output {
        self.index(index.as_str())
    }
}
