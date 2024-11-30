
# Let's get soroban: Cross-contract calls.







# What is cross-contract call?

- Cross-contract calls allow one smart contract to interact with another smart contract on the Stellar network. 

- This enables modular development and reuse of existing contract functionality.









- 1. Cross contract calls is also known as inter-contract calls.

- 2. It is made by invoking another contract by its contract ID.

- 3. Contracts to invoke can be imported into your contract with the use of contractimport!(file = "...")










When a contract is imported, the imported contract generate:

- 1. A ContractClient type that can be used to invoke functions on the contract.

- 2. Any types in the contract with #[contracttype].









## What is a Module?.

mod contract_a {
    ....
}


- A module is a way to organize code into logical units and used to manage scope and privacy in Soroban smart contracts.


- It's essentially a namespace that contains code elements like functions, structs, traits, and other items.

- Module is private by Default.

- A pub keyword is being used to make items accessible outside the module(public).









[Read more about cross-contract-calls](https://developers.stellar.org/docs/learn/encyclopedia/contract-development/contract-interactions/cross-contract)
[Read about Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

- Deploy with this:

stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/soroban_workspace_contract_b.wasm \
    --source alice \
    --network testnet



- Invoke your contract with this:

---
stellar contract invoke \
    --id <contract_id B> \
    --source <your_identity> \
    --network testnet \
    -- \
    add_with \
    --contract <contract_id A> \
    --x 5 \
    --y 7
---
