pub mod applications;
pub mod bootstrap;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::BufReader,
    mem,
    sync::OnceLock,
};

use serde::Serialize;
use serde_json::Value;

struct SubPath<'a> {
    dest: &'a str,
    len: f64,
}

impl<'a> PartialEq for SubPath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}

impl<'a> Eq for SubPath<'a> {}

impl<'a> PartialOrd for SubPath<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for SubPath<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.len.total_cmp(&self.len)
    }
}

struct SubPathProp<'a> {
    prev_of_dest: &'a str,
    len: f64,
    last_link: &'a Link,
    is_min: bool,
}

#[derive(Debug)]
pub struct Link {
    pub next: String,
    pub cost: f64,
    pub line: String,
    pub direction: String,
}

#[derive(Debug, Serialize)]
pub struct Path<'a> {
    pub segments: Vec<PathSegment<'a>>,
    pub len: f64,
}

#[derive(Debug, Serialize)]
pub struct PathSegment<'a> {
    pub line: &'a str,
    pub direction: &'a str,
    pub stations: Vec<&'a str>,
    pub len: f64,
}

#[derive(Debug, Default)]
pub struct Map {
    map: HashMap<String, Vec<Link>>,
}

impl Map {
    pub fn add_link(&mut self, station: String, link: Link) {
        self.map.entry(station).or_default().push(link);
    }

    pub fn find_path_raw<'a>(&'a self, start: &'a str, dest: &str) -> Option<Vec<&'a Link>> {
        if !self.map.contains_key(start) || !self.map.contains_key(dest) {
            return None;
        }
        if start == dest {
            return Some(vec![]);
        }

        let mut sub_paths = BinaryHeap::new();
        let mut sub_path_props = HashMap::new();

        for link in &self.map[start] {
            sub_paths.push(SubPath {
                dest: &link.next,
                len: link.cost,
            });
            sub_path_props.insert(
                link.next.as_str(),
                SubPathProp {
                    prev_of_dest: start,
                    len: link.cost,
                    last_link: link,
                    is_min: false,
                },
            );
        }

        while let Some(sub_path) = sub_paths.pop() {
            let prop = sub_path_props.get_mut(sub_path.dest).unwrap();
            if prop.is_min {
                continue;
            }
            prop.is_min = true;

            if sub_path.dest == dest {
                let mut prop = &sub_path_props[sub_path.dest];
                let mut res = vec![];
                loop {
                    res.push(prop.last_link);
                    if prop.prev_of_dest == start {
                        res.reverse();
                        return Some(res);
                    }
                    prop = &sub_path_props[prop.prev_of_dest];
                }
            }

            for link in &self.map[sub_path.dest] {
                let next = link.next.as_str();
                let alt_len = sub_path.len + link.cost;

                if sub_path_props
                    .get(next)
                    .is_some_and(|prop| alt_len >= prop.len)
                {
                    continue;
                }

                sub_path_props.insert(
                    next,
                    SubPathProp {
                        prev_of_dest: sub_path.dest,
                        len: alt_len,
                        last_link: link,
                        is_min: false,
                    },
                );
                sub_paths.push(SubPath {
                    dest: next,
                    len: alt_len,
                });
            }
        }
        None
    }

    pub fn find_path<'a>(&'a self, start: &'a str, dest: &str) -> Option<Path<'a>> {
        fn new_segment<'a>(
            stations: &mut Vec<&'a str>,
            last_link: &'a Link,
            segment_len: f64,
        ) -> PathSegment<'a> {
            let last_station = *stations.last().unwrap();
            PathSegment {
                line: &last_link.line,
                direction: &last_link.direction,
                stations: mem::replace(stations, vec![last_station]),
                len: segment_len,
            }
        }

        let links = self.find_path_raw(start, dest)?;

        let mut segments = vec![];
        let mut stations = vec![start];

        let mut segment_len = 0.0;
        let mut len = 0.0;

        for i in 0..links.len() {
            let link = links[i];
            if i != 0 && link.line != links[i - 1].line {
                segments.push(new_segment(&mut stations, &links[i - 1], segment_len));
                segment_len = 0.0;
            }

            stations.push(&link.next);
            segment_len += link.cost;
            len += link.cost;
        }

        if let Some(last_link) = links.last() {
            segments.push(new_segment(&mut stations, last_link, segment_len));
        }
        Some(Path { segments, len })
    }
}

pub fn get_map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(read_map_from_json)
}

fn read_map_from_json() -> Map {
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
    map
}
