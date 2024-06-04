use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Link {
    pub line: String,
    pub cost: f64,
}

#[derive(Debug)]
pub struct Map {
    stations: HashMap<String, Vec<(Link, String)>>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            stations: HashMap::new(),
        }
    }

    fn station(&mut self, name: &str) -> &mut Vec<(Link, String)> {
        self.stations.entry(name.into()).or_default()
    }

    pub fn add_link(&mut self, fst: &str, snd: &str, link: Link) {
        self.station(fst).push((link.clone(), snd.into()));
        self.station(snd).push((link, fst.into()));
    }

    pub fn find_path(&self, from: &str, to: &str) -> Vec<(Link, String)> {
        todo!()
    }
}
