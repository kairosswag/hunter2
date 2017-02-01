use dictionary;
use dictionary::Dictionary;
use dictionary::Rank;

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
        let matchers : Vec<Box<Matcher>> = vec![dict_matcher];
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
    matcher_name : String,
    matches : Vec<Match>,
}

#[derive(Clone, Copy, Debug)]
struct Match {
    idx_match_start : usize,
    idx_match_end   : usize,
    match_len       : usize,
}

impl Match {
    fn assemble(idx_match_start : usize, idx_match_end : usize) -> Match {
        Match {
            idx_match_start : idx_match_start, 
            idx_match_end : idx_match_end, 
            match_len : idx_match_end - idx_match_start
        }
    }
}

pub trait Matcher {
    fn match_pwd(&self, &str) -> MatchResult;
}

pub struct DictionaryMatcher {
    dicts : Vec<Box<Dictionary>>,
}

impl DictionaryMatcher {
    fn new(dict : Vec<Box<Dictionary>>) -> DictionaryMatcher {
        DictionaryMatcher {dicts : dict}
    }
}

impl Matcher for DictionaryMatcher {

    fn match_pwd(&self, pwd : &str) -> MatchResult {
        let mut matches = Vec::new();
        let pwd_lc = pwd.to_lowercase();
        for dict in &self.dicts {
            for subs_start in 0 .. pwd_lc.len() {
                for subs_end in subs_start + 2 .. pwd_lc.len() + 1 {
                    let pw_sub = &pwd_lc[subs_start..subs_end];
                    if dict.contains(pw_sub) {
                        if let Rank::Ranking(x) = dict.rank_of(pw_sub) {
                            matches.push(Match::assemble(subs_start, subs_end));
                        }
                    }
                }
            }
        }
        MatchResult { matcher_name : "DictionaryMatcher".to_string(), matches : matches}
    }
}
