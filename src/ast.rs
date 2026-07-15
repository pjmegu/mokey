#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let(String, Expr),
    Return(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Var(String),
    BuiltinVar(String),
    Plus(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}
