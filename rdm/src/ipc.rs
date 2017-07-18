use std::io::{self, Read, Write};
use std::net::{Shutdown};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::str;
use std::sync::Arc;
use std::thread;

use rdmcommon::ipc;

use futures::future::{self, Future, BoxFuture};
use tokio_core::reactor::Handle;
use tokio_service::Service;
use tokio_uds::{UnixListener, UnixStream};

pub struct IpcManager {
    listener: UnixListener
}

pub struct IpcService;

impl IpcManager {
    pub fn new(handle: &Handle) -> Result<IpcManager, ipc::IpcError> {
        let mut l = UnixListener::bind("/home/florian/tmp/sock", handle)?;
        Ok(IpcManager {
            listener: l
        })
    }
}

impl Service for IpcService {
    // These types must match the corresponding protocol types:
    type Request = ipc::IpcMessage;
    type Response = ipc::IpcMessage;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immediate.
        future::ok(req).boxed()
    }
}

/*fn listen(listener: &mut UnixListener) {
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
            },
            Err(_)  => {
                println!("[ipc::accept_greeter]: Got invalid message. Closing stream..");
                close_stream(&stream);
                return;
            }
        }
    }
}

fn handle_message(stream: &UnixStream) -> Result<ipc::IpcMessage, ipc::IpcError>{
    /*let msg = ipc::IpcMessage::from_reader(stream)?;

    println!("[ipc::handle_message]: Got valid message: {:?}", &msg);
    let resp = ipc::IpcMessage::assemble(ipc::IpcMessageKind::ServerHello, None);
    Ok(resp)*/
    Err(ipc::IpcError::WrongMagic)
}

fn close_stream(stream: &UnixStream) {
    println!("[ipc::close_stream] Shutting down stream: {:?}", stream);
    let _ = stream.shutdown(Shutdown::Both);
}