#[macro_use] extern crate lazy_static;

extern crate regex;

mod matching;
mod dictionary;
mod matchers;

/// Represents the strength (or weakness) of a password.
#[derive(Debug)]
pub struct Strength {
    /// estimated guesses needed to crack password
    pub guesses: u32,
    /// order of magnitude of strength.guesses
    pub guesses_log10: u32,
    /// dictionary of back-of-the-envelope crack time estimations in seconds
    pub crack_time_seconds : CrackTimeSeconds,
    /// Integer from 0-4 (useful for implementing a strength bar)
    /// 0 # too guessable: risky password. (guesses < 10^3)
    /// 1 # very guessable: protection from throttled online attacks. (guesses < 10^6)
    /// 2 # somewhat guessable: protection from unthrottled online attacks. (guesses < 10^8)
    /// 3 # safely unguessable: moderate protection from offline slow-hash scenario. (guesses < 10^10)
    /// 4 # very unguessable: strong protection from offline slow-hash scenario. (guesses >= 10^10)
    pub score : u32,
    /// verbal feedback to help choose better passwords. set when score <= 2.
    pub feedback : Feedback,
    /// the list of patterns that zxcvbn based the guess calculation on.
    pub sequence : String,
    /// how long it took zxcvbn to calculate an answer, in milliseconds.
    pub calc_time : u32,
}

#[derive(Debug)]
pub struct CrackTimeSeconds {
    /// online attack on a service that ratelimits password auth attempts.
    online_throttling_100_per_hour : u32,
    /// online attack on a service that doesn't ratelimit,
    /// or where an attacker has outsmarted ratelimiting.
    online_no_throttling_10_per_second : u32,
    /// offline attack. assumes multiple attackers,
    /// proper user-unique salting, and a slow hash function
    /// moderate work factor, such as bcrypt, scrypt, PBKDF2.
    offline_slow_hashing_1e4_per_second : u32,
    /// offline attack with user-unique salting but a fast hash
    /// function like SHA-1, SHA-256 or MD5. A wide range of
    /// reasonable numbers anywhere from one billion - one trillion
    /// guesses per second, depending on number of cores and machines.
    /// ballparking at 10B/sec.
    offline_fast_hashing_10e10_per_second : u32,
}

impl Strength {
    fn new() -> Strength {
        let cts = CrackTimeSeconds {online_throttling_100_per_hour : 0, online_no_throttling_10_per_second : 0, offline_slow_hashing_1e4_per_second : 0, offline_fast_hashing_10e10_per_second : 0};
        let feed = Feedback {warnings: Vec::new(), suggestions: Vec::new()};
        Strength {guesses : 0, guesses_log10 : 0, crack_time_seconds : cts, score : 0, feedback : feed, sequence : "wut".to_string(), calc_time : 0}
    }
}

#[derive(Debug)]
pub struct Feedback {
    /// explains what's wrong, eg. 'this is a top-10 common password'.
    warnings : Vec<Weakness>,
    /// a possibly-empty list of suggestions to help choose a less
    /// guessable password. eg. 'Add another word or two'
    suggestions : Vec<Weakness>,
}


#[derive(Debug, PartialEq)]
pub enum Weakness {
    TooShort,
    NoNumbers,
    None
}

pub fn estimate_strength(pwd: &str) -> Strength {
    let mut omni = matching::Omnimatch::new(pwd);
    for aye in &omni.execute() {
        println!("{:?}", aye);
    }
    println!("uh?");
    Strength::new()
}


#[cfg(test)]
mod tests {

    use super::Weakness;

    #[test]
    fn it_works() {
    }

    #[test]
    fn test_too_short() {
        let _ = super::estimate_strength("peter");
    }
}
