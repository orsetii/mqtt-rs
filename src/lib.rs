extern crate num_enum;
pub mod server;
pub mod packet;
mod error;

pub use error::ParseError;
pub type Result<T> = std::result::Result<T, error::ParseError>;
