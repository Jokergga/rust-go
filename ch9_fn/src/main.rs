fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn change_i32(mut x: i32) {
    x = x + 1;
    println!("x = {}", x);
}

fn modify_i32(x: &mut i32) {
    *x = *x + 1;
}

fn main() {
    let a = 1;
    let b = 2;
    println!("a + b = {}", add(a, b));

    change_i32(a);
    println!("a = {}", a);

    let mut x = 2;
    modify_i32(&mut x);
    println!("x = {}", x);
}
