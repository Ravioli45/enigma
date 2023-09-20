use std::fmt;

#[derive(Debug)]
pub enum ErrorType{
    InvalidPair,
    PairNotFound,
    LetterIsUsed,
}

/// Represents a pair of linked letters on a plugboard
#[derive(Clone)]
struct PlugboardPair{
    first: char,
    second: char
}
impl PlugboardPair{
    /// Create a new plugboard pair
    pub fn new(the_first: char, the_second: char) -> PlugboardPair{
        PlugboardPair{
            first: the_first,
            second: the_second
        }
    }

    /// check if a char c in contained in the pair
    pub fn contains(&self, c: &char) -> bool{
        self.first == *c || self.second == *c
    }

    /// return other Some pair member if c in pair,
    /// otherwise None
    pub fn other(&self, c: &char) -> Option<char>{
        if *c == self.first{
            Some(self.second)
        }
        else if *c == self.second{
            Some(self.first)
        }
        else{
            None
        }
    }
}
impl fmt::Display for PlugboardPair{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}{}", self.first, self.second)
    }
}

pub(crate) struct Plugboard{
    pairs: Vec<PlugboardPair>
}
impl Plugboard{
    pub fn new() -> Plugboard{
        Plugboard{
            pairs: Vec::<PlugboardPair>::with_capacity(13)
        }
    }

    pub fn make_pair(&mut self, pair: &str) -> Result<(), ErrorType>{
        // if pair can be turned into a valid pair
        if Plugboard::is_valid_pair(pair){
            // seperate the chars that represent the pair
            let byte_pair = pair.as_bytes();
            let first: char = byte_pair[0].to_ascii_uppercase() as char;
            let second: char = byte_pair[1].to_ascii_uppercase() as char;

            // check if one of the letters is already used in a pair
            for pair in &self.pairs{
                if pair.contains(&first) || pair.contains(&second){
                    //println!("Letters can't be used in more than one pair");
                    return Err(ErrorType::LetterIsUsed);
                }
            }
            self.pairs.push(PlugboardPair::new(first, second));
            Ok(())
        }
        else{
            Err(ErrorType::InvalidPair)
        }
    }

    pub fn remove_pair(&mut self, pair: &str) -> Result<(), ErrorType>{
        // if pair represents a valid pair
        if Plugboard::is_valid_pair(pair){
            // seperate the chars that represent the pair
            let byte_pair = pair.as_bytes();
            let first: char = byte_pair[0].to_ascii_uppercase() as char;
            let second: char = byte_pair[1].to_ascii_uppercase() as char;
            
            for i in 0..self.pairs.len(){
                if self.pairs[i].contains(&first) && self.pairs[i].contains(&second){
                    //index = i.clone();
                    //found = true;
                    // oh boy
                    self.pairs.remove(i);
                    return Ok(());
                }
            }
            Err(ErrorType::PairNotFound)
        }
        else{
            Err(ErrorType::InvalidPair)
        }
    }

    /// Returns corresponding char from plugboard
    /// or same char if it is not found in any pairs
    pub fn swap_char(&self, c: &char) -> char{
        for pair in &self.pairs{
            if let Some(s) = pair.other(c){
                return s;
            }
        };
        return *c;
    }

    /// returns true of pair is valid and false otherwise
    fn is_valid_pair(pair: &str) -> bool{
        if pair.len() == 2{
            let mut cs = pair.chars();
            let first: char = cs.nth(0).unwrap();
            let second: char = cs.nth(0).unwrap();

            (first.is_ascii_alphabetic() && second.is_ascii_alphabetic()) && (first != second)
        }
        else{
            false
        }
    }

}