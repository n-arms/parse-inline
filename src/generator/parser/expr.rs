use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Rules {
    rules: Vec<Rule>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
    name: String,
    expr: Expr,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}
/*
impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Literal(l) => l.fmt(f),
            Expr::And(l, r) => write!(f, "And({:?}, {:?})", l, r),
            Expr::Or(l, r) => write!(f, "Or({:?}, {:?})", l, r)
        }
    }
}*/

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Literal {
    Term(String),
    NonTerm(String),
}

pub fn pretty_print(margin: usize, e: Expr) {
    match e {
        Expr::Literal(l) => println!("{}{:?}", vec![' '; margin].iter().collect::<String>(), l),
        Expr::And(l, r) => {
            println!("{}And (", vec![' '; margin].iter().collect::<String>());
            pretty_print(margin + 1, *l);
            pretty_print(margin + 1, *r);
            println!("{})", vec![' '; margin].iter().collect::<String>());
        },
        Expr::Or(l, r) => {
            println!("{}Or (", vec![' '; margin].iter().collect::<String>());
            pretty_print(margin + 1, *l);
            pretty_print(margin + 1, *r);
            println!("{})", vec![' '; margin].iter().collect::<String>());
        },
    }
}
