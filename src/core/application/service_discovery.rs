use std::{collections::HashMap, sync::{Arc, RwLock}};
use crate::{core::domain::server::ServiceDiscoveryUsecase, pool::{consistent_hash::{Node, Ring}, pool::NodePool}};
use crate::utils::hash::ip_to_hash;
use crate::common::error::ErrorResponse;

// #[derive(Clone)]
pub struct ServiceDiscovery {
    pub service_map: HashMap<String, Ring>
}


impl ServiceDiscovery {
    pub fn new(service_map: HashMap<String, Ring>) -> Self {
        ServiceDiscovery { service_map: service_map  }
    }
}

impl ServiceDiscoveryUsecase for ServiceDiscovery {
    //register_node put new node to a namespace Ring and return its generated id
    async fn register_node(&self, namespace:String, ip_address: String) -> Result<u32, ErrorResponse> {
        let unique_id = ip_to_hash(&ip_address);
        let ring = self.service_map.get(&namespace);

        match ring {
            Some(ring) => {
                let _ = ring.add_server(ip_address);
            },
            None => {
                return Err(ErrorResponse::BadRequest("namespace not found".to_string()))
            }
        };


        Ok(unique_id)
    }

    async fn service_lookup(&self, namespace: String, client_ip_address: String) -> Result<String, ErrorResponse> {
        let ring = self.service_map.get(&namespace);

        match ring {
            Some(ring) => {
                match ring.get(client_ip_address) {
                    Ok(service_ip) => {
                        return Ok(service_ip);
                    },
                    Err(e) => {
                        return Err(ErrorResponse::Internal(e.to_string()));
                    }
                }
            },
            None => {
                return Err(ErrorResponse::BadRequest("namespace not found".to_string()))
            }
        }
    }

    async fn remove_node(&self, namespace: String, ip_address: String) -> Result<(), ErrorResponse> {
        todo!()
    }

    async fn node_heartbeat(&self, namespace: String, ip_address: String) -> Result<(),ErrorResponse> {
        todo!()
    }

    async fn node_failure_report(&self, namespace: String, ip_addres:String) -> Result<(), ErrorResponse> {
        todo!()
    }


}