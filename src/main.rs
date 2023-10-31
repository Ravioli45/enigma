use std::io::{Write, self};
use std::str::SplitAsciiWhitespace;
use std::collections::HashMap;
use enigma::{Reflector, Machine, Rotor, RotorState};
use enigma::errors::{RotorError, PlugError};

const TOO_FEW_ARGS: &str = "Not enough arguments given";

/// handles plug related input options
fn plug_handler(words: &mut SplitAsciiWhitespace, machine: &mut Machine){
    //todo!("Implement");

    //let option = words.next();
    //let pair = words.next();
    let (Some(option), Some(pair)) = (words.next(), words.next()) else{
        println!("{}", TOO_FEW_ARGS);
        return;
    };
    //println!("{:?}", pair);
    let result;

    if option == "add"{
        result = machine.add_plug(pair);
    }
    else if option == "remove"{
        result = machine.remove_plug(pair);
    }
    else{
        println!("Invalid option");
        return;
    }

    match result{
        Err(PlugError::InvalidPair) => println!("Invalid pair given"),
        Err(PlugError::LetterIsUsed) => println!("One of the letters is already used in another pair"),
        Err(PlugError::PairNotFound) => println!("Could not find pair to remove"),
        _ => {},
    }
}

fn rotor_handler(words: &mut SplitAsciiWhitespace, machine: &mut Machine){
    let (Some(rotor_option), Some(state_option), Some(char_option)) = (words.next(), words.next(), words.next()) else{
        println!("{}", TOO_FEW_ARGS);
        return;
    };

    // get char from &str
    let char_option: char = match char_option.chars().collect::<Vec<char>>()[..]{
        [c] => Some(c),
        _ => None,
    }.unwrap_or('!');

    let rotor_ref: &mut RotorState = match rotor_option{
        "fast" => machine.get_fast_state_mut_ref(),
        "medium" => machine.get_medium_state_mut_ref(),
        "slow" => machine.get_slow_state_mut_ref(),
        _ => {
            println!("Invalid rotor option");
            return;
        }
    };

    let result: Result<(), RotorError>;
    if state_option == "position"{
        result = rotor_ref.set_position(char_option);
    }
    else if state_option == "ring"{
        result = rotor_ref.set_ring_setting(char_option);
    }
    else{
        println!("Invalid option");
        return;
    }

    match result{
        Err(RotorError::InvalidOption) => println!("Invalid setting"),
        Ok(()) => {},
    }
}

fn set_handler<'a>(words: &mut SplitAsciiWhitespace, machine: &mut Machine<'a>, rotor_map: &HashMap<&str, &'a Rotor>){
    let (Some(rotor_position), Some(rotor)) = (words.next(), words.next()) else{
        println!("{}", TOO_FEW_ARGS);
        return;
    };
    //println!("{}, {}", rotor, rotor_position);
    
    if let Some(r) = rotor_map.get(rotor){
        if rotor_position == "fast"{
            machine.set_fast_rotor(*r);
        }
        else if rotor_position == "medium"{
            machine.set_medium_rotor(*r);
        }
        else if rotor_position == "slow"{
            machine.set_slow_rotor(*r);
        }
        else{
            println!("Invalid rotor position");
        }
    }
    else{
        println!("Invalid option");
    }
}

fn reflector_handler<'a>(words: &mut SplitAsciiWhitespace, machine: &mut Machine<'a>, reflector_map: &HashMap<&str, &'a Reflector>){

    let Some(reflector_option) = words.next() else{
        println!("{}", TOO_FEW_ARGS);
        return;
    };

    if let Some(new_reflector) = reflector_map.get(reflector_option){
        machine.set_reflector(*new_reflector);
    } 
    else{
        println!("Invalid option");
    }
}

fn main(){

    let rotor_one_f: [i8; 26] = [4, 9, 10, 2, 7, 1, -3, 9, 13, 16, 3, 8, 2, 9, 10, -8, 7, 3, 0, -4, -20, -13, -21, -6, -22, -16];
    let rotor_one_i: [i8; 26] = [20, 21, 22, 3, -4, -2, -1, 8, 13, 16, -9, -7, -10, -3, -2, 4, -9, 6, 0, -8, -3, -13, -9, -7, -10, -16];
    let rotor_one: Rotor = Rotor::new("I".to_string(), rotor_one_f, rotor_one_i, 'q');

    let rotor_two_f: [i8; 26] = [0, 8, 1, 7, 14, 3, 11, 13, 15, -8, 1, -4, 10, 6, -2, -13, 0, -11, 7, -6, -5, 3, -17, -2, -10, -21];
    let rotor_two_i: [i8; 26] = [0, 8, 13, -1, 21, 17, 11, 4, -3, -8, -7, -1, 2, 6, 10, 5, 0, -11, -14, -6, -13, 2, -10, -15, -3, -7];
    let rotor_two: Rotor = Rotor::new("II".to_string(), rotor_two_f, rotor_two_i, 'e');

    
    let rotor_three_f: [i8; 26] = [1, 2, 3, 4, 5, 6, -4, 8, 9, 10, 13, 10, 13, 0, 10, -11, -8, 5, -12, -19, -10, -9, -2, -5, -8, -11];
    let rotor_three_i: [i8; 26] = [19, -1, 4, -2, 11, -3, 12, -4, 8, -5, 10, -6, 9, 0, 11, -8, 8, -9, 5, -10, 2, -10, -5, -13, -10, -13];
    let rotor_three: Rotor = Rotor::new("III".to_string(), rotor_three_f, rotor_three_i, 'v');

    let ukw_b_out: [char; 26] = ['y', 'r', 'u', 'h', 'q', 's', 'l', 'd', 'p', 'x', 'n', 'g', 'o', 'k', 'm', 'i', 'e', 'b', 'f', 'z', 'c', 'w', 'v', 'j', 'a', 't'];
    let ukw_b: Reflector = Reflector::new("ukw-b".to_string(), &ukw_b_out);

    let ukw_a_out: [char; 26] = ['e', 'j', 'm', 'z', 'a', 'l', 'y', 'x', 'v', 'b', 'w', 'f', 'c', 'r', 'q', 'u', 'o', 'n', 't', 's', 'p', 'i', 'k', 'h', 'g', 'd'];
    let ukw_a: Reflector = Reflector::new("ukw-a".to_string(), &ukw_a_out);
    //rotor_one.set_position('p');
    //rotor_two.set_position('d');
    //rotor_one.set_ring_setting('B');
    let mut machine_one: Machine = Machine::new(&rotor_one, &rotor_two, &rotor_three, &ukw_b);
    //machine_one.set_fast_position('p').unwrap();
    machine_one.get_fast_state_mut_ref().set_position('p').unwrap();
    //machine_one.set_medium_position('d').unwrap();
    machine_one.get_medium_state_mut_ref().set_position('d').unwrap();
    //machine_one.set_fast_ring('b').unwrap();
    machine_one.get_fast_state_mut_ref().set_ring_setting('b').unwrap();
    match machine_one.add_plug("ob"){
        Err(e) => println!("{:?}", e),
        Ok(_) => println!("yes"),
    }
    match machine_one.add_plug("ab"){
        Err(e) => println!("{:?}", e),
        Ok(_) => println!("yes"),
    }
    //match machine_one.remove_plug("ob"){
    //    Err(e) => println!("{:?}", e),
    //    Ok(_) => println!("yes"),
    //}

    //println!("{}", machine_one.encode_message("abcdefghijklmnopqrstuvwxyz"));
    //println!("{}", machine_one.encode_message("Aoawa zqjzr!"));
    let mut available_rotors: HashMap<&str, &Rotor> = HashMap::with_capacity(3);
    available_rotors.insert(rotor_one.get_name(), &rotor_one);
    available_rotors.insert(rotor_two.get_name(), &rotor_two);
    available_rotors.insert(rotor_three.get_name(), &rotor_three);

    let mut available_reflectors: HashMap<&str, &Reflector> = HashMap::with_capacity(2);
    available_reflectors.insert(ukw_b.get_name(), &ukw_b);
    available_reflectors.insert(ukw_a.get_name(), &ukw_a);

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
        //input.make_ascii_lowercase();

        words = input.split_ascii_whitespace();

        match words.next() {
            Some("exit") => break,
            Some("encode") => {
                print!("~");
                io::stdout().flush().expect("flushless?");
                input.clear();
                io::stdin().read_line(&mut input).expect("kaboom");
                print!("{}", machine_one.encode_message(&input));
            }
            Some("plug") => {
                plug_handler(&mut words, &mut machine_one);
            }
            Some("rotor") => {
                rotor_handler(&mut words, &mut machine_one);
            }
            Some("set") => {
                set_handler(&mut words, &mut machine_one, &available_rotors);
            }
            Some("reflector") => {
                reflector_handler(&mut words, &mut machine_one, &available_reflectors);
            }
            Some("show") => {
                machine_one.show_states();
            }
            None => {/*literally do nothing*/}
            _ => println!("Invalid input")
        }

    };
}
