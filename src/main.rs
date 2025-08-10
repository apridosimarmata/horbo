use crate::grpc::horbo_server::HorboServer;
use crate::pool::consistent_hash::{build, Ring};
use crate::server::HorboServiceController;
use core::schema::{init, ServiceDefinition};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Certificate, Identity, Server as TonicServer, ServerTlsConfig};
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
        services.insert(name.clone(), build(name, ip_addres));
    }

    /* build and serve grpc */
    let svc = HorboServer::new(HorboServiceController {
        service: Arc::new(Mutex::new(
            core::application::service_discovery::ServiceDiscovery::new(services),
        )),
    });

    /* mTLS support */
    let server_cert = fs::read("./keys/server.crt")?;
    let server_key = fs::read("./keys/server.key")?;
    let server_identity = Identity::from_pem(server_cert, server_key);

    let client_ca_cert = fs::read("./keys/ca.crt")?;
    let client_ca = Certificate::from_pem(client_ca_cert);
    let tls_config = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca);

    TonicServer::builder()
        .tls_config(tls_config)?
        .add_service(svc)
        .serve("[::1]:50051".parse()?)
        .await?;
    Ok(())
}
