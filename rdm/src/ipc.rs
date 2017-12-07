use std::io::{self, Read, Write};
use std::net::{Shutdown};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::sync::Arc;
use std::thread;

use rdmcommon::ipc;

use futures::{future, Future, Stream, Sink};
use tokio_core::reactor::{Core, Handle};
use tokio_io::AsyncRead;
use tokio_service::{Service, NewService};
use tokio_uds::UnixListener;

pub struct IpcManager {
    listener: UnixListener
}

pub struct IpcService;

/*impl IpcManager {
    pub fn new(handle: &Handle) -> Result<IpcManager, ipc::IpcError> {
        let listener = UnixListener::bind("/home/florian/tmp/sock", handle)?;
        Ok(IpcManager {listener: listener})
    }

    pub fn run<S>(&mut self, service: S)
        where S: NewService<Request = ipc::IpcMessage,
                        Response = ipc::IpcMessage,
                        Error = ipc::IpcError> + 'static
    {
        let connections = self.listener.incoming();

        let server = connections.for_each(move |(socket, _peer_addr)| {
            let (writer, reader) = socket.framed(ipc::IpcMessageCodec).split();
            let service = s.new_service()?;

            let responses = reader.and_then(move |req| service.call(req));
            let server = writer.send_all(responses)
                .then(|_| Ok(()));
            self.handle.spawn(server);

            Ok(())
        });
    }
}*/

impl Service for IpcService {
    type Request = ipc::IpcMessage;
    type Response = ipc::IpcMessage;
    type Error = ipc::IpcError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, msg: Self::Request) -> Self::Future {
        match msg {
            ipc::IpcMessage::ClientHello => Box::new(future::ok(ipc::IpcMessage::ServerHello)),
            _   => Box::new(future::err(ipc::IpcError::UnknownMessageType))
        }
    }
}

pub fn serve<S>(s: S) -> Result<(), ipc::IpcError>
    where S: NewService<Request = ipc::IpcMessage,
                        Response = ipc::IpcMessage,
                        Error = ipc::IpcError> + 'static
{
    let mut core = Core::new()?;
    let handle = core.handle();

    let listener = UnixListener::bind("/home/florian/tmp/sock", &handle)?;

    let connections = listener.incoming();
    let srv = connections.for_each(move |(socket, _peer_addr)| {
        println!("Serving new client..");
        let (writer, reader) = socket.framed(ipc::IpcMessageCodec).split();
        let service = s.new_service()?;

        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses)
            .then(|_| Ok(()));
        handle.spawn(server);

        Ok(())
    });

    core.run(srv).map_err(|err| ipc::IpcError::from(err))
}