//! Functions for serializing a scene. The output format is similar to Rust's syntax.
//!
//! Currently only deserializes.

use std::str::{Chars, FromStr};
use std::iter::{Iterator, Peekable};

#[inline]
fn skip_while<I: Iterator, F>(iter: &mut Peekable<I>, fun: F) -> bool where F: Fn(&I::Item) -> bool {
    let mut ret = false;
    while match iter.peek() {
        None => false,
        Some(e) => fun(e)
    } {
        ret = true;
        iter.next();
    }
    return ret;
}

struct TakeWhile<'a, I: Iterator + 'a, F: Fn(&I::Item) -> bool> {
    iter: &'a mut Peekable<I>,
    fun: F,
}

impl<'a, I: Iterator, F: Fn(&I::Item) -> bool> Iterator for TakeWhile<'a, I, F> where I::Item: Copy {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        while let Some(&el) = self.iter.peek() {
            if (self.fun)(&el) {
                self.iter.next();
                return Some(el)
            } else {
                return None
            }
        }
        None
    }
}

#[inline]
fn take_while<'a, I: Iterator, F>(iter: &'a mut Peekable<I>, fun: F) -> TakeWhile<'a, I, F> where F: Fn(&I::Item) -> bool {
    TakeWhile { iter: iter, fun: fun }
}

#[derive(Debug)]
enum Token {
    Identifier(String),
    Number(f32),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Colon,
    Comma,
}

struct Tokenizer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        skip_while(&mut self.iter, |c: &char| {c.is_whitespace()});
        let c = {
            let ch = self.iter.peek();
            match ch {
                Some(c) => *c,
                None => return None,
            }
        };
        match c {
            '{' => {self.iter.next(); Some(Token::LBrace)},
            '}' => {self.iter.next(); Some(Token::RBrace)},
            '[' => {self.iter.next(); Some(Token::LBracket)},
            ']' => {self.iter.next(); Some(Token::RBracket)},
            '(' => {self.iter.next(); Some(Token::LParen)},
            ')' => {self.iter.next(); Some(Token::RParen)},
            ':' => {self.iter.next(); Some(Token::Colon)},
            ',' => {self.iter.next(); Some(Token::Comma)},
            '#' => {skip_while(&mut self.iter, |c| {(*c) != '\n'}); self.next()},
            '/' => {self.iter.next(); match self.iter.next() {
                Some('/') => {skip_while(&mut self.iter, |c| {(*c) != '\n'});},
                Some('*') => {loop {
                    skip_while(&mut self.iter, |c| {(*c) != '*'});
                    self.iter.next();
                    if let Some('/') = self.iter.next() {break}
                }},
                _ => return None
            }; self.next()},
            'A' ... 'Z' | 'a' ... 'z' | '_' => Some(Token::Identifier(take_while(&mut self.iter, |c| {match *c {'A' ... 'Z' | 'a' ... 'z' | '0' ... '9' | '_' => true, _ => false}}).collect())),
            _ => match f32::from_str(&take_while(&mut self.iter, |c| {match *c {'A' ... 'Z' | 'a' ... 'z' | '0' ... '9' | '_' | '.' | '-' | '+' => true, _ => false}}).collect::<String>()) {
                Ok(e) => Some(Token::Number(e)),
                Err(_) => None
            },
        }
    }
}

pub fn deserialize(text: &str) {
    unimplemented!()
}

pub fn print_tokens(text: &str) {
    for tok in (Tokenizer { iter: text.chars().peekable() }) {
        println!("{:?}", tok);
    }
}
