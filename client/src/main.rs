use client::files;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;

const LHOST: &str = "127.0.0.1";
const LPORT: &str = "7777";

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", LHOST, LPORT))?;

    let mut string_buf: String = String::new();
    let mut char_buf: String = String::new();
    let mut msg: Vec<String> = Vec::new();
    let mut comi: bool = false;

    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut string_buf)?;

    string_buf = string_buf.trim().to_string();

    for i in string_buf.chars() {
        // if i == '\'' {
        //     comi = true;
        // } else {
        //     char_buf.push(i);
        // }

        // if i == ' ' && comi == true {
        //     // msg.push(char_buf.clone());
        //     // char_buf = String::new();
        //     continue;
        // } else {
        //     char_buf.push(i);
        // }
        match i {
            '\'' => {
                if comi {
                    comi = false;
                } else {
                    comi = true;
                }
            }
            ' ' => {
                if comi == true {
                    char_buf.push(i);
                }
                if comi == false {
                    msg.push(char_buf.clone());
                    char_buf = String::new();
                }
            }
            _ => {
                char_buf.push(i);
            }
        }
    }
    msg.push(char_buf.clone());

    let mut arr: Box<[u8]> = Box::new([]);

    if msg[0] == "send" {
        stream.write_all(msg[0].as_bytes());

        let file_path = &msg[1];
        let path: PathBuf = PathBuf::from(file_path);

        let mut file = files::Files::new(path);

        file.content_files();

        file.metadata(&msg[1]);
    }
    if msg[0] == "quit" {
        stream.write_all(msg[0].as_bytes());
    }

    Ok(())
}
