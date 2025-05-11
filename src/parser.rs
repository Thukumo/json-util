use crate::{JsonValue, Number, ParseError};
use std::collections::HashMap;

fn parse_value(s: &str) -> Result<JsonValue, ParseError> {
    Ok(
        if s.starts_with('"') {
            JsonValue::String(s[1..s.len() - 1].to_string())
        } else if s == "null" {
            JsonValue::Null
        } else if s == "true" {
            JsonValue::Bool(true)
        } else if s == "false" {
            JsonValue::Bool(false)
        } else if s.chars().any(|c| !c.is_ascii_digit() && c != '-') {
            JsonValue::Number(Number::Float(s.parse()?))
        } else {
            JsonValue::Number(Number::Int(s.parse()?))
        }
    )
}

fn parse_obj(tokens: &[String], pos: usize) -> Result<(usize, JsonValue), ParseError> {
    let mut result = HashMap::new();
    let mut key = None;
    let (first_pos, mut pos) = (pos, pos+1);

    loop {
        let token = &tokens[pos];
        match token.as_str() {
            "{" => {
                let (diff, val) = parse_obj(tokens, pos)?;
                result.insert(key.take().unwrap(), val);
                pos += diff;
            }
            "}" => {
                return Ok((pos - first_pos, JsonValue::Object(result)));
            }
            "[" => {
                let (diff, val) = parse_arr(tokens, pos)?;
                result.insert(key.take().unwrap(), val);
                pos += diff;
            }
            ":" | "," => {}
            _ => {
                if key.is_none() {
                    key = Some(token[1..token.len() - 1].to_string());
                } else {
                    result.insert(key.take().unwrap(), parse_value(token)?);
                }
            }
        };
        pos += 1;
    }
}

fn parse_arr(tokens: &[String], pos: usize) -> Result<(usize, JsonValue), ParseError> {
    let (first_pos, mut pos) = (pos, pos+1);
    let mut result = Vec::with_capacity(tokens.len()/2);

    loop {
        let token = &tokens[pos];
        match token.as_str() {
            "{" => {
                let (diff, val) = parse_obj(tokens, pos)?;
                result.push(val);
                pos += diff;
            }
            "," => {}
            "[" => {
                let (diff, val) = parse_arr(tokens, pos)?;
                result.push(val);
                pos += diff;
            }
            "]" => {
                return Ok((pos - first_pos, JsonValue::Array(result)));
            }
            _ => {
                result.push(parse_value(token)?);
            }
        }
        pos += 1;
    }
}

pub fn parse(s: &str) -> Result<JsonValue, ParseError> {
    Ok(parse_obj(&s.split('"')
        .fold((Vec::new(), String::new(), true), |state, s| {
        let (mut state, mut current, odd) = state;
        if s.ends_with('\\') {
            current.reserve_exact(s.len() + 1);
            current.push_str(s);
            current.push('"');
            (state, current, odd)
        } else {
            current.reserve_exact(s.len());
            current.push_str(s);
            if odd {
                state.extend(current.chars().fold((Vec::new(), false), |state, c| {
                    let (mut state, splitter) = state;
                    if c.is_whitespace() {
                        (state, splitter)
                    } else {
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
    }).0, 0)?.1)
}
