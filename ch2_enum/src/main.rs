enum Color {
    Red,
    Green,
    Blue,
    Black,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        _ => println!("Black"),
    }
}

enum BuildingLocation {
    Number(i32),
    Name(String), // 不用 &str
    Unknown,
}

impl BuildingLocation {
    fn print_location(&self) {
        match self {
            BuildingLocation::Number(n) => println!("Number: {}", n),
            BuildingLocation::Name(s) => println!("Name: {}", &   s ),
            BuildingLocation::Unknown => println!("Unknown"),
        }
    }
}


fn main() {
    print_color(Color::Red);
    print_color(Color::Black);

    let a = Color::Green;
    print_color(a);
     // let b = a;  a is moved


     let house = BuildingLocation::Name("Joker".to_string());
     house.print_location();
     let house = BuildingLocation::Number(123);
     house.print_location();
     let house = BuildingLocation::Unknown;
     house.print_location();
}
