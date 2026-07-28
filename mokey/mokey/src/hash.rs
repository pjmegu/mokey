use crate::CtxInner;

pub trait Hashable {
    fn hash(&self, hasher: &mut blake3::Hasher, ctx: &CtxInner);
}

impl Hashable for u16 {
    fn hash(&self, hasher: &mut blake3::Hasher, _ctx: &CtxInner) {
        hasher.update(&self.to_le_bytes());
    }
}
