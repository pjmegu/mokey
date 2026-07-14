use crate::ast;

#[derive(Debug, PartialEq)]
pub struct IR {
    pub ops: Vec<Op>,
}

#[derive(Debug, PartialEq)]
pub enum VType {
    Int,
}

#[derive(Debug, PartialEq)]
pub struct Op {
    pub kind: OpKind,
    pub result_type: VType,
}

#[derive(Debug, PartialEq)]
pub enum OpKind {
    NOP,
    ConstInt(i64),
    Plus(usize, usize),
}

#[derive(Debug, PartialEq)]
pub enum GenIRError {}

pub fn genir(root: ast::Root) -> Result<IR, GenIRError> {
    let g = Generator::new();
    g.generate(root)
}

struct Generator {
    ops: Vec<Op>,
    pos: usize,
}

impl Generator {
    fn new() -> Self {
        Self {
            ops: Vec::new(),
            pos: 0,
        }
    }

    fn set(&mut self, pos: usize) {
        if pos > self.ops.len() {
            panic!("Pos Out Of Range: {} (len: {})", pos, self.ops.len())
        }

        self.pos = pos;
    }

    fn insert(&mut self, op: Op) -> usize {
        self.ops.insert(self.pos, op);
        self.pos
    }

    fn push(&mut self, op: Op) -> usize {
        self.ops.push(op);
        self.ops.len() - 1
    }

    fn generate(mut self, ast: ast::Root) -> Result<IR, GenIRError> {
        self.expr(&ast.expr)?;
        Ok(IR { ops: self.ops })
    }

    fn expr(&mut self, ast: &ast::Expr) -> Result<usize, GenIRError> {
        match ast {
            ast::Expr::Int(i) => {
                let op = self.push(Op {
                    kind: OpKind::ConstInt(*i),
                    result_type: VType::Int,
                });
                Ok(op)
            }
            ast::Expr::Plus(lhs, rhs) => {
                let lhs = self.expr(lhs)?;
                let rhs = self.expr(rhs)?;
                let op = self.push(Op {
                    kind: OpKind::Plus(lhs, rhs),
                    result_type: VType::Int,
                });
                Ok(op)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        assert_eq!(
            genir(ast::Root {
                expr: ast::Expr::Int(15)
            }),
            Ok(IR {
                ops: vec![Op {
                    kind: OpKind::ConstInt(15),
                    result_type: VType::Int
                }]
            })
        )
    }
    #[test]
    fn plus() {
        assert_eq!(
            genir(ast::Root {
                expr: ast::Expr::Plus(Box::new(ast::Expr::Int(15)), Box::new(ast::Expr::Int(24)))
            }),
            Ok(IR {
                ops: vec![
                    Op {
                        kind: OpKind::ConstInt(15),
                        result_type: VType::Int
                    },
                    Op {
                        kind: OpKind::ConstInt(24),
                        result_type: VType::Int
                    },
                    Op {
                        kind: OpKind::Plus(0, 1),
                        result_type: VType::Int
                    }
                ]
            })
        )
    }
}
