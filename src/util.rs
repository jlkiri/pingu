use pnet::{datalink, ipnetwork::IpNetwork};
use std::net::Ipv4Addr;

pub fn local_addr() -> Ipv4Addr {
    let interfaces = datalink::interfaces();
    let default_interface = interfaces
        .iter()
        .find(|e| !e.is_loopback() && !e.ips.is_empty())
        .expect("Could not find a default interface.");
    let addr = default_interface.ips.iter().next().unwrap();

    match addr {
        IpNetwork::V4(ipv4) => ipv4.ip(),
        _ => unimplemented!(),
    }
}
