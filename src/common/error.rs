use std::fmt::Display;


#[derive(Debug)]
pub enum ErrorResponse {
    Internal(String),
    BadRequest(String),
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorResponse::Internal(err) => write!(f, "internal error: {}", err),
            ErrorResponse::BadRequest(err) => write!(f, "bad request: {}", err),
        }
    }
}

impl std::error::Error for ErrorResponse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorResponse::Internal(_) => None,
            ErrorResponse::BadRequest(_) => None,
        }
    }
}