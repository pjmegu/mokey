#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub expr: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Plus(Box<Expr>, Box<Expr>),
}
