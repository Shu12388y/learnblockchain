/*
        Traits

    Traits are similar to interface in JS. They let you define the shape/interface
    of what you're building
 */

use std::f64::consts::PI;

 trait Shape{
    fn area(&self)->f64;
 }


struct Circle{
    radius:f64
}

struct Rect{
    width:f64,
    height:f64
}


impl Shape for Circle{
    fn area(&self)->f64{
        return PI * self.radius * self.radius;
    }
}



impl Shape for Rect{
    fn area(&self) -> f64{
        return self.width * self.height;
    }
}



fn print_area<T:Shape>(s:T){
        println!("{}",s.area());
}

fn main() {
    let c = Circle{
        radius:1.2
    };
    let s = Rect{
        width:1.2,
        height:2.3
    };

    print_area(s);
}
