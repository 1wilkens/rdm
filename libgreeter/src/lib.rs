#[macro_use]
extern crate slog;

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_uds;

extern crate rdmcommon;

use std::io;

use rdmcommon::ipc;
use rdmcommon::util;

use futures::{Future, Stream, Sink};
use slog::Logger;
use tokio_core::reactor::{Core, Handle};
use tokio_io::AsyncRead;
use tokio_uds::UnixStream;

pub struct RdmGreeter {
    log: Logger,
    core: Core,
    receiver: Box<Stream<Item = ipc::IpcMessage, Error = ipc::IpcError>>,
    sender: Box<Sink<SinkItem = ipc::IpcMessage, SinkError = ipc::IpcError>>,
}

#[derive(Debug)]
pub enum RdmGreeterError {
    Ipc(ipc::IpcError),
    Io(io::Error),
    FailedHandshake
}

impl From<io::Error> for RdmGreeterError {
    fn from(err: io::Error) -> RdmGreeterError {
        RdmGreeterError::Io(err)
    }
}

impl From<ipc::IpcError> for RdmGreeterError {
    fn from(err: ipc::IpcError) -> RdmGreeterError {
        RdmGreeterError::Ipc(err)
    }
}

impl RdmGreeter {
    pub fn new<L : Into<Option<Logger>>>(logger: L) -> Result<RdmGreeter, RdmGreeterError> {
        let log = logger.into().unwrap_or(util::plain_logger());
        let mut core = Core::new()?;
        let handle = core.handle();
        // TODO: move this into private fn
        debug!(log, "[RdmGreeter::new] Connecting server socket");
        let sock = UnixStream::connect("/home/florian/tmp/sock", &handle)?;
        let (tx, rx) = sock.framed(ipc::IpcMessageCodec).split();

        debug!(log, "[RdmGreeter::new] Sending ClientHello");
        let tx = core.run(tx.send(ipc::IpcMessage::ClientHello))?;

        debug!(log, "[RdmGreeter::new] Reading server response");
        let (resp, rx) = core.run(rx.take(1).into_future().map_err(|(err, _)| err))?;
        debug!(log, "[RdmGreeter::new] Got server response"; "response" => ?resp);

        match resp {
            Some(ipc::IpcMessage::ServerHello) => Ok(RdmGreeter {
                                                        core: core,
                                                        receiver: Box::new(rx),
                                                        sender: Box::new(tx),
                                                        log: log,
                                                  }),
            _ => Err(RdmGreeterError::FailedHandshake)
        }
    }
}

impl Drop for RdmGreeter {
    fn drop(&mut self) {
        debug!(self.log, "[RdmGreeter::drop] Dropping RdmGreeter..");
    }
}