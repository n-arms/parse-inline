use crate::parser::parser::{Parser, ParserError};

use std::marker::PhantomData;
pub struct AndThen<P, F, A> {
    p: P,
    f: F,
    _phantom: PhantomData<A>
}

impl<P, F, A> AndThen<P, F, A> {
    #[allow(dead_code)]
    pub fn new(p: P, f: F) -> AndThen<P, F, A> {
        AndThen{p, f, _phantom: PhantomData}
    }
}

impl<A, B, P1: Parser<A>, P2: Parser<B>, F: Fn(A) -> P2> Parser<B> for AndThen<P1, F, A> 
{
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<B, ParserError>) {
        match self.p.run(text) {
            (rest, Ok(out)) => {
                (self.f)(out).run(rest)
            },
            (_, Err(e)) => (text, Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::*;

    #[test]
    fn and_then() {
        succeed_with(Any{}.and_then(|_| Any{}), "123", "3", '2');
        should_fail(Any{}.and_then(|_| Any{}), "1");

        succeed_with(Any{}.and_then(|_| Eof{}), "1", "", ());
        should_fail(Any{}.and_then(|_| Eof{}), "12");

        succeed_with(Eof{}.and_then(|_| Eof{}), "", "", ());
        should_fail(Eof{}.and_then(|_| Eof{}), "1");

    }
}
