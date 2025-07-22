//
use std::pin::Pin;

use tonic::{Response, Status};

use crate::grpc::{self, horbo_server::Horbo, *};

pub struct HorboService {}

impl Horbo for HorboService {
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn register_agent<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<grpc::AgentRegistrationRequest>,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = std::result::Result<
                        tonic::Response<grpc::AgentRegistrationResponse>,
                        tonic::Status,
                    >,
                > + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        my_handler()
    }
}

fn my_handler()
-> Pin<Box<dyn Future<Output = Result<Response<AgentRegistrationResponse>, Status>> + Send>> {
    Box::pin(test2())
}

async fn test2() -> Result<Response<AgentRegistrationResponse>, Status> {
    Ok(Response::new(AgentRegistrationResponse {
        client_id: "1".to_string(),
    }))
}