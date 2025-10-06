fn main() {
    let mut str1 = String::from("shubham");
    let str2 = &mut str1;
    str2.push_str(" Paul");
    println!("{}",str1);


    let str3 = &mut str1;
    println!("{}",str3);

}
