use std::env::args;
use async_std::io;
use async_std::fs::File;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

enum ServerState {
    Ready,
    Offline
}

async fn full_connect(addr: &String) -> ServerState {
    match TcpStream::connect(addr).await {
        Ok(_) => ServerState::Ready,
        Err(_) => ServerState::Offline
    }
}

#[async_std::main]
async fn main() -> io::Result<()> {
    for path in args().skip(1) {
        let file = File::open(&path).await?;
        let mut lines = io::BufReader::new(file).lines();    

        while let Some(line) = lines.next().await {
            let addr = line?;
            task::spawn(async move {
                match full_connect(&addr).await {
                    ServerState::Ready => println!("{}", addr),
                    ServerState::Offline => {}
                }
            });
        }
    }
    Ok(())
}