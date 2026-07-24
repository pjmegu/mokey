use crate::{CtxInner, op::OpDef};

pub struct Register<'ctx, 'r> {
    ctx: &'r mut CtxInner<'ctx>,
}

impl<'ctx, 'r> Register<'ctx, 'r> {
    pub(crate) fn new(ctx: &'r mut CtxInner<'ctx>) -> Self {
        Register { ctx }
    }

    pub fn regist_op(&mut self, op: impl OpDef + 'ctx) {
        self.ctx.define_op(op);
    }
}

pub trait Registry {
    fn regist(&self, reg: Register<'_, '_>);
}
