// Enums

// Enums in rust are similar to enums similar to typescript.
// They allow you to define a type of enumerating its possible variants

#[warn(dead_code)]
enum Directions {
    North,
    East,
    West,
    South,
}

// Enums with value
enum Shapes {
    Square(u32, u32),
    Circle(u32),
    Rectangle(u32, u32),
}

fn main() {
    // let my_direction = Directions::North;
    // println!("{:?}",my_direction);

    // Pattern matching
    // match my_direction{
    //     Directions::North =>println!("North Direction"),
    //     _ => println!("Default")
    // }

    let circle = Shapes::Circle(32);
    let square = Shapes::Square(12, 12);

    println!("{}",print_area(circle));
}

fn print_area(c: Shapes) -> u32 {
    return match c {
        Shapes::Circle(side) => 2 * 3 * side,
        Shapes::Square(s1, s2) => s1 * s2,
        Shapes::Rectangle(s1,s2) => s1 * s2,
    };
}
