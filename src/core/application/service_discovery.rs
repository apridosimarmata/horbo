use std::{collections::HashMap, sync::{Arc, RwLock}};
use crate::{core::domain::server::ServiceDiscoveryUsecase, pool::consistent_hash::{Node, Ring}};
use crate::utils::hash::ip_to_hash;


// #[derive(Clone)]
pub struct ServiceDiscovery {
    pub service_map: HashMap<String, Ring>
}


impl ServiceDiscovery {
    fn new(service_map: HashMap<String, Ring>) -> Self {
        ServiceDiscovery { service_map: service_map  }
    }
}

impl ServiceDiscoveryUsecase for ServiceDiscovery {
    //register_node put new node to a namespace Ring and return its generated id
    async fn register_node(&self, namespace:String, ip_address: String) -> Result<u32, Box<dyn std::error::Error>> {
        let id = ip_to_hash(&ip_address);
        let write_obj = self.service_map.get(&namespace);
        match write_obj {
            Some(r) => {
                match  r.nodes.write() {
                    Ok(mut nodes) => {
                        let node = Node{
                            id: id.clone(),
                            ip: ip_address,
                            healthy: true,
                        };
                        nodes.push(Arc::new(node));

                        println!("current state {:?}", nodes);
                    },
                    Err(e) => {
                        println!("{:?}",e )
                    }
                }
            
            },
            None => {
                 println!("namespace doesnot exists")
            }
        };


        Ok(id)

    }

    async fn remove_node(&self, namespace: String, ip_address: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn node_heartbeat(&self, namespace: String, ip_address: String) -> Result<(),Box<dyn std::error::Error>> {
        todo!()
    }

    async fn node_failure_report(&self, namespace: String, ip_addres:String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn service_lookup(&self, namespace: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}