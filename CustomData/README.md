// Let's get soroban: Custom Data types





# Custom Type: 

Custom types are defined using the #[contracttype] attribute on either a struct or an enum.



## Struct

- Structs are stored on ledger as a map of key-value pairs.



// Struct Example

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}




##  Enum (integer) Example.

- Enums containing integer values are stored on ledger as 32-bit unsigned integer (u32).

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum State {
    A = 1,
    B = 2,
} 



## Enum (unit and tuple) Example.
- Enums containing unit and tuple variants are stored on ledger as a two element vector, 

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    A,
    B(..),
}