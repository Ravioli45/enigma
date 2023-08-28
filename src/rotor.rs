#[derive(Clone)]
pub struct Rotor{
    forward_shifts: [i8; 26],
    inverse_shifts: [i8; 26],
    turnover: usize,
    turn_offset:usize,
}
impl Rotor{

    ///Takes information and constructs a new rotor
    ///new rotor is returned
    pub fn new(the_f_shifts: [i8;26], the_s_shifts: [i8;26], the_turnover: char) -> Rotor{

        Rotor{
            forward_shifts: the_f_shifts,
            inverse_shifts: the_s_shifts,
            turnover: the_turnover as usize,
            turn_offset: 0,
        }

    }

    ///Turns rotor by incrementing turn_offset.
    /// resets turn_offset to zero when 26 is reached
    pub fn turn(&mut self){
        self.turn_offset += 1;
        if self.turn_offset > 25{
            self.turn_offset -= 26;
        }
    }

    ///encode a character on its first time through rotor
    /// TODO pub(crate)
    pub fn encode_forward(&self, c: &char) -> char{
        let mut index: usize = *c as usize;
        let mut new: i8 = index as i8;

        index -= 97;
        index += self.turn_offset;
        if index > 25{
            index -= 26;
        }

        new += self.forward_shifts[index];

        if new > 122{
            new -= 26;
        }
        else if new < 97{
            new += 26
        }

        //println!("{}", index);
        //println!("{}", new);
        (new as u8) as char
    }
    ///encode a character on its second time through rotor
    pub fn encode_inverse(&self, c: &char) -> char{
        let mut index: usize = *c as usize;
        let mut new: i8 = index as i8;

        index -= 97;
        index += self.turn_offset;
        if index > 25{
            index -= 26;
        }

        new += self.inverse_shifts[index];

        if new > 122{
            new -= 26;
        }
        else if new < 97{
            new += 26;
        }

        //  println!("{}", index);
        //println!("{}", new);
        (new as u8) as char
    }
}