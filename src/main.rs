#![expect(unused)]

mod app;
mod args;
mod ui;

use clap::Parser;
use color_eyre::{
    eyre::{Context, Result},
    owo_colors::OwoColorize,
};
use itertools::Itertools;
use std::sync::mpsc;
use walkdir::WalkDir;

use crate::app::{
    MsgMetaData, client,
    people_info::{Kilograms, Pet, PetType},
    server,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = args::Args::parse();

    let messages_dir = args.messages_dir.unwrap_or("messages".into());

    let messages = WalkDir::new(messages_dir)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| e.file_type().is_file())
        .filter_map(|e| e.ok().map(|entry| entry.into_path()))
        .collect_vec();

    println!("all messages: {messages:?}");

    let (ready_tx, ready_rx) = mpsc::channel();
    let client_addr = String::from("127.0.0.1:8888");
    let server_addr = client_addr.clone();

    let client_jh = std::thread::spawn(|| client::client(client_addr, ready_rx));
    let server_jh = std::thread::spawn(|| server::server(server_addr, ready_tx));

    let () = server_jh.join().unwrap().context("server failure")?;
    let () = client_jh.join().unwrap().context("client failed")?;

    Ok(())
}
