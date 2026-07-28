use std::collections::HashMap;

use crate::{CtxInner, hash::Hashable, util};

#[derive(Debug)]
pub struct Op {
    def: OpDefHash,

    regions: Vec<Region>,
    props: (),
}

impl Op {
    pub fn difinition(&self) -> OpDefHash {
        self.def
    }
}

impl Hashable for Op {
    fn hash(&self, hasher: &mut blake3::Hasher, ctx: &CtxInner) {
        hasher.update(b"Op");
        Hashable::hash(&self.def, hasher, ctx);
        for reg in &self.regions {
            Hashable::hash(reg, hasher, ctx);
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct OpId(u64);

impl OpId {
    pub(crate) fn new() -> Self {
        Self(util::get_rand())
    }
}

impl Hashable for OpId {
    fn hash(&self, hasher: &mut blake3::Hasher, ctx: &CtxInner) {
        hasher.update(b"OpId");
        Hashable::hash(ctx.ops.get(&self).unwrap(), hasher, ctx);
    }
}

#[derive(Debug)]
pub struct OpEdge {
    from: OpId,
    to: HashMap<OpId, u16>,
}

impl Hashable for OpEdge {
    fn hash(&self, hasher: &mut blake3::Hasher, ctx: &CtxInner) {
        hasher.update(b"OpEdge");
        Hashable::hash(&self.from, hasher, ctx);
        for (op, index) in &self.to {
            Hashable::hash(op, hasher, ctx);
            Hashable::hash(index, hasher, ctx);
        }
    }
}

#[derive(Debug)]
pub enum Region {
    Graph(GraphRegion),
    Flat(FlatRegion),
}

impl Hashable for Region {
    fn hash(&self, hasher: &mut blake3::Hasher, ctx: &CtxInner) {
        hasher.update(b"Region");
        match self {
            Region::Graph(gr) => {
                Hashable::hash(gr, hasher, ctx);
            }
            Region::Flat(fr) => {
                Hashable::hash(fr, hasher, ctx);
            }
        }
    }
}

#[derive(Debug)]
pub struct GraphRegion {
    ops: Vec<OpId>,
    edges: Vec<OpEdge>,
}

impl Hashable for GraphRegion {
    fn hash(&self, hasher: &mut blake3::Hasher, ctx: &CtxInner) {
        hasher.update(b"GraphRegion");
        for op in &self.ops {
            Hashable::hash(op, hasher, ctx);
        }

        for edge in &self.edges {
            Hashable::hash(edge, hasher, ctx);
        }
    }
}

#[derive(Debug)]
pub struct FlatRegion {
    ops: Vec<OpId>,
}

impl Hashable for FlatRegion {
    fn hash(&self, hasher: &mut blake3::Hasher, ctx: &CtxInner) {
        hasher.update(b"FlatRegion");
        for op in &self.ops {
            Hashable::hash(op, hasher, ctx);
        }
    }
}

pub trait OpDef: std::fmt::Debug {
    fn dialect_name(&self) -> &str;
    fn op_name(&self) -> &str;
    fn hash(&self) -> u64 {
        util::get_hash_from_dialect_and_name(self.dialect_name(), self.op_name())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct OpDefHash(u64);

impl OpDefHash {
    pub(crate) fn from(def: &impl OpDef) -> Self {
        Self(def.hash())
    }
}

impl Hashable for OpDefHash {
    fn hash(&self, hasher: &mut blake3::Hasher, _ctx: &CtxInner) {
        hasher.update(b"OpDefHash");
        hasher.update(&self.0.to_le_bytes());
    }
}
