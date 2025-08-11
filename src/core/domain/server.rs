use crate::{
    common::error::ErrorResponse, core::domain::data::UtilizationMetric, grpc::{HeartbeatResponse, LookupResponse},
};

pub trait ServiceDiscoveryUsecase {
    async fn register_node(
        &self,
        namespace: String,
        ip_address: String,
    ) -> Result<u32, ErrorResponse>;
    async fn mark_node_unhealthy(
        &self,
        namespace: String,
        ip_address: String,
    ) -> Result<(), ErrorResponse>;
    async fn service_lookup(
        &self,
        namespace: String,
        client_ip_address: String,
    ) -> Result<LookupResponse, ErrorResponse>;

    /* Updated to return response object instead domain */
    async fn node_heartbeat(
        &self,
        namespace: String,
        ip_address: String,
        metric: UtilizationMetric,
    ) -> Result<HeartbeatResponse, ErrorResponse>;
}
