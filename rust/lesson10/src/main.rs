// Ownership and Borrowing

// Ownership
/*
1. Each value in rust has an owner
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped

for example
let mut name = String::from("Shubham");
In this
name --> Owner
"shubham" ---> value ("shubham owner is name")
If the name get out of scope then "shubham will be removed from the heap memory"
It will free up the memory

example 1:


fn main(){
    let name = String::from("Shubham");
    let len = get_len(name);

    let (len,name) = get_len(name);
    println!("{}",len);

    println("{}",name); ----> This will give the error as the name is removed
                        from the heap memory and now the s --> is the owner of
                        the "Shubham" that's why we are not able use name.
                        The solution is the return the ownership.
}


fn get_len(s:String)->usize{
    return s.len();
}

fn get_len(s:String) ->(usize ,String){
    return (s.len(),s);
}

*/

fn main() {
    let mut name = String::from("shubham");
    let _name2 = name;

    name.push_str("paul");

    println!("{}", name);
}
