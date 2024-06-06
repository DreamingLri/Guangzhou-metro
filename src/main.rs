use std::{fs::File, io::BufReader};

use guangzhou_metro::{Link, Map};
use serde_json::Value;

use axum::{
    routing::{get, post},
    Router,
};

use std::net::SocketAddr;
use tokio::net::TcpListener;

fn main() {
    let file = File::open("map.json").expect("failed to open map.json");
    let rdr = BufReader::new(file);
    let val: Value = serde_json::from_reader(rdr).expect("failed to parse JSON");

    let Value::Object(obj) = val else {
        panic!("not an object");
    };

    let mut map = Map::new();

    for (line, arr) in obj {
        let Value::Array(arr) = arr else {
            panic!("not an array");
        };
        if arr.len() % 2 != 1 {
            panic!("even array length");
        }

        for i in 0..arr.len() / 2 {
            let [Value::String(fst), Value::Number(cost), Value::String(snd)] =
                &arr[i * 2..i * 2 + 3]
            else {
                panic!("type mismatch");
            };

            let link = Link {
                line: line.clone(),
                cost: cost.as_f64().expect("not a f64"),
            };
            map.add_link(fst, snd, link);
        }
    }

    println!("{map:#?}");
}

// #[tokio::main]
// async fn main() {
//
//     let app = Router::new()
//         .route("/", get(root));
//
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//         .await
//         .unwrap();
//
//     axum::serve(listener, app).await.unwrap();
// }
//
// async fn root() -> &'static str{
//     "Hello, World!"
// }
