# Deployer - Fractory Contract.

- A factory contract is a smart contract that acts like a template, allowing for the creation and deployment of multiple identical smart contracts. 


To know more, check here (https://enlear.academy/factory-contracts-in-blockchain-technology-a-simple-explanation-and-use-case-2e740ec0874f).

- env.mock_all_auths();
It is used to enable mock authorization in order to get the recorded authorization payload that we can verify.



# COMMANDS

COMMAND INSTALLATION:

`stellar contract install --source-account pro --wasm contract/target/wasm32-unknown-unknown/release/hello_world.wasm --network testnet`


HASH: 
`6e9e94c84990d0ca42d6270b07e8c3e0782c3984a71b0ba83ac2242f12a951d9`



INSTALL THE DEPLOYER/  FACTORY CONTRACT

`stellar contract install \
  --wasm factory/target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source pro \
  --network testnet
`

HASH

```cdd48e5b34e439b7ebd1a328f05c8a395aa39935c446f467cb7f62735deb5a9f```


DEPLOY

```stellar contract deploy \
  --wasm-hash cdd48e5b34e439b7ebd1a328f05c8a395aa39935c446f467cb7f62735deb5a9f \
  --source pro \
  --network testnet \
  -- --admin pro
```


DEPLOYED ID

```CCJD7C6RWWATZOITFA475EXSUT2ZGJDLMCL45AQNKAR5OMAV2FMBWNSW```



INVOKE

```stellar contract invoke \
  --id CBHD4BTOGCLPW46ELYLS2ISTJBEPGHGAURUBBMX6BKSZA7APSUOGTKIU \
  --source pro \
  --network testnet \
  -- \
  deploy \
  --wasm_hash 6e9e94c84990d0ca42d6270b07e8c3e0782c3984a71b0ba83ac2242f12a951d9 \
  --salt 123 \
  --constructor_args '[{"u32":5}]'
```
