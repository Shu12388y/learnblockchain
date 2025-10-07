// Generic over Structs


struct Rect<T>{
    width:T,
    height:T
}

// impl<T> here <T> is the  generic  defining the generic
// Rect<T> here is the <T>  using the generic


impl<T> Rect<T>{
    fn area(&self)->T{
        return self.width * self.height;
    }
}

fn main() {
    println!("Hello, world!");
}
