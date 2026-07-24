use std::cell::RefCell;

use rand::{
    RngExt, distr::{Distribution, StandardUniform}, rngs::SmallRng,
};

thread_local! {
    static RAND: RefCell<SmallRng> = RefCell::new(rand::make_rng());
}

pub fn get_rand<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    RAND.with_borrow_mut(|r| r.random())
}

pub fn get_hash_from_dialect_and_name(dialect_name: &str, name: &str) -> u64 {
    let str = format!("{dialect_name}/{name}");
    get_hash(str.as_str())
}

pub fn get_hash(s: &str) -> u64 {
    let mut buf = [0u8; 8];
    blake3::Hasher::new()
        .update(s.as_bytes())
        .finalize_xof()
        .fill(&mut buf);
    u64::from_le_bytes(buf)
}
