use matching::{Match, Matcher, MatchResult};
use dictionary::{Dictionary, Rank};
use regex::Regex;

/// Dictionary Matcher will simply go over all dictionaries given and give all matching substrings
pub struct DictionaryMatcher {
    dicts: Vec<Box<Dictionary>>,
}

/// Date Matcher will try to find patterns for the most common date formats.
pub struct DateMatcher {
    maybe_date_no_sep: Regex,
    maybe_date_sep: Regex,
}

//###################################################### Helper Methods etc.

impl DictionaryMatcher {
    pub fn new(dict: Vec<Box<Dictionary>>) -> DictionaryMatcher {
        DictionaryMatcher { dicts: dict }
    }
}

impl DateMatcher {
    pub fn new() -> DateMatcher {
        DateMatcher {
            maybe_date_no_sep: Regex::new(r"^\d{4,8}$").unwrap(),
            maybe_date_sep:
                Regex::new(r"^((?:\d{1,2})|\d{4})[\.,;\-/](\d{1,2})[\.,;\-/]((?:\d{1,2})|\d{4})$")
                .unwrap(),
        }
    }

    fn valid_no_seperator(elem: &str) -> bool {
        lazy_static! {
            static ref date_reg_4 : Vec<Regex> = {
                let mut date_regs : Vec<Regex> = Vec::new();
                date_regs.push(Regex::new(r"[1-9]{2}[0-9]{2}").unwrap());
                date_regs.push(Regex::new(r"[0-9]{2}[1-9]{2}").unwrap());
                date_regs
            };

            static ref date_reg : Vec<Regex> = {
                let mut date_regs : Vec<Regex> = Vec::new();
                date_regs.push(Regex::new(r"^(?:[01]?[0-9][0-3]?[0-9])(?:[12][0-9])?[0-9]{2}$").unwrap());
                date_regs.push(Regex::new(r"^(?:[0-3]?[0-9][01]?[0-9])(?:[12][0-9])?[0-9]{2}$").unwrap());
                date_regs.push(Regex::new(r"^(?:[12][0-9])?[0-9]{2}(?:[01]?[0-9][0-3]?[0-9])$").unwrap());
                date_regs.push(Regex::new(r"^(?:[12][0-9])?[0-9]{2}(?:[0-3]?[0-9][01]?[0-9])$").unwrap());
                date_regs
            };

        }
        if elem.len() == 4 {
            return date_reg_4.iter().any(|dr| dr.is_match(elem));
        }
        date_reg.iter().any(|dr| dr.is_match(elem))
    }

    fn valid_with_sep(elem1: &str, elem2 : &str, elem3 : &str) -> bool {
        if let Ok(num) = elem2.parse::<i32>()  {
            if num < 1 && num > 12 { return false; }

            let (d1, y1) = DateMatcher::valid_d_y(elem1);
            let (d3, y3) = DateMatcher::valid_d_y(elem3);
            return (d1 && y3) || (d3 && y1);
        } else {
            return false;
        }
    }

    fn valid_d_y(elem : &str) -> (bool, bool) {
        if let Ok(num) = elem.parse::<i32>() {
            let val_d = num < 32 && num > 0;
            
            let val_y = (elem.len() == 4 && num > 999 && num < 3000) 
                            || (elem.len() == 2);
            return (val_d, val_y);
        } else {
            return (false, false);
        }
    }
}


//###################################################### Matcher implementations.


impl Matcher for DictionaryMatcher {
    // Concept:
    // Go through all substrings (longer than 2 chars)
    // and look if they are in any dictionary.
    fn match_pwd(&self, pwd: &str) -> MatchResult {
        let mut matches = Vec::new();
        let pwd_lc = pwd.to_lowercase();
        for dict in &self.dicts {
            for subs_start in 0..pwd_lc.len() {
                for subs_end in subs_start + 2..pwd_lc.len() + 1 {
                    let pw_sub = &pwd_lc[subs_start..subs_end];
                    if dict.contains(pw_sub) {
                        if let Rank::Ranking(rank) = dict.rank_of(pw_sub) {
                            matches.push(Match::assemble(subs_start, subs_end));
                        }
                    }
                }
            }
        }
        MatchResult {
            matcher_name: "DictionaryMatcher".to_string(),
            matches: matches,
        }
    }
}

impl Matcher for DateMatcher {
    // Concept:
    // Find maybe-dates which resemble dates which are here defined as 3-tuple
    // - starting or ending with a 2 or 4 digit year
    // - seperated by 2 or 0 seperators (01.08.15 vs 010815)
    // - having leading zeroes / be zero padded (01.08.15 vs 1.8.15)
    // - a month between 1 and 12 (incl)
    // - a day between 1 and 31 (incl)
    // Note that this is not a date matcher, things like aug 1st 15 will not be matched.
    // Note that in contrast to the original implementation this will allow m-d-y as well as d-m-y
    fn match_pwd(&self, pwd: &str) -> MatchResult {
        let mut matches = Vec::new();
        let len = pwd.len();
        // date without seperators is 4-8 characters; with seperators it's 6-10 characters.
        if len >= 4 {
            for start_pos in 0..len - 4 {
                for window_size in 4..9 {
                    if start_pos + window_size > len {
                        break;
                    };

                    let slice = &pwd[start_pos..start_pos + window_size];
                    if self.maybe_date_no_sep.is_match(slice) &&
                       DateMatcher::valid_no_seperator(slice) {
                        matches.push(Match::assemble(start_pos, start_pos + window_size));
                    }
                }
            }
        }
        if len >= 6 {
            for start_pos in 0..len - 6 {
                for window_size in 6..10 {
                    if start_pos + window_size > len {
                        break;
                    };

                    let slice = &pwd[start_pos..start_pos + window_size];
                    if self.maybe_date_sep.captures_iter(slice)
                            .any(|capt| DateMatcher::valid_with_sep(&capt[0], &capt[1], &capt[2])) {
                        matches.push(Match::assemble(start_pos, start_pos + window_size));
                    }
                }
            }
        }

        MatchResult {
            matcher_name: "DateMatcher".to_string(),
            matches: matches,
        }
    }
}


//###################################################### Matcher tests.
#[cfg(test)]
mod tests {

    use super::DateMatcher;

    #[test]
    fn DateRegexTest() {
        let shall_pass = vec!["01.02.03", "12.2.15", "12.02.15", "12.2.2015", "12.02.2015"];
        let shall_not_pass =
            vec!["..", ".1.", "123.1.1", "12345.1.1", "1.1.123", "1.1.12345", "01.01.12345"];
        let DateMatcher { maybe_date_no_sep, maybe_date_sep } = DateMatcher::new();

        let count = shall_pass.iter().filter(|date| !maybe_date_sep.is_match(date)).count();
        assert_eq!(0, count);

        let count =
            shall_not_pass.iter().filter(|nondate| maybe_date_sep.is_match(nondate)).count();
        assert_eq!(0, count);
    }

    #[test]
    fn Bla() {
        use regex::Regex;
        let maybe_date_sep = Regex::new(r"^(?P<g1>(\d{1,2})|\d{4})[\.,;\-/](?P<g2>\d{1,2})[\.,;\-/](?P<g3>(\d{1,2})|\d{4})$").unwrap();
        let maybe_date_no_sep = Regex::new(r"^\d{4,8}$").unwrap();
        let pwd = "16011993ab";
        let len = pwd.len();
        // date without seperators is 4-8 characters; with seperators it's 5-10 characters.
        for start_pos in 0..len - 4 {
            for window_size in 4..9 {
                if start_pos + window_size > len {
                    break;
                };

                let slice = &pwd[start_pos..start_pos + window_size];
                if maybe_date_no_sep.is_match(slice) {
                    println!("found {:?}", slice);
                }
            }
        }
    }

}