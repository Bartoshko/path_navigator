#[macro_use]
extern crate error_chain;

use std::cmp::PartialEq;
mod vertex;
mod errors;
mod data;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
