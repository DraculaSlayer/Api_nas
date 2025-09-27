use std::fs::{self, File, Metadata};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::thread;
use std::time::Duration;

const LHOST: &str = "127.0.0.1";
const LPORT: &str = "7777";

fn conection_manager(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 4];
    let mut meta_buf = [0; 1024];

    stream.read(&mut buf);

    let msg: String = String::from_utf8(buf.to_vec()).unwrap();

    if &msg != "send" {
        return Ok(());
    }

    stream.write_all("good".as_bytes());

    stream.read(&mut meta_buf);

    let file_meta: String = String::from_utf8(meta_buf.to_vec()).unwrap();

    let mut metadata: Vec<String> = Vec::new();
    let mut string_buf: String = String::new();

    for i in file_meta.chars() {
        match i {
            ';' => {
                metadata.push(string_buf);
                string_buf = String::new();
            }
            '\0' => {
                continue;
            }
            _ => {
                string_buf.push(i);
            }
        }
    }
    metadata.push(string_buf.clone());

    let mut file = File::create(metadata[0].clone())?;

    let mut file_buf = [0; 1024];

    stream.write_all("good".as_bytes());

    let mut check: bool = false;

    let size_file_client: u64 = metadata[1].parse().expect("FALLO");

    while !check {
        stream.read(&mut file_buf);

        let mut number_chunk = 1;

        println!("Chunk {}: {:?}", number_chunk, file_buf);

        for i in file_buf {
           
            if i == 0 { break; }
            file.write(&[i])?;

            let size_file_server = fs::metadata(metadata[0].clone())?.len();

            if size_file_client == size_file_server {
                check = true;
            }
        }

        number_chunk += 1;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", LHOST, LPORT))?;

    for stream in listener.incoming() {
        thread::spawn(|| {
            conection_manager(stream.unwrap());
        });
    }

    Ok(())
}
