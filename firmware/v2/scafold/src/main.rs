#![no_std]

// with this structure we can develop algorithm that work in no_std but
// able to test in host pc directly, after algorithm is work and tested,
// we can use this on real no_std mcu, with use this crate as external library

fn main() {
    todo!()
}

fn no_std_add(a: f32, b: f32) -> f32 {
    a + b
}

#[cfg(test)]
extern crate std;

// choose better name for scafold
#[cfg(test)]
mod scafold {

    use super::*;
    use std::println;

    #[test]
    fn test_add_no_std() {
        let result = no_std_add(1.1, 2.2);
        println!("Hello, world!, {result}");
    }
}
