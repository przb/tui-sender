use color_eyre::{Result, eyre::Context};
use std::{
    io::{Read, Write},
    net::ToSocketAddrs,
    sync::mpsc::Sender,
};

use crate::msgs::{Foo, FooResponse};

pub fn server<A>(addr: A, ready: Sender<()>) -> Result<()>
where
    A: ToSocketAddrs,
{
    let s = std::net::TcpListener::bind(addr).context("failed to bind to socket")?;
    let mut buf = vec![0; 1_000];

    ready.send(()).unwrap();

    let (mut stream, addrs) = s.accept().context(format!(
        "failed to accept connection (at {})",
        s.local_addr()
            .map(|ad| ad.to_string())
            .unwrap_or("[unknown address]".into())
    ))?;

    // TODO log this instead
    println!("accepted connection at {}", addrs);

    let num_read = stream
        .read(&mut buf)
        .context("failed to read from stream")?;

    let f: Foo = serde_json::from_slice(&buf.get(0..num_read).unwrap()).with_context(|| {
        format!(
            "failed to deserialize data:\n{}",
            String::from_utf8_lossy(&buf)
        )
    })?;

    buf.clear();

    let message = format!(
        "Hello {}, you are {} years old? That's ancient!",
        f.name, f.age
    );

    let response = FooResponse { message };

    let res_bytes = serde_json::to_vec(&response).expect("failed to serialize response");
    let () = stream
        .write_all(&res_bytes)
        .context("failed to write to stream")?;

    Ok(())
}
