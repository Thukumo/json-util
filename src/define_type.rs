use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
pub struct TypeUnmatchedError ();

#[derive(Debug, Clone)]
pub enum JsonValue {
    String(String),
    Number(Number),
    Bool(bool),
    Null,
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}
#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}
impl TryInto<f64> for Number {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<f64, Self::Error> {
        if let Self::Float(num) = self {
            Ok(num)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl TryInto<i64> for Number {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<i64, Self::Error> {
        if let Self::Int(num) = self {
            Ok(num)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}

impl TryInto<String> for JsonValue {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<String, Self::Error> {
        if let Self::String(s) = self {
            Ok(s)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl TryInto<Number> for JsonValue {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<Number, Self::Error> {
        if let Self::Number(num) = self {
            Ok(num)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl TryInto<bool> for JsonValue {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<bool, Self::Error> {
        if let Self::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl TryInto<HashMap<String, JsonValue>> for JsonValue {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<HashMap<String, Self>, Self::Error> {
        if let Self::Object(obj) = self {
            Ok(obj)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl TryInto<Vec<JsonValue>> for JsonValue {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<Vec<JsonValue>, Self::Error> {
        if let Self::Array(arr) = self {
            Ok(arr)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl Index<&str> for JsonValue {
    type Output = Self;
    fn index(&self, index: &str) -> &Self::Output {
        if let Self::Object(hoge) = self {
            hoge.get(index).expect("Key not found in object")
        } else {
            panic!()
        }
    }
}
impl Index<String> for JsonValue {
    type Output = Self;
    fn index(&self, index: String) -> &Self::Output {
        if let Self::Object(hoge) = self {
            hoge.get(&index).unwrap()
        } else {
            panic!()
        }
    }
    
}
