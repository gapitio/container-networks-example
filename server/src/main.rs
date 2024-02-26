use std::{io::Read, net::TcpListener};

fn main() {
    let server_socket_address =
        std::env::var("SERVER_SOCKET_ADDRESS").expect("SERVER_SOCKET_ADDRESS must be set");
    println!("Starting server at {}", server_socket_address);
    let server_socket_address: std::net::SocketAddr = server_socket_address.parse().unwrap();

    let listener = TcpListener::bind(server_socket_address).unwrap();
    println!("Server running at {}", server_socket_address);
    let (mut stream, addr) = listener.accept().unwrap();
    println!("Accepted connection from {}", addr);
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();
    let message = std::str::from_utf8(&buf[..n]).unwrap();
    println!("Read {} bytes from stream with the message: {:?}", n, message);
}
