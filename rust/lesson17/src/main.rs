/*
         Option enums
    The option enum was introduce in rust to handle the concept of
    nutability in a safe and expressive way. Unlike many programming 
    language that use a null or similar keyword to represent the 
    absence of a value. Rust doesn't have null


*/

pub enum Option<T>{
    Some(T),
    None

}

fn main() {
    println!("Hello, world!");
}
