use std::env;
// use std::io::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{TcpListener, TcpStream},
    spawn,
};

const PAYLOAD_SIZE: usize = 16;
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
        let _ = stream.write(user_input.as_bytes()).await;
    }
}

async fn server() {
    let server_address = "127.0.0.1:6900";
    println!("Server starting at {}", &server_address);
    let listener = TcpListener::bind(server_address).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        spawn(async move { handle_client(socket).await });
    }
}
async fn handle_client(mut socket: TcpStream) {
    loop {
        let mut buf = [0; PAYLOAD_SIZE];
        socket.read_exact(&mut buf).await.unwrap();
        match str::from_utf8(&buf) {
            Ok(result) => println!("{result}"),
            Err(_) => eprintln!("String formating issue!!!!"),
        };
    }
}
