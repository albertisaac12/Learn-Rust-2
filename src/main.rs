
use crate::garden::vegetables::asparagus;

pub mod garden; // include the code present in src/garden.rs
fn main() {
    println!("Hello, world!");
    asparagus::say_hello();
}
