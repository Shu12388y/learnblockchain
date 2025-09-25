// In rust, variable are immutiable and we can't reassign the value
// To male it mutable use mut

fn main() {
    let mut sum: u8 = 3;
    sum = 4;
    println!("Hello, world! {}", sum);
}
