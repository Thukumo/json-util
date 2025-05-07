use std::collections::HashMap;

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

#[allow(dead_code)]
impl JsonValue {
    fn at(&self, s: &str) -> Result<Self, TypeUnmatchedError> {
        if let Self::Object(hoge) = self {
            Ok(hoge[&s.to_string()].clone())
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
        if let JsonValue::Number(num) = self {
            Ok(num)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl TryInto<bool> for JsonValue {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<bool, Self::Error> {
        if let JsonValue::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl TryInto<HashMap<String, JsonValue>> for JsonValue {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<HashMap<String, JsonValue>, Self::Error> {
        if let JsonValue::Object(obj) = self {
            Ok(obj)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
impl TryInto<Vec<JsonValue>> for JsonValue {
    type Error = TypeUnmatchedError;
    fn try_into(self) -> Result<Vec<JsonValue>, Self::Error> {
        if let JsonValue::Array(arr) = self {
            Ok(arr)
        } else {
            Err(TypeUnmatchedError())
        }
    }
}
