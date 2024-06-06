use std::{fs::File, io::BufReader};

use guangzhou_metro::{Link, Map};
use serde_json::Value;

fn main() {
    let file = File::open("map.json").expect("failed to open map.json");
    let rdr = BufReader::new(file);
    let val: Value = serde_json::from_reader(rdr).expect("failed to parse JSON");

    let Value::Object(obj) = val else {
        panic!("not an object");
    };

    let mut map = Map::default();

    for (line, arr) in obj {
        let Value::Array(arr) = arr else {
            panic!("not an array");
        };
        if arr.len() % 2 != 1 {
            panic!("even array length");
        }

        let (Value::String(start), Value::String(end)) = (&arr[0], arr.last().unwrap()) else {
            panic!("type mismatch");
        };

        for i in 0..arr.len() / 2 {
            let [Value::String(fst), Value::Number(cost), Value::String(snd)] =
                &arr[i * 2..i * 2 + 3]
            else {
                panic!("type mismatch");
            };

            let cost = cost.as_f64().expect("not a f64");
            map.add_link(
                fst.clone(),
                Link {
                    next: snd.clone(),
                    cost,
                    line: line.clone(),
                    direction: end.clone(),
                },
            );
            map.add_link(
                snd.clone(),
                Link {
                    next: fst.into(),
                    cost,
                    line: line.clone(),
                    direction: start.clone(),
                },
            );
        }
    }

    // println!("{map:#?}");
    println!("{:#?}", map.find_path("顺德学院站", "增城广场"));
}
