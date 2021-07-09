use crate::generator::parser::expr::*;

fn get<'a>(text: &'a [char]) -> (&'a [char], Option<char>) {
    if text.len() == 0 {
        (text, None)
    } else {
        (&text[1..], Some(text[0]))
    }
}

fn skip_whitespace<'a>(text: &'a [char]) -> &'a [char] {
    if text[0] == ' ' || text[0] == '\n' {
        skip_whitespace(&text[1..])
    } else {
        text
    }
}

pub fn parse_rules<'a>(text: &'a [char]) -> (&'a [char], Result<Rules, String>) {
    todo!();
}

pub fn parse_rule<'a>(text: &'a [char]) -> (&'a [char], Result<Rule, String>) {
    todo!();
}

pub fn parse_expr<'a>(text: &'a [char]) -> (&'a [char], Result<Expr, String>) {
    todo!();
}


pub fn parse_and<'a>(text: &'a [char]) -> (&'a [char], Result<(Expr, Expr), String>) {
    todo!();
}

pub fn parse_literal<'a>(text: &'a [char]) -> (&'a [char], Result<Literal, String>) {
    match parse_non_term(text) {
        (text, Ok(l)) => (text, Ok(Literal::NonTerm(l))),
        _ => match parse_term(text) {
            (text, Ok(l)) => (text, Ok(Literal::Term(l))),
            _ => (text, Err(String::from("failed to parse term or nonterm")))
        }
    }
}

pub fn parse_non_term<'a>(text: &'a [char]) -> (&'a [char], Result<String, String>) {
    let mut nt = String::new();
    let mut i = 0;
    while text.len() > i && text[i].is_alphabetic() {
        nt.push(text[i]);
        i += 1;
    }
    if i == 0 {
        (text, Err(String::from("found 0 alpha chars")))
    } else {
        (&text[i..], Ok(nt))
    }
}

pub fn parse_term<'a>(text: &'a [char]) -> (&'a [char], Result<String, String>) {
    let mut rest: &'a [char] = text;
    let open = get(rest);

    if let (_, None) = open {
        return (text, Err(String::from("eof during parse string")))
    } else if let (text, Some(c)) = open {
        if c != '"' {
            return (text, Err(String::from("failed to parse \" during string")))
        }
        rest = text;
    }
    let mut i = 0;
    let mut s = String::new();
    while rest.len() > i && rest[i] != '"' {
        s.push(rest[i]);
        i += 1;
    }
    match get(&rest[i..]) {
        (rest, Some('"')) => (rest, Ok(s)),
        _ => (text, Err(String::from("failed to parse closing \" in string")))
    }
}

#[cfg(test)]
mod tests {
    fn should_fail<A: Eq>(r: (&[char], Result<A, String>)) {
        match r {
            (_, Err(_)) => (),
            (_, Ok(a)) => panic!("should fail didnt fail")
        }
    }
    fn succeed_with<A: Eq + std::fmt::Debug>(r: (&[char], Result<A, String>), rest: &[char], out: A) {
        match r {

            (r, Ok(o)) => {
                assert_eq!(r, rest);
                assert_eq!(o, out);
            },
            (_, Err(e)) => panic!("succeed_with failed with error {}", e)
        }
    }
    use super::*;
    fn to_arr(s: &str) -> Vec<char> {
        s.chars().collect::<Vec<_>>()
    }
    #[test]
    fn term() {
        should_fail(parse_term(&to_arr("12 NonTerm")));
        should_fail(parse_term(&to_arr("NonTerm")));
        should_fail(parse_term(&to_arr("\"Term")));
        should_fail(parse_term(&to_arr("")));

        succeed_with(parse_term(&to_arr("\"Term\"")), &to_arr(""), String::from("Term"));
        succeed_with(parse_term(&to_arr("\"Term\" other stuff")), &to_arr(" other stuff"), String::from("Term"));

    }

    #[test]
    fn non_term() {
        should_fail(parse_non_term(&to_arr("12 NonTerm")));
        should_fail(parse_non_term(&to_arr(" NonTerm")));
        should_fail(parse_non_term(&to_arr("")));
        should_fail(parse_non_term(&to_arr("\"Term\"")));

        succeed_with(parse_non_term(&to_arr("NonTerm")), &to_arr(""), String::from("NonTerm"));
        succeed_with(parse_non_term(&to_arr("NonTerm other stuff")), &to_arr(" other stuff"), String::from("NonTerm"));
    }

    #[test]
    fn literal() {
        should_fail(parse_literal(&to_arr("\"Term")));
        should_fail(parse_literal(&to_arr(" Term")));
        should_fail(parse_literal(&to_arr("")));

        succeed_with(parse_literal(&to_arr("NonTerm")), &to_arr(""), Literal::NonTerm(String::from("NonTerm")));
        succeed_with(parse_literal(&to_arr("\"Term\"")), &to_arr(""), Literal::Term(String::from("Term")));
    }
}
