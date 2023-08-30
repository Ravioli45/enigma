use crate::{Rotor, Reflector};

/// Struct representing an enigma machine
/// Consists of three rotors and a reflector
/// TODO implement reflector
pub struct Machine{
    fast_rotor: Rotor,
    medium_rotor: Rotor,
    slow_rotor: Rotor,
    reflector: Reflector
}
impl Machine{

    pub fn new(the_fast: Rotor, the_medium: Rotor, the_slow: Rotor, the_reflect: Reflector) -> Machine{

        Machine{
            fast_rotor: the_fast,
            medium_rotor: the_medium,
            slow_rotor: the_slow,
            reflector: the_reflect,
        }

    }

    pub fn encode_message(&mut self, message: &str) -> String{

        let mut encoded: String = String::with_capacity(message.len());

        for c in message.chars(){
            let c = c.to_ascii_lowercase();

            //check if char is letter
            if !c.is_alphabetic(){
                encoded.push(c);
                continue;
            }

            //turn first
            if self.fast_rotor.turn(){
                if self.medium_rotor.turn(){
                    self.slow_rotor.turn();
                }
            }


            //encode later
            let mut e: char = self.fast_rotor.encode_forward(&c);
            e = self.medium_rotor.encode_forward(&e);
            e = self.slow_rotor.encode_forward(&e);
            e = self.reflector.encode(&e);
            e = self.slow_rotor.encode_inverse(&e);
            e = self.medium_rotor.encode_inverse(&e);
            e = self.fast_rotor.encode_inverse(&e);

            encoded.push(e);

        };

        return encoded;
    }

}