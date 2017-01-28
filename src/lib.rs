

#[derive(Debug, PartialEq)]
pub enum Weakness {
    TooShort,
    NoNumbers,
    None
}

pub fn estimate_strenght(password: &str) -> (i32, Weakness) {
    simple::estimate_by_lenght(password)
}

mod simple {
    use super::Weakness;

    /// estimates password strenght according to its lenght
    pub fn estimate_by_lenght(password: &str) -> (i32, Weakness) {
        let len = password.len();
        match len {
            1 ... 5 => (1, Weakness::TooShort),
            _ => (10, Weakness::None),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::Weakness;

    #[test]
    fn it_works() {
    }

    #[test]
    fn test_too_short() {
        let (_, weak) = super::estimate_strenght("abc");
        assert_eq!(Weakness::TooShort, weak);
    }
}
