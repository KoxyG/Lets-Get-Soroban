# Let's get soroban: Cross-contract call


- 1. Cross contract calls can also be called inter-contract calls.
- 2. Cross contract calls are made by invoking another contract by its contract ID.
- 3. Contracts to invoke can be imported into your contract with the use of contractimport!(file = "...")


The import will code generate:

- 1. A ContractClient type that can be used to invoke functions on the contract.
- 2. Any types in the contract that were annotated with #[contracttype].