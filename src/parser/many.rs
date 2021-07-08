use crate::parser::parser::*;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub struct Many<P> {
    p: P
}

impl<P> Many<P> {
    #[allow(dead_code)]
    pub fn new(p: P) -> Many<P>{
        Many{p}
    }
}

impl<A, P: Parser<A>> Parser<VecDeque<A>> for Many<P> {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<VecDeque<A>, ParserError>) {
        let mut rest = text;
        let mut result = VecDeque::new();
        loop {
            match self.p.run(rest) {
                (t, Ok (o)) => {
                    rest = t;
                    result.push_back(o);
                },
                _ => {break;}
            }
        }
        (rest, Ok(result))
    }
}

#[derive(Copy, Clone)]
pub struct Many1<P: Copy> {
    p: P
}

impl<P: Copy> Many1<P> {
    pub fn new(p: P) -> Self {
        Many1{ p }
    }
}

impl<A: Copy, P: Parser<A> + Copy> Parser<VecDeque<A>> for Many1<P> {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<VecDeque<A>, ParserError>) {
        self.p.and_then(|o| {
            self.p.many().map(move |mut v| {
                    v.push_front(o); 
                    v
                })
        }).run(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn many() {
        succeed_with(Any{}.many(), "123", "", VecDeque::from(vec!['1', '2', '3']));
        succeed_with(Any{}.many(), "", "", VecDeque::new());

        succeed_with(Eof{}.many(), "123", "123", VecDeque::new());
        // Eof.many will run forever on "", so this isnt tested
    }

    #[test]
    fn many1() {
        succeed_with(Any{}.many1(), "123", "", VecDeque::from(vec!['1', '2', '3']));
        should_fail(Any{}.many1(), "");

        succeed_with(Eof{}.many(), "123", "123", VecDeque::new());
        // see above for no Eof.many1 on ""
    }
}
