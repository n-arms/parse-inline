use crate::parser::parser::{Parser, ParserError};

#[derive(Copy, Clone)]
pub struct OrElse<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<P1, P2> OrElse<P1, P2> {
    #[allow(dead_code)]
    pub fn new(p1: P1, p2: P2) -> OrElse<P1, P2> {
        OrElse {p1, p2}
    }
}

impl<A, P1, P2> Parser<A> for OrElse<P1, P2> 
where
    P1: Parser<A>,
    P2: Parser<A>
{
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<A, ParserError>) {
        if let (rest, Ok (out)) = self.p1.run(text) {
            (rest, Ok (out))
        } else {
            self.p2.run(text)
        }
    }
}
