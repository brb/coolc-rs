#![feature(io)]
use std::io;
use std::io::Chars;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::collections::HashMap;

// TODO store strings in a symbol table

struct Token {
}

#[derive(Debug, Clone)]
enum TokenType {
    // keywords
    Class,
    Else,
    Fi,
    If,
    In,
    Inherits,
    Let,
    Loop,
    Pool,
    Then,
    While,
    Case,
    Esac,
    Of,
    New,
    Isvoid,
    Not,
    // integer
    IntConst(String),
    // bool
    BoolConst(bool),
    // type
    TypeID(String),
    // object
    ObjectID(String),
    // single char symbol (e.g. ";")
    SSymbol(char),
    // <- ???
    Assign,
    // => ???
    DArrow,
    // <= ??
    Le,
    // string
    StrConst(String),
    // scanner err
    Error(String),
}

struct Error {
    lineno: usize,
    pos: usize,
    msg: String,
}

impl Error {
    fn new(lineno: usize, pos: usize, msg: String) -> Self {
        Error{lineno: lineno, pos: pos, msg: msg}
    }
}

struct Scanner<T> {
    read: Chars<T>,
    ch: char,
    pos: usize,
    lineno: usize,
    keywords: HashMap<String, TokenType>,
    errors: Vec<Error>,
}

impl<T: io::Read> Scanner<T> {
    fn new(read: Chars<T>) -> Scanner<T> {
        let mut keywords = HashMap::new();
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("fi".to_string(), TokenType::Fi);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("in".to_string(), TokenType::In);
        keywords.insert("inherits".to_string(), TokenType::Inherits);
        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("loop".to_string(), TokenType::Loop);
        keywords.insert("pool".to_string(), TokenType::Pool);
        keywords.insert("then".to_string(), TokenType::Then);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("case".to_string(), TokenType::Case);
        keywords.insert("esac".to_string(), TokenType::Esac);
        keywords.insert("of".to_string(), TokenType::Of);
        keywords.insert("new".to_string(), TokenType::New);
        keywords.insert("isvoid".to_string(), TokenType::Isvoid);
        keywords.insert("not".to_string(), TokenType::Not);

        Scanner{
            read: read,
            ch: ' ',
            pos: 0,
            lineno: 1,
            keywords: keywords,
            errors: Vec::new(),
        }
    }

    // TODO implement iterator
    fn next(&mut self) -> TokenType {
        self.skip_whitespaces();

        if self.is_digit() {
            let val = self.scan_number();
            return TokenType::IntConst(val);
        }
        else if self.is_letter() {
            let is_obj = self.is_uppercase();
            let val = self.scan_identifier();

            if is_obj {
                return TokenType::ObjectID(val);
            }
            else if val == "true" {
                return TokenType::BoolConst(true);
            }
            else if val == "false" {
                return TokenType::BoolConst(false);
            }
            else if let Some(ttype) = self.keywords.get(&val) {
                return ttype.clone();
            } else {
                return TokenType::TypeID(val);
            }
        }
        else if self.is_symbol() {
            let ch = self.ch;
            self.read_char();

            if !self.is_symbol() {
                return TokenType::SSymbol(ch);
            }
            else if ch == '<' && self.ch == '-' {
                return TokenType::Assign;
            }
            else if ch == '=' && self.ch == '>' {
                return TokenType::DArrow;
            }
            else if ch == '<' && self.ch == '=' {
                return TokenType::Le;
            }
            else if ch == '(' && ch == '*' {
                self.skip_comment();
                return self.next();
            }
            else if ch == '*' && ch == ')' {
                self.err("Unmateched *)");
                return self.next();
            }
        }
        else if self.ch == '"' {
            panic!("NYI scan_string");
        }
        else if self.is_eof() {
            panic!("NYI return None");
        }

        let ch = self.ch;
        self.err(&format!("Invalid char: {}", ch));
        return self.next();
    }

    fn err(&mut self, msg: &str) {
        let err = Error::new(self.lineno, self.pos, msg.to_string());
        self.errors.push(err);
    }

    fn scan_number(&mut self) -> String {
        // TODO avoid many allocations
        let mut val = String::new();
        while self.is_digit() {
            val.push(self.ch);
            self.read_char();
        }
        val
    }

    fn scan_identifier(&mut self) -> String {
        let mut val = String::new();
        while self.is_digit() || self.is_letter() {
            val.push(self.ch);
            self.read_char();
        }
        val
    }

    fn read_char(&mut self) {
        match self.read.next() {
            None => {
                self.ch = 0 as char;
            },
            Some(Ok(ch)) => {
                self.ch = ch;
            },
            Some(err) => panic!("read_char: {:?}", err),
        }
        self.pos += 1;
    }

    fn is_uppercase(&self) -> bool {
        self.ch >= 'A' && self.ch <= 'Z'
    }

    fn is_letter(&self) -> bool {
        (self.ch >= 'a' && self.ch <= 'z') || (self.ch >= 'A' && self.ch <= 'Z')
    }

    fn is_digit(&self) -> bool {
        self.ch >= '0' && self.ch <= '9'
    }

    fn is_eof(&self) -> bool {
        self.ch == 0 as char
    }

    fn is_symbol(&self) -> bool {
        let c = self.ch;
        c == '{' || c == '}' || c == '(' || c == ')' || c == '*' ||
        c == ':' || c == '<' || c == '>' || c == '-' || c == '@' ||
        c == '.' || c == '=' || c == '+' || c == '/' || c == '~' ||
        c == ';'
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' {
            self.read_char();
        }
    }

    fn skip_comment(&mut self) {
        let mut star = false;
        loop {
            self.read_char();

            if self.ch == '*' {
                star = true;
            }
            else if star && self.ch == ')' {
                break;
            }
            else if self.is_eof() {
                self.err("EOF in comment");
            } else {
                star = false;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut s = Scanner::new(file.chars());
    println!("scanning...");
    let token = s.next();
    println!("done; token={:?}", token);
}
