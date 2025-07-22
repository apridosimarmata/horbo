use std::error::Error;


pub trait ServiceDiscoveryUsecase  {
    async fn register_node(namespace:String, ip_address: String) -> Result<(), Box<dyn Error>>;
    async fn remove_node(namespace: String, ip_address: String) -> Result<(), Box<dyn Error>>;
    async fn node_heartbeat(namespace: String, ip_address: String) -> Result<(),Box<dyn Error>>;
    async fn node_failure_report(namespace: String, ip_addres:String) -> Result<(), Box<dyn Error>>;
    async fn service_lookup(namespace: String) -> Result<(), Box<dyn Error>>;
}