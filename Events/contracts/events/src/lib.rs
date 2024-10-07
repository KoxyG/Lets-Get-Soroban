#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

// Let's get soroban: Events

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
   
    pub fn increment(env: Env) -> u32 {
       
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNTER, &count);

        // env.events().publish(topics, data);
        
        env.events().publish((COUNTER, symbol_short!("increment")), count);


        count
    }
}
mod test;