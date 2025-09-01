#![no_std]

pub mod ble;
pub mod fmt;
pub mod middleware;
pub mod models;
pub mod reader;
pub mod services;
pub mod writer;

pub fn no_std_add(a: f32, b: f32) -> f32 {
    a + b
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_no_std_add() {
        let result = no_std_add(2.3, 5.4);
        println!(result);
    }
}
