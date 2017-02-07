use std::collections::HashMap;
use std::rc::Rc;


pub fn default_dict_lib() -> Rc<Vec<Box<Dictionary>>> {
    let surnames = Box::new(RankedSurnames::new());
    let dicts : Vec<Box<Dictionary>> = vec![
        surnames,
    ];
    Rc::new(dicts)
}

pub trait Dictionary {
     fn get_name(&self) -> &str;
     fn contains(&self, &str) -> bool;
     fn rank_of(&self, &str) -> Rank;
}

pub enum Rank {
    Ranking(usize),
    NoRank,
}

struct RankedSurnames<'a> {
    data : HashMap<&'a str, usize>,
}

impl<'a> RankedSurnames<'a> {
    fn new() -> RankedSurnames<'a> {
        let surnames = include_str!("../data/surnames.txt");
        let mut map : HashMap<&str, usize> = HashMap::new();
        for (linenumber, surname) in surnames.lines().enumerate() {
            map.insert(surname, linenumber);
        }
        RankedSurnames{ data : map }
    }
}

impl<'a> Dictionary for RankedSurnames<'a> {
    fn get_name(&self) -> &str {
        "RankedSurnames"
    }
    fn contains(&self, key : &str) -> bool {
        self.data.contains_key(key)
    }
    fn rank_of(&self, key : &str) -> Rank {
        match self.data.get(key) {
            Some(&rank) => Rank::Ranking(rank),
            None => Rank::NoRank,
        }
    }
}