use std::error::Error;


pub trait ServiceDiscoveryUsecase  {
    async fn register_node(&self, namespace:String, ip_address: String) -> Result<u32, Box<dyn Error>>;
    async fn remove_node(&self, namespace: String, ip_address: String) -> Result<(), Box<dyn Error>>;
    async fn node_heartbeat(&self, namespace: String, ip_address: String) -> Result<(),Box<dyn Error>>;
    async fn node_failure_report(&self, namespace: String, ip_addres:String) -> Result<(), Box<dyn Error>>;
    async fn service_lookup(&self, namespace: String) -> Result<(), Box<dyn Error>>;
}