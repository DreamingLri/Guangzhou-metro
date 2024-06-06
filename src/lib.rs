pub mod app;
pub mod bootstrap;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    mem,
};

use serde::Serialize;

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
        let links = self.find_path_raw(start, dest)?;

        let mut segments = vec![];
        let mut stations = vec![start];

        let mut segment_len = 0.0;
        let mut len = 0.0;

        for i in 0..links.len() {
            let link = &links[i];
            if i != 0 && link.line != links[i - 1].line {
                let last_station = *stations.last().unwrap();
                let segment = PathSegment {
                    line: &links[i - 1].line,
                    direction: &links[i - 1].direction,
                    stations: mem::replace(&mut stations, vec![last_station]),
                    len: segment_len,
                };
                segments.push(segment);
                segment_len = 0.0;
            }

            stations.push(&link.next);
            segment_len += link.cost;
            len += link.cost;
        }
        Some(Path { segments, len })
    }
}
