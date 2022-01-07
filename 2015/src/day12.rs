use aoc_runner_derive::{aoc, aoc_generator};
use serde_json::Value;

fn parse_input(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn solve_p1(input: &Value) -> i64 {
    match input {
        Value::Array(arr) => arr.iter().map(|val| solve_p1(val)).sum(),
        Value::Object(obj) => obj.iter().map(|(_, val)| solve_p1(val)).sum(),
        Value::Number(num) => num.as_i64().unwrap(),
        _ => 0,
    }
}

fn solve_p2(input: &Value) -> i64 {
    match input {
        Value::Array(arr) => arr.iter().map(|val| solve_p2(val)).sum(),
        Value::Object(obj) => {
            if obj.iter().any(|(_, val)| match val {
                Value::String(val) => val == "red",
                _ => false,
            }) {
                0
            } else {
                obj.iter().map(|(_, val)| solve_p2(val)).sum()
            }
        }
        Value::Number(num) => num.as_i64().unwrap(),
        _ => 0,
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Value {
    parse_input(input)
}

#[aoc(day12, part1)]
pub fn wrapper_p1(input: &Value) -> i64 {
    solve_p1(input)
}

#[aoc(day12, part2)]
pub fn wrapper_p2(input: &Value) -> i64 {
    solve_p2(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_p1() {
        let inputs = vec![
            (r#"[1,2,3]"#, 6),
            (r#"{"a":2,"b":4}"#, 6),
            (r#"[[[3]]]"#, 3),
            (r#"{"a":{"b":4},"c":-1}"#, 3),
            (r#"{"a":[-1,1]}"#, 0),
            (r#"[-1,{"a":1}]"#, 0),
            (r#"{}"#, 0),
            (r#"{}"#, 0),
        ];

        for (input, expect) in inputs {
            let parsed_input = super::parse_input(input);
            assert_eq!(expect, super::solve_p1(&parsed_input));
        }
    }

    #[test]
    fn test_p2() {
        let inputs = vec![
            (r#"[1,2,3]"#, 6),
            (r#"[1,{"c":"red","b":2},3]"#, 4),
            (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
            (r#"[1,"red",5]"#, 6),
        ];

        for (input, expect) in inputs {
            let parsed_input = super::parse_input(input);
            assert_eq!(expect, super::solve_p2(&parsed_input));
        }
    }
}
