#![allow(unreachable_code)]

use std::{error::Error, os::fd::AsRawFd, str::FromStr, thread};

use http_header_server::http::{
    request::HTTPRequest,
    response::{ContentType, HTTPResponse, StatusCode},
};
use nix::{
    sys::socket::{
        accept, bind, listen, recv, send, socket, AddressFamily, Backlog, MsgFlags, SockFlag,
        SockType, SockaddrIn,
    },
    unistd::close,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn create_tcp_server(addr: &str) -> Result<impl AsRawFd> {
    let socket_fd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )?;
    let socket_addr = SockaddrIn::from_str(addr)?;

    bind(socket_fd.as_raw_fd(), &socket_addr)?;

    listen(&socket_fd, Backlog::new(1)?)?;

    return Ok(socket_fd);
}

fn handle_connection(fd: impl AsRawFd) {
    let mut buf = [0; 1024];

    while let Ok(n_bytes) = recv(fd.as_raw_fd(), &mut buf, MsgFlags::empty()) {
        if n_bytes == 0 {
            let _ = close(fd.as_raw_fd());
            break;
        }
        let buf_str = String::from_utf8_lossy(&buf[..n_bytes]);

        let http_request = match HTTPRequest::from_str(&buf_str) {
            Ok(request) => request,
            Err(e) => {
                eprintln!("Error while parsing the request {e:?}");
                continue;
            }
        };

        let default_message = r#"{"message":"Yay!"}"#.to_string();

        let response = HTTPResponse::new(
            StatusCode::OK,
            ContentType::Json,
            serde_json::to_string(&http_request.headers).unwrap_or(default_message),
        );

        println!("HTTP Request: {http_request:?}");
        println!("HTTP Response: {response:?}");

        let _ = send(
            fd.as_raw_fd(),
            response.to_string().as_bytes(),
            MsgFlags::empty(),
        );
    }
}

fn main() -> Result<()> {
    let tcp_fd = create_tcp_server("127.0.0.1:8080")?;

    println!("Listining on port 8080");
    loop {
        let conn_fd = accept(tcp_fd.as_raw_fd())?;
        thread::spawn(move || handle_connection(conn_fd));
    }
    Ok(())
}
