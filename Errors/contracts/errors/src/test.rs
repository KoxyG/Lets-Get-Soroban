#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Events, vec, Env, IntoVal};

#[test]
fn test_normal_operation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    // INCREMENT
    assert_eq!(client.increment(), 1, "First increment should return 1");
    assert_eq!(client.increment(), 2, "Second increment should return 2");
    assert_eq!(client.increment(), 3, "Third increment should return 3");
    assert_eq!(client.increment(), 4, "Fourth increment should return 4");

    // Get state after 4 increments
    assert_eq!(client.get_state(), State { count: 4, last_incr: 4 });

    // EVENTS
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("STATE"), symbol_short!("increment")).into_val(&env),
                State { count: 1, last_incr: 1}.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("STATE"), symbol_short!("increment")).into_val(&env),
                State { count: 2, last_incr: 2}.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("STATE"), symbol_short!("increment")).into_val(&env),
                State { count: 3, last_incr: 3}.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("STATE"), symbol_short!("increment")).into_val(&env),
                State { count: 4, last_incr: 4}.into_val(&env)
            )
        ],
        "Events should match the four increments"
    );
}

#[test]
#[should_panic(expected = "Error(Contract(1))")]
fn test_error_condition() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    // Increment to the limit
    for _ in 0..4 {
        client.increment();
    }

    // This should panic with LimitReached error
    client.increment();
}

#[test]
fn test_error_and_state() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    // Increment to the limit
    for _ in 0..4 {
        client.increment();
    }

    // Try to increment again, which should fail
    let result = client.try_increment();
    assert!(result.is_err(), "Fifth increment should result in an error");

    // Verify that the state hasn't changed after the error
    assert_eq!(client.get_state(), State { count: 4, last_incr: 4 });
}