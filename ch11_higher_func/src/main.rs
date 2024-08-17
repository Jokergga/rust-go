fn mul_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    // f(f(x))
    f(x)
} 

fn main() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    println!("{}", mul_twice(add_one, 5));

    // 数学计算
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let res1: Vec<_> = numbers.iter().map(|&x| x + x).collect();
    println!("{:?}", res1);

    // filter
    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res2: Vec<_> = res1.iter().filter(|&x| x % 2 == 0).collect();
    println!("{:?}", res2);

    // fold
    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = res2.iter().fold(0, |acc, &x| acc + x );
    println!("sum: {:?}", sum);


}
