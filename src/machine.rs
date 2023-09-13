use crate::{Rotor, Reflector};

// TODO implement plugboard
/// Struct representing an enigma machine
/// Consists of three rotors and a reflector
pub struct Machine<'a>{
    fast_rotor: &'a mut Rotor,
    medium_rotor: &'a mut Rotor,
    slow_rotor: &'a mut Rotor,
    reflector: &'a Reflector
}
impl Machine<'_>{

    pub fn new<'a>(the_fast: &'a mut Rotor, the_medium: &'a mut Rotor, the_slow: &'a mut Rotor, the_reflect: &'a Reflector) -> Machine<'a>{

        Machine{
            fast_rotor: the_fast,
            medium_rotor: the_medium,
            slow_rotor: the_slow,
            reflector: the_reflect,
        }

    }

    pub fn encode_message(&mut self, message: &str) -> String{

        let mut encoded: String = String::with_capacity(message.len());
        let mut was_lowercase: bool;

        let mut medium_will_step: bool = false;
        let mut slow_will_step: bool = false;

        for c in message.chars(){

            //check if char is letter
            if !c.is_alphabetic(){
                encoded.push(c);
                continue;
            }

            was_lowercase = c.is_ascii_lowercase();
            let c = c.to_ascii_uppercase();
            
            //rotors turn one key press after turnover is reached
            if slow_will_step{
                self.slow_rotor.turn();

                //replicates double-step present in Enigma I
                slow_will_step = self.medium_rotor.turn();
            }
            if medium_will_step {
                slow_will_step = self.medium_rotor.turn();
            }
            medium_will_step = self.fast_rotor.turn();

            //encode later
            let mut e: char = self.fast_rotor.encode_forward(&c);
            e = self.medium_rotor.encode_forward(&e);
            e = self.slow_rotor.encode_forward(&e);
            e = self.reflector.encode(&e);
            e = self.slow_rotor.encode_inverse(&e);
            e = self.medium_rotor.encode_inverse(&e);
            e = self.fast_rotor.encode_inverse(&e);

            if was_lowercase{
                encoded.push(e.to_ascii_lowercase());
            }
            else{
                encoded.push(e);
            }

        };

        return encoded;
    }

}
