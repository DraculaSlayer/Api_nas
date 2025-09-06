use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const LHOST: &str = "127.0.0.1";
const LPORT: &str = "7777";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind(format!("{}:{}", LHOST, LPORT)).await?;

    loop {
        let (mut stream, _) = server.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match stream.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Fallo al leer los bytes: {:?}", e);
                        return;
                    }
                };

                let s: String = String::from_utf8(buf.to_vec()).expect("Fallo");

                println!("{}", s);

                if let Err(e) = stream.write_all(&buf[0..n]).await {
                    eprintln!("Fallo en escribir: {:?}", e);
                    return;
                }
            }
        });
    }

    Ok(())
}
