#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct ConstructorContract;

#[contractimpl]
impl ConstructorContract {

    /// Constructor that initializes the counter with a starting value
    pub fn __constructor(env: Env, counter: u32) {
        env.storage().instance().set(&"counter", &counter);
    } 

    /// Returns the current counter value
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&"counter").unwrap()
    }

    /// Increments the counter by 1
    pub fn increment(env: Env) -> u32 {

        let count: u32 = env.storage().instance().get(&"counter").unwrap();

        let new_count = count + 1;
        env.storage().instance().set(&"counter", &new_count);
        new_count
    }
}