use std::sync::Arc;
use crate::{common::error::ErrorResponse, pool::consistent_hash::Node};

pub trait NodePool {
    fn get(&self, client_ip_addr: String) -> Result<String, ErrorResponse> ;
    fn add_server(&self, ip_addr: String) -> Result<String, ErrorResponse>;
    fn remove(&self, id: u32);
}