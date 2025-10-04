// Struct

fn main() {
    let name = String::from("peacock");
    let bird = Birds {
        name: name,
        attack: 5,
    };

    bird.print_name();
    bird.print_attack();
}

// Define a struct
struct Birds {
    name: String,
    attack: i64,
}

// implement methods
impl Birds {
    fn print_name(&self) {
        println!("{}", self.name);
    }

    fn print_attack(&self) {
        println!("{}", self.attack);
    }
}
