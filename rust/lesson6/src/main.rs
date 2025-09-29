// Slice 
// slice the arr
// Like Arr = [1,2,4,5]
// slice = [1 .. 3] = [2,4]

fn main() {
    let arr = [1,2,4,5];
    let slice = &arr[1 .. 3];
    println!("{:?}",arr);
    println!("{:?}",slice);
}
