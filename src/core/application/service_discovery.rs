use crate::core::domain::server::ServiceDiscoveryUsecase;


struct ServiceDiscovery {

}


impl ServiceDiscovery {
    fn new() -> Self {
        ServiceDiscovery {  }
    }
}

impl ServiceDiscoveryUsecase for ServiceDiscovery {
    async fn register_node(namespace:String, ip_address: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn remove_node(namespace: String, ip_address: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn node_heartbeat(namespace: String, ip_address: String) -> Result<(),Box<dyn std::error::Error>> {
        todo!()
    }

    async fn node_failure_report(namespace: String, ip_addres:String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn service_lookup(namespace: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}