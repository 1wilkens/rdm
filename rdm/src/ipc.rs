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
        /*for socket in self.listener.incoming() {
            thread::spawn(move || accept_greeter(socket.unwrap()));
        }*/
    }
}

impl IpcService {
    fn handle_message(&self, msg: ipc::IpcMessage) -> io::Result<ipc::IpcMessage> {
        match msg {
            ipc::IpcMessage::ClientHello => Ok(ipc::IpcMessage::ServerHello),
            _   => Err(io::Error::new(io::ErrorKind::InvalidData, ipc::IpcError::UnknownMessageType))
        }
    }
}