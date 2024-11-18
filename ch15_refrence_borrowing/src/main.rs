fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);


    let r3 = &mut s;
    println!("{}", r3);

    let result: &str;
    {
        let r4 = &s;
        result = f(r4);
    }

    println!("{}", result);

}


fn f<'a>(s: &'a str) -> &'a str {
    s
}