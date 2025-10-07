/*
    Generics

Generics is rust are a way to write flexiable, resuable and type-safe code .
They allow you to define function, structs, enums or methods that can operate
on different types of data. Without knowing the exact types ahead of time.
Instead of specifying a concreate type, you can use a placeholder that will be
replaced by a specific type when the code is compiled.

*/

fn main() {
    let res = sum(1, 2);
    println!("{}", res);
    println!("{}",sum_gen(1,2));
}

fn sum(a: u32, b: u32)->u32 {
    return a + b;
}


// Generies

// std -> standard libary
// ops -> operation
// Add -> Add operation

fn sum_gen<T: std::ops::Add<Output = T>>(a:T,b:T) -> T{
    return a +b;
}