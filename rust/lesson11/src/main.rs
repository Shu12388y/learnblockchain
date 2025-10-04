/*
    Borrowing and references

Rather than transferring ownership to a function,
you can let function borrow the variable


So rather than giving ownership to a variable we can borrow the referance of it


*/

fn main() {
    let name = String::from("Shubham");
    let res = get_len(&name); // We just borrow the reference
    println!("{}", res);
}

fn get_len(s: &String) -> usize {
    return s.len();
}



/*
When you pass a variable by reference, the variable is still owned by first 
function. It is only borrowed by the get_len function


Rules of borrowing

1. you can only have one mutable reference. If there is an mutable reference, there
   can't be other immutable  or mutable reference.

2. You can have multiple immutable references

*/




/*
        Immutable Reference


    fn main(){
        let s1 = String::from("Shubham");
        let s2 = &s1;
        let s3 = &s1;
        println!("{} {}",s1,s2,s3);

        No Issue 

    
    
    
    }


*/