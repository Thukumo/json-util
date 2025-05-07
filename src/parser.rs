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

fn parse_obj(file: &[String], mut pos: usize) -> JsonValue {
    let mut brace_count = 0;
    let mut bracket_count = 0;
    let mut result = HashMap::<String, JsonValue>::new();
    let mut key = None;

    loop {
        let token = &file[pos];
        println!("{}", token);
        match token.as_str() {
            "{" => {
                if brace_count == 1 && bracket_count == 0 {
                    result.insert(key.take().unwrap(), parse_obj(file, pos));
                }
                brace_count += 1;
            }
            "}" => {
                brace_count -= 1;
            }
            "[" => {
                if bracket_count == 0 && brace_count == 1 {
                    result.insert(key.take().unwrap(), parse_arr(file, pos));
                }
                bracket_count += 1;
            }
            "]" => {
                bracket_count -= 1;
            }
            ":" => {}
            _ if brace_count != 1 || bracket_count != 0 => {}
            _ => {
                if key.is_none() {
                    key = Some(token[1..token.len() - 1].to_string());
                } else {
                    result.insert(key.take().unwrap(), parse_value(token));
                }
            }
        };
        if brace_count == 0 {
            return JsonValue::Object(result);
        }
        pos += 1;
    }
}

fn parse_arr(file: &[String], mut pos: usize) -> JsonValue {
    let mut bracket_count = 0;
    let mut brace_count = 0;
    let mut result = vec![];

    loop {
        let token = &file[pos];
        match token.as_str() {
            "{" => {
                if brace_count == 0 && bracket_count == 1 {
                    result.push(parse_obj(file, pos));
                }
                brace_count += 1;
            }
            "}" => {
                brace_count -= 1;
            }
            "[" => {
                if bracket_count == 1 && brace_count == 0 {
                    result.push(parse_arr(file, pos));
                }
                bracket_count += 1;
            }
            "]" => {
                bracket_count -= 1;
            }
            _ if bracket_count != 1 || brace_count != 0 => {}
            _ => {
                result.push(parse_value(token));
            }
        }
        if bracket_count == 0 {
            return JsonValue::Array(result);
        }
        pos += 1;
    }
}

pub fn parse(path: PathBuf) -> JsonValue {
    let tmp = read_to_string(path).expect("Failed to read the file");
    let tmp = tmp.split('"').collect::<Vec<&str>>();
    let mut file = vec![tmp[0].to_string()];
    // i=1のときはpopされない(0が波かっこなため)
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
    parse_obj(&file, 0)
}
