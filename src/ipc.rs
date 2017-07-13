use std::io::{Read, Write};
use std::net::{Shutdown};
use std::os::unix::net::{UnixListener, UnixStream};
use std::str;

pub enum MessageKind {
    RequestAuthentication
}

impl MessageKind {
    pub fn from_bytes(bytes: &[u8]) -> Option<MessageKind> {
        match str::from_utf8(bytes) {
            Ok(s)   => match s {
                "RA"    => Some(MessageKind::RequestAuthentication),
                _       => None
            },
            Err(_)  => None
        }
    }
}

pub fn test_ipc() {
    use std::thread;

    let mut sock = UnixListener::bind("/home/florian/tmp/sock").unwrap();
    println!("Opened socket");

    for stream in sock.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_greeter(stream));
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
    println!("Accepting new client!");

    let mut msg = Vec::with_capacity(8);
    push_short(&mut msg, 0x3177);
    msg.extend(b"PW");
    msg.extend(b"Hi!?");
    stream.write(&msg);

    match stream.read_exact(&mut msg) {
        Ok(_)   => match handle_message(&msg) {
            Ok(_)   => println!("successfully parsed message"),
            Err(_)  => {
                close_socket(stream);
                return;
            }
        },
        Err(e)  => println!("Error during recv: {}", e)
    }
    close_socket(stream);
}

fn handle_message(msg: &[u8]) -> Result<(), ()>{
    if msg.len() != 8 {
        println!("Wrong length!");
        return Err(());
    }

    if msg[0] != 0x31 || msg[1] != 0x77 {
        println!("Wrong magic!");
        return Err(());
    }
    let kind = match MessageKind::from_bytes(&msg[2..4]) {
        Some(k) => k,
        None    => {
            println!("Invalid kind");
            return Err(());
        }
    };

    let len = read_int(&msg[5..8]);

    Ok(())
}

fn push_short(msg: &mut Vec<u8>, val: i16) {
    msg.push(((val >> 8) & 0xFF) as u8);
    msg.push((val & 0xFF) as u8);
}

fn read_int(msg: &[u8]) -> i32 {
    1337
}

fn close_socket(stream: UnixStream) {
    println!("Shutting down stream..");
    stream.shutdown(Shutdown::Both);
}