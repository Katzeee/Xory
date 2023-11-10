use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReqErr {
    #[error("Something wrong in parameter: {0}.")]
    ReqParamError(String),
    #[error("Not authorized.")]
    NotAuthorizedError,
    #[error("Login failed.")]
    LoginError,
}
