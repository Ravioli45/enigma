use std::collections::HashMap;

const ALPHA: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

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
          w.insert(ALPHA[i], outs[i].to_ascii_uppercase());  
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