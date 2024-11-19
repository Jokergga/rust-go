// use std::str::Chars;

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Token {
//     /// token 类型
//     pub kind: Kind,

//     /// 源文本中的起始偏移量
//     pub start: usize,

//     /// 源文本中的结束偏移量
//     pub end: usize,
// }

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Kind {
//     Eof, // 文件结尾
//     Plus,
// }

// struct Lexer<'a> {
//     /// 源文本
//     source: &'a str,

//     /// 剩余的字符
//     chars: Chars<'a>
// }

// impl<'a> Lexer<'a> {
//     pub fn new(source: &'a str) -> Self {
//         Self {
//             source,
//             chars: source.chars()
//         }
//     }

//     fn read_next_kind(&mut self) -> Kind {
//         while let Some(c) = self.chars.next() {
//             match c {
//               '+' => return Kind::Plus,
//               _ => {}
//             }
//         }
//         Kind::Eof
//     }

//     fn read_next_token(&mut self) -> Token {
//         let start = self.offset();
//         let kind = self.read_next_kind();
//         let end = self.offset();
//         Token { kind, start, end }
//     }

//     /// 获取从源文本中的偏移长度，以 UTF-8 字节表示
//     fn offset(&self) -> usize {
//         self.source.len() - self.chars.as_str().len()
//     }
// }

// fn peek(&self) -> Option<char> {
//     self.chars.clone().next()
// }

// fn match_keyword(&self, ident: &str) -> Kind {
//     // 所有关键字的长度都在1到10之间
//     if ident.len() == 1 || ident.len() > 10 {
//         return Kind::Identifier;
//     }
//     match ident {
//         "if" => Kind::If,
//         "while" => Kind::While,
//         "for" => Kind::For,
//         _ => Kind::Identifier
//     }
// }
// fn str_str(haystack: String, needle: String) -> i32 {
//     let n = haystack.len();
//     let m = needle.len();
//     let haystack = haystack.into_bytes();
//     let needle = needle.into_bytes();
//     let mut next = vec![-1; m];
//     println!("{:?}", next);
//     let mut j = -1;
//     let mut i = 0;
//     while i + 1 < m as i32 {
//         if j == -1 || needle[i as usize] == needle[j as usize] {
//             i += 1;
//             j += 1;
//             next[i as usize] = j;
//         } else {
//             j = next[j as usize];
//         }
//     }
//     println!("{:?}", next);
//     i = 0;
//     j = 0;
//     while i < n as i32 {
//         println!("i {i} j {j}");
//         if j == -1 || haystack[i as usize] == needle[j as usize] {
//             i += 1;
//             j += 1;
//         } else {
//             println!("===={j}====");
//             j = next[j as usize];
//             println!("====next{j}====");
//         }
//         if j == m as i32 {
//             return i - m as i32;
//         }
//     }
//     -1
// }

// fn str_str(haystack: String, needle: String) -> i32 {
//     let n = haystack.len();
//     let m: usize = needle.len();
//     let haystack = haystack.into_bytes();
//     let needle = needle.into_bytes();
//     let mut i: i32 = 0;
//     let mut j: i32 = 0;
//     while i < n as i32 {
//         if haystack[i as usize] == needle[j as usize] {
//             i += 1;
//             j += 1;
//         } else {
//             i += 1;
//             j = 0;
//         };
//         if j == m as i32 {
//             return i - m as i32;
//         }
//     }
//     -1
// }

// fn main() {
//     // println!("Hello, world!");
//     let res = str_str(
//         "mississippi".to_string(),
//         "issip".to_string(),
//     );
//     println!("{:?}", res);
// }


// fn main() {
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";
 
//     let records = penguin_data.lines();
 
//     for (i, record) in records.enumerate() {
//       if i == 0 || record.trim().len() == 0 {
//         continue;
//       }
 
//       // 声明一个 fields 变量，类型是 Vec
//       // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//       // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
//       let fields: Vec<_> = record
//         .split(',')
//         .map(|field| field.trim())
//         .collect();
//       if cfg!(debug_assertions) {
//           // 输出到标准错误输出
//         eprintln!("debug: {:?} -> {:?}",
//                record, fields);
//       }
 
//       let name = fields[0];
//       // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
//       //
//       // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//       //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//       //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
//       //
//       // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//       if let Ok(length) = fields[1].parse::<f32>() {
//           // 输出到标准输出
//           println!("{}, {}cm", name, length);
//       }
//     }
//   }
