use axum::{http::StatusCode, response::{ IntoResponse, Response }};
use derive_more::From;

use crate::handlers::login::LoginError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),
    #[from]
    LoginError(LoginError),

    // -- Externals
    #[from]
    Io(()), // as example
    #[from]
    DbError(surrealdb::Error),
}

// impl Error {
//     pub fn custom(val: impl std::fmt::Display) -> Self {
//         Self::Custom(val.to_string())
//     }
// }

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
