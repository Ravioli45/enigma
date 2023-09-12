/// Represents a singular enigma machine rotor
#[derive(Clone)]
pub struct Rotor{
    forward_shifts: [i8; 26],
    inverse_shifts: [i8; 26],
    turnover: usize,
    turn_offset: usize,
    ring_offset: usize,
}
impl Rotor{

    ///Takes information and constructs a new rotor
    ///new rotor is returned
    pub fn new(the_f_shifts: [i8;26], the_s_shifts: [i8;26], the_turnover: char) -> Rotor{

        Rotor{
            forward_shifts: the_f_shifts,
            inverse_shifts: the_s_shifts,
            turnover: (the_turnover as usize) - 97,
            turn_offset: 0,
            ring_offset: 0,
        }
    }

    ///Turns rotor by incrementing turn_offset.
    /// resets turn_offset to zero when 26 is reached
    pub fn turn(&mut self) -> bool{
        self.turn_offset += 1;
        if self.turn_offset > 25{
            self.turn_offset -= 26;
        }

        self.turn_offset == self.turnover

    }

    /// Gets current rotor Position
    pub fn get_position(&self) -> char{
        char::from((self.turn_offset+65) as u8)
    }

    /// Sets position of rotor to given position
    pub fn set_position(&mut self, new_position: char){
        if new_position.is_ascii_alphabetic(){
            self.turn_offset = (new_position.to_ascii_uppercase() as usize) - 65;
        }
        else{
            panic!("Invalid position for rotor");
        }
    }

    /// Gets current ring position
    pub fn get_ring_setting(&self) -> char{
        char::from((self.ring_offset+65) as u8)
    }

    /// Sets ring setting to given char
    pub fn set_ring_setting(&mut self, new_setting: char){
        if new_setting.is_ascii_alphabetic(){
            self.ring_offset = (new_setting.to_ascii_uppercase() as usize) - 65;
        }
        else{
            panic!("Invalid position for rotor");
        }
    }

    /// encode a character on its first time through rotor
    /// expects uppercase chars
    pub(crate) fn encode_forward(&self, c: &char) -> char{
        let mut index: usize = *c as usize;
        let mut new: i8 = index as i8;

        index -= 65;
        index += self.turn_offset;
        index += 26 - self.ring_offset;
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
    ///encode a character on its second time through rotor
    /// expects uppercase chars
    pub(crate) fn encode_inverse(&self, c: &char) -> char{
        let mut index: usize = *c as usize;
        let mut new: i8 = index as i8;

        index -= 65;
        index += self.turn_offset;
        index += 26 - self.ring_offset;
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
