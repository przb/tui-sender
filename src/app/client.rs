use color_eyre::{Result, eyre::Context};
use std::{
    io::{Read, Write},
    net::ToSocketAddrs,
    sync::mpsc::Receiver,
};

use crate::app::msgs::{GreetPerson, GreetPersonResp};

pub fn client<A>(addr: A, read: Receiver<()>) -> Result<()>
where
    A: ToSocketAddrs,
{
    read.recv().unwrap();

    let mut stream =
        std::net::TcpStream::connect(addr).context("failed to connect to remote server")?;
    let mut buf = vec![0; 1_000];

    let request = GreetPerson {
        name: "Oreo".into(),
        age: 1,
    };

    let req_bytes = serde_json::to_vec(&request).context("failed to serialize request")?;
    stream
        .write_all(&req_bytes)
        .context("client failed to write to tcp stream")?;

    let num_read = stream
        .read(&mut buf)
        .context("failed to read from stream")?;

    let response: GreetPersonResp = serde_json::from_slice(buf.get(0..num_read).unwrap())
        .with_context(|| {
            format!(
                "failed to deserialize response:\n{}",
                String::from_utf8_lossy(&buf)
            )
        })?;

    println!("got res: {response:?}");
    Ok(())
}
