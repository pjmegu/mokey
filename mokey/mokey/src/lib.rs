pub mod op;
pub mod reg;
mod util;

pub mod prelude {
    pub use crate::Ctx;
    pub use crate::op::{Op, OpDef, OpDefHash, OpId};
    pub use crate::reg::{Register, Registry};
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    op::{Op, OpDef, OpDefHash, OpId},
    reg::{Register, Registry},
};

#[derive(Debug, Clone)]
pub struct Ctx<'ctx>(Rc<RefCell<CtxInner<'ctx>>>);

impl<'ctx> Ctx<'ctx> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(CtxInner::new())))
    }

    pub fn regist(&self, registry: impl Registry) {
        self.0.borrow_mut().regist(registry)
    }

    pub fn add_op(&self, op: Op) {
        self.0.borrow_mut().add_op(op);
    }
}

#[derive(Debug)]
struct CtxInner<'ctx> {
    ops: HashMap<OpId, Op>,

    op_defs: HashMap<OpDefHash, Box<dyn OpDef + 'ctx>>,
}

impl<'ctx> CtxInner<'ctx> {
    fn new() -> Self {
        Self {
            ops: HashMap::new(),
            op_defs: HashMap::new(),
        }
    }

    fn regist(&mut self, registry: impl Registry) {
        let reg = Register::new(self);
        registry.regist(reg);
    }

    pub(crate) fn define_op(&mut self, def: impl OpDef + 'ctx) {
        let res = self.op_defs.insert(OpDefHash::from(&def), Box::new(def));
        assert!(res.is_none())
    }

    pub(crate) fn add_op(&mut self, op: Op) {
        let res = self.ops.insert(OpId::new(), op);
        assert!(res.is_none())
    }
}
