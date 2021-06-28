mod icmp;
mod ipv4;
mod settings;

use pretty_hex::*;
use socket2::{Domain, Protocol, Socket, Type};
use std::mem::{MaybeUninit};
use std::net::{Ipv4Addr, SocketAddrV4};

const IPV4_MAX_PACKET_SIZE: usize = 65535;
const ICMP_MAX_PACKET_SIZE: usize = 65507;


fn main() -> anyhow::Result<()> {
    let mut socket = Socket::new(Domain::IPV4, Type::RAW, Protocol::ICMPV4.into())?;

    let mut icmp_packet =  icmp::Packet::new(&mut [0u8; 8]);
    icmp_packet.set_message_type(icmp::Message::EchoRequest);
    icmp_packet.set_code(icmp::Code::EchoRequest);
    icmp_packet.set_identifier(0xabcd);
    icmp_packet.set_seq_number(0);
    icmp_packet.set_payload("hello!");
    icmp_packet.fill_checksum();

    let opt = true;
    settings::include_ip_header(&socket, &opt);

    let mut ip_packet = ipv4::Packet::new([0u8; 20]);
    ip_packet.set_version(4);
    ip_packet.set_ttl(64);
    ip_packet.set_identifier(0xabcd);
    ip_packet.set_protocol(ipv4::NextProtocol::ICMP);
    ip_packet.set_src(Ipv4Addr::new(0, 0, 0, 0));
    ip_packet.set_header_len(5);
    ip_packet.set_dest(Ipv4Addr::new(8, 8, 8, 8));
    ip_packet.set_payload(icmp_packet.as_buf());
    ip_packet.set_total_length();
    ip_packet.fill_checksum();

    let mut reply_buf: [MaybeUninit<u8>; 128] = unsafe { MaybeUninit::uninit().assume_init() };

    let buffed = ip_packet.as_buf();
    println!("{:?}", buffed.hex_dump());

    let len = socket.send_to(&buffed, &SocketAddrV4::new(Ipv4Addr::new(8, 8, 8, 8), 0).into())?;

    let (read_len, a) = socket.recv_from(&mut reply_buf)?;
    let readable_buf: [u8; 128] = unsafe { std::mem::transmute(reply_buf) };
    let reply_packet = icmp::Packet::new(&readable_buf[20..read_len]);
    let d = &readable_buf[..read_len];
    println!("{:?}", d.hex_dump());
    println!("{}", String::from_utf8_lossy(reply_packet.payload()));

    Ok(())
}
