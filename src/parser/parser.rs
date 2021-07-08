use std::fmt;
use crate::parser::and_then::AndThen;
use crate::parser::or_else::OrElse;
use crate::parser::many::Many;
use crate::parser::many::Many1;
use crate::parser::map::Map;

#[derive(Debug, Eq, PartialEq)]
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
    fn and_then<B, P2, F>(self, other: F) -> AndThen<Self, F, A>
    where
        P2: Parser<B>,
        F: Fn(A) -> P2,
        Self: Sized
        
    {
        AndThen::new(self, other)
    }
    fn or_else<P2: Parser<A>>(self, other: P2) -> OrElse<Self, P2> 
    where
        Self: Sized
    {
        OrElse::new(self, other)
    }
    fn many(self) -> Many<Self> 
    where
        Self: Sized
    {
        Many::new(self)
    }
    fn map<B, F: Fn(A) -> B>(self, f: F) -> Map<Self, F, A> 
    where
        Self: Sized
    {
        Map::new(self, f)
    }
    fn many1(self) -> Many1<Self>
    where
        Self: Sized + Copy
    {
        Many1::new(self)
    }
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[allow(dead_code)]
pub fn should_fail<A: std::fmt::Debug>(p: impl Parser<A>, text: &str) {
    match p.run(text.chars().collect::<Vec<_>>().as_slice()) {
        (_, Ok(o)) => panic!("parser did not fail, succeeded with result {:?}", o),
        _ => ()
    }
}

#[allow(dead_code)]
pub fn succeed_with<A: std::fmt::Debug + std::cmp::Eq>(p: impl Parser<A>, text: &str, rest: &str, out: A) {
    match p.run(text.chars().collect::<Vec<_>>().as_slice()) {
        (text2, Ok(o)) => {
            assert_eq!(rest.chars().collect::<Vec<_>>().as_slice(), text2);
            assert_eq!(o, out);
        },
        (_, Err(e)) => panic!("parser failed with error {:?}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn any() {
        succeed_with(Any{}, "123", "23", '1');
        succeed_with(Any{}, "1", "", '1');
        should_fail(Any{}, "");
    }

    #[test]
    fn eof() {
        succeed_with(Eof{}, "", "", ());
        should_fail(Eof{}, "1");
    }
}
