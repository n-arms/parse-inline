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

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Literal {
    Term(String),
    NonTerm(String),
}
