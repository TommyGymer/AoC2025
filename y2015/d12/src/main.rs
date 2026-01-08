use std::fs;

use serde_json::Value;

fn tree_sum(value: &Value) -> i64 {
    match value {
        Value::Null => 0,
        Value::Bool(bool) => 0,
        Value::Number(i) => i.as_i64().unwrap(),
        Value::String(string) => 0,
        Value::Array(arr) => arr.iter().map(|item| tree_sum(item)).sum(),
        Value::Object(map) => {
            if !map.contains_key("red")
                && map.values().into_iter().filter(|v| *v == "red").count() == 0
            {
                map.iter().map(|(key, v)| tree_sum(v)).sum()
            } else {
                0
            }
        }
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap().trim().to_owned();

    let parsed: Value = serde_json::from_str(&input).unwrap();

    println!("{:#?}", tree_sum(&parsed));
}
