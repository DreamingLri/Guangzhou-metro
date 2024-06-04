use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Edge {
    pub line: String,
    pub cost: f64,
}

#[derive(Debug)]
struct Station {
    name: String,
    neighbors: Vec<(Edge, Weak<RefCell<Station>>)>,
}

#[derive(Debug)]
pub struct Map {
    stations: HashMap<String, Rc<RefCell<Station>>>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            stations: HashMap::new(),
        }
    }

    fn get_station<'a>(&'a mut self, name: &str) -> &'a Rc<RefCell<Station>> {
        self.stations.entry(name.into()).or_insert_with(|| {
            Rc::new(RefCell::new(Station {
                name: name.into(),
                neighbors: vec![],
            }))
        })
    }

    pub fn add_edge(&mut self, line: &str, from: &str, to: &str, cost: f64) {
        let to_station = Rc::downgrade(self.get_station(to));
        self.get_station(from).borrow_mut().neighbors.push((
            Edge {
                line: line.into(),
                cost,
            },
            to_station,
        ));
    }

    pub fn find_path(&self, from: &str, to: &str) -> Vec<(Edge, String)> {
        todo!()
    }
}
