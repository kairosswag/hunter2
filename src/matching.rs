use dictionary;
use matchers::{DictionaryMatcher, DateMatcher};

/// This struct will hold all matchers and execute them.
pub struct Omnimatch<'a> {
    pwd : &'a str,
    matchers : Vec<Box<Matcher>>,
}

impl<'a> Omnimatch<'a> {
    pub fn new(password : &'a str) -> Omnimatch<'a> {
        let mut omni = Omnimatch {pwd : password, matchers : Vec::new()};
        let default_matchers = Omnimatch::get_default();
        omni.set_matchers(default_matchers);
        omni
    }

    fn get_default() -> Vec<Box<Matcher>> {
        let dicts = dictionary::default_dict_lib();
        let dict_matcher = Box::new(DictionaryMatcher::new(dicts));
        let date_matcher = Box::new(DateMatcher::new());
        let matchers : Vec<Box<Matcher>> = vec![
            dict_matcher,
            date_matcher,
        ];
        matchers
    }

    pub fn set_matchers(&mut self, matchers : Vec<Box<Matcher>>) {
        self.matchers = matchers;
    }

    pub fn execute(&mut self) -> Vec<Box<MatchResult>> {
        let mut match_results : Vec<Box<MatchResult>> = Vec::new();
        for matcher in &self.matchers {
            match_results.push(Box::new(matcher.match_pwd(self.pwd)));
        }
        match_results
    }
}

#[derive(Debug)]
pub struct MatchResult {
    pub matcher_name : String,
    pub matches : Vec<Match>,
}

#[derive(Clone, Copy, Debug)]
pub struct Match {
    pub idx_match_start : usize,
    pub idx_match_end   : usize,
    pub match_len       : usize,
}

impl Match {
    pub fn assemble(idx_match_start : usize, idx_match_end : usize) -> Match {
        Match {
            idx_match_start : idx_match_start, 
            idx_match_end : idx_match_end, 
            match_len : idx_match_end - idx_match_start
        }
    }
}

/// Interface for a password matcher.
pub trait Matcher {
    fn match_pwd(&self, &str) -> MatchResult;
}
