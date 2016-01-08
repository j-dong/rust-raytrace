//! Functions for serializing a scene. The output format is similar to Rust's syntax.
//!
//! Currently only deserializes.

use std::str::Chars;
use std::iter::{Iterator, Peekable};

#[inline]
fn skip_while<I: Iterator>(&mut iter: Peekable<I>, fun: F) where F: Fn(&I::Item) -> bool {
    while match iter.peek() {
        None => false,
        Some(e) => fun(e)
    } {
        iter.next();
    }
}

struct TakeWhile<I: Iterator, 'a> {
    iter: &'a mut Peekable<I>,
}

impl<I: Iterator> Iterator for TakeWhile<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        match self.iter.peek() {
            None => None,
            Some(e) => {
                self.iter.next();
                e
            }
        }
    }
}

#[inline]
fn take_while<I: Iterator, 'a>(&'a mut iter: Peekable<I>, fun: F) -> TakeWhile<I, 'a> where F: Fn(&I::Item) -> bool {
    TakeWhile { iter: iter }
}

enum Token {
    Identifier(String),
    Number(f32),
    LBrace(),
    RBrace(),
    LBracket(),
    RBracket(),
    Colon(),
}

struct Tokenizer {
    iter: Peekable<Chars>,
}

impl Iterator<Token> for Tokenizer {
    fn next(&mut self) -> Option<Token> {
        match self.iter.peek() {
            '{' => Token::LBrace(),
            '}' => Token::RBrace(),
            '[' => Token::LBracket(),
            ']' => Token::RBracket(),
            ':' => Token::Colon(),
            'A' ... 'Z' | 'a' ... 'z' | '_' => Token::Identifier(take_while(&mut self.iter, |c| {c.is_alphanumeric() || c == '_'}).collect()),
            _ => FromStr::from_str(),
        }
    }
}

pub fn deserialize(text: &str) {
    //
}
