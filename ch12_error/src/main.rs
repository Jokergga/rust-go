
fn drvide_func(a: i32, b: i32) -> Result<f64, String> {
    if b == 0 {
        return Err("b can not be zero".to_string());
    }
    let a = a as f64;
    let b = b as f64;
    return Ok(a / b);
}

fn find_element(a: &[i32], b: i32) -> Option<usize> {
    for (i, v) in a.iter().enumerate() {
        if *v == b {
            return Some(i);
        }
    }
    return None;
}

fn main() {
    // result
    match drvide_func(10, 0) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
    match drvide_func(10, 2) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }

    // option
    let arr = [1, 2, 3, 4, 5];
    match find_element(&arr, 3) {
        Some(v) => println!("{}", v),
        None => println!("not found"),
    }
    println!("{:?}", find_element(&arr, 6));

    // panic
    let vec1 = vec![1, 2, 3, 4, 5];
    vec1[43];
}
