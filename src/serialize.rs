//! Functions for serializing a scene. The output format is similar to Rust's syntax.
//!
//! Currently only deserializes.

use std::str::{Chars, FromStr};
use std::iter::{Iterator, Peekable};

#[derive(Copy, Clone, Debug)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn newline(&mut self) {
        self.row += 1;
        self.col = 0;
    }

    fn next(&mut self) {
        self.col += 1;
    }

    fn set(&mut self, other: &Location) {
        self.row = other.row;
        self.col = other.col;
    }
}

trait Processable : Iterator {
    fn process(it: Option<Self::Item>, loc: &mut Location) -> Option<Self::Item>;
}

impl<I: Iterator<Item = char>> Processable for I {
    fn process(it: Option<char>, loc: &mut Location) -> Option<char> {
        if let Some(c) = it {
            if c == '\n' {
                loc.newline();
            } else {
                loc.next();
            }
        }
        it
    }
}

struct LL1<I: Processable> {
    iter: I,
    location: Location,
    peeked: Option<I::Item>,
}

impl<I: Processable> LL1<I> {
    #[inline]
    fn new(iter: I) -> LL1<I> {
        LL1 { iter: iter, location: Location { row: 1, col: 0 }, peeked: None }
    }

    #[inline]
    fn peek(&mut self) -> Option<&I::Item> {
        if self.peeked.is_none() {
            self.peeked = <I as Processable>::process(self.iter.next(), &mut self.location);
        }
        match self.peeked {
            Some(ref value) => Some(value),
            None => None,
        }
    }
}

impl<I: Processable> Iterator for LL1<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        match self.peeked {
            Some(_) => self.peeked.take(),
            None => <I as Processable>::process(self.iter.next(), &mut self.location),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lo, hi) = self.iter.size_hint();
        if self.peeked.is_some() {
            let lo = lo.saturating_add(1);
            let hi = hi.and_then(|x| x.checked_add(1));
            (lo, hi)
        } else {
            (lo, hi)
        }
    }
}

struct Acceptor<I: Processable> {
    iter: LL1<I>,
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
struct TakeWhile<'a, I: Processable + 'a, F: Fn(&I::Item) -> bool> {
    iter: &'a mut LL1<I>,
    fun: F,
}

impl<'a, I: Processable, F: Fn(&I::Item) -> bool> Iterator for TakeWhile<'a, I, F> where I::Item: Copy {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if match self.iter.peek() { Some(e) => (self.fun)(e), None => false } {
            self.iter.next()
        } else {
            None
        }
    }
}

impl<I: Processable> Acceptor<I> {
    #[inline]
    fn take(&mut self) -> Option<I::Item> { self.iter.next() }

    #[inline]
    fn peek(&mut self) -> Option<&I::Item> { self.iter.peek() }

    #[inline]
    fn accept<F>(&mut self, fun: F) -> Option<I::Item> where F: Fn(&I::Item) -> bool {
        if match self.peek() { Some(e) => fun(e), None => false } {
            self.take()
        } else {
            None
        }
    }

    #[inline]
    fn skip(&mut self) -> bool { self.take().is_some() }

    #[inline]
    fn skip_if<F>(&mut self, fun: F) -> bool where F: Fn(&I::Item) -> bool {
        if match self.peek() { Some(e) => fun(e), None => false } {
            self.skip();
            true
        } else {
            false
        }
    }

    #[inline]
    fn skip_while<F>(&mut self, fun: F) -> bool where F: Fn(&I::Item) -> bool {
        let mut ret = false;
        while match self.peek() { Some(e) => fun(e), None => false } {
            self.skip();
            ret = true;
        }
        return ret;
    }

    #[inline]
    fn take_while<F>(&mut self, fun: F) -> TakeWhile<I, F> where F: Fn(&I::Item) -> bool {
        TakeWhile { iter: &mut self.iter, fun: fun }
    }
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
    acceptor: Acceptor<Chars<'a>>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.acceptor.skip_while(|c: &char| {c.is_whitespace()});
        let c = {
            match self.acceptor.peek() {
                Some(&c) => c,
                None => return None,
            }
        };
        match c {
            '{' => {self.acceptor.skip(); Some(Token::LBrace)},
            '}' => {self.acceptor.skip(); Some(Token::RBrace)},
            '[' => {self.acceptor.skip(); Some(Token::LBracket)},
            ']' => {self.acceptor.skip(); Some(Token::RBracket)},
            '(' => {self.acceptor.skip(); Some(Token::LParen)},
            ')' => {self.acceptor.skip(); Some(Token::RParen)},
            ':' => {self.acceptor.skip(); Some(Token::Colon)},
            ',' => {self.acceptor.skip(); Some(Token::Comma)},
            '#' => {self.acceptor.skip_while(|c| {(*c) != '\n'}); self.next()},
            '/' => {
                self.acceptor.skip(); // discard /
                match self.acceptor.take() {
                    Some('/') => {self.acceptor.skip_while(|c| {(*c) != '\n'});},
                    Some('*') => {loop {
                        self.acceptor.skip_while(|c| {(*c) != '*'});
                        self.acceptor.skip(); // discard *
                        if let Some('/') = self.acceptor.take() {break}
                    }},
                    _ => return None
                }
                self.next()
            },
            'A' ... 'Z' | 'a' ... 'z' | '_' => Some(Token::Identifier(self.acceptor.take_while(|c| {match *c {'A' ... 'Z' | 'a' ... 'z' | '0' ... '9' | '_' => true, _ => false}}).collect())),
            _ => match f32::from_str(&self.acceptor.take_while(|c| {match *c {'A' ... 'Z' | 'a' ... 'z' | '0' ... '9' | '_' | '.' | '-' | '+' => true, _ => false}}).collect::<String>()) {
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
    for tok in (Tokenizer { acceptor: Acceptor { iter: LL1::new(text.chars()) } }) {
        println!("{:?}", tok);
    }
}
