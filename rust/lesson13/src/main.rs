// Struct

// Structs in rust let you structure data together.



/*
    If your struct contain numbers, booleans and Strings
    Then the number and booleans values are stored in stack
    But the string, vector and hashmap are stored in heap and the pointer to
    the string, vector and hashmap is stored in the stack pointing to the
    heap memory


*/




struct Rect{
    height:f32,
    width:f32
}



// Implement function inside the struct
impl  Rect{

    // Member function
    fn area(&self) -> f32{
        return self.width * self.height;
    }

    // Static function
    fn radius(){
        println!("This is reactangle");
    }

}



fn main() {

    let r = Rect{
        width:10.0,
        height:12.0
    };
    println!("{}",r.width);
    println!("{}",r.area());
    Rect::radius();
}
