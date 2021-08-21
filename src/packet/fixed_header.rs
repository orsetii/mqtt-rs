use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use super::super::*;


#[derive(Debug)]
pub struct FixedHeader {
    r#type: Type,
    flags:  u8,
    publish_flags: Option<flags::PublishFlags>,
    remaining_length: u32,
}

impl FixedHeader {
    pub fn new(r#type: Type, flags: u8, remaining_length: u32, publish_flags: Option<flags::PublishFlags> ) -> Self {
        Self {
            r#type,
            flags,
            remaining_length,
            publish_flags,
        }
    }
    /// Attempts to construct a `FixedHeader` from a byte buffer.
    pub fn parse_from_vec(buf: Vec<u8>) -> Result<FixedHeader> {

        let r#type = Self::parse_type(buf[0]);
        let flags = Self::parse_flags(buf[0], r#type)?;
        let remaining_length = Self::parse_remaining_length(&buf[1..5])?;
        let publish_flags = match r#type {
            Type::Publish => Some(flags::PublishFlags::from(flags)),
            _             => None,
        };
        Ok(Self::new(r#type, flags, remaining_length, publish_flags))
    }

    fn parse_type(byte: u8) -> Type {
        // This is literally impossible to fail as we 
        // account for 2^4 possible values
        Type::try_from(byte >> 4).unwrap()
    }
    /// dummy function that calls `Type::check_reserved_flags`
    fn parse_flags(byte: u8, r#type: Type) -> Result<u8> {
        let byte = byte & 0x0F;
        if r#type == Type::Publish {
            return Ok(byte);
        }
        match r#type.check_reserved_flags(byte) {
            false => Err(ParseError::InvalidFlags{
                expected: r#type.map_to_reserved_flags(),
                found: byte,
            }),
            true => Ok(byte),
        }

    }
    fn parse_remaining_length(bytes: &'_ [u8]) -> Result<u32> {
        let mut total: u32 = 0;
        // SO THIS WORKS BUT THE ONE IN THE LOOP
        // FUCKING DOESNT AHHH TOFIX  AJKFGKAJGNkj
        let curr = bytes[0];
        if bytes[0] == curr {
            println!("yo")
        }
        for i in 0..4 {
            let curr = bytes[i];
            let first_7_bits = curr & 0b01111111;
            // what the actual fuck
            // how the fuck does the line below fail
            // IM COMPARING TWO SIMPLE FUCKING u8 VALUES????!?!?!?
            if curr == first_7_bits {
                break;
            }
            total = (total << 7) | first_7_bits as u32;
        }
        if total == 0 {
            return Err(ParseError::InvalidHeader{
                expected: "a remaining length field of more than 0".to_string(), 
                found: format!("{}", total)
            });
        }
        Ok(total)
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
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
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
    #[inline(always)]
    pub fn map_to_reserved_flags(&self) -> u8 {
        use flags::*;
        match *self {
            Type::Reserved => unreachable!(),
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
        }
    }

    /// returns a true/false if the flags match the reserved 
    /// value for the specified `Type`.
    pub fn check_reserved_flags(&self, val: u8) -> bool {
        let reserved_bits = self.map_to_reserved_flags();
        // Nothing is harmed by ANDing the value like this,
        // but if provided a value such as the 1st byte
        // of the fixed header without the Type bits stripped,
        // it will still work as intended.
        (val & 0b00001111) == reserved_bits
    }
}

pub mod flags {
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
    impl From<u8> for PublishFlags {
        fn from(v: u8) -> Self {
            Self {
                dup: (v >> 3) == 1,
                // This isnt capable of failing so safe unwrap.
                qos: QosLevel::from(v & 0b0110),
               retain: (v & 0b1) == 1, 
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum QosLevel {
        Zero = 0,
        One  = 1,
        Two  = 2,
        Unknown
    }
    impl From<u8> for QosLevel {
        fn from(v: u8) -> QosLevel {
            use QosLevel::*;
            match v {
                0 => Zero,
                1 => One,
                2 => Two,
                _ => Unknown,
            }
        }
    }



    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_qos_flags() {
            let f = PUBLISH::from_bits(0b0110).unwrap();
            assert_eq!(f, PUBLISH::QOS);
        }

        // Checking that we always fail with the exact correct message on all possible failure values.
        #[test]
        fn test_qos_parsing() {
            for i in 3..=8 {
                assert_eq!(QosLevel::from(i), QosLevel::Unknown);
            }
        }
        #[test]
        fn test_header_parsing() {
            let f = [0x30, 0x16, 0x00, 0x0f, 0x4c, 0x75, 0x61, 0x20, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x54, 0x65, 0x73, 0x74, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x0a];
            let hdr = super::super::FixedHeader::parse_from_vec(Vec::from(f));
            if hdr.is_err() {
                panic!();
            }
        }
    }

}

// TODO: make some tests here...

