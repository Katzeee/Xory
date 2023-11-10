use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReqErr {
    #[error("Something wrong in parameter: {0}")]
    ReqParamError(String),
}
