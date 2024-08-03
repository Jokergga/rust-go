fn main() {
    let num = 90;

    match num {
        80 => println!("80"),
        90 => println!("90"),
        _ => println!("Some else"),
    }

    match num {
        25..=50 => println!("25..=50"),
        51..=100 => println!("51..=100"),
        _ => println!("Some else"),
    }

    match num {
        25 | 50 | 75 => println!("25 or 50 or 75"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Some else"),
    }

    match num {
        x if x > 60 => println!("bad"),
        x if x == 60 => println!("luck"),
        _ => println!("Some else"),
    }

    let num = 60;

    let res = match num {
        x if x < 60 => "bad".to_string(),
        x if x == 60 => "luck".to_string(),
        _ => "Some else".to_string(),
    };

    println!("res value: {}", res);

}
