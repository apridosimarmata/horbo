use std::error::Error;

use crate::{common::error::ErrorResponse, core::domain::data::UtilizationMetric};


pub trait ServiceDiscoveryUsecase  {
    async fn register_node(&self, namespace:String, ip_address: String) -> Result<u32, ErrorResponse>;
    async fn remove_node(&self, namespace: String, ip_address: String) -> Result<(), ErrorResponse>;
    async fn node_failure_report(&self, namespace: String, ip_addres:String) -> Result<(), ErrorResponse>;
    async fn service_lookup(&self, namespace: String, client_ip_address: String) -> Result<String, ErrorResponse>;
    async fn node_heartbeat(&self, namespace: String, ip_address: String, metric: UtilizationMetric) -> Result<(),ErrorResponse>;
}