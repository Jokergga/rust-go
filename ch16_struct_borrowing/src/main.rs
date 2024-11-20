struct MyString<'a> {
    text: &'a str,
}

impl<'a> MyString<'a> {
    fn get_length(&self) -> unsize {
        self.text.len()
    }
    fn modify_data(&mut self) {
        self.text = "world";
    }
}

fn main() {
    let str1 = String::from("value");
    let mut x = MyString {
        text: str1.as_str(),
    };
    x.modify_data();
    println!("{}", x.text);
}
