use std::io::{Read, Write};
use std::net::{Shutdown};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::net::{UnixListener, UnixStream};
use std::str;
use std::thread;

use rdmcommon::ipc;

pub fn test_ipc() {
    let mut sock = UnixListener::bind("/home/florian/tmp/sock").unwrap();
    println!("Opened socket");

    for stream in sock.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                {
                    let fd = &mut stream.as_raw_fd();
                    println!("fd: {:?}", fd);
                }
                thread::spawn(move || handle_greeter(stream));
            }
            Err(err) => {
                /* connection failed */
                break;
            }
        }
    }

    println!("Dropping socket");
    drop(sock);
    println!("Dropped socket");
}

fn handle_greeter(mut stream: UnixStream) {
    println!("[handle_greeter]: Accepting new client!");

    loop {
        println!("[handle_greeter]: Waiting for client message..");

        match handle_message(&stream) {
            Ok(resp)  => {
                println!("[handle_greeter]: Writing response");
                resp.write_to_stream(&mut stream);
            },
            Err(_)  => {
                close_socket(stream);
                return;
            }
        }
    }
}

fn handle_message(stream: &UnixStream) -> Result<ipc::IpcMessage, ipc::IpcParseError>{
    let msg = ipc::IpcMessage::from_reader(stream)?;

    println!("[handle_message]: Got valid message: {:?}", &msg);
    let resp = ipc::IpcMessage::assemble(ipc::IpcMessageKind::ServerHello, None);
    Ok(resp)
}

fn close_socket(stream: UnixStream) {
    println!("Shutting down stream: {:?}", stream);
    stream.shutdown(Shutdown::Both);
}