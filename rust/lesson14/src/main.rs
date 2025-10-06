// modules in rust 
// Let define a module as Rect and use it in main


use rect::Rect;
pub mod rect;


fn main() {

    let r = Rect{
        width:12.0,
        height:12.0
    };

    println!("{}",r.width);
    println!("{}",r.area());
    Rect::radius();


}
