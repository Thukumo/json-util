use crate::{JsonValue, Number};
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

fn parse_value(s: &str) -> JsonValue {
    if s.starts_with('"') {
        JsonValue::String(s[1..s.len() - 1].to_string())
    } else if s.starts_with("n") {
        JsonValue::Null
    } else if s.starts_with("t") {
        JsonValue::Bool(true)
    } else if s.starts_with("f") {
        JsonValue::Bool(false)
    } else if s.contains('.') {
        JsonValue::Number(Number::Float(s.parse().unwrap()))
    } else {
        JsonValue::Number(Number::Int(s.parse().unwrap()))
    }
}

fn parse_obj(file: &[String], pos: usize) -> (usize, JsonValue) {
    let mut result = HashMap::<String, JsonValue>::new();
    let mut key = None;
    let (first_pos, mut pos) = (pos, pos);

    loop {
        let token = &file[pos];
        match token.as_str() {
            "{" => {
                if pos != first_pos {
                    let (diff, val) = parse_obj(file, pos);
                    result.insert(key.take().unwrap(), val);
                    pos += diff;
                }
            }
            "}" => {
                return (pos - first_pos, JsonValue::Object(result));
            }
            "[" => {
                let (diff, val) = parse_arr(file, pos);
                result.insert(key.take().unwrap(), val);
                pos += diff;
            }
            ":" | "]" => {}
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
    let (first_pos, mut pos) = (pos, pos);
    let mut result = vec![];

    loop {
        let token = &file[pos];
        match token.as_str() {
            "{" => {
                let (diff, val) = parse_obj(file, pos);
                result.push(val);
                pos += diff;
            }
            "}" => {}
            "[" => {
                if pos != first_pos {
                    let (diff, val) = parse_arr(file, pos);
                    result.push(val);
                    pos += diff;
                }
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

pub fn parse(path: PathBuf) -> JsonValue {
    let tmp = read_to_string(path).expect("Failed to read the file");
    let tmp = tmp.split('"').collect::<Vec<&str>>();
    let mut file = vec![tmp[0].to_string()];
    for i in 1..tmp.len() {
        if tmp[i-1].ends_with('\\') {
            if let Some(last) = file.pop() {
                file.push(last + "\"" + tmp[i]);
            }
        } else {
            file.push(tmp[i].to_string());
        }
    }
    // ここに入る時点で、fileはString(Keyを含む)の文字列とそれ以外の要素の文字列が交互に格納されている
    let file = file.into_iter().enumerate().flat_map(|(i, s)| {
        if i % 2 == 0 {
            s.chars().filter(|c| !c.is_ascii_whitespace())
                .fold((Vec::<String>::new(), String::new()), |state, c| {
                let (mut res, mut current) = state;
                let c = c.to_string();
                if "{}[]:,".contains(&c) {
                    if !current.is_empty() {
                        res.push(current);
                        current = String::new();
                    }
                    if c != "," {
                        res.push(c);
                    }
                } else {
                    current += &c;
                }
                (res, current)
            }).0
        } else {
            vec!["\"".to_string() + &s + "\""]
        }
    }).collect::<Vec<String>>();
    parse_obj(&file, 0).1
}
