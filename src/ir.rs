use crate::ast;

pub struct IR {
    ops: Vec<Op>,
}

pub enum VType {
    Int,
}

pub struct Op {
    kind: OpKind,
    result_type: VType,
}

pub enum OpKind {
    NOP,
    ConstInt(i64),
    Plus(usize, usize),
}

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
