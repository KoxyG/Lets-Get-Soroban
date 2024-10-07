# Custom Type: 

## Struct

- Structs are stored on ledger as a map of key-value pairs, where the key is up to a 32 character string(must not be more than this) which represente the field name, and the value is the value encoded.

#[contracttype]
#[derive(Clone, Copy,  Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

##  Enum

- Enums containing integer values are stored on ledger as the u32 value.

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Enum {
    A = 1,
    B = 2,
} 