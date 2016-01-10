//! Functions for serializing a scene. The output format is similar to Rust's syntax.
//!
//! Currently only deserializes.

use std::str::{Chars, FromStr};
use std::fmt;
use std::error::Error;
use std::iter::{Iterator, Peekable};

use ::camera::*;
use ::scene::*;
use ::types::{Vec3, Pnt3};
use ::color::*;

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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}

trait Processable : Iterator {
    fn process(&self, it: Option<Self::Item>, loc: &mut Location) -> Option<Self::Item>;
}

impl<I: Iterator<Item = char>> Processable for I {
    fn process(&self, it: Option<char>, loc: &mut Location) -> Option<char> {
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
            let e = self.iter.next();
            self.peeked = self.iter.process(e, &mut self.location);
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
            None => { let e = self.iter.next(); self.iter.process(e, &mut self.location)} ,
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

trait Expectable<I> {
    fn expect<F>(&mut self, fun: F, desc: &str) -> Result<I, SyntaxError> where F: Fn(&I) -> bool;
}

// TODO: FIXME: TOTAL HACK
impl<I: Processable> Expectable<I::Item> for Acceptor<I> where I::Item: fmt::Debug {
    #[inline]
    fn expect<F>(&mut self, fun: F, desc: &str) -> Result<I::Item, SyntaxError> where F: Fn(&I::Item) -> bool {
        match if let Some(e) = self.peek() {
            if fun(e) {
                // guaranteed take() is not None
                1
            } else {
                2
            }
        } else {
            3
        } {
            1 => Ok(self.take().unwrap()),
            2 => Err(SyntaxError { etype: SyntaxErrorType::Expect(format!("{}, not {:?}", desc, self.peek().unwrap())), location: self.iter.location }),
            3 => Err(SyntaxError { etype: SyntaxErrorType::Expect(format!("{} (end of file)", desc)), location: self.iter.location }),
            _ => panic!("at the disco"),
        }
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

#[derive(Debug)]
enum SyntaxErrorType {
    InvalidToken,
    InvalidNumber { num: String, err: <f32 as FromStr>::Err },
    Expect(String),
    Undefined(String),
    Missing,
    NoClass(String),
}

impl fmt::Display for SyntaxErrorType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyntaxErrorType::InvalidToken => write!(fmt, "invalid token"),
            SyntaxErrorType::InvalidNumber { num: ref num, err: _ } => write!(fmt, "invalid number: {}", num),
            SyntaxErrorType::Expect(ref s) => write!(fmt, "expected {}", s),
            SyntaxErrorType::Undefined(ref s) => write!(fmt, "undefined field: {}", s),
            SyntaxErrorType::Missing => write!(fmt, "missing one or more fields"),
            SyntaxErrorType::NoClass(ref s) => write!(fmt, "no such class: {}", s),
        }
    }
}

#[derive(Debug)]
struct SyntaxError {
    etype: SyntaxErrorType,
    location: Location,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.location, self.etype)
    }
}

impl Error for SyntaxError {
    fn description(&self) -> &str {
        match self.etype {
            SyntaxErrorType::InvalidToken => "invalid token",
            SyntaxErrorType::InvalidNumber { num: _, err: _ } => "invalid number",
            SyntaxErrorType::Expect(_) => "expected something, got another",
            SyntaxErrorType::Undefined(_) => "undefined field",
            SyntaxErrorType::Missing => "missing fields",
            SyntaxErrorType::NoClass(_) => "no such class",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.etype {
            SyntaxErrorType::InvalidNumber { num: _, err: ref e } => Some(e),
            _ => None
        }
    }
}

struct Tokenizer<'a> {
    acceptor: Acceptor<Chars<'a>>,
    error: Option<SyntaxError>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.error.is_some() { return None }
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
                    _ => {
                        self.error = Some(SyntaxError { etype: SyntaxErrorType::InvalidToken, location: self.acceptor.iter.location });
                        return None
                    }
                }
                self.next()
            },
            'A' ... 'Z' | 'a' ... 'z' | '_' => Some(Token::Identifier(self.acceptor.take_while(|c| {match *c {'A' ... 'Z' | 'a' ... 'z' | '0' ... '9' | '_' => true, _ => false}}).collect())),
            '0' ... '9' | '.' | '-' | '+' => {
                let num = self.acceptor.take_while(|c| {match *c {'A' ... 'Z' | 'a' ... 'z' | '0' ... '9' | '_' | '.' | '-' | '+' => true, _ => false}}).collect::<String>();
                match f32::from_str(&num) {
                    Ok(e) => Some(Token::Number(e)),
                    Err(e) => { self.error = Some(SyntaxError { etype: SyntaxErrorType::InvalidNumber { num: num, err: e }, location: self.acceptor.iter.location }); None }
                }
            },
            _ => { self.error = Some(SyntaxError { etype: SyntaxErrorType::InvalidToken, location: self.acceptor.iter.location }); None }
        }
    }
}

impl<'a> Processable for Tokenizer<'a> {
    fn process(&self, it: Option<Token>, loc: &mut Location) -> Option<Token> {
        loc.set(&self.acceptor.iter.location);
        it
    }
}

pub fn deserialize(text: &str) {
    unimplemented!()
}

pub fn print_tokens(text: &str) {
    let mut tokenizer = Tokenizer { acceptor: Acceptor { iter: LL1::new(text.chars()) }, error: None };
    for tok in &mut tokenizer {
        println!("{:?}", tok);
    }
    if let Some(err) = tokenizer.error {
        println!("There was an error:");
        println!("{}", err);
    }
}

#[inline]
fn parse_f32(toks: &mut Acceptor<Tokenizer>) -> Result<f32, SyntaxError> {
    Ok(match try!(toks.expect(|t| {match *t {Token::Number(_) => true, _ => false}}, "Number")) { Token::Number(x) => x, _ => panic!("at the disco") })
}

#[inline]
fn parse_i32(toks: &mut Acceptor<Tokenizer>) -> Result<i32, SyntaxError> {
    let num = try!(parse_f32(toks));
    if num.fract().abs() > 0.01 {
        println!("Warning: {} stored as integer", num);
    }
    if num.abs() > 1677215.0 {
        println!("Warning: integer values past ~2^24+1 are not exact");
    }
    Ok(num.round() as i32)
}

fn parse_vec3(toks: &mut Acceptor<Tokenizer>) -> Result<Vec3, SyntaxError> {
    try!(toks.expect(|t| {match *t {Token::LParen => true, _ => false}}, "LParen"));
    let x = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::Comma => true, _ => false}}, "Comma"));
    let y = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::Comma => true, _ => false}}, "Comma"));
    let z = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::RParen => true, _ => false}}, "RParen"));
    Ok(Vec3::new(x, y, z))
}

fn parse_pnt3(toks: &mut Acceptor<Tokenizer>) -> Result<Pnt3, SyntaxError> {
    try!(toks.expect(|t| {match *t {Token::LParen => true, _ => false}}, "LParen"));
    let x = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::Comma => true, _ => false}}, "Comma"));
    let y = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::Comma => true, _ => false}}, "Comma"));
    let z = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::RParen => true, _ => false}}, "RParen"));
    Ok(Pnt3::new(x, y, z))
}

fn parse_color(toks: &mut Acceptor<Tokenizer>) -> Result<Color, SyntaxError> {
    try!(toks.expect(|t| {match *t {Token::Identifier(ref x) => x == "rgb", _ => false}}, "Identifier(\"rgb\")"));
    try!(toks.expect(|t| {match *t {Token::LParen => true, _ => false}}, "LParen"));
    let r = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::Comma => true, _ => false}}, "Comma"));
    let g = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::Comma => true, _ => false}}, "Comma"));
    let b = try!(parse_f32(toks));
    try!(toks.expect(|t| {match *t {Token::RParen => true, _ => false}}, "RParen"));
    Ok(Color::from_rgb(r, g, b))
}

fn parse_object(toks: &mut Acceptor<Tokenizer>) -> Result<Object, SyntaxError> { unimplemented!() }
fn parse_directional_light(toks: &mut Acceptor<Tokenizer>) -> Result<DirectionalLight, SyntaxError> { unimplemented!() }
fn parse_point_light(toks: &mut Acceptor<Tokenizer>) -> Result<PointLight, SyntaxError> { unimplemented!() }

fn get_light(toks: &mut Acceptor<Tokenizer>) -> Result<Light, SyntaxError> { unimplemented!() }
fn camera_stub() -> Box<Camera> { unimplemented!() }

#[inline]
fn parse_vec<E>(toks: &mut Acceptor<Tokenizer>, parser: fn(&mut Acceptor<Tokenizer>) -> Result<E, SyntaxError>) -> Result<Vec<E>, SyntaxError> {
    try!(toks.expect(|t| {match *t {Token::LBracket => true, _ => false}}, "LBracket"));
    let mut result = Vec::new();
    while toks.accept(|t| {match *t {Token::RBracket => true, _ => false}}).is_none() {
        result.push(try!(parser(toks)));
    }
    // right bracket accepted already
    Ok(result)
}

#[inline]
fn parse_box_light_model<E>(toks: &mut Acceptor<Tokenizer>) -> Result<Box<LightModel>, SyntaxError> {
    if let Token::Identifier(class) = try!(toks.expect(|t| match *t {Token::Identifier(_) => true, _ => false}, "Identifier")) {
        match class.as_ref() {
            "DirectionalLight" => Ok(Box::new(try!(parse_directional_light(toks)))),
            "PointLight" => Ok(Box::new(try!(parse_point_light(toks)))),
            _ => Err(SyntaxError { etype: SyntaxErrorType::NoClass(class), location: toks.iter.location }),
        }
    } else {
        panic!("at the disco");
    }
}

fn parse_scene(toks: &mut Acceptor<Tokenizer>) -> Result<Scene, SyntaxError> {
    try!(toks.expect(|t| {match *t {Token::LBrace => true, _ => false}}, "LBrace"));
    let mut objects = None;
    let mut lights = None;
    while toks.accept(|t| {match *t {Token::RBrace => true, _ => false}}).is_none() {
        if let Token::Identifier(name) = try!(toks.expect(|t| match *t {Token::Identifier(_) => true, _ => false}, "Identifier")) {
            match name.as_ref() {
                "objects" => {
                    try!(toks.expect(|t| {match *t {Token::Colon => true, _ => false}}, "LBrace"));
                    objects = Some(try!(parse_vec(toks, parse_object)));
                },
                "lights" => {
                    try!(toks.expect(|t| {match *t {Token::Colon => true, _ => false}}, "LBrace"));
                    lights = Some(try!(parse_vec(toks, get_light)));
                },
                _ => return Err(SyntaxError { etype: SyntaxErrorType::Undefined(name), location: toks.iter.location }),
            }
        } else {
            panic!("at the disco");
        }
    }
    // right brace accepted already
    match (objects, lights) {
        (Some(objects), Some(lights)) => Ok(Scene { objects: objects, lights: lights, camera: camera_stub() } ),
        _ => Err(SyntaxError { etype: SyntaxErrorType::Missing, location: toks.iter.location }),
    }
}
