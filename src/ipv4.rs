use byteorder::{ByteOrder, LittleEndian, NativeEndian};

pub struct IPv4Packet {
    version: u8,
}

pub fn ipv4_checksum(pkt: &[u8]) -> u16 {
    let header = &pkt[..=20];
    let mut result = 0x0;
    for i in (0..20).step_by(4) {
        let v = NativeEndian::read_u16(&header[i..i + 4]);
        result += v;
    }
    !result
}
