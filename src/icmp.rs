use byteorder::{ByteOrder, NativeEndian, NetworkEndian};
use pnet::packet::util::checksum;
use pretty_hex::*;

pub struct Message(u8);
pub struct Code(u8);

impl Message {
    pub const ECHO_REQUEST: Self = Message(8);
}

impl Code {
    pub const ECHO_REQUEST: Self = Code(0);
}

mod field {
    use std::ops::{Range, RangeFrom};

    type Field = Range<usize>;

    pub const MSG_TYPE: usize = 0;
    pub const CODE: usize = 1;
    pub const CHECKSUM: Field = 2..4;
    pub const IDENT: Field = 4..6;
    pub const SEQ_NUM: Field = 6..8;
    pub const PAYLOAD: RangeFrom<usize> = 9..;
}

pub struct Packet {
    buffer: Vec<u8>,
}

impl Packet {
    pub fn new<T: AsRef<[u8]>>(buf: T) -> Self {
        Self {
            buffer: buf.as_ref().to_vec(),
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buffer
    }

    pub fn payload(&self) -> &[u8] {
        &self.buffer[field::PAYLOAD]
    }

    pub fn seq_number(&self) -> u16 {
        NetworkEndian::read_u16(&self.buffer[field::SEQ_NUM])
    }

    pub fn hex_dump(&self) -> Hex<Vec<u8>> {
        self.buffer.hex_dump()
    }

    pub fn set_message_type(&mut self, msg: Message) {
        let buf = &mut self.buffer;
        buf[field::MSG_TYPE] = msg.0;
    }

    pub fn set_code(&mut self, code: Code) {
        let buf = &mut self.buffer;
        buf[field::CODE] = code.0;
    }

    pub fn set_identifier(&mut self, identifier: u16) {
        let buf = &mut self.buffer;
        NetworkEndian::write_u16(&mut buf[field::IDENT], identifier);
    }

    pub fn set_seq_number(&mut self, sec_num: u16) {
        let buf = &mut self.buffer;
        NetworkEndian::write_u16(&mut buf[field::SEQ_NUM], sec_num);
    }

    pub fn fill_checksum(&mut self) {
        let buf = self.buffer.as_mut();
        let checksum = checksum(&buf, 1);
        NetworkEndian::write_u16(&mut buf[field::CHECKSUM], checksum);
    }

    pub fn set_payload<T: AsRef<[u8]>>(&mut self, payload: T) {
        self.buffer.extend_from_slice(payload.as_ref());
    }
}
