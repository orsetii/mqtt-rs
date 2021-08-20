use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MqttError {
    details: String,
}

impl MqttError {
    fn new(msg: &str) -> MqttError {
        MqttError{details: msg.to_string()}
    }
}

impl fmt::Display for MqttError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MqttError {
    fn description(&self) -> &str {
        &self.details
    }
}


pub type Result<T> = std::result::Result<T,MqttError>;
