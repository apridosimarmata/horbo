use crate::{
    common::error::ErrorResponse, core::domain::data::UtilizationMetric, grpc::{AgentRegistrationResponse, HeartbeatResponse, LookupResponse},
};

pub trait ServiceDiscoveryUsecase {
    async fn register_node(
        &self,
        namespace: String,
        ip_address: String,
    ) -> Result<AgentRegistrationResponse, ErrorResponse>;

    async fn node_heartbeat(
        &self,
        namespace: String,
        ip_address: String,
        metric: UtilizationMetric,
    ) -> Result<HeartbeatResponse, ErrorResponse>;

    async fn service_lookup(
        &self,
        namespace: String,
        client_ip_address: String,
    ) -> Result<LookupResponse, ErrorResponse>;

    async fn mark_node_unhealthy(
        &self,
        namespace: String,
        ip_address: String,
    ) -> Result<(), ErrorResponse>;
}
