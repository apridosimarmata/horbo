use crate::common::error::ErrorResponse;
use crate::core::domain::data::Node;
use crate::{
    core::domain::{data::UtilizationMetric, server::ServiceDiscoveryUsecase},
    pool::{consistent_hash::Ring, pool::NodePool},
};
use std::collections::HashMap;

pub struct ServiceDiscovery {
    pub service_map: HashMap<String, Ring>,
    pub unhealthy_services: HashMap<String, Ring>,
}

impl ServiceDiscovery {
    pub fn new(service_map: HashMap<String, Ring>) -> Self {
        ServiceDiscovery {
            service_map: service_map,
            unhealthy_services: HashMap::new(),
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
    /// Based on the node's reported CPU and memory usage, it determines whether the node
    /// is healthy or not, then updates its health status in the service ring accordingly.
    ///
    /// A node is considered healthy if:
    /// - `cpu_usage` < 80.00
    /// - `memory_usage` < 85.00
    ///
    /// After updating the node's status, the function compiles a list of all unhealthy nodes
    /// across all namespaces and returns it.
    ///
    /// Arguments:
    /// - `namespace`: The namespace the node belongs to.
    /// - `ip_address`: IP address of the node sending the heartbeat.
    /// - `metric`: Current CPU and memory utilization of the node.
    ///
    /// Returns:
    /// - `Ok(HashMap<String, Vec<Node>>)` containing all unhealthy nodes grouped by namespace.
    /// - `Err(ErrorResponse)` if updating the node’s health status fails.
    ///
    /// Notes:
    /// - If the namespace doesn't exist in `service_map`, health status is not updated, but the function proceeds.
    /// - All unhealthy nodes from all namespaces are included in the response regardless of which node sent the heartbeat.

    async fn node_heartbeat(
        &self,
        namespace: String,
        ip_address: String,
        metric: UtilizationMetric,
    ) -> Result<HashMap<String, Vec<Node>>, ErrorResponse> {
        let mut is_healthy = false;
        if metric.cpu_usage < 80.00 && metric.memory_usage < 85.00 {
            is_healthy = true
        }

        let ring = self.service_map.get(&namespace);

        match ring {
            Some(ring) => match ring.set_health_status(ip_address, is_healthy) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            None => {}
        }

        /* Build unhealthy nodes response */
        let mut unhealthy_nodes: HashMap<String, Vec<Node>> = HashMap::new();
        for (namespace, nodes) in self.unhealthy_services.iter() {
            unhealthy_nodes.insert(namespace.clone(), nodes.repr());
        }

        Ok(unhealthy_nodes)
    }

    /// Marks a node as unhealthy in the specified namespace.
    ///
    /// This function updates the node status inside the service ring and
    /// registers it in the unhealthy ring for the given namespace.
    ///
    /// Behavior:
    /// - Attempts to set the node’s health status to `false` in the healthy service ring (`service_map`).
    /// - Then adds the node to the unhealthy ring (`unhealthy_services`) for tracking.
    ///
    /// Arguments:
    /// - `namespace`: The namespace to which the node belongs.
    /// - `ip_address`: The IP address of the node to be marked unhealthy.
    ///
    /// Returns:
    /// - `Ok(())` if the node was successfully marked as unhealthy or if the namespace doesn't exist.
    /// - `Err(ErrorResponse)` if updating health status or adding to the unhealthy ring fails.
    ///
    /// Notes:
    /// - If the namespace is not found in either `service_map` or `unhealthy_services`, the function exits silently.
    async fn mark_node_unhealthy(
        &self,
        namespace: String,
        ip_address: String,
    ) -> Result<(), ErrorResponse> {
        let ring = self.service_map.get(&namespace);

        match ring {
            Some(ring) => match ring.set_health_status(ip_address.clone(), false) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            None => return Ok(()),
        }

        let unhealthy_ring = self.unhealthy_services.get(&namespace);

        match unhealthy_ring {
            Some(ring) => match ring.add_server(ip_address) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            None => {}
        }

        Ok(())
    }
}
