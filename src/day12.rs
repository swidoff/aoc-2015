use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> String {
    let file = File::open("input/day12.txt").unwrap();
    let mut str = String::new();
    BufReader::new(file).read_to_string(&mut str).unwrap();
    str
}

#[derive(Eq, PartialEq)]
enum Json {
    Number(i64),
    String(String),
    List(Vec<Json>),
    Object(HashMap<String, Json>),
}

//
// Expr -> List | Object | Number | String
// List -> [ Expr, ... ]
// Object -> { String: Object, .. }
// String -> " char... "
// Number -> -? digit...

fn parse_json(str: &str) -> Json {
    let mut chars = VecDeque::from_iter(str.chars());
    parse_expression(&mut chars)
}

fn parse_expression(q: &mut VecDeque<char>) -> Json {
    match q.front() {
        Some('[') => Json::List(parse_list(q)),
        Some('{') => Json::Object(parse_object(q)),
        Some('"') => Json::String(parse_string(q)),
        Some(_) => Json::Number(parse_number(q)),
        _ => panic!("parse error"),
    }
}

fn parse_list(q: &mut VecDeque<char>) -> Vec<Json> {
    let mut elements = Vec::new();
    q.pop_front().unwrap();
    if *q.front().unwrap() != ']' {
        elements.push(parse_expression(q));

        while let Some(',') = q.front() {
            q.pop_front().unwrap();
            elements.push(parse_expression(q))
        }
    }
    q.pop_front().unwrap();
    elements
}

fn parse_object(q: &mut VecDeque<char>) -> HashMap<String, Json> {
    let mut fields = HashMap::new();
    q.pop_front().unwrap();

    if *q.front().unwrap() != '}' {
        let key = parse_string(q);
        q.pop_front().unwrap();
        let value = parse_expression(q);
        fields.insert(key, value);

        while let Some(',') = q.front() {
            q.pop_front().unwrap();
            let key = parse_string(q);
            q.pop_front().unwrap();
            let value = parse_expression(q);
            fields.insert(key, value);
        }
    }
    q.pop_front().unwrap();
    fields
}

fn parse_string(q: &mut VecDeque<char>) -> String {
    let mut res = String::new();
    q.pop_front().unwrap();
    while let Some(c) = q.front() {
        if *c == '"' {
            break;
        } else {
            res.push(q.pop_front().unwrap());
        }
    }

    q.pop_front().unwrap();
    res
}

fn parse_number(q: &mut VecDeque<char>) -> i64 {
    let mut res = String::new();
    while let Some(c) = q.front() {
        if c.is_digit(10) || *c == '-' {
            res.push(q.pop_front().unwrap());
        } else {
            break;
        }
    }

    i64::from_str(res.as_str()).unwrap()
}

fn sum_numbers_part1(json: &Json) -> i64 {
    match json {
        Json::Number(v) => *v,
        Json::String(_) => 0,
        Json::List(ls) => ls.iter().fold(0, |sum, v| sum + sum_numbers_part1(v)),
        Json::Object(flds) => flds.values().fold(0, |sum, v| sum + sum_numbers_part1(v)),
    }
}

fn sum_numbers_part2(json: &Json) -> i64 {
    match json {
        Json::Number(v) => *v,
        Json::List(ls) => ls.iter().fold(0, |sum, v| sum + sum_numbers_part2(v)),
        Json::Object(flds) if flds.values().all(|v| *v != Json::String("red".to_string())) => {
            flds.values().fold(0, |sum, v| sum + sum_numbers_part2(v))
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::{parse_json, read_file, sum_numbers_part1, sum_numbers_part2};

    #[test]
    fn test_part1_example() {
        assert_eq!(6, sum_numbers_part1(&parse_json("[1,2,3]")));
        assert_eq!(6, sum_numbers_part1(&parse_json("{\"a\":2,\"b\":4}")));
        assert_eq!(3, sum_numbers_part1(&parse_json("[[[3]]]")));
        assert_eq!(
            3,
            sum_numbers_part1(&parse_json("{\"a\":{\"b\":4},\"c\":-1}"))
        );
        assert_eq!(0, sum_numbers_part1(&parse_json("{\"a\":[-1,1]}")));
        assert_eq!(0, sum_numbers_part1(&parse_json("[-1,{\"a\":1}]")));
        assert_eq!(0, sum_numbers_part1(&parse_json("[]")));
        assert_eq!(0, sum_numbers_part1(&parse_json("{}")));
    }

    #[test]
    fn test_part1() {
        println!("{}", sum_numbers_part1(&parse_json(read_file().as_str())))
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(6, sum_numbers_part2(&parse_json("[1,2,3]")));
        assert_eq!(
            4,
            sum_numbers_part2(&parse_json("[1,{\"c\":\"red\",\"b\":2},3]"))
        );
        assert_eq!(
            0,
            sum_numbers_part2(&parse_json("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"))
        );
        assert_eq!(6, sum_numbers_part2(&parse_json("[1,\"red\",5]")));
    }

    #[test]
    fn test_part2() {
        println!("{}", sum_numbers_part2(&parse_json(read_file().as_str())))
    }
}
