use crate::parser::parser::*;
use crate::parser::chars::Chars;

pub struct And<'p, P1, P2> {
    p1: &'p P1,
    p2: &'p P2,
}

impl<'p, P1, P2> And<'p, P1, P2> {
    #[allow(dead_code)]
    pub fn new(p1: &'p P1, p2: &'p P2) -> Self {
        And {p1, p2}
    }
}

impl<'p, A, B, P1: Parser<A>, P2: Parser<B>> Parser<(A, B)> for And<'p, P1, P2> {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<(A, B), ParserError>) {
        match self.p1.run(text) {
            (rest, Ok(o)) => match self.p2.run(rest) {
                (rest, Ok(o2)) => (rest, Ok((o, o2))),
                (_, Err(e)) => (text, Err(e))
            },
            (_, Err(e)) => (text, Err(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn and() {
        let p1 = Chars::new(String::from("12"));
        let p2 = Chars::new(String::from("34"));
        succeed_with(&p1.and(&p2), "12345", "5", (String::from("12"), String::from("34")));
        should_fail(&p1.and(&p2), "123");
        should_fail(&p1.and(&p2), "1");
    }
}
