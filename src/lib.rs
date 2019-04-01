#[macro_use]
extern crate error_chain;

// use dbg;
use std::cmp::PartialEq;

mod errors;

pub mod vertex;
pub mod data;
pub mod connection;

#[cfg(test)]
mod tests {
    #[test]
    fn test_it_works() {
        assert_eq!(2 + 2, 4);
    }
}
