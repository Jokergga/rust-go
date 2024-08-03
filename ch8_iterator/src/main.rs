fn main() {
    // ---loop---
    // loop {
    //     println!("Hello, world!");
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }

    // ---while---
    // let mut i = 1;
    // while i < 10 {
    //     println!("i = {}", i);
    //     i += 1;
    // }

    // ---for---
    // let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // for element in arr {
    //     println!("{}", element);
    // };
    // 
    // for i in 0..10 {
    //     println!("{}", i);
    // };
    // for i in 0..=10 {
    //     println!("{}", i);
    // };


    // 循环
    let numbers = [1, 2, 3, 4, 5];
    let mut for_numbers = Vec::new();
    for &number in numbers.iter() {
        let item = number * number;
        for_numbers.push(item);
    }
    println!("for: {:?}", for_numbers);

    // 迭代
    let numbers = [1, 2, 3, 4, 5].to_vec();
    let iter_number: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("iter: {:?}", iter_number);

}
