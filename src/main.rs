mod icmp;
mod ipv4;
mod settings;
mod util;

use icmp::{Code, Message};
use settings::include_ip_header;
use socket2::{Domain, Protocol, Socket, Type};
use std::env;
use std::mem::MaybeUninit;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::process::exit;
use std::time::Duration;
use std::time::Instant;
use util::local_addr;

const PAYLOAD: &'static str = "banana";

fn main() -> anyhow::Result<()> {
    let addr = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: pingu DEST");
        exit(1);
    });

    let socket = Socket::new(Domain::IPV4, Type::RAW, Protocol::ICMPV4.into())?;
    let mut seq = 0;

    loop {
        seq += 1;

        let mut icmp_packet = icmp::Packet::new(&mut [0u8; 8]);
        icmp_packet.set_message_type(Message::ECHO_REQUEST);
        icmp_packet.set_code(Code::ECHO_REQUEST);
        icmp_packet.set_identifier(0x1234);
        icmp_packet.set_seq_number(seq);
        icmp_packet.set_payload(PAYLOAD);
        icmp_packet.fill_checksum();

        include_ip_header(&socket, true);

        let mut ip_packet = ipv4::Packet::new([0u8; 20]);
        ip_packet.set_version(4);
        ip_packet.set_ttl(64);
        ip_packet.set_identifier(0x4321);
        ip_packet.set_protocol(ipv4::NextProtocol::ICMP);
        ip_packet.set_src(local_addr());
        ip_packet.set_header_len(5);
        ip_packet.set_dest(str::parse::<Ipv4Addr>(&addr)?);
        ip_packet.set_payload(icmp_packet.into_inner());
        ip_packet.set_total_length();
        ip_packet.fill_checksum();

        let mut reply_buf: [MaybeUninit<u8>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };
        let remote_addr = SocketAddrV4::new(Ipv4Addr::new(8, 8, 8, 8), 0);

        let start = Instant::now();

        socket.send_to(&ip_packet.into_inner(), &remote_addr.into())?;

        let (bytes_received, a) = socket.recv_from(&mut reply_buf)?;

        let remote_ip = a.as_socket_ipv4().unwrap();
        let response_buf: [u8; 1024] = unsafe { std::mem::transmute(reply_buf) };
        let response_ip_pkt = ipv4::Packet::new(&response_buf[..bytes_received]);
        let response_icmp_pkt = icmp::Packet::new(response_ip_pkt.payload());
        
        println!(
            "{} bytes from {}: icmp_seq={} ttl={} time={}",
            bytes_received,
            remote_ip.ip(),
            response_icmp_pkt.seq_number(),
            response_ip_pkt.ttl(),
            start.elapsed().as_millis()
        );

        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
