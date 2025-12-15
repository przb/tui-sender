use std::sync::mpsc;

use color_eyre::eyre::{Context, Result};

mod msgs {

    #[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Foo {
        pub name: String,
        pub age: u8,
    }

    #[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct FooResponse {
        pub message: String,
    }
}

mod server {
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
}

mod client {
    use color_eyre::{Result, eyre::Context};
    use std::{
        io::{Read, Write},
        net::ToSocketAddrs,
        sync::mpsc::Receiver,
    };

    use crate::msgs::{Foo, FooResponse};

    pub fn client<A>(addr: A, read: Receiver<()>) -> Result<()>
    where
        A: ToSocketAddrs,
    {
        read.recv().unwrap();

        let mut stream =
            std::net::TcpStream::connect(addr).context("failed to connect to remote server")?;
        let mut buf = vec![0; 1_000];

        let request = Foo {
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

        let response: FooResponse = serde_json::from_slice(buf.get(0..num_read).unwrap())
            .with_context(|| {
                format!(
                    "failed to deserialize response:\n{}",
                    String::from_utf8_lossy(&buf)
                )
            })?;

        println!("got res: {response:?}");
        Ok(())
    }
}

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
