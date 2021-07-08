use crate::parser::parser::*;

use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Map<P, F, A> {
    p: P,
    f: F,
    _phantom: PhantomData<A>
}

impl<P, F, A> Map<P, F, A> {
    #[allow(dead_code)]
    pub fn new(p: P, f: F) -> Map<P, F, A> {
        Map{p, f, _phantom: PhantomData}
    }
}

impl<A, B, P: Parser<A>, F: Fn(A) -> B> Parser<B> for Map<P, F, A> {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<B, ParserError>) {
        match self.p.run(text) {
            (rest, Ok(o)) => (rest, Ok((self.f)(o))),
            (_, Err(e)) => (text, Err(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map(){
        succeed_with(Any{}.map(Some), "123", "23", Some('1'));
        should_fail(Any{}.map(Some), "");
    }
}
