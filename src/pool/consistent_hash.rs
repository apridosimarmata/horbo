use crate::common::error::ErrorResponse;
use crate::core::domain::data::Node;
use crate::grpc::{Node as NodeGrpc};
use crate::pool::pool::NodePool;
use crate::utils::hash::ip_to_hash;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Ring {
    pub namespace: String,
    pub nodes: RwLock<Vec<Node>>,
}

pub fn build(namespace :String,ip_list: Vec<String>) -> Ring {
    let res = Ring {
        namespace: namespace,
        nodes: RwLock::new(Vec::new()),
        // registered_ips: Vec::new(),
    };

    let mut nodes_mapper: HashMap<u32, Node> = HashMap::new();
    let mut node_ids: Vec<u32> = Vec::new();

    for ip_addr in ip_list {
        let node_id = ip_to_hash(&ip_addr);

        let bind_id = ip_addr.clone();
        nodes_mapper.insert(
            node_id,
            Node {
                id: ip_to_hash(&ip_addr),
                ip: bind_id,
                healthy: true,
            },
        );

        // should do health check here
        node_ids.push(node_id);
    }

    node_ids.sort();

    for id in node_ids {
        match nodes_mapper.get(&id) {
            Some(n) => {
                let mut guard = res.nodes.write().unwrap();
                guard.push(n.clone());
            }
            None => {}
        }
    }

    res
}

impl NodePool for Ring {
    fn get(&self, client_ip_addr: String) -> Result<String, ErrorResponse> {
        let client_id = ip_to_hash(&client_ip_addr);
        let read_nodes = self.nodes.read();

        match read_nodes {
            Ok(nodes) => {
                if nodes.len() == 0 {
                    return Err(ErrorResponse::Internal(
                        "no service found in namespace".to_string(),
                    ));
                }

                let pos = nodes
                    .iter()
                    .position(|item| (item.id >= client_id && item.healthy));

                match pos {
                    Some(pos) => return Ok(nodes[pos].id.to_string()),
                    None => {
                        // Pick closest node to client_id
                        let mut closest_id: u32 = 0;

                        for node in nodes.iter() {
                            if node.id >= client_id {
                                return Ok(closest_id.to_string());
                            }

                            if node.healthy {
                                closest_id = node.id;
                            }
                        }

                        return Err(ErrorResponse::Internal(
                            "no healthy service found in namespace".to_string(),
                        ));
                    }
                }
            }
            Err(e) => {
                return Err(ErrorResponse::Internal(e.to_string()));
            }
        }
    }

    fn add_server(&self, ip_addr: String) -> Result<u32, ErrorResponse> {
        let node_id = ip_to_hash(&ip_addr);
        let write_nodes = self.nodes.write();

        match write_nodes {
            Ok(mut nodes) => {
                // Linear search is just enough to find index for insertion
                // justification: won't be holding a lot of node inside the vec
                let pos = nodes.iter().position(|item| item.id >= node_id);

                match pos {
                    Some(i) if nodes[i].id == node_id => return Ok(node_id),
                    Some(i) => nodes.insert(
                        i,
                        Node {
                            id: node_id,
                            ip: ip_addr.clone(),
                            healthy: true,
                        },
                    ),
                    None => nodes.push(Node {
                        id: node_id,
                        ip: ip_addr.clone(),
                        healthy: true,
                    }),
                }
            }
            Err(e) => {
                return Err(ErrorResponse::Internal(e.to_string()));
            }
        }

        Ok(node_id)
    }

    fn set_health_status(&self, ip_addr: String, is_healthy: bool) -> Result<(), ErrorResponse> {
        let write_nodes = self.nodes.write();

        match write_nodes {
            Ok(mut nodes) => {
                let node_id = ip_to_hash(&ip_addr);
                let pos = nodes.iter().position(|item| item.id == node_id);
                match pos {
                    Some(pos) => {
                        match nodes.get_mut(pos) {
                            Some(node) => {
                                if node.healthy != is_healthy {
                                    // Replaces old node with new copy
                                    nodes[pos] = Node {
                                        id: node.id,
                                        ip: node.ip.clone(),
                                        healthy: is_healthy,
                                    };
                                }

                                return Ok(());
                            }
                            None => {}
                        }
                    }
                    None => {
                        return Err(ErrorResponse::BadRequest(
                            "can't find service inside the namespace".to_string(),
                        ))
                    }
                }
            }
            Err(e) => {
                return Err(ErrorResponse::Internal(e.to_string()));
            }
        }

        Ok(())
    }
    
    fn remove_server(&self, ip_addr: String) -> Result<(), ErrorResponse> {
        todo!()
    }
}

impl Ring {
    pub fn repr(&self) -> Vec<NodeGrpc> {
        let read_nodes = self.nodes.read();
        let mut result:Vec< NodeGrpc> = Vec::new();

        match read_nodes {
            Ok(nodes) => {
                for node in nodes.iter() {
                    result.push(NodeGrpc{
                        id:node.id.to_string(),
                        ip_address: node.ip.clone(),
                        namespace: self.namespace.clone(),
                    });
                }

                return result
            },
            Err(_) => {
                /* Don't return error as this func is used
                in each response to a heartbeat */
                return result
            }
        }
    }
}