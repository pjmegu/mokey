use std::collections::HashMap;

use crate::util;

#[derive(Debug)]
pub struct Op {
    def: OpDefHash,

    regions: Vec<Region>,
    props: (),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OpId(u64);

impl OpId {
    pub(crate) fn new() -> Self {
        Self(util::get_rand())
    }
}

#[derive(Debug)]
pub struct OpEdge {
    from: OpId,
    to: HashMap<OpId, u16>,
}

#[derive(Debug)]
pub enum Region {
    Graph(GraphRegion),
    Flat(FlatRegion),
}

#[derive(Debug)]
pub struct GraphRegion {
    ops: Vec<OpId>,
    edges: Vec<OpEdge>,
}

#[derive(Debug)]
pub struct FlatRegion {
    ops: Vec<OpId>,
}

pub trait OpDef: std::fmt::Debug {
    fn dialect_name(&self) -> &str;
    fn op_name(&self) -> &str;
    fn hash(&self) -> u64 {
        util::get_hash_from_dialect_and_name(self.dialect_name(), self.op_name())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OpDefHash(u64);

impl OpDefHash {
    pub(crate) fn from(def: &impl OpDef) -> Self {
        Self(def.hash())
    }
}
