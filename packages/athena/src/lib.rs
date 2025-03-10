pub use athena_macros::*;

pub mod entities;
pub mod environment;
pub mod prelude;
pub mod errors;

pub trait RequestError {
    fn get_error_code(&self) -> &str;
}