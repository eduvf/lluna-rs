// read.rs

use std::{iter::Peekable, str::Chars};

pub enum Expr {
    Int(i32),
    Flt(f32),
    Sym(String),
    List(Vec<Expr>),
    Func(fn(&[Expr]) -> Result<Expr, ()>)
}

pub enum Tok {
    Paren(char),
    Int(i32),
    Sym(String),
    Atom(char)
}

pub fn lex(code: &str) -> Vec<Tok> {
    let mut tokens: Vec<Tok> = vec![];
    let mut iter = code.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '(' | ')' => {
                tokens.push(Tok::Paren(c));
                println!("Paren: {}", c)
            },
            '0'..='9' => {
                tokens.push(scan_num(c, &mut iter));
            },
            'a'..='z' => {
                todo!()
            }
            _ => {
                tokens.push(Tok::Atom(c));
                println!("{}", c)
            }
        }
    }
    return tokens;
}

fn scan_num(c: char, iter: &mut Peekable<Chars>) -> Tok {
    let mut num_str = c.to_string();
    while let Some(d) = iter.peek() {
        match d {
            '0'..='9' => {
                num_str.push(*d);
                iter.next();
            },
            _ => break
        }
    }
    println!("Int: {}", num_str);
    Tok::Int(num_str.parse::<i32>().unwrap())
}
