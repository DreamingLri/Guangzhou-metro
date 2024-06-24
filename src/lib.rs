pub mod applications;
pub mod bootstrap;

use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    fs::File,
    io::BufReader,
    iter, mem,
    sync::OnceLock,
};

use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;

struct SubPath<'a> {
    dest: &'a str,
    len: f64,
}

impl<'a> PartialEq for SubPath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
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

#[derive(Clone, Copy)]
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
    line_stations: IndexMap<String, Vec<String>>,
    map: HashMap<String, Vec<Link>>,
}

impl Map {
    pub fn add_station(&mut self, station: String, line: String) {
        self.line_stations.entry(line).or_default().push(station);
    }

    pub fn line_stations(&self) -> &IndexMap<String, Vec<String>> {
        &self.line_stations
    }

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
        let mut sub_path_props_by_dest = HashMap::new();

        for link in &self.map[start] {
            sub_paths.push(SubPath {
                dest: &link.next,
                len: link.cost,
            });
            sub_path_props_by_dest.insert(
                &link.next[..],
                SubPathProp {
                    prev_of_dest: start,
                    len: link.cost,
                    last_link: link,
                    is_min: false,
                },
            );
        }

        while let Some(sub_path) = sub_paths.pop() {
            let prop = sub_path_props_by_dest.get_mut(sub_path.dest).unwrap();
            if prop.is_min {
                continue;
            }
            prop.is_min = true;

            if sub_path.dest == dest {
                let mut prop = &sub_path_props_by_dest[sub_path.dest];
                let mut path = vec![];
                loop {
                    path.push(prop.last_link);
                    if prop.prev_of_dest == start {
                        path.reverse();
                        return Some(path);
                    }
                    prop = &sub_path_props_by_dest[prop.prev_of_dest];
                }
            }

            for link in &self.map[sub_path.dest] {
                let next = &link.next[..];
                let alt_prop = SubPathProp {
                    prev_of_dest: sub_path.dest,
                    len: sub_path.len + link.cost,
                    last_link: link,
                    is_min: false,
                };

                match sub_path_props_by_dest.entry(next) {
                    Entry::Occupied(entry) => {
                        let prop = entry.into_mut();
                        if alt_prop.len < prop.len {
                            assert!(!prop.is_min);
                            *prop = alt_prop;
                        } else {
                            continue;
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(alt_prop);
                    }
                }

                sub_paths.push(SubPath {
                    dest: next,
                    len: alt_prop.len,
                });
            }
        }
        None
    }

    pub fn find_path<'a>(&'a self, mut start: &'a str, dest: &str) -> Option<Path<'a>> {
        let segments: Vec<_> = self
            .find_path_raw(start, dest)?
            .chunk_by(|a, b| a.line == b.line)
            .map(|links| PathSegment {
                line: &links[0].line,
                direction: &links[0].direction,
                stations: iter::once(mem::replace(&mut start, &links.last().unwrap().next))
                    .chain(links.iter().map(|link| &link.next[..]))
                    .collect(),
                len: links.iter().map(|link| link.cost).sum(),
            })
            .collect();
        let len = segments.iter().map(|seg| seg.len).sum();

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

    let Value::Array(lines) = val else {
        panic!("not an object");
    };

    let mut map = Map::default();

    for line in lines {
        let Value::String(name) = &line["name"] else {
            panic!("not a string");
        };
        let Value::Array(arr) = &line["data"] else {
            panic!("not an array");
        };
        if arr.len() % 2 != 1 {
            panic!("even array length");
        }
        let (Value::String(start), Value::String(end)) = (&arr[0], arr.last().unwrap()) else {
            panic!("type mismatch");
        };

        map.add_station(start.clone(), name.clone());
        for i in 0..arr.len() / 2 {
            let [Value::String(fst), Value::Number(cost), Value::String(snd)] =
                &arr[i * 2..i * 2 + 3]
            else {
                panic!("type mismatch");
            };
            map.add_station(snd.clone(), name.clone());

            let cost = cost.as_f64().expect("not a f64");
            map.add_link(
                fst.clone(),
                Link {
                    next: snd.clone(),
                    cost,
                    line: name.clone(),
                    direction: end.clone(),
                },
            );
            map.add_link(
                snd.clone(),
                Link {
                    next: fst.clone(),
                    cost,
                    line: name.clone(),
                    direction: start.clone(),
                },
            );
        }
    }
    map
}
