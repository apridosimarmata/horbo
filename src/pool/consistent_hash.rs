use crate::common::error::ErrorResponse;
use crate::pool::pool::NodePool;
use crate::utils::hash::ip_to_hash;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Ring {
    pub nodes: RwLock<Vec<Arc<Node>>>,
    // pub registered_ips: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub ip: String,
    pub healthy: bool,
}

pub fn build(ip_list: Vec<String>) -> Ring {
    let res = Ring {
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
                guard.push(Arc::new(n.clone()));
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
                        Arc::new(Node {
                            id: node_id,
                            ip: ip_addr.clone(),
                            healthy: true,
                        }),
                    ),
                    None => nodes.push(Arc::new(Node {
                        id: node_id,
                        ip: ip_addr.clone(),
                        healthy: true,
                    })),
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
                        let node = nodes.get_mut(pos);
                        match node {
                            Some(node) => {
                                if node.healthy != is_healthy {
                                    // Replaces old node with new copy
                                    nodes[pos] = Arc::new(Node {
                                        id: node.id,
                                        ip: node.ip.clone(),
                                        healthy: is_healthy,
                                    });
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
}
