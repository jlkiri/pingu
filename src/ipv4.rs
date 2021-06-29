use std::{convert::TryInto, net::Ipv4Addr};

use byteorder::{ByteOrder, NetworkEndian};
use pnet::packet::util::checksum;
use pretty_hex::*;

use self::field::TTL;

pub struct NextProtocol(u8);

impl NextProtocol {
    pub const ICMP: Self = NextProtocol(1);
}

mod field {
    use std::ops::RangeFrom;
    type Field = std::ops::Range<usize>;

    pub const VER_IHL: usize = 0;
    pub const DSCP_ECN: usize = 1;
    pub const LENGTH: Field = 2..4;
    pub const IDENT: Field = 4..6;
    pub const FLG_OFF: Field = 6..8;
    pub const TTL: usize = 8;
    pub const PROTOCOL: usize = 9;
    pub const CHECKSUM: Field = 10..12;
    pub const SRC_ADDR: Field = 12..16;
    pub const DST_ADDR: Field = 16..20;
    pub const PAYLOAD: RangeFrom<usize> = 20..;
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

    pub fn hex_dump(&self) -> Hex<Vec<u8>> {
        self.buffer.hex_dump()
    }

    pub fn payload(&self) -> &[u8] {
        &self.buffer[field::PAYLOAD]
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buffer
    }

    pub fn ttl(&self) -> u8 {
        self.buffer[TTL]
    }

    pub fn set_version(&mut self, version: u8) {
        let buf = &mut self.buffer;
        buf[field::VER_IHL] = buf[field::VER_IHL] | (version << 4);
    }

    pub fn set_total_length(&mut self) {
        let len = self.buffer.len();
        let buf = &mut self.buffer;
        NetworkEndian::write_u16(&mut buf[field::LENGTH], len as u16);
    }

    pub fn set_identifier(&mut self, identifier: u16) {
        let buf = &mut self.buffer;
        NetworkEndian::write_u16(&mut buf[field::IDENT], identifier);
    }

    pub fn set_header_len(&mut self, len: u8) {
        let buf = &mut self.buffer;
        buf[field::VER_IHL] = buf[field::VER_IHL] | len;
    }

    pub fn set_ttl(&mut self, ttl: u8) {
        let buf = &mut self.buffer;
        buf[field::TTL] = ttl;
    }

    pub fn set_protocol(&mut self, protocol: NextProtocol) {
        let buf = &mut self.buffer;
        buf[field::PROTOCOL] = protocol.0;
    }

    pub fn set_src(&mut self, addr: Ipv4Addr) {
        let buf = &mut self.buffer;
        NetworkEndian::write_u32(&mut buf[field::SRC_ADDR], addr.try_into().unwrap());
    }

    pub fn set_dest(&mut self, addr: Ipv4Addr) {
        let buf = &mut self.buffer;
        NetworkEndian::write_u32(&mut buf[field::DST_ADDR], addr.try_into().unwrap());
    }

    pub fn fill_checksum(&mut self) {
        let buf = self.buffer.as_mut();
        let checksum = checksum(&buf, 6);
        NetworkEndian::write_u16(&mut buf[field::CHECKSUM], checksum);
    }

    pub fn set_payload<T: AsRef<[u8]>>(&mut self, payload: T) {
        self.buffer.extend_from_slice(payload.as_ref());
    }
}
