//
use std::{pin::Pin, sync::Arc};
use tokio::sync::Mutex;

use tonic::{Request, Response, Status};

use crate::{
    core::{
        application::service_discovery::ServiceDiscovery, domain::server::ServiceDiscoveryUsecase,
    },
    grpc::{self, horbo_server::Horbo, *},
};

pub struct HorboServiceController {
    pub service: Arc<Mutex<ServiceDiscovery>>,
}

impl Horbo for HorboServiceController {
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn register_agent<'life0, 'async_trait>(
        &'life0 self,
        request: Request<AgentRegistrationRequest>,
    ) -> Pin<
        Box<
            dyn Future<Output = std::result::Result<Response<AgentRegistrationResponse>, Status>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(self.register_node(request))
    }

    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn service_lookup<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<LookupRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<
                        tonic::Response<LookupResponse>,
                        tonic::Status,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(self.service_lookup(request))
    }
}

impl HorboServiceController {
    async fn service_lookup(
        &self,
        request: Request<LookupRequest>,
    ) -> Result<Response<LookupResponse>, Status> {
        let client_ip_address = request.remote_addr();

        match client_ip_address {
            Some(ip) => {
                let services = self.service.lock().await;
                let req_inner = request.into_inner();


                let service_ip = services.service_lookup(req_inner.namespace, ip.to_string()).await;
                match service_ip {
                    Ok(service_ip )=> {
                        return Ok(Response::new(LookupResponse{
                            ip_address: service_ip,
                        }));
                    },
                    Err(e) => {
                        return Err(Status::internal(e.to_string()));
                    }
                }
            },
            None => {
                return Err(Status::invalid_argument("client ip is not valid"));
            }
        }
    }

    async fn register_node(
        &self,
        request: Request<AgentRegistrationRequest>,
    ) -> Result<Response<AgentRegistrationResponse>, Status> {
        let ip_address = request.remote_addr();

        match ip_address {
            Some(ip) => {
                let services = self.service.lock().await;
                let req_inner = request.into_inner();
                let id = services
                    .register_node(req_inner.namespace, ip.to_string())
                    .await;

                match id {
                    Ok(_id) => Ok(Response::new(AgentRegistrationResponse {
                        client_id: _id.to_string(),
                    })),
                    Err(_) => Err(Status::invalid_argument("namespace doesn't exists")),
                }
            }
            None => {
                return Err(Status::invalid_argument("ip is not valid"));
            }
        }
    }
}
