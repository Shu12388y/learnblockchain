// Conditions

fn main() {

    let n:u8 = 10;
    // If else
    if n > 5{
        println!("N is greater than 5");
    }else{
        println!("N is less than 5");
    }



    // for loops
    for i in 0 ..6{
        println!("{}",i);
    }


    // while loop
    let mut j = 0; 
    while  j != 12{
        println!("yes..");
        j += 1;
    }



    // match
    let k:u8 = 6;

    match i {
        0 => println!("it is Zero");
        1 | 2 => println!("1,2");
        3 .. =4 => println!("3,4"); // this will go from 3 to 4 as we use ..= it is inclusive
        _ =>println!("default")
    }


    println!("Hello, world!");
}
