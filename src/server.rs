//
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

use tonic::{Request, Response, Status};

use crate::{
    core::{
        application::service_discovery::ServiceDiscovery,
        domain::{data::UtilizationMetric, server::ServiceDiscoveryUsecase},
    },
    grpc::{horbo_server::Horbo, *},
};

pub struct HorboServiceController {
    pub service: Arc<Mutex<ServiceDiscovery>>,
}

impl Horbo for HorboServiceController {
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
                    Output = std::result::Result<tonic::Response<LookupResponse>, tonic::Status>,
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

    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn service_failure_report<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<FailureReportRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<()>, tonic::Status>,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(self.handle_failure_report(request))
    }

    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn heartbeat<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<HeartbeatRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<HeartbeatResponse>, tonic::Status>,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(self.heartbeat(request))
    }
}

impl HorboServiceController {
    async fn handle_failure_report(
        &self,
        request: Request<FailureReportRequest>,
    ) -> Result<Response<()>, Status> {
        // TODO: How to tell if each request is legitimate request and being made by registered node
        let ip_address = request.remote_addr();

        match ip_address {
            Some(_) => {
                let services = self.service.lock().await;
                let req_inner = request.into_inner();

                let res = services
                    .mark_node_unhealthy(req_inner.namespace, req_inner.ip_address)
                    .await;
                match res {
                    Ok(_) => return Ok(().into()),
                    Err(e) => {
                        return Err(Status::internal(e.to_string()));
                    }
                }
            }
            None => {
                return Err(Status::invalid_argument("ip is not valid"));
            }
        }
    }

    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        let ip_address = request.remote_addr();

        match ip_address {
            Some(ip) => {
                let services = self.service.lock().await;
                let req_inner = request.into_inner();

                let res = services
                    .node_heartbeat(
                        req_inner.namespace.clone(),
                        ip.to_string(),
                        UtilizationMetric {
                            cpu_usage: req_inner.cpu_usage,
                            memory_usage: req_inner.memory_usage,
                        },
                    )
                    .await;

                match res {
                    Ok(unhealthy_nodes) => {
                        return Ok(Response::new(unhealthy_nodes))
                    }
                    Err(e) => {
                        return Err(Status::internal(e.to_string()));
                    }
                }
            }
            None => {
                return Err(Status::invalid_argument("ip is not valid"));
            }
        }
    }

    async fn service_lookup(
        &self,
        request: Request<LookupRequest>,
    ) -> Result<Response<LookupResponse>, Status> {
        let client_ip_address = request.remote_addr();

        match client_ip_address {
            Some(ip) => {
                let services = self.service.lock().await;
                let req_inner = request.into_inner();

                let service_ip = services
                    .service_lookup(req_inner.namespace.clone(), ip.to_string())
                    .await;
                match service_ip {
                    Ok(service_ip) => {
                        return Ok(Response::new(LookupResponse {
                            ip_address: service_ip,
                            namespace: req_inner.namespace, // todo: send the namespace from the Ring object
                        }));
                    }
                    Err(e) => {
                        return Err(Status::internal(e.to_string()));
                    }
                }
            }
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
                        service_id: _id.to_string(),
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
