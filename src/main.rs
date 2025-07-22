use std::collections::HashMap;
use tonic::transport::Server as TonicServer;
use core::schema::{ServiceDefinition, init};
use crate::grpc::horbo_server::HorboServer;
use crate::pool::consistent_hash::{Ring, build};
use crate::server::HorboService;

mod grpc;
mod pool;
mod core;
mod utils;
mod server;

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
    let svc = HorboServer::new(HorboService{});
    TonicServer::builder()
        .add_service(svc)
        .serve("[::1]:50051".parse()?)
        .await?;
    Ok(())
}