use std::sync::Arc;

use crate::pool::consistent_hash::Node;

pub trait NodePool {
    fn get(&self, client_ip_addr: String) -> Option<Arc<Node>>;
    fn add_server(&self, ip_addr: String);
    fn remove(&self, id: u32);
}