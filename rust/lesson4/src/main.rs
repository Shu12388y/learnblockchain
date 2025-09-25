fn main() {
    println!("The result is {}",is_even(1));
    println!("Hello, world!");
}


// Every function in rust private. To make it pubic we have to make pb
pub fn is_even(num:u8)->bool{
    let digit:u8 = num % 2;
    return digit == 0;

}