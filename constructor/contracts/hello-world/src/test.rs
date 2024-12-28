#![cfg(test)]

use crate::{ConstructorContract, ConstructorContractClient};
use soroban_sdk::{Env};


#[test]
fn test_constructor() {
    // Create a test environment
    let env = Env::default();
    

    // Register the contract
    let contract_id = env.register(ConstructorContract, 
        (42_u32,)
    );

    // Create a client for the contract
    let client = ConstructorContractClient::new(&env, &contract_id);

    // Assert the counter value is set correctly
    let current_count = client.get();
    assert_eq!(current_count, 42);

    // Increment the counter
    client.increment();

    // Assert the counter value has incremented
    let incremented_count = client.get();
    assert_eq!(incremented_count, 43);
}
