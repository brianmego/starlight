use axum::{http::StatusCode, response::{ IntoResponse, Response }};
use derive_more::From;

use crate::handlers::login::LoginError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(From)]
pub enum Error {
    #[from]
    LoginError(LoginError),

    // -- Externals
    #[from]
    DbError(surrealdb::Error),
}

// impl Error {
//     pub fn custom(val: impl std::fmt::Display) -> Self {
//         Self::Custom(val.to_string())
//     }
// }

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoginError(arg0) => f.debug_tuple("LoginError").field(arg0).finish(),
            Self::DbError(arg0) => f.debug_tuple("DbError").field(arg0).finish(),
        }
    }
}

impl std::error::Error for Error {}
