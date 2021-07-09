use crate::parser::parser::*;

pub struct Chars {
    s: String
}

impl Chars {
    #[allow(dead_code)]
    pub fn new(s: String) -> Self {
        Chars {s}
    }
}

impl Parser<String> for Chars {
    fn run<'a>(&self, text: &'a [char]) -> (&'a [char], Result<String, ParserError>) {
        if text.len() < self.s.len() {
            (text, Err(ParserError::new(String::from("text is not long enough in Chars.run"))))
        } else if text.iter()
            .zip(self.s.chars())
            .all(|(x, y)| x == &y)
        {
            (&text[self.s.len()..], Ok(text[..self.s.len()].iter().collect()))
        } else {
            (text, Err(ParserError::new(String::from("text does not match chars in Chars.run"))))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chars() {
        let p = Chars::new(String::from("123"));
        succeed_with(&p, "123456", "456", String::from("123"));
        should_fail(&p, "12");
        should_fail(&p, "456");
    }
}
