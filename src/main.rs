use log::{info, trace};
use simple_logger;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};
use tokio;

#[tokio::main]
async fn main() {
    simple_logger::init().unwrap();

    let port: &str = "8080";
    let host: &str = "localhost:";
    let server_addr = format!("{}{}", &host, &port);

    let listener = TcpListener::bind(&server_addr).unwrap();
    info!("Server is Running on port : {}", &port);

    loop {
        trace!("Awaiting connection...");
        let (mut socket, addr) = listener.accept().unwrap();
        info!("client connected :{}", addr);

        tokio::spawn(async move {
            loop {
                let mut reader = BufReader::new(&mut socket);
                let mut line: String = String::new();

                let bytes_read: usize = reader.read_line(&mut line).unwrap();

                if bytes_read == 0 {
                    break;
                }
                let _ = socket
                    .write_all(&mut line.as_bytes())
                    .expect("failed to write data to socket");

                let _ = socket.flush().expect("failed to flush socket");
                info!("client {} sent message : {:?}", addr, line.trim_end());
                line.clear();
            }
        });
    }
}
