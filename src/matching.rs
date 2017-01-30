
/// This struct will hold all matchers and execute them.
pub struct Omnimatch<'a> {
    pwd : &'a str,
    matchers : Vec<&'a Matcher>,
}

impl<'a> Omnimatch<'a> {
    pub fn new(password : &'a str) -> Omnimatch<'a> {
        let mut omni = Omnimatch {pwd : password, matchers : Vec::new()};
        let default_matchers = Omnimatch::get_default();
        omni.set_matchers(default_matchers);
        omni
    }

    fn get_default<'b>() -> Vec<&'b Matcher> {
        unimplemented!()
    }

    pub fn set_matchers(&mut self, matchers : Vec<&'a Matcher>) {
        self.matchers = matchers;
    }

    pub fn execute(&mut self) -> Vec<&MatchResult> {
        let mut match_results : Vec<&MatchResult> = Vec::new();
        for matcher in &self.matchers {
            match_results.push(matcher.match_pwd(self.pwd));
        }
        match_results
    }
}

pub struct MatchResult<'a> {
    matcher_name : String,
    matches : Vec<&'a Match>,
}

struct Match {
    idx_match_start : u32,
    idx_match_end   : u32,
    match_len       : u32,
}

pub trait Matcher {
    fn match_pwd(&self, &str) -> &MatchResult;
}
