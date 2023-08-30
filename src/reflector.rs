use std::collections::HashMap;

const ALPHA: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// Enigma machine reflector.
/// Sends signal back through rotors
#[derive(Clone)]
pub struct Reflector{
    wiring: HashMap<char, char>,
}
impl Reflector{
    pub fn new(outs: &[char; 26]) -> Reflector{
        
        let mut w: HashMap<char, char> = HashMap::new();

        for i in 0..26{
          w.insert(ALPHA[i], outs[i]);  
        };

        Reflector{
            wiring: w,
        }

    }

    /// Returns letter that is wired to the received letter
    pub fn encode(&self, c: &char) -> char{
        self.wiring[c]
    }
}