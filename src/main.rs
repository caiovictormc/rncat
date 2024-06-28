mod common;

use std::{io::Write, net::TcpStream, time::Duration};
use clap::Parser;
use common::read_write;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    command: String,
    addr: String,
    port: u16,
}


pub async fn server(addr: &str, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", addr, port);

    let client = tokio::net::TcpListener::bind(addr).await.map_err(|_| "Failed to bind")?;
    let (handle, _) = client.accept().await.map_err(|_| "connection error")?;
    let (reader, writer) = handle.into_split();
    read_write(reader, writer).await;

    Ok(())
}


pub async fn connect(addr: &str, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", addr, port);

    let client = tokio::net::TcpStream::connect(addr).await.map_err(|_| "Failed to connect")?;
    let (reader, writer) = client.into_split();
    read_write(reader, writer).await;

    Ok(())
}


fn main() {
    let args = Args::parse();

    let runtime = tokio::runtime::Runtime::new().unwrap();

    match args.command.as_str() {
        "server" => {
            println!("Starting server on {}:{}", args.addr, args.port);
            runtime.block_on(async {
                tokio::select! {
                    _ = server(&args.addr, &args.port) => {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            });
        },
        "connect" => {
            println!("Connecting to {}:{}", args.addr, args.port);
            runtime.block_on(async {
                tokio::select! {
                    _ = connect(&args.addr, &args.port) => {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            });
        },
        _ => {
            eprintln!("Invalid command");
        }
    }

    runtime.shutdown_timeout(Duration::from_secs(0));
}
