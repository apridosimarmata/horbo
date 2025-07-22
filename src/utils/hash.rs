use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

pub fn ip_to_hash(ip: &str) -> u32 {
    let mut hasher = XxHash64::with_seed(0);
    ip.hash(&mut hasher);
    (hasher.finish() & 0x00FF_FFFF) as u32
}