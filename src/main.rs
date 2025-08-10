use std::env;
// use std::io::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{TcpListener, TcpStream},
    spawn,
};

const PAYLOAD_SIZE: usize = 10;
fn error(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        error("Not enough argument provided!")
    }
    match args[1].as_str() {
        "client" => client().await,
        "server" => server().await,
        _ => error("Unknown argument!"),
    }
}

async fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:6900").await.unwrap();
    println!("Connected");
    // let mut data = [0; PAYLOAD_SIZE];
    loop {
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("There should be a message!");
        let _ = stream.write_all(&user_input.into_bytes()).await;
    }
}

async fn server() {
    let server_address = "127.0.0.1:6900";
    println!("Server starting at {}", &server_address);
    let listener = TcpListener::bind(server_address).await.unwrap();
    loop {
        let (socket, port) = listener.accept().await.unwrap();
        spawn(async move { handle_client(socket, port).await });
    }
}
async fn handle_client(mut socket: TcpStream, port: std::net::SocketAddr) {
    println!("[server] New Client from {port}");
    loop {
        let mut buf = [0; PAYLOAD_SIZE];
        let n = socket.read(&mut buf).await.unwrap();
        if n == 0 {
            println!("[server] Client exited gracefully!");
            return;
        }
        let remove_empty = &buf[..n]
            .iter()
            .copied()
            .filter(|&x| x != 0u8 || x != b'\n')
            .collect::<Vec<u8>>();
        let string_literal = str::from_utf8(remove_empty);
        match string_literal {
            Ok(result) => print!("[client: {port}] {result}"),
            Err(_) => println!("[server] issue with string formating"),
        }
    }
}
