use socket2::{Domain, Socket, Type};
use std::net::{SocketAddr, TcpListener};
use std::io::Result;

fn main() -> Result<()> {
    // Create a TCP listener bound to two addresses.
    let socket = Socket::new(Domain::PACKET, Type::DGRAM, None)?;

    let address: SocketAddr = "[::1]:12345".parse().unwrap();
    socket.bind(&address.into())?;
    socket.set_only_v6(false)?;
    socket.listen(128)?;

    let listener: TcpListener = socket.into();

    Ok(())
}
