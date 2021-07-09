use std::fmt;
use crate::parser::or_else::OrElse;
use crate::parser::many::Many;
use crate::parser::and::And;

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
    fn or_else<'p, P2: Parser<A>>(&'p self, other: &'p P2) -> OrElse<Self, P2> 
    where
        Self: Sized
    {
        OrElse::new(self, other)
    }
    fn many<'p>(&'p self) -> Many<'p, Self> 
    where
        Self: Sized
    {
        Many::new(self)
    }
    fn and<'p, B, P2: Parser<B>>(&'p self, other: &'p P2) -> And<'p, Self, P2> 
    where
        Self: Sized
    {
        And::new(self, other)
    }
}

#[allow(dead_code)]
pub fn should_fail<A: std::fmt::Debug>(p: &impl Parser<A>, text: &str) {
    match p.run(text.chars().collect::<Vec<_>>().as_slice()) {
        (_, Ok(o)) => panic!("parser did not fail, succeeded with result {:?}", o),
        _ => ()
    }
}

#[allow(dead_code)]
pub fn succeed_with<A: std::fmt::Debug + std::cmp::Eq>(p: &impl Parser<A>, text: &str, rest: &str, out: A) {
    match p.run(text.chars().collect::<Vec<_>>().as_slice()) {
        (text2, Ok(o)) => {
            assert_eq!(rest.chars().collect::<Vec<_>>().as_slice(), text2);
            assert_eq!(o, out);
        },
        (_, Err(e)) => panic!("parser failed with error {:?}", e)
    }
}
