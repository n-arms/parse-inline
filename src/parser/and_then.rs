use crate::parser::parser::{Parser, ParserError};
use std::marker::PhantomData;
/*
pub struct AndThen<A, B, P, P2, F> 
where
    P: Parser<A>,
    P2: Parser<B>,
    F: Fn(A) -> P2
{
    first: P,
    second: F,
    _phantom: PhantomData<(A, B, P2)>
}
impl<A, B, P, P2, F> AndThen<A, B, P, P2, F> 
where
    P: Parser<A>,
    P2: Parser<B>,
    F: Fn(A) -> P2
{
    pub fn new(first: P, second: F) -> AndThen<A, B, P, P2, F> {
        AndThen{ first, second, _phantom: PhantomData }
    }
}
impl<A, B, P, P2, F> Parser<B> for AndThen<A, B, P, P2, F>
where
    P: Parser<A>,
    P2: Parser<B>,
    F: Fn(A) -> P2
{
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<B, ParserError>) {
        match self.first.run(text) {
            (rest, Ok(out)) => {
                (self.second)(out).run(rest)
            },
            (_, Err(e)) => (text, Err(e)),
        }
    }
}*/

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
