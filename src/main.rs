mod ipv4;

use libc;
use libc::c_void;
use pnet::packet::icmp::IcmpType;
use pnet::packet::icmp::IcmpTypes::EchoRequest;
use pnet::packet::icmp::*;
use pnet::packet::*;
use pnet::util::checksum;
use pnet_sys::setsockopt;

use pretty_hex::*;
use smoltcp::socket::SocketRef;
use smoltcp::wire::{Icmpv4Message, IpProtocol};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::io::Read;
use std::mem::{self, MaybeUninit};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, TcpListener};
// use std::os::windows::prelude::AsRawSocket;
use byteorder::NetworkEndian;
use byteorder::*;
mod settings;

const IPV4_MAX_PACKET_SIZE: usize = 65535;
const ICMP_MAX_PACKET_SIZE: usize = 65507;

use smoltcp::wire::Ipv4Packet;

fn main() -> anyhow::Result<()> {
    let mut socket = Socket::new(Domain::IPV4, Type::RAW, Protocol::ICMPV4.into())?;

    let mut ip: [u8; 52] = [0u8; 52];
    let mut icmp_buf = [0u8; 32];

    let opt = true;

    settings::include_ip_header(&socket, &opt);

    /* socket.bind(&SockAddr::from(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))).expect("failed bind.");

    loop {
        let len = socket.read(&mut buf)?;
        println!("{:?}", &buf[..len]);
    } */

    /* let mut ipv4packet = Ipv4Packet::new_checked(buf).expect("Failed to construct an IPv4 packet.");
    let local_addr = *socket
        .local_addr()?
        .as_socket_ipv4()
        .unwrap_or_else(|| panic!("Not a IPv4 socket!"))
        .ip();

    println!("{}",local_addr);
    ipv4packet.set_src_addr(smoltcp::wire::Ipv4Address::from(local_addr));
    ipv4packet.set_dst_addr(smoltcp::wire::Ipv4Address::from_bytes(&[8, 8, 8, 8]));
    ipv4packet.set_protocol(IpProtocol::Icmp);
    ipv4packet.set_version(4);
    ipv4packet.set_header_len(20); */

    let mut icmp_packet =
        MutableIcmpPacket::new(&mut icmp_buf).expect("Failed to construct an ICMP packet.");

    icmp_packet.set_icmp_type(EchoRequest);
    icmp_packet.set_payload("hello!".as_bytes());
    icmp_packet.set_checksum(checksum(icmp_packet.packet(), 8));

    let mut reply_buf: [MaybeUninit<u8>; 128] = unsafe { MaybeUninit::uninit().assume_init() };

    /*
    45 00 00 ??
    ab cd 00 00
    40 01 ?? ??
    00 00 00 00
    08 08 08 08
    */

    /* let local_addr = *socket
        .local_addr()?
        .as_socket_ipv4()
        .unwrap_or_else(|| panic!("Not a IPv4 socket!"))
        .ip();

    println!("local addr: {}", local_addr); */

    ip[0] = 0x45;
    NetworkEndian::write_u16(&mut ip[2..=3], icmp_packet.packet().len() as u16);
    NetworkEndian::write_u16(&mut ip[4..=5], 0xabcd);
    NetworkEndian::write_u16(&mut ip[8..=9], 0x4001);
    NetworkEndian::write_u32(&mut ip[12..=15], 0x00000000);
    NetworkEndian::write_u32(&mut ip[16..=19], 0x08080808);

    let icmp_pkt = icmp_packet.packet();

    for i in 0..52 - 20 {
        ip[i + 20] = icmp_pkt[i].to_be();
    }

    let mut pkt = Ipv4Packet::new_checked(ip).expect("Dderp");
    pkt.fill_checksum();

    println!("{:?}", pkt.hex_dump());

    //loop {
    // let len = socket.send_to(unsafe { any_as_u8_slice(&icmp_packet) }, &SocketAddrV4::new(Ipv4Addr::new(8, 8, 8, 8), 0).into())?;
    let len = socket.send_to(
        &pkt.into_inner(),
        &SocketAddrV4::new(Ipv4Addr::new(8, 8, 8, 8), 0).into(),
    )?;
    /* let len = socket.send_to(
        icmp_packet.packet(),
        &SocketAddrV4::new(Ipv4Addr::new(8, 8, 8, 8), 0).into(),
    )?; */
    // let len = socket.send(&ipv4packet.into_inner())?;
    println!("{} sent", len);
    let (read_len, a) = socket.recv_from(&mut reply_buf)?;
    let readable_buf: [u8; 128] = unsafe { std::mem::transmute(reply_buf) };
    // println!("response: {:?}", &readable_buf[..read_len]);
    let reply_packet = IcmpPacket::new(&readable_buf[..read_len]).expect("FAIL");
    let d = &readable_buf[..read_len];
    println!("{:?}", d.hex_dump());

    Ok(())
}
