/// Enigma machine reflector.
/// Sends signal back through rotors
#[derive(Clone)]
pub struct Reflector{
    wiring: [char; 26],
}
impl Reflector{
    pub fn new(outs: &[char; 26]) -> Reflector{
        
        // an ins array would always just be the alphabet in order

        let mut w: [char; 26] = ['!'; 26];
        for i in 0..26{
            w[i] = outs[i].to_ascii_uppercase();
        }

        Reflector{
            wiring: w,
        }

    }

    /// Returns letter that is wired to the received letter
    /// expects uppercase letter
    pub fn encode(&self, c: &char) -> char{
        let index = (*c as u8) as usize;
        self.wiring[index - 65]
    }
}
