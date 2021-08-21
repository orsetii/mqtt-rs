use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    //#[error("data store disconnected")]
    //Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("invalid QoS level (must be 0 >= 2, found {0})")]
    InvalidQos(u8),
    #[error("invalid flag bits(expected {expected:0b} found {found:0b})")]
    InvalidFlags{
        expected: u8,
        found: u8,
    },
    #[error("unknown mqtt error")]
    Unknown,
}
