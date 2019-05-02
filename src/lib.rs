#[macro_use] extern crate error_chain;
#[macro_use] extern crate approx;

use dbg;
use std::cmp::PartialEq;

mod errors;

pub mod vertex;
pub mod data;
pub mod components;
pub mod dijkstra;

#[cfg(test)]
mod lib_tests {
    #[test]
    fn test_it_works() {
        assert_eq!(2 + 2, 4);
    }
}

