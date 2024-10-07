#![no_std]
use soroban_sdk::{contracttype, contract, contractimpl, symbol_short, Env, Symbol};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}


const STATE: Symbol = symbol_short!("STATE");


// Let's get soroban: Custom Data types

#[contract]
pub struct CustomDataContract;

#[contractimpl]
impl CustomDataContract {

    pub fn get_state(env: Env) -> State {
        env.storage().instance().get(&STATE).unwrap_or(State { count: 0, last_incr: 0 })
        // If no value set, assume 0.
    }
   
    pub fn increment(env: Env) -> u32 {


       let mut state = Self::get_state(env.clone());

        
        state.count += 1;
        state.last_incr = state.count;

        
        env.storage().instance().set(&STATE, &state);

 
        env.events().publish((STATE, symbol_short!("increment")), state);
        
        state.count
    }
}
mod test;