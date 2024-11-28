#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::{Events,  Address as _}, Address, vec, Env, IntoVal};


#[test]
fn test_normal_operation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let user_1 = Address::generate(&env);
    env.mock_all_auths();
   
    // INCREMENT

     assert_eq!(client.increment(&user_1), 1, "First increment should return 1");
     assert_eq!(client.increment(&user_1), 2, "Second increment should return 2");
     assert_eq!(client.increment(&user_1), 3, "Third increment should return 3");
     assert_eq!(client.increment(&user_1), 4, "Fourth increment should return 4");

    // // Get state after 4 increments
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
