#![no_std]
use soroban_sdk::{contracttype, Address, contract, contractimpl, symbol_short, panic_with_error, contracterror, Env, Symbol};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

const STATE: Symbol = symbol_short!("STATE");
const MAX: u32 = 4;


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    LimitReached = 1,
}



#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {

    pub fn get_state(env: Env) -> State {
        env.storage().instance().get(&STATE).unwrap_or(State { count: 0, last_incr: 0 })
    }

    
    pub fn increment(env: Env, user: Address) -> u32 {

        
        user.require_auth();
        
       let mut state = Self::get_state(env.clone()); 
       

       state.count += 1;
       state.last_incr = state.count;

       if state.count <= MAX {   

        env.storage().instance().set(&STATE, &state);
        env.events().publish((STATE, symbol_short!("increment")), state);
        
       state.count
       } else {
            
        panic_with_error!(env, Error::LimitReached);
      }

       
    }
   

}
mod test;