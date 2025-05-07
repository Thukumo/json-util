use crate::{JsonValue, Number};
use std::{collections::HashMap, path::PathBuf};

fn parse_obj(file: &Vec<String>, pos: usize) -> JsonValue {
    let mut pos = pos;
    let mut brace = 0;
    let mut bracket = 0;
    let mut res = HashMap::<String, JsonValue>::new();
    let mut key = None;
    loop {
        let s = &file[pos];
        match s.as_str() {
            "{" => {
                if brace == 1 {
                    res.insert(key.take().unwrap(),parse_obj(file, pos));
                }
                brace += 1;
            }
            "}" => {
                brace -= 1;
            }
            "[" => {
                if bracket == 0 {
                    res.insert(key.take().unwrap(), parse_arr(file, pos));
                }
                bracket += 1;
            }
            "]" => {
                bracket -= 1;
            }
            _ if brace != 1 || bracket != 0 => {}
            ":" => {}
            _ => {
                if key.is_none() {
                    key = Some(s.clone()[1..s.len()-1].to_string());
                } else {
                    // array, objectは上の方で処理しているので、"値"だけを扱えばよい
                    res.insert(key.take().unwrap(),  {
                        if s.starts_with('\"') {
                        //ダブルクォーテーションを取り除く必要がある
                            JsonValue::String(s.clone()[1..s.len()-1].to_string())
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
                    });
                }
            }
        };
        if brace == 0 {
            return JsonValue::Object(res);
        }
        pos += 1;
    }
}

fn parse_arr(file: &Vec<String>, pos: usize) -> JsonValue {
    let mut pos = pos;
    let mut bracket = 0;
    let mut brace = 0;
    let mut res = vec!();

    loop {
        let s = &file[pos];
        match s.as_str() {
            "{" => {
                if brace == 0 {
                    res.push(parse_obj(file, pos));
                }
                brace += 1;
            }
            "}" => {
                brace -= 1;
            }
            "[" => {
                if bracket == 1 {
                    res.push(parse_arr(file, pos));
                }
                bracket += 1;
            }
            "]" => {
                bracket -= 1;
            }
            _ if bracket != 1 || brace != 0 => {}
            _ => {
                if s.starts_with('\"') {
                    //ダブルクォーテーションを取り除く必要がある
                    res.push(JsonValue::String(s.clone()[1..s.len()-1].to_string()));
                } else if s.starts_with("n") {
                    res.push(JsonValue::Null);
                } else if s.starts_with("t") {
                    res.push(JsonValue::Bool(true));
                } else if s.starts_with("f") {
                    res.push(JsonValue::Bool(false));
                }
            }
        }
        if bracket == 0 {
            return JsonValue::Array(res);
        }
        pos += 1;
    }
}

pub fn parse(path: PathBuf) -> JsonValue {
    let mut in_string = false;
    let mut in_string2 = false;
    /*
    filterが終わった時点でin_stringはfalseなので使いまわせるが、Rustではイテレータのメゾットチェーンは遅延評価らしいので、
    filterで暗黙的に作られてる可変参照のライフタイム的にfoldでin_stringを使用することができない
    */
    let file = std::fs::read_to_string(path).unwrap().chars().filter(|c| {
        match c {
            '\"' => {
                in_string = !in_string;
                true
            }
            _ => {
                in_string || !c.is_whitespace()
            }
        }
    }).fold((String::new(), Vec::<String>::new()), |state, c|{
        let (mut cur, mut res) = state;
        if in_string2 {
            match c {
                '\"' => {
                    in_string2 = false;
                }
                _ => {}
            }
            cur.push(c);
        } else {
            match c {
                '\"' => {
                    in_string2 = true;
                    cur.push(c);
                }
                ',' => {
                    if !cur.is_empty() {
                        res.push(cur.clone());
                    }
                    cur.clear();
                }
                _ if "{}:".contains(c) => {
                    if !cur.is_empty() {
                        res.push(cur.clone());
                    }
                    cur.clear();
                    res.push(c.to_string());
                }
                _ => {
                    cur.push(c);
                }
            }
        }
        (cur, res)
    }).1;
    // メモ ストリーミングもアリ
    parse_obj(&file, 0)
}
