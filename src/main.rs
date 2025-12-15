mod app;
mod client;
mod msgs;
mod server;
mod ui;

use color_eyre::eyre::{Context, Result};
use std::sync::mpsc;

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    let (ready_tx, ready_rx) = mpsc::channel();
    let client_addr = String::from("127.0.0.1:8888");
    let server_addr = client_addr.clone();

    let client_jh = std::thread::spawn(|| client::client(client_addr, ready_rx));
    let server_jh = std::thread::spawn(|| server::server(server_addr, ready_tx));

    let () = server_jh.join().unwrap().context("server failure")?;
    let () = client_jh.join().unwrap().context("client failed")?;

    Ok(())
}
