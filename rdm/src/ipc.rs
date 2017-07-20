use std::io::{self, Read, Write};
use std::net::{Shutdown};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::net::{UnixListener, UnixStream};
use std::str;
use std::sync::Arc;
use std::thread;

use rdmcommon::ipc;

use futures::future::{self, Future, BoxFuture};
use tokio_core::reactor::Handle;
use tokio_service::Service;

pub struct IpcManager {
    listener: UnixListener
}

struct IpcService;

impl IpcManager {
    pub fn new() -> Result<IpcManager, ipc::IpcError> {
        let l = UnixListener::bind("/home/florian/tmp/sock")?;
        Ok(IpcManager {
            listener: l
        })
    }

    pub fn start(&mut self) {
        for socket in self.listener.incoming() {
            thread::spawn(move || accept_greeter(socket.unwrap()));
        }
    }
}

impl IpcService {
    fn handle_message(&self, msg: ipc::IpcMessage) -> io::Result<ipc::IpcMessage> {
        match msg {
            ipc::IpcMessage::ClientHello => Ok(ipc::IpcMessage::ServerHello),
            _   => Err(io::Error::new(io::ErrorKind::InvalidData, ipc::IpcError::InvalidMessageKind))
        }
    }
}

/*impl Service for IpcService {
    // These types must match the corresponding protocol types:
    type Request = ipc::IpcMessage;
    type Response = ipc::IpcMessage;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        match self.handle_message(req) {
    let resp = 
            Ok(resp) => future::ok(resp).boxed(),
            Err(err) => future::err(err).boxed()
        }
    }
}

fn listen(listener: &mut UnixListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                {
                    let fd = &mut stream.as_raw_fd();
                    println!("fd: {:?}", fd);
                }
                thread::spawn(move || accept_greeter(stream));
            }
            Err(_) => {
                break;
            }
        }
    }
}*/

fn accept_greeter(mut stream: UnixStream) {
    println!("[ipc::accept_greeter]: Accepting new client!");

    loop {
        println!("[ipc::accept_greeter]: Waiting for client message..");

        match handle_message(&stream) {
            Ok(resp)  => {
                println!("[ipc::accept_greeter]: Got valid message. Writing response..");
                stream.write_all(&resp.as_bytes());
            },
            Err(_)  => {
                println!("[ipc::accept_greeter]: Got invalid message. Closing stream..");
                close_stream(&stream);
                return;
            }
        }
    }
}

fn handle_message(mut stream: &UnixStream) -> Result<ipc::IpcMessage, ipc::IpcError>{
    let service = IpcService;

    let mut buf = [0u8; ipc::HEADER_SIZE];
    stream.read_exact(&mut buf)?;

    let msg = ipc::IpcMessage::from_bytes(&buf[2..4]);
    match msg {
        Some(msg) => Ok(service.handle_message(msg).unwrap()),
        None => Err(ipc::IpcError::InvalidMessageKind)
    }
}

fn close_stream(stream: &UnixStream) {
    println!("[ipc::close_stream] Shutting down stream: {:?}", stream);
    let _ = stream.shutdown(Shutdown::Both);
}