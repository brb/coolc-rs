#![feature(io)]
use std::io;
use std::io::Chars;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::collections::HashMap;

struct Token {
}

#[derive(Debug, Clone)]
enum TokenType {
    // keywords
    CLASS,
    ELSE,
    FI,
    IF,
    IN,
    INHERITS,
    LET,
    LOOP,
    POOL,
    THEN,
    WHILE,
    CASE,
    ESAC,
    OF,
    NEW,
    ISVOID,
    NOT,
    // integer
    INT_CONST(String),
    // bool
    BOOL_CONST(bool),
    // type
    TYPEID(String),
    // object
    OBJECTID(String),
    // single char symbol (e.g. ";")
    SSYMBOL(char),
    // <- ???
    ASSIGN,
    // => ???
    DARROW,
    // <= ??
    LE,
    // string
    STR_CONST(String),
    // scanner err
    ERROR(String),
}

struct Scanner<T> {
    read: Chars<T>,
    ch: char,
    pos: usize,
    lineno: usize,
    keywords: HashMap<String, TokenType>,
}

impl<T: io::Read> Scanner<T> {
    fn new(read: Chars<T>) -> Scanner<T> {
        let mut keywords = HashMap::new();
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("fi".to_string(), TokenType::FI);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("in".to_string(), TokenType::IN);
        keywords.insert("inherits".to_string(), TokenType::INHERITS);
        keywords.insert("let".to_string(), TokenType::LET);
        keywords.insert("loop".to_string(), TokenType::LOOP);
        keywords.insert("pool".to_string(), TokenType::POOL);
        keywords.insert("then".to_string(), TokenType::THEN);
        keywords.insert("while".to_string(), TokenType::WHILE);
        keywords.insert("case".to_string(), TokenType::CASE);
        keywords.insert("esac".to_string(), TokenType::ESAC);
        keywords.insert("of".to_string(), TokenType::OF);
        keywords.insert("new".to_string(), TokenType::NEW);
        keywords.insert("isvoid".to_string(), TokenType::ISVOID);
        keywords.insert("not".to_string(), TokenType::NOT);

        Scanner{
            read: read,
            ch: ' ',
            pos: 0,
            lineno: 1,
            keywords: keywords,
            // err ?
        }
    }

    // TODO implement iterator
    fn next(&mut self) -> TokenType {
        self.skip_whitespaces();

        if self.is_digit() {
            let val = self.scan_number();
            return TokenType::INT_CONST(val);
        }
        else if self.is_letter() {
            let is_obj = self.is_uppercase();
            let val = self.scan_identifier();

            if is_obj {
                return TokenType::OBJECTID(val);
            }
            else if val == "true" {
                return TokenType::BOOL_CONST(true);
            }
            else if val == "false" {
                return TokenType::BOOL_CONST(false);
            }
            else if let Some(ttype) = self.keywords.get(&val) {
                return ttype.clone();
            } else {
                return TokenType::TYPEID(val);
            }
        }
        else if self.is_symbol() {
            let ch = self.ch;
            self.read_char();

            if !self.is_symbol() {
                return TokenType::SSYMBOL(ch);
            }
            else if ch == '<' && self.ch == '-' {
                return TokenType::ASSIGN;
            }
            else if ch == '=' && self.ch == '>' {
                return TokenType::DARROW;
            }
            else if ch == '<' && self.ch == '=' {
                return TokenType::LE;
            }
            else if ch == '(' && ch == '*' {
                panic!("NYI skip_comment");
            }
            else if ch == '*' && ch == ')' {
                panic!("ERROR invalid comment");
            }
        }
        else if self.ch == '"' {
            panic!("NYI scan_string");
        }
        else if self.is_eof() {
            panic!("NYI return None");
        }

        TokenType::LE
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
            _ => panic!("read_char"),
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
