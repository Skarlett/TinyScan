use std::env::args;
use async_std::io;
use async_std::fs::File;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;


#[async_std::main]
async fn main() -> io::Result<()> {
    for path in args().skip(1) {
        let file = File::open(&path).await?;
        let mut lines = io::BufReader::new(file).lines();    

        while let Some(Ok(addr)) = lines.next().await {
            task::spawn(async move {
                match TcpStream::connect(addr).await {
                    Ok(_) => {},
                    Err(_) => {}
                }
            });
        }
    }
    Ok(())
}


