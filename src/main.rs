use crate::grpc::horbo_server::HorboServer;
use crate::pool::consistent_hash::{build, Ring};
use crate::server::HorboServiceController;
use core::schema::{init, ServiceDefinition};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server as TonicServer;

mod common;
mod core;
mod grpc;
mod pool;
mod server;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* get service definition */
    let services_definition: ServiceDefinition = match init() {
        Ok(def) => def,
        Err(e) => {
            panic!("{}", e)
        }
    };

    /* init `services` singleton */
    let mut services: HashMap<String, Ring> = HashMap::new();
    for (name, ip_addres) in services_definition.services.into_iter() {
        services.insert(name, build(ip_addres));
    }

    /* build and serve grpc */
    let svc = HorboServer::new(HorboServiceController {
        service: Arc::new(Mutex::new(
            core::application::service_discovery::ServiceDiscovery::new(services),
        )),
    });
    TonicServer::builder()
        .add_service(svc)
        .serve("[::1]:50051".parse()?)
        .await?;
    Ok(())
}
