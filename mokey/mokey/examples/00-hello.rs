use std::cell::OnceCell;

use mokey::prelude::*;

fn main() {
    let ctx = Ctx::new();
    let mut reg = Reg::default();
    ctx.regist(&mut reg);
}

#[derive(Default)]
struct Reg {
    hello_op: OnceCell<OpDefHash>,
}

impl Reg {
    fn hello_op(&self) -> OpDefHash {
        self.hello_op.get().expect("before registration").clone()
    }
}

impl Registry for Reg {
    fn regist(&mut self, mut reg: Register<'_, '_>) {
        self.hello_op.get_or_init(|| reg.regist_op(HelloOp {}));
    }
}

#[derive(Debug)]
struct HelloOp;
impl OpDef for HelloOp {
    fn dialect_name(&self) -> &str {
        "example"
    }

    fn op_name(&self) -> &str {
        "hello"
    }
}
