/// Enigma machine reflector.
/// Sends signal back through rotors
#[derive(Clone)]
pub struct Reflector{
    name: String,
    wiring: [char; 26],
}
impl Reflector{
    pub fn new(the_name: String, outs: &[char; 26]) -> Reflector{
        
        // an ins array would always just be the alphabet in order

        let mut w: [char; 26] = ['!'; 26];
        for i in 0..26{
            w[i] = outs[i].to_ascii_uppercase();
        }

        Reflector{
            name: the_name,
            wiring: w,
        }

    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    /// Returns letter that is wired to the received letter
    /// expects uppercase letter
    pub(crate) fn encode(&self, c: &char) -> char{
        let index = *c as usize;
        self.wiring[index - 65]
    }
}
