use std::fmt;
use crate::parser::and_then::AndThen;
use crate::parser::or_else::OrElse;

pub struct ParserError {
    message: String
}

impl ParserError {
    #[allow(dead_code)]
    pub fn new(message: String) -> ParserError {
        ParserError{ message }
    }
}


impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: {}", self.message)    
    }
}

pub trait Parser<A> {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<A, ParserError>);
    fn and_then<B, P2, F>(&self, other: F) -> AndThen<&Self, F, A>
    where
        P2: Parser<B>,
        F: Fn(A) -> P2,
        Self: Sized
        
    {
        AndThen::new(self, other)
    }
    fn or_else<P2: Parser<A>>(&self, other: P2) -> OrElse<&Self, P2> {
        OrElse::new(self, other)
    }
}

pub struct Any;
impl Parser<char> for Any {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<char, ParserError>) {
        if text.len() > 0 {
            (&text[1..], Ok(text[0]))
        } else {
            (text, Err (ParserError::new(String::from("found eof while parsing any"))))
        }
    }
}

pub struct Eof;
impl Parser<()> for Eof {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<(), ParserError>) {
        if text.len() == 0 {
            (text, Ok (()))
        } else {
            (text, Err (ParserError::new(String::from("remaining chars while parsing eof"))))
        }
    }
}
