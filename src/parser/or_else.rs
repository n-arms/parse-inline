use crate::parser::parser::*;
use crate::parser::chars::Chars;

pub struct OrElse<'p, P1, P2> {
    p1: &'p P1,
    p2: &'p P2,
}

impl<'p, P1, P2> OrElse<'p, P1, P2> {
    #[allow(dead_code)]
    pub fn new(p1: &'p P1, p2: &'p P2) -> OrElse<'p, P1, P2> {
        OrElse {p1, p2}
    }
}

impl<'p, A, P1, P2> Parser<A> for OrElse<'p, P1, P2> 
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn or_else() {
        let p1 = Chars::new(String::from("banana"));
        let p2 = Chars::new(String::from("banan"));
        succeed_with(&p1.or_else(&p2), "banana", "", String::from("banana"));
        succeed_with(&p1.or_else(&p2), "banan", "", String::from("banan"));
        should_fail(&p1.or_else(&p2), "bana");
    }
}
