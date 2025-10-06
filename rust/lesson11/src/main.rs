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
        println!("{} {}",s1,s2,s3); --> No Issue 

        In this if s2 try to change the value of then s4 which give error
        we can't change the immutable references

        for that we can have only one immutable reference that can change the value
        for that we have use
        let s2 = &mut s1; ---> Now s2 can change the value of s1



        example 2:
            fn main(){
            
                let mut name = String::from("Shubham");
                let ref1 = &mut name;
                ref1.push_str("paul");
-------------------------------------------------------
                let ref2 = &name;
                println!("{}",ref2);

                This will work as the life span of the ref1 getting ended 
                as we are not using in the below line of code and 
                ref2 will point to name and as ref1 is not changing this value and 
                this will not give the error
                So, this code will work
            
            
            
            }   
        
    
    
    }


*/