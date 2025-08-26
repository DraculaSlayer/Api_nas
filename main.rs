use std::io::BufReader;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::sync::mpsc;
use std::thread;

//Define los la ip y el puerto
const LHOST: &str = "127.0.0.1";
const LPORT: &str = "7777";

fn listen(tx_pipe: mpsc::Sender<u32>) -> std::io::Result<()> {
    //Inicia el servidor con el host y los puerto ya definidos
    let listener = TcpListener::bind(format!("{}:{}", LHOST, LPORT))?;

    //Hace un bucle que lista todo las coneciones
    for stream in listener.incoming() {
        //Pregunta si stream no es un error
        match stream {
            Ok(stream) => {
                //Imprime el cliente en el que haya conecion
                println!(
                    "Received a connection! - {}:{}",
                    stream.peer_addr()?.ip(),
                    stream.peer_addr()?.port()
                );

                //Clona el tx_pipe para crear un hilo
                let txp = tx_pipe.clone();
                thread::spawn(move || {
                    connect_handler(stream, txp);
                });
                let _ = tx_pipe.send(1);
            }
            Err(e) => println!("Error! - {}", e),
        }
    }

    //"Ejecuta" la variable listener
    drop(listener);

    Ok(())
}

fn connect_handler(stream: TcpStream, tx_pipe: mpsc::Sender<u32>) -> std::io::Result<()> {
    //Crea un buffer cloando las propiedades del cliente
    let mut buf = BufReader::new(stream.try_clone()?);

    loop {
        //let mut s = [0u8; 4096];
        let mut s = String::new();

        //Lee la informacion que se manda el cliente y la imprime en pantalla
        match buf.read_line(&mut s) {
            Ok(b) => {
                //Si la variable "b" (los bytes que manda) es igual a 0 cancela todo el bucle
                if b == 0 {
                    break;
                }

                //Imprime los bytes que se manda y la cadena de caracteres
                println!("Received data ({} bytes): {}", b, s);

                //Ve si el cliente mando el comando "quit" para terminar la conecion
                if s.contains("quit") {
                    tx_pipe.send(2);
                    break;
                }
            }
            Err(e) => println!("Error receiving data! - {}", e),
        }
    }

    //Imprime cual cliente se desconecto
    println!(
        "Client {}:{} dropped",
        stream.peer_addr().unwrap().ip(),
        stream.peer_addr().unwrap().port()
    );
    stream.shutdown(Shutdown::Both)?;

    Ok(())
}

fn main() {
    println!("{}", format!("Initializing: {}:{}", LHOST, LPORT));
    println!("Ctrl+C to exit");

    //Crea un canal de "Transferencia" y de "Reception"
    let (channel_tx, channel_rx) = mpsc::channel();

    //Clona el canal de envio y ejecuta "listen(tx_pipe)"
    let tx_pipe = channel_tx.clone();
    thread::spawn(move || {
        listen(tx_pipe);
    });

    //Hace recuento de el total de cliente que obtuvo
    let mut connections = 0;
    loop {
        match channel_rx.recv() {
            Ok(signal) => match signal {
                1 => connections += 1,
                2 => break,
                _ => println!("Invalid signal received: {}", signal),
            },
            Err(e) => {
                println!("Pipe broken - {}", e);
            }
        }
    }

    println!("Total connections: {}", connections);
    println!("Exiting");
}
