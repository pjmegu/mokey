use mokey::prelude::*;

fn main() {
    let ctx = Ctx::new();
    ctx.regist(Reg {});
}

struct Reg;
impl Registry for Reg {
    fn regist(&self, mut reg: Register<'_, '_>) {
        reg.regist_op(HelloOp {});
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
