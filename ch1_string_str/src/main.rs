struct Person<'a> {
    name: &'a str,
    color: String,
    age: i32,
}

fn print<'a>(data: &'a str) {
    println!("{}", data);
}

fn print_name<'a>(name: &'a String) {
    println!("{}", name);
}


fn main() {
    // String &str
    // String::from
    // to_string
    // to_owned
    let name = String::from("Value C++");
    // let course = "Rust".to_owned();
    let course = "Rust".to_string();
    let new_name = name.replace("C++", "CPP");
    println!("{name} {course} {new_name}");

    let rust = "\x52\x75\x73\x74";
    println!("{rust}");

    
    let color = "green".to_string();
    let name = "Joker";
    let people = Person {
        name,
        color,
        age: 20,
    };
    // println!("{:?}", people)

    // func
    let value = "value".to_owned();
    print(&value);
    print("Data");

    print_name(&value);
    // error only &String 
    // print_name("Data");
}
