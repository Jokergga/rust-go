// [dervive(Debug)]
struct Counter {
    number: i32,
}

impl Counter {
    fn new(number: i32) -> Self {
        Self { number }
    }

    // 不可变借用
    fn get_number(&self) -> i32 {
        self.number
    } // Counter::get_number(slef: &Self)
      // 不可变借用
    fn add(&mut self, n: i32) {
        self.number += n;
    }
    // move
    fn give_up(self) {
        println!("{}", self.number);
    }
    fn combine(c1: Self, c2: Self) -> Self {
        Self {
            number: c1.number + c2.number,
        }
    }
}

fn main() {
    let counter = Counter::new(10);
    println!("{}", counter.number);

    let mut c1 = Counter::new(0);
    println!("c1 number {}", c1.get_number());
    println!("c1 number {}", c1.get_number());
    c1.add(2);
    println!("c1 number {}", c1.get_number());
    println!("c1 number {}", c1.get_number());
    c1.give_up();
    // println!("c1 number {}", c1.get_number()); // moved

    let c1 = Counter::new(1);
    let c2 = Counter::new(2);
    let c3 = Counter::combine(c1, c2);
    // error: moved
    // println!("c1 number {}", c1.get_number());
    // println!("c2 number {}", c2.get_number());

    println!("c3 number {}", c3.number);
}
