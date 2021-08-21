pub mod fixed_header;
pub mod variable_header;

pub use fixed_header::FixedHeader;
pub use variable_header::VariableHeader;

#[derive(Debug)]
pub struct ControlPacket {
    pub fixed_header:       FixedHeader,
    pub variable_header: VariableHeader,
    pub payload:                Payload,
}

impl ControlPacket {
    pub fn new() -> ControlPacket {
        todo!()
    }
}

impl Default for ControlPacket {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct Payload {

}
