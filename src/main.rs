use std::io::{Write, self};
use enigma::{Rotor, Reflector, Machine};
use enigma::plugboard::ErrorType;

fn main(){

    let rotor_one_f: [i8; 26] = [4, 9, 10, 2, 7, 1, -3, 9, 13, 16, 3, 8, 2, 9, 10, -8, 7, 3, 0, -4, -20, -13, -21, -6, -22, -16];
    let rotor_one_i: [i8; 26] = [20, 21, 22, 3, -4, -2, -1, 8, 13, 16, -9, -7, -10, -3, -2, 4, -9, 6, 0, -8, -3, -13, -9, -7, -10, -16];
    let rotor_one: Rotor = Rotor::new(rotor_one_f, rotor_one_i, 'q');

    let rotor_two_f: [i8; 26] = [0, 8, 1, 7, 14, 3, 11, 13, 15, -8, 1, -4, 10, 6, -2, -13, 0, -11, 7, -6, -5, 3, -17, -2, -10, -21];
    let rotor_two_i: [i8; 26] = [0, 8, 13, -1, 21, 17, 11, 4, -3, -8, -7, -1, 2, 6, 10, 5, 0, -11, -14, -6, -13, 2, -10, -15, -3, -7];
    let rotor_two: Rotor = Rotor::new(rotor_two_f, rotor_two_i, 'e');

    
    let rotor_three_f: [i8; 26] = [1, 2, 3, 4, 5, 6, -4, 8, 9, 10, 13, 10, 13, 0, 10, -11, -8, 5, -12, -19, -10, -9, -2, -5, -8, -11];
    let rotor_three_i: [i8; 26] = [19, -1, 4, -2, 11, -3, 12, -4, 8, -5, 10, -6, 9, 0, 11, -8, 8, -9, 5, -10, 2, -10, -5, -13, -10, -13];
    let rotor_three: Rotor = Rotor::new(rotor_three_f, rotor_three_i, 'v');

    let ukw_b_out: [char; 26] = ['y', 'r', 'u', 'h', 'q', 's', 'l', 'd', 'p', 'x', 'n', 'g', 'o', 'k', 'm', 'i', 'e', 'b', 'f', 'z', 'c', 'w', 'v', 'j', 'a', 't'];
    let ukw_b: Reflector = Reflector::new(&ukw_b_out);

    //rotor_one.set_position('p');
    //rotor_two.set_position('d');
    //rotor_one.set_ring_setting('B');
    let mut machine_one: Machine = Machine::new(&rotor_one, &rotor_two, &rotor_three, &ukw_b);
    machine_one.set_fast_position('p');
    machine_one.set_medium_position('d');
    machine_one.set_fast_ring('b');
    match machine_one.add_plug("ob"){
        Err(e) => println!("{:?}", e),
        Ok(_) => println!("yes"),
    }
    match machine_one.add_plug("ab"){
        Err(e) => println!("{:?}", e),
        Ok(_) => println!("yes"),
    }
    match machine_one.remove_plug("ob"){
        Err(e) => println!("{:?}", e),
        Ok(_) => println!("yes"),
    }

    //println!("{}", machine_one.encode_message("abcdefghijklmnopqrstuvwxyz"));
    //println!("{}", machine_one.encode_message("Aoawa zqjzr!"));

    let mut input: String = String::new();
    let mut words;
    println!("Rust Enigma v2");
    println!("'exit' to quit");
    loop{

        input.clear();
        print!(">");
        io::stdout().flush().expect("flushless?");
        io::stdin().read_line(&mut input).expect("kaboom");

        // trims whitespace from input
        input = input.trim().to_string();
        input.make_ascii_lowercase();

        words = input.split_ascii_whitespace();

        match words.next() {
            Some("exit") => break,
            Some("encode") => {
                print!("~");
                io::stdout().flush().expect("flushess?");
                input.clear();
                io::stdin().read_line(&mut input).expect("kaboom");
                print!("{}", machine_one.encode_message(&input));
            }
            Some("plug") => {
                // TODO implement
            }
            Some("set") => {
                // TODO implement
            }
            _ => println!("Invalid input")
        }

    };
}
