pub struct Rotor{
    forward_shifts: [i8; 26],
    inverse_shifts: [i8; 26],
    turnover: char,
    offset:i8,
}
impl Rotor{

    ///Takes information and constructs a new rotor
    ///new rotor is returned
    pub fn new(the_f_shifts: [i8;26], the_s_shifts: [i8;26], the_turnover: char) -> Rotor{

        Rotor{
            forward_shifts: the_f_shifts,
            inverse_shifts: the_s_shifts,
            turnover: the_turnover,
            offset :0,
        }

    }

    ///Turns rotor by incrementing offset.
    /// resets offset to zero when 26 is reached
    pub fn turn(&mut self){
        self.offset += 1;
        if self.offset > 25{
            self.offset = 0;
        }
    }

    /// TODO implement, split into forward and inverse encrypt? 
    /// Takes a character and returns it encoded
    pub fn encode(&self, c: char) -> char{
        c
    }
}