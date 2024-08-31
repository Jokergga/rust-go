use std::str::Chars;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    /// token 类型
    pub kind: Kind,

    /// 源文本中的起始偏移量
    pub start: usize,

    /// 源文本中的结束偏移量
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof, // 文件结尾
    Plus,
}

struct Lexer<'a> {
    /// 源文本
    source: &'a str,

    /// 剩余的字符
    chars: Chars<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars()
        }
    }

    fn read_next_kind(&mut self) -> Kind {
        while let Some(c) = self.chars.next() {
            match c {
              '+' => return Kind::Plus,
              _ => {}
            }
        }
        Kind::Eof
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        Token { kind, start, end }
    }

    /// 获取从源文本中的偏移长度，以 UTF-8 字节表示
    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}

fn peek(&self) -> Option<char> {
    self.chars.clone().next()
}

fn match_keyword(&self, ident: &str) -> Kind {
    // 所有关键字的长度都在1到10之间
    if ident.len() == 1 || ident.len() > 10 {
        return Kind::Identifier;
    }
    match ident {
        "if" => Kind::If,
        "while" => Kind::While,
        "for" => Kind::For,
        _ => Kind::Identifier
    }
}


fn main() {
    println!("Hello, world!");
}



