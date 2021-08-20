//! MQTT control packets consist of three parts:
//! 
//! | Part  | 
//! |-------------- | 
//! | Fixed Header, present in all MQTT Control Packets    | 
//! | Variable Header, present in some MQTT Control Packets|
//! | Payload, present in some MQTT Control packets        |
use super::super::*;
use nom::{alt,named,IResult};
use nom::bits::{bits, complete::{tag,take}};
use nom::sequence::tuple;


#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    fixed_hdr: FixedHeader,
}
/// Fixed Header Layout:
/// ```text
/// +---------+-----+-----+----+----+--------+--------+-------+-------+
/// | Bit     | 7   | 6   | 5  | 4  |    3   |   2    |   1   |   0   |
/// +---------+-----+-----+----+----+--------+--------+-------+-------+
/// | byte 1  | MQTT Control Packet | Flags specific to               |
/// |         |   type              |   each MQTT Control Packet type |
/// +---------+---------------------+---------------------------------+
/// | byte 2… | Remaining Length                                      |
/// +---------+-------------------------------------------------------+`

#[derive(Debug, PartialEq, Eq)]
pub struct FixedHeader {
    r#type: Type,
    flags: u8,
}


pub fn parse_packet(packet: &'_ [u8]) {

    

}

#[derive(Debug,PartialEq,Eq)]
pub enum Type {
    Reserved,

    Connect,
    ConnAck,

    Publish,
    PubAck,
    PubRec,
    PubRel,
    PubComp,

    Subscribe,
    SubAck,

    Unsubscribe,
    UnsubAck,

    PingReq,
    PingResp,
    
    Disconnect,
    Auth,

    Unknown(usize)
}

// Parses the type (4 bit field) from the fixed packet header.
named!(packet_type<Type>, alt!(
    bits!(tag_bits!(4usize, 0))            => { |_| Type::Reserved    } |
    bits!(tag_bits!(4usize, 1))            => { |_| Type::Connect     } |
    bits!(tag_bits!(4usize, 2))            => { |_| Type::ConnAck     } |
    bits!(tag_bits!(4usize, 3))            => { |_| Type::Publish     } |
    bits!(tag_bits!(4usize, 4))            => { |_| Type::PubAck      } |
    bits!(tag_bits!(4usize, 5))            => { |_| Type::PubRec      } |
    bits!(tag_bits!(4usize, 6))            => { |_| Type::PubRel      } |
    bits!(tag_bits!(4usize, 7))            => { |_| Type::PubComp     } |
    bits!(tag_bits!(4usize, 8))            => { |_| Type::Subscribe   } |
    bits!(tag_bits!(4usize, 9))            => { |_| Type::SubAck      } |
    bits!(tag_bits!(4usize, 10))           => { |_| Type::Unsubscribe } |
    bits!(tag_bits!(4usize, 11))           => { |_| Type::UnsubAck    } |
    bits!(tag_bits!(4usize, 12))           => { |_| Type::PingReq     } |
    bits!(tag_bits!(4usize, 13))           => { |_| Type::PingResp    } |
    bits!(tag_bits!(4usize, 15))           => { |_| Type::Auth        }
    )
);

named!(take_flags<u8>, bits!(take_bits!(4usize)));




fn nom_fixed_header(header: &'_ [u8]) -> IResult<&[u8], FixedHeader> {
    
    let (input, (r#type, flags)) = tuple((packet_type, take_flags))(header)?;

    Ok((input, FixedHeader { r#type, flags}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_fixed_header() {
        let buf = [0x30, 0x16, 0x00, 0x0f, 0x4c, 0x75, 0x61, 0x20, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x54, 0x65, 0x73, 0x74, 0x48, 0x65, 0x6c, 0x6c, 0x6f];
        println!("{:#?}",nom_fixed_header(&buf));
    }
}

