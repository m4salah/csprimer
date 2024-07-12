use nix::sys::socket::{
    bind, recvfrom, sendto, socket, AddressFamily, MsgFlags, SockFlag, SockType, SockaddrIn,
};
use std::{os::fd::AsRawFd, str::FromStr};

fn main() {
    let socket_fd = socket(
        AddressFamily::Inet,
        SockType::Datagram,
        SockFlag::empty(),
        None,
    )
    .expect("Failed to open socket file descriptor");

    let addr = SockaddrIn::from_str("127.0.0.1:8080").expect("Failed to make the address");

    bind(socket_fd.as_raw_fd(), &addr).expect("Failed to bind socket");

    let mut buf = [0; 1024];
    while let Ok((n_bytes, Some(client_addr))) =
        recvfrom::<SockaddrIn>(socket_fd.as_raw_fd(), &mut buf)
    {
        println!("{client_addr} sent {n_bytes} bytes");
        let buf_str = String::from_utf8_lossy(&buf[..n_bytes]).to_uppercase();
        let _ = sendto(
            socket_fd.as_raw_fd(),
            buf_str.as_bytes(),
            &client_addr,
            MsgFlags::empty(),
        );
    }
}
