use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use super::super::*;


#[derive(Debug)]
pub struct FixedHeader {
    r#type: Type,
    flags:  u8,
    remaining_length: u32
}

impl FixedHeader {
    pub fn new(r#type: Type, flags: u8, remaining_length: u32) -> Self {
        Self {
            r#type,
            flags,
            remaining_length
        }
    }
    /// Attempts to construct a `FixedHeader` from a byte buffer.
    pub fn parse_from_buf(buf: &'_ [u8]) -> Result<FixedHeader> {

        
    }

    fn parse_type(byte: u8) -> Option<Type> {
        Type::try_from(byte >> 4).ok()
    }
    /// dummy function that calls `Type::check_reserved_flags`
    fn parse_flags(byte: u8, r#type: Type) -> bool {
        r#type.check_reserved_flags(byte)
    }
}



/// +-------------+-----------+---------------------+------------------------------------------+
/// |   Name      |   Value   |   Direction of      |   Description                            |
/// |             |           |   flow              |                                          |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | Reserved    | 0         | Forbidden           | Reserved                                 |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | CONNECT     | 1         | Client to Server    | Connection request                       |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | CONNACK     | 2         | Server to Client    | Connect acknowledgment                   |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | PUBLISH     | 3         | Client to Server or | Publish message                          |
/// |             |           | Server to Client    |                                          |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | PUBACK      | 4         | Client to Server or | Publish acknowledgment (QoS 1)           |
/// |             |           | Server to Client    |                                          |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | PUBREC      | 5         | Client to Server or | Publish received (QoS 2 delivery part 1) |
/// |             |           | Server to Client    |                                          |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | PUBREL      | 6         | Client to Server or | Publish release (QoS 2 delivery part 2)  |
/// |             |           | Server to Client    |                                          |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | PUBCOMP     | 7         | Client to Server or | Publish complete (QoS 2 delivery part 3) |
/// |             |           | Server to Client    |                                          |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | SUBSCRIBE   | 8         | Client to Server    | Subscribe request                        |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | SUBACK      | 9         | Server to Client    | Subscribe acknowledgment                 |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | UNSUBSCRIBE | 10        | Client to Server    | Unsubscribe request                      |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | UNSUBACK    | 11        | Server to Client    | Unsubscribe acknowledgment               |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | PINGREQ     | 12        | Client to Server    | PING request                             |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | PINGRESP    | 13        | Server to Client    | PING response                            |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | DISCONNECT  | 14        | Client to Server or | Disconnect notification                  |
/// |             |           | Server to Client    |                                          |
/// +-------------+-----------+---------------------+------------------------------------------+
/// | AUTH        | 15        | Client to Server or | Authentication exchange                  |
/// |             |           |   Server to Client  |                                          |
/// +-------------+-----------+---------------------+------------------------------------------+
#[derive(TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum Type {
   Reserved     = 0,
   Connect      = 1,
   ConnAck      = 2,
   Publish      = 3,
   PubAck       = 4,
   PubRec       = 5,
   PubRel       = 6,
   PubComp      = 7,
   Subscribe    = 8,
   SubAck       = 9,
   Unsubscribe  = 10,
   UnsubAck     = 11,
   PingReq      = 12,
   PingResp     = 13,
   Disconnect   = 14,
   Auth         = 15,
}

impl Type {

    /// returns a true/false if the flags match the reserved 
    /// value for the specified `Type`.
    pub fn check_reserved_flags(&self, val: u8) -> bool {
        use flags::*;
        let reserved_bits = match *self {
            Type::Connect => CONNECT,
            Type::ConnAck => CONNACK,
            Type::PubAck  => PUBACK,
            Type::PubRec  => PUBREC,
            Type::PubRel  => PUBREL,
            Type::PubComp => PUBCOMP,
            Type::Subscribe => SUBSCRIBE,
            Type::SubAck    => SUBACK,
            Type::Unsubscribe => UNSUBSCRIBE,
            Type::UnsubAck => UNSUBACK,
            Type::PingReq => PINGREQ,
            Type::PingResp => PINGRESP,
            Type::Disconnect => DISCONNECT,
            Type::Auth => AUTH,
            Type::Publish => unreachable!(),
        };
        // Nothing is harmed by ANDing the value like this,
        // but if provided a value such as the 1st byte
        // of the fixed header without the Type bits stripped,
        // it will still work as intended.
        (val & 0b00001111) == reserved_bits
    }
}

pub mod flags {
    use super::*;
    use bitflags::bitflags;
    pub const CONNECT: u8 = 0b0000;
    pub const CONNACK: u8 = 0b0000;
    pub const PUBACK: u8 = 0b0000;
    pub const PUBREC: u8 = 0b0000;
    pub const PUBREL: u8 = 0b0010;
    pub const PUBCOMP: u8 = 0b0000;
    pub const SUBSCRIBE: u8 = 0b0010;
    pub const SUBACK: u8 = 0b0000;
    pub const UNSUBSCRIBE: u8 = 0b0010;
    pub const UNSUBACK: u8 = 0b0000;
    pub const PINGREQ: u8 = 0b0000;
    pub const PINGRESP: u8 = 0b0000;
    pub const DISCONNECT: u8 = 0b0000;
    pub const AUTH: u8 = 0b0000;

    bitflags! {
        pub struct PUBLISH: u8 {
            const DUP = 0b1000;
            /// Two-bit field.
            const QOS = 0b0110;
            const RETAIN = 0b0001;
        }
    }
    #[derive(Debug)]
    pub struct PublishFlags {
        dup: bool,
        qos: QosLevel,
        retain: bool
    }

    #[derive(TryFromPrimitive, Debug)]
    #[repr(u8)]
    pub enum QosLevel {
        Zero = 0,
        One  = 1,
        Two  = 2,
    }

    pub fn parse_qos_level(qos_val: u8) -> Result<QosLevel> {
        QosLevel::try_from(qos_val).map_err(|_| ParseError::InvalidQos(format!("{}", qos_val)))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_qos_flags() {
            let f = flags::PUBLISH::from_bits(0b0110).unwrap();
            assert_eq!(f, flags::PUBLISH::QOS);
        }

        // Checking that we always fail with the exact correct message on all possible failure values.
        #[test]
        fn test_qos_parsing() {
            for i in 3..=8 {
                assert_eq!(parse_qos_level(i).unwrap_err(), ParseError::InvalidQos(format!("{}", i)))
            }
        }
    }

}

// TODO: make some tests here...

