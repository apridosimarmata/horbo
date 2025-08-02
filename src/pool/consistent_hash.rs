use crate::common::error::ErrorResponse;
use crate::pool::pool::NodePool;
use crate::utils::hash::ip_to_hash;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Ring {
    pub nodes: RwLock<Vec<Arc<Node>>>,
    pub registered_ips: Vec<String>,
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
        registered_ips: Vec::new(),
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
    fn get(&self, client_ip_addr: String) -> Option<Arc<Node>> {
        let client_ip_hash = ip_to_hash(&client_ip_addr);

        let read_nodes = self.nodes.read();
        let mut res: Option<Arc<Node>> = None;

        match read_nodes {
            Ok(r) => {
                for node in r.clone().into_iter() {
                    let bind = node.clone();

                    if res.is_none() {
                        res = Some(node.clone());
                    }

                    if node.id > client_ip_hash && node.healthy {
                        res = Some(bind.clone());
                        break;
                    }
                }
            }
            Err(e) => return None,
        }

        match res {
            Some(n) => Some(n),
            None => None,
        }
    }

    fn add_server(&self, ip_addr: String) -> Result<String, ErrorResponse> {
        let node_id = ip_to_hash(&ip_addr);
        let write_nodes = self.nodes.write();

        match write_nodes {
            Ok(mut n) => {
                // linear scan for now, do better insertion later: binary search to find the index, then insert
                let pos = n.iter().position(|item| item.id >= node_id);

                match pos {
                    Some(i) if n[i].id == node_id => return Ok(node_id.to_string()),
                    Some(i) => n.insert(
                        i,
                        Arc::new(Node {
                            id: node_id,
                            ip: ip_addr.clone(),
                            healthy: true,
                        }),
                    ),
                    None => n.push(Arc::new(Node {
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

        Ok(node_id.to_string())
    }

    fn remove(&self, id: u32) {
        let write_nodes = self.nodes.write();

        let mut remove_at: usize = 0;

        match write_nodes {
            Ok(mut n) => {
                // linear scan for now, do better insertion later: binary search, then insert
                let bind = n.clone();
                for item in bind.iter().enumerate() {
                    if item.1.id > id {
                        remove_at = item.0;
                        break;
                    }
                }

                n.remove(remove_at);
            }
            Err(e) => {}
        }
    }
}
