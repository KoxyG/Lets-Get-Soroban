#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Events, vec, Env, IntoVal};


// Let's get soroban: Custom Data types


#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CustomDataContract);
    let client = CustomDataContractClient::new(&env, &contract_id);

   // INCREMENT
    assert_eq!(client.increment(), 1);
   
   // EVENTS
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("STATE"), symbol_short!("increment")).into_val(&env),
                State { count: 1, last_incr: 1}.into_val(&env)
            )
        ]
    );

    // DATATYPE
    assert_eq!(client.get_state(), State { count: 1, last_incr: 1 });


}