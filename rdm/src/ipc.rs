use std::io::{self, Read, Write};
use std::net::Shutdown;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::sync::Arc;
use std::thread;

use rdmcommon::{ipc, util};

use futures::{future, Future, Sink, Stream};
use slog::Logger;
use tokio_core::reactor::{Core, Handle};
use tokio_io::AsyncRead;
use tokio_service::{NewService, Service};
use tokio_uds::UnixListener;

pub struct IpcManager {
    log: Logger,
    listener: UnixListener,
    handle: Handle,
}

pub struct IpcService(Logger);

impl IpcManager {
    pub fn new<L: Into<Option<Logger>>>(
        logger: L,
        handle: &Handle,
    ) -> Result<IpcManager, ipc::IpcError> {
        let log = logger.into().unwrap_or_else(util::plain_logger);
        let handle = handle.clone();

        debug!(log, "[IpcManager::new] Binding server socket");
        let listener = UnixListener::bind("/home/florian/tmp/sock", &handle)?;
        Ok(IpcManager {
            log: log,
            listener: listener,
            handle: handle,
        })
    }

    /*pub fn run<S>(&mut self, new_service: S, core: Core)
        where S: NewService<Request = ipc::IpcMessage,
                        Response = ipc::IpcMessage,
                        Error = ipc::IpcError> + 'static
    {
        let handle = &self.handle;
        let listener = &mut self.listener;

        let f = listener.incoming().for_each(|(socket, _peer_addr)| {
            let (writer, reader) = socket.framed(ipc::IpcMessageCodec).split();
            let service = new_service.new_service().expect("[IpcManager::run] Failed to call new_service()");

            let responses = reader.and_then(move |req| service.call(req));
            let handler = writer.send_all(responses)
                .then(|_| Ok(()));

            handle.spawn(handler);

            Ok(())
        });
    }*/
}

impl IpcService {
    pub fn new<L: Into<Option<Logger>>>(logger: L) -> IpcService {
        let log = logger.into().unwrap_or_else(util::plain_logger);
        IpcService(log)
    }
}

impl Service for IpcService {
    type Request = ipc::IpcMessage;
    type Response = ipc::IpcMessage;
    type Error = ipc::IpcError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, msg: Self::Request) -> Self::Future {
        debug!(self.0, "[IpcService::call] Received message"; "msg" => ?msg);
        match msg {
            ipc::IpcMessage::ClientHello => Box::new(future::ok(ipc::IpcMessage::ServerHello)),
            _ => Box::new(future::err(ipc::IpcError::UnknownMessageType)),
        }
    }
}

pub fn serve<S>(s: S) -> Result<(), ipc::IpcError>
where
    S: NewService<Request = ipc::IpcMessage, Response = ipc::IpcMessage, Error = ipc::IpcError>
        + 'static,
{
    let mut core = Core::new()?;
    let handle = core.handle();

    let listener = UnixListener::bind("/home/florian/tmp/sock", &handle)
        .expect("[serve] Failed to bind socket");

    let connections = listener.incoming();
    let srv = connections.for_each(move |(socket, _peer_addr)| {
        println!("[serve] Serving new client..");
        let (writer, reader) = socket.framed(ipc::IpcMessageCodec).split();
        let service = s.new_service()?;

        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses).then(|_| Ok(()));
        handle.spawn(server);

        Ok(())
    });

    core.run(srv).map_err(ipc::IpcError::from)
}
