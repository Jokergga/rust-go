#[derive(Debug, Copy, Clone)]
struct Book {
    page: i32,
    rating: f64,

}

fn main() {
    let x = vec![1, 2, 3];
    let y = x.clone();
    println!("x: {:?}", x);
    println!("y: {:?}", y);

    let x = "ss".to_string();
    let y = x.clone();
    println!("x: {:?}", x);
    println!("y: {:?}", y);

    let b1 = Book {
        page: 100,
        rating: 4.5,
    };
    let b2 = b1; // move
    println!("b1: {:?}", b1);
    println!("b2: {:?}", b2);

}
