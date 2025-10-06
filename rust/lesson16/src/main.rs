// Error handling

use std::fs;

// enum Result{
//     OK(String),
//     Err(String),
// }


fn main() {


    let content = fs::read_to_string("a.txt");

    match content{
        Ok(content)=>println!("{}",content),
        Err(e)=>println!("Error in reading file")
    }
}
