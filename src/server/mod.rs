use super::{packet::{FixedHeader, ControlPacket, VariableHeader}, Result, ParseError};




/// Control Packets are structured as follows:
/// +----------------------------------------+
/// | Fixed Header,                          |
/// |   present in all MQTT Control Packets  |
/// +----------------------------------------+
/// | Variable Header,                       |
/// |   present in some MQTT Control Packets |
/// +----------------------------------------+
/// | Payload, present in                    |
/// |   some MQTT Control Packets            |
/// +----------------------------------------+
pub fn parse_packet(buf: &'_ [u8]) -> Result<ControlPacket> {

    Err(ParseError::Unknown)
}



fn parse_fixed_header(buf: &'_ [u8]) -> Result<FixedHeader> {
    
    Err(ParseError::Unknown)
}

fn parse_variable_header(buf: &'_ [u8]) -> Result<VariableHeader> {

    Err(ParseError::Unknown)
}
