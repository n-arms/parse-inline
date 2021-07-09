#[derive(Debug)]
pub struct Rules {
    rules: Vec<Rule>
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    expr: Expr,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Literal(String),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}
