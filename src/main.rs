use std::net::ToSocketAddrs;

use color_eyre::eyre::{Context, Result, bail};

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Foo {
    name: String,
    age: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct FooResponse {
    message: String,
}

fn server<A>(addr: A) -> Result<()>
where
    A: ToSocketAddrs,
{
    let s = std::net::TcpListener::bind(addr).context("failed to bind to socket")?;

    let (stream, addrs) = s.accept().context(format!(
        "failed to accept connection (at {})",
        s.local_addr()
            .map(|ad| ad.to_string())
            .unwrap_or("[unknown address]".into())
    ))?;

    // TODO log this instead
    println!("accepted connection at {}", addrs);

    let f: Foo = serde_json::from_reader(stream).context("failed to read from stream")?;

    let message = format!(
        "Hello {}, you are {} years old? That's ancient!",
        f.name, f.age
    );

    let response = FooResponse { message };

    println!("response: {response:?}");

    Ok(())
}

fn client<A>(addr: A) -> Result<()>
where
    A: ToSocketAddrs,
{
    let stream =
        std::net::TcpStream::connect(addr).context("failed to connect to remote server")?;

    let request = Foo {
        name: "Oreo".into(),
        age: 1,
    };

    serde_json::to_writer(stream, &request).context("client failed to write to tcp stream")?;

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    let client_addr = String::from("127.0.0.1:8888");
    let server_addr = client_addr.clone();

    let client_jh = std::thread::spawn(|| client(client_addr));
    let server_jh = std::thread::spawn(|| server(server_addr));

    let () = server_jh.join().unwrap().context("server failed")?;
    let () = client_jh.join().unwrap().context("client failed")?;

    Ok(())
}
