use std::sync::Arc;
use crate::{common::error::ErrorResponse, pool::consistent_hash::Node};

pub trait NodePool {
    fn get(&self, client_ip_addr: String) -> Result<String, ErrorResponse> ;
    fn add_server(&self, ip_addr: String) -> Result<u32, ErrorResponse>;
    fn set_health_status(&self, ip_addr: String, is_healthy: bool) -> Result<(), ErrorResponse>;
}