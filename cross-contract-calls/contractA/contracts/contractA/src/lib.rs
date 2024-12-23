#![no_std]

use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct ContractA;

// use a snake case name
#[contractimpl]
impl ContractA {
    pub fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }
}