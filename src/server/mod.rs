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
fn bob() {} // TODO REMOVE
