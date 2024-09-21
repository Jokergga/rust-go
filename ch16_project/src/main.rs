#![allow(unused_variables)]
// type File = String;

// fn open(f: &mut File) -> bool {
//     true
// }
// fn close(f: &mut File) -> bool {
//     true
// }

// #[allow(dead_code)]
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

// fn main() {
//     // let mut f1 = File::from("f1.txt");
//     // open(&mut f1);
//     // //read(&mut f1, &mut vec![]);
//     // close(&mut f1);

//     let s = "中国人";
//     let a = &s[0..3];
//     println!("{}",a);
// }

struct Unit;
trait SomeTrait {
    // ...定义一些行为
}

// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);
} 

// 填空，让代码工作
fn do_something_with_unit(u: Unit) {   }