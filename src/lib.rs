mod rotor;
pub use rotor::Rotor;
pub use rotor::RotorState;
//use rotor::RotorError;    

mod machine;
pub use machine::Machine;

mod reflector;
pub use reflector::Reflector;

mod plugboard;
use plugboard::Plugboard;
//pub(crate) use plugboard::PlugError;

pub mod errors{
    pub use crate::plugboard::PlugError;
    pub use crate::rotor::RotorError;
}