use crate::{Rotor, Reflector, Plugboard, plugboard::PlugError, RotorState};

/// Struct representing an enigma machine
/// Consists of three rotors and a reflector
pub struct Machine<'a>{
    fast_rotor: &'a Rotor,
    medium_rotor: &'a Rotor,
    slow_rotor: &'a Rotor,
    fast_state: RotorState,
    medium_state: RotorState,
    slow_state: RotorState,
    reflector: &'a Reflector,
    plugboard: Plugboard,
}
impl Machine<'_>{

    pub fn new<'a>(the_fast: &'a Rotor, the_medium: &'a Rotor, the_slow: &'a Rotor, the_reflect: &'a Reflector) -> Machine<'a>{

        Machine{
            fast_rotor: the_fast,
            medium_rotor: the_medium,
            slow_rotor: the_slow,
            fast_state: RotorState::new(),
            medium_state: RotorState::new(),
            slow_state: RotorState::new(),
            reflector: the_reflect,
            plugboard: Plugboard::new(),
        }

    }

    pub fn encode_message(&mut self, message: &str) -> String{

        let mut encoded: String = String::with_capacity(message.len());
        let mut was_lowercase: bool;

        let mut medium_will_step: bool = self.fast_rotor.get_turnover() == self.fast_state.get_position();
        let mut slow_will_step: bool = self.slow_rotor.get_turnover() == self.slow_state.get_position();

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
                self.slow_state.turn();

                //replicates double-step present in Enigma I
                self.medium_state.turn();
                slow_will_step = self.medium_rotor.get_turnover() == self.medium_state.get_position();
            }
            if medium_will_step {
                self.medium_state.turn();
                slow_will_step = self.medium_rotor.get_turnover() == self.medium_state.get_position();
            }
            self.fast_state.turn();
            medium_will_step = self.fast_rotor.get_turnover() == self.fast_state.get_position();

            //encode later
            let mut e: char = self.plugboard.swap_char(&c);

            e = self.fast_rotor.encode_forward(&e, &self.fast_state);
            e = self.medium_rotor.encode_forward(&e, &self.medium_state);
            e = self.slow_rotor.encode_forward(&e, &self.slow_state);
            e = self.reflector.encode(&e);
            e = self.slow_rotor.encode_inverse(&e, &self.slow_state);
            e = self.medium_rotor.encode_inverse(&e, &self.medium_state);
            e = self.fast_rotor.encode_inverse(&e, &self.fast_state);

            e = self.plugboard.swap_char(&e);

            if was_lowercase{
                encoded.push(e.to_ascii_lowercase());
            }
            else{
                encoded.push(e);
            }

        };

        return encoded;
    }

    // guess what these do
    pub fn set_fast_position(&mut self, c: char){
        self.fast_state.set_position(c);
    }
    pub fn set_medium_position(&mut self, c: char){
        self.medium_state.set_position(c);
    }
    pub fn set_slow_position(&mut self, c: char){
        self.slow_state.set_position(c);
    }
    pub fn set_fast_ring(&mut self, c: char){
        self.fast_state.set_ring_setting(c);
    }
    pub fn set_medium_ring(&mut self, c: char){
        self.medium_state.set_ring_setting(c);
    }
    pub fn set_slow_ring(&mut self, c: char){
        self.slow_state.set_ring_setting(c);
    }

    pub fn add_plug(&mut self, pair: &str) -> Result<(), PlugError>{
        self.plugboard.make_pair(pair)
    }

    pub fn remove_plug(&mut self, pair: &str) -> Result<(), PlugError>{
        self.plugboard.remove_pair(pair)
    }
}
