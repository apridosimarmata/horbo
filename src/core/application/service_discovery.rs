use crate::common::error::ErrorResponse;
use crate::{
    core::domain::{data::UtilizationMetric, server::ServiceDiscoveryUsecase},
    pool::{consistent_hash::Ring, pool::NodePool},
};
use std::collections::HashMap;

pub struct ServiceDiscovery {
    pub service_map: HashMap<String, Ring>,
}

impl ServiceDiscovery {
    pub fn new(service_map: HashMap<String, Ring>) -> Self {
        ServiceDiscovery {
            service_map: service_map,
        }
    }
}

impl ServiceDiscoveryUsecase for ServiceDiscovery {
    /// Registers a node (server) into the consistent hash ring under a given namespace.
    ///
    /// # Arguments
    /// - `namespace`: The logical group to which the node belongs (e.g., service name or environment).
    /// - `ip_address`: The IP address of the node being registered.
    ///
    /// # Returns
    /// - `Ok(unique_id)` where `unique_id` is the hashed ID derived from the node's IP address.
    /// - `Err(ErrorResponse::BadRequest)` if the namespace doesn't exist in the service map.
    ///
    /// # Behavior
    /// - Returns a unique hash for the given IP address.
    /// - Looks up the corresponding consistent hash ring for the namespace.
    /// - Adds the node to the ring if the namespace exists.
    /// - Returns an error if the namespace is unknown.
    async fn register_node(
        &self,
        namespace: String,
        ip_address: String,
    ) -> Result<u32, ErrorResponse> {
        let ring = self.service_map.get(&namespace);

        match ring {
            Some(ring) => {
                let unique_id = ring.add_server(ip_address);
                match unique_id {
                    Ok(id) => return Ok(id),
                    Err(e) => return Err(e),
                }
            }
            None => return Err(ErrorResponse::BadRequest("namespace not found".to_string())),
        };
    }

    /// Looks up a service instance for the given client IP using consistent hashing.
    ///
    /// # Arguments
    /// - `namespace`: The logical group of services to look up from.
    /// - `client_ip_address`: The IP address of the client requesting a service.
    ///
    /// # Returns
    /// - `Ok(service_ip)`: The selected service IP address from the consistent hash ring.
    /// - `Err(ErrorResponse::BadRequest)`: If the namespace doesn't exist.
    /// - `Err(ErrorResponse::Internal)`: If the ring lookup fails due to an internal error.
    ///
    /// # Behavior
    /// - Retrieves the consistent hash ring associated with the given namespace.
    /// - Uses the client IP as a key to find the appropriate service IP from the ring.
    /// - Handles and forwards any errors that occur during lookup.
    async fn service_lookup(
        &self,
        namespace: String,
        client_ip_address: String,
    ) -> Result<String, ErrorResponse> {
        let ring = self.service_map.get(&namespace);

        match ring {
            Some(ring) => match ring.get(client_ip_address) {
                Ok(service_ip) => {
                    return Ok(service_ip);
                }
                Err(e) => return Err(e),
            },
            None => return Err(ErrorResponse::BadRequest("namespace not found".to_string())),
        }
    }

    /// Handles heartbeat from a node in the specified namespace.
    ///
    /// If the node's CPU or memory usage exceeds defined thresholds,
    /// it is considered unhealthy and will be removed from the service ring.
    ///
    /// Thresholds:
    /// - CPU usage > 80%
    /// - Memory usage > 85%
    ///
    /// Arguments:
    /// - `namespace`: The namespace the node belongs to.
    /// - `ip_address`: IP address of the node sending the heartbeat.
    /// - `metric`: Current CPU and memory utilization of the node.
    ///
    /// Returns:
    /// - `Ok(())` if the node is healthy or successfully removed.
    /// - `Err(ErrorResponse)` if removal fails or other error occurs.
    ///
    /// Notes:
    /// - If the namespace doesn't exist in `service_map`, the function returns `Ok(())` silently.
    /// TODO: also return updated version of unhealthy node list
    async fn node_heartbeat(
        &self,
        namespace: String,
        ip_address: String,
        metric: UtilizationMetric,
    ) -> Result<(), ErrorResponse> {
        let mut is_healthy = false;
        if metric.cpu_usage < 80.00 && metric.memory_usage < 85.00 {
            is_healthy = true
        }

        let ring = self.service_map.get(&namespace);

        match ring {
            Some(ring) => match ring.set_health_status(ip_address, is_healthy) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            },
            None => return Ok(()),
        }
    }

    // TODO: also return updated version of unhealthy node list
    async fn mark_node_unhealthy(
        &self,
        namespace: String,
        ip_address: String,
    ) -> Result<(), ErrorResponse> {
        let ring = self.service_map.get(&namespace);

        match ring {
            Some(ring) => match ring.set_health_status(ip_address, false) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            },
            None => return Ok(()),
        }
    }

    async fn node_failure_report(
        &self,
        namespace: String,
        ip_addres: String,
    ) -> Result<(), ErrorResponse> {
        todo!()
    }
}
