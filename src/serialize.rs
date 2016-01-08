//! Functions for serializing a scene. The output format is similar to Rust's syntax.
//!
//! Currently only deserializes.

use std::str::Chars;
use std::iter::{Iterator, Peekable};

struct Tokenizer {
    iter: Peekable<Chars>,
}

enum Token {
    Identifier(String),
    Number(f32),
    LBrace(),
    RBrace(),
    Colon(),
}

fn skip_while<E, I: Iterator<E>>(&mut iter: Peekable<I>, fun: F) where F: Fn(&E) -> bool {
    while match iter.peek() {
        None => false,
        Some(e) => fun(e)
    } {
        iter.next();
    }
}

impl Iterator<Token> for Tokenizer {
    fn next(&mut self) -> Option<Token> {
        //
    }
}

pub fn deserialize(text: &str) {
    //
}
