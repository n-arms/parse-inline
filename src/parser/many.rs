use crate::parser::parser::*;

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

impl<A, P: Parser<A>> Parser<Vec<A>> for Many<P> {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<Vec<A>, ParserError>) {
        let mut rest = text;
        let mut result: Vec<A> = vec![];
        loop {
            match self.p.run(rest) {
                (t, Ok (o)) => {
                    rest = t;
                    result.push(o);
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
        succeed_with(Any{}.many(), "123", "", vec!['1', '2', '3']);
        succeed_with(Any{}.many(), "", "", vec![]);

        succeed_with(Eof{}.many(), "123", "123", vec![]);
        // Eof.many will run forever on "", so this isnt tested
    }
}
