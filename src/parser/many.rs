use crate::parser::parser::*;
use crate::parser::chars::Chars;
use std::collections::VecDeque;

pub struct Many<'p, P> {
    p: &'p P
}

impl<'p, P> Many<'p, P> {
    #[allow(dead_code)]
    pub fn new(p: &'p P) -> Self {
        Many{p}
    }
}

impl<'p, A, P: Parser<A>> Parser<VecDeque<A>> for Many<'p, P> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn many() {
        succeed_with(&Chars::new(String::from("1")).many(), "111", "", VecDeque::from(vec![String::from("1"), String::from("1"), String::from("1")]));
        succeed_with(&Chars::new(String::from("1")).many(), "", "", VecDeque::new());
    }
}
