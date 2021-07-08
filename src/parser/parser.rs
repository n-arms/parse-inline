use std::fmt;

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
