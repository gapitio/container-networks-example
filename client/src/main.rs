use std::{
    io::Write,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
};

fn main() {
    let server_socket_address =
        std::env::var("SERVER_SOCKET_ADDRESS").expect("SERVER_SOCKET_ADDRESS must be set");
    println!("Connecting to server at {}", server_socket_address);
    let server_socket_address: SocketAddr = server_socket_address
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let mut stream = TcpStream::connect(server_socket_address).unwrap();
    let buf = b"Hello from client";
    stream.write_all(buf).unwrap();
    stream.flush().unwrap();
    let message = std::str::from_utf8(buf).unwrap();
    println!("Sent message: {:?} to server", message);
}
