fn find_first_even(numbers: Vec<i32>) -> Option<i32> {
    // for number in numbers {
    //     if number % 2 == 0 {
    //         return Some(*number);
    //     }
    // }
    // None

    let first_event = numbers.iter().find(|&number| number % 2 == 0)?;
    return Some(*first_event);
}

// 传递错误
fn parse_numbers(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let number = input.parse::<i32>()?;
    Ok(number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result_ok: Result<i32, &str> = Ok(32);
    let value = result_ok.unwrap();
    println!("value: {}", value);

    let result_ok: Result<i32, &str> = Ok(45);
    let value = result_ok?;
    println!("value: {}", value);

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    match find_first_even(numbers) {
        Some(value) => println!("find: {}", value),
        None => println!("not find"),
    }

    match parse_numbers("d") {
        Ok(value) => println!("parse: {}", value),
        Err(err) => println!("parse error: {}", err),
    }

    Ok(())
}
