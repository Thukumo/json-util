use crate::{JsonValue, Number};
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

fn parse_value(s: &str) -> JsonValue {
    if s.starts_with('"') {
        JsonValue::String(s[1..s.len() - 1].to_string())
    } else if s.starts_with('n') {
        JsonValue::Null
    } else if s.starts_with('t') {
        JsonValue::Bool(true)
    } else if s.starts_with('f') {
        JsonValue::Bool(false)
    } else if s.chars().any(|c| c == '.' || c == 'e' || c == 'E') {
        JsonValue::Number(Number::Float(s.parse().unwrap()))
    } else {
        JsonValue::Number(Number::Int(s.parse().unwrap()))
    }
}

fn parse_obj(file: &[String], pos: usize) -> (usize, JsonValue) {
    let mut result = HashMap::new();
    let mut key = None;
    let (first_pos, mut pos) = (pos, pos+1);

    loop {
        let token = &file[pos];
        match token.as_str() {
            "{" => {
                let (diff, val) = parse_obj(file, pos);
                result.insert(key.take().unwrap(), val);
                pos += diff;
            }
            "}" => {
                return (pos - first_pos, JsonValue::Object(result));
            }
            "[" => {
                let (diff, val) = parse_arr(file, pos);
                result.insert(key.take().unwrap(), val);
                pos += diff;
            }
            ":" | "," => {}
            _ => {
                if key.is_none() {
                    key = Some(token[1..token.len() - 1].to_string());
                } else {
                    result.insert(key.take().unwrap(), parse_value(token));
                }
            }
        };
        pos += 1;
    }
}

fn parse_arr(file: &[String], pos: usize) -> (usize, JsonValue) {
    let (first_pos, mut pos) = (pos, pos+1);
    let mut result = Vec::new();

    loop {
        let token = &file[pos];
        match token.as_str() {
            "{" => {
                let (diff, val) = parse_obj(file, pos);
                result.push(val);
                pos += diff;
            }
            "," => {}
            "[" => {
                let (diff, val) = parse_arr(file, pos);
                result.push(val);
                pos += diff;
            }
            "]" => {
                return (pos - first_pos, JsonValue::Array(result));
            }
            _ => {
                result.push(parse_value(token));
            }
        }
        pos += 1;
    }
}

pub fn parse(path: &PathBuf) -> Result<JsonValue, std::io::Error> {
    Ok(parse_obj(&read_to_string(path)?.split('"')
        .fold((Vec::new(), String::new(), true), |state, s| {
        let (mut state, mut current, odd) = state;
        if s.ends_with('\\') {
            current.push_str(&s);
            current.push('"');
            (state, current, odd)
        } else {
            current.reserve_exact(s.len());
            current.push_str(s);
            if odd {
                state.extend(current.chars().filter(|c| !c.is_whitespace())
                    .fold((Vec::new(), false), |state, c| {
                    let (mut state, splitter) = state;
                    match c {
                        '{' | '}' | '[' | ']' | ':' | ',' => {
                            state.push(c.to_string());
                            (state, true)
                        }
                        _ => {
                            if splitter {
                                state.push(c.to_string());
                            } else {
                                state.last_mut().unwrap().push(c);
                            }
                            (state, false)
                        }
                    }
                }).0);
            } else {
                let mut s_p = String::with_capacity(current.len() + 2);
                s_p.push('"');
                s_p.push_str(&current);
                s_p.push('"');
                state.push(s_p);
            }
            (state, String::new(), !odd)
        }
    }).0, 0).1)
}
