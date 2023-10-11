#[derive(Debug)]
pub enum RotorError{
    InvalidOption
}

/// Represents a singular enigma machine rotor
#[derive(Clone)]
pub struct Rotor{
    forward_shifts: [i8; 26],
    inverse_shifts: [i8; 26],
    turnover: usize,
}
impl Rotor{

    ///Takes information and constructs a new rotor.
    ///The new rotor is returned.
    pub fn new(the_f_shifts: [i8;26], the_i_shifts: [i8;26], the_turnover: char) -> Rotor{

        Rotor{
            forward_shifts: the_f_shifts,
            inverse_shifts: the_i_shifts,
            turnover: (the_turnover.to_ascii_uppercase() as usize) - 65,
        }
    }

    pub fn get_turnover(&self) -> char{
        char::from((self.turnover + 65) as u8)
    }

    /// Encodes a char on its first time through a rotor.
    /// Expects uppercase chars.
    pub(crate) fn encode_forward(&self, c: &char, r: &RotorState) -> char{
        let mut index: usize = *c as usize;
        let mut new: i8 = index as i8;

        index -= 65;
        index += r.turn_offset;
        index += 26 - r.ring_setting;
        if index > 25{
            index -= 26 * (index / 26);
        }

        new += self.forward_shifts[index];

        if new > 90{
            new -= 26;
        }
        else if new < 65{
            new += 26
        }

        //println!("{}", index);
        //println!("{}", new);
        (new as u8) as char
    }
    /// Encodes a character on its second time through a rotor.
    /// Expects uppercase chars.
    pub(crate) fn encode_inverse(&self, c: &char, r: &RotorState) -> char{
        let mut index: usize = *c as usize;
        let mut new: i8 = index as i8;

        index -= 65;
        index += r.turn_offset;
        index += 26 - r.ring_setting;
        if index > 25{
            index -= 26 * (index / 26);
        }

        new += self.inverse_shifts[index];

        if new > 90{
            new -= 26;
        }
        else if new < 65{
            new += 26;
        }

        //  println!("{}", index);
        //println!("{}", new);
        (new as u8) as char
    }
}
/// Represents the things about a rotor that can be changed from the machine.
/// Specifically the current position and ring setting.
#[derive(Clone)]
pub struct RotorState{
    turn_offset: usize,
    ring_setting: usize,
}
impl RotorState{
    pub fn new() -> RotorState{
        RotorState { turn_offset: 0, ring_setting: 0 }
    }
    /// Updates RotorState by incrementing turn_offset.
    /// resets turn_offset to zero when 26 is reached
    pub fn turn(&mut self){
        self.turn_offset += 1;
        if self.turn_offset > 25{
            self.turn_offset -= 26;
        }
    }
    /// Returns position as an uppercase char.
    pub fn get_position(&self) -> char{
        char::from((self.turn_offset+65) as u8)
    }
    /// set rotor position in machine
    pub fn set_position(&mut self, new_position: char) -> Result<(), RotorError>{
        if new_position.is_ascii_alphabetic(){
            self.turn_offset = (new_position.to_ascii_uppercase() as usize) - 65;
            Ok(())
        }
        else{
            //panic!("Invalid position for rotor");
            Err(RotorError::InvalidOption)
        }
    }
    /// gets current ring setting as uppercase char
    pub fn get_ring_setting(&self) -> char{
        char::from((self.ring_setting+65) as u8)
    }
    /// sets ring setting to a given char
    pub fn set_ring_setting(&mut self, new_setting: char) -> Result<(), RotorError>{
        if new_setting.is_ascii_alphabetic(){
            self.ring_setting = (new_setting.to_ascii_uppercase() as usize) - 65;
            Ok(())
        }
        else{
            //panic!("Invalid position for rotor");
            Err(RotorError::InvalidOption)
        }
    }
}
