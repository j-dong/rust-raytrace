//! Functions for serializing a scene. The output format is similar to Rust's syntax.
//!
//! Currently only deserializes.

use std::str::Chars;
use std::iter::Iterator;

struct Tokenizer {
    iter: Chars,
}

enum Token {
    Identifier(String),
    Number(f32),
    LBrace(),
    RBrace(),
    Colon(),
}

impl Iterator<Token> for Tokenizer {
    fn next(&mut self) -> Option<Token> {
        //
    }
}

pub fn deserialize(text: &str) {
    //
}
