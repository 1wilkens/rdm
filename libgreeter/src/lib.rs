#[macro_use]
extern crate slog;

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_uds;

extern crate rdmcommon;

use slog::Logger;

use futures::{Future, Stream, Sink};
use tokio_core::reactor::{Core, Handle};
use tokio_io::AsyncRead;
use tokio_uds::UnixStream;

use rdmcommon::ipc;
use rdmcommon::util;

pub struct RdmGreeter {
    core: Core,
    receiver: Box<Stream<Item = ipc::IpcMessage, Error = ipc::IpcError>>,
    sender: Box<Sink<SinkItem = ipc::IpcMessage, SinkError = ipc::IpcError>>,
    log: Logger
}

impl RdmGreeter {
    pub fn new<L : Into<Option<Logger>>>(logger: L) -> Result<RdmGreeter, ipc::IpcError> {
        let log = logger.into().unwrap_or(util::plain_logger());
        let mut core = Core::new()
            .expect("[rdmgreeter] Failed to instantiate new Core");
        let handle = core.handle();
        // TODO: move this into private fn
        debug!(log, "[RdmGreeter::new] Connecting server socket");
        let sock = UnixStream::connect("/home/florian/tmp/sock", &handle)?;
        let (tx, rx) = sock.framed(ipc::IpcMessageCodec).split();
        debug!(log, "[RdmGreeter::new] Sending ClientHello");
        let tx = core.run(tx.send(ipc::IpcMessage::ClientHello))?;
        debug!(log, "[RdmGreeter::new] Reading server response");
        let (resp, rx) = core.run(rx.take(1).into_future()).expect("[rdmgreeter] Failed to receive handshake");

        debug!(log, "[RdmGreeter::new] Got server response"; "response" => ?resp);
        match resp {
            Some(ipc::IpcMessage::ServerHello) => Ok(RdmGreeter {
                                                core: core,
                                                receiver: Box::new(rx),
                                                sender: Box::new(tx),
                                                log: log,
                                            }),
            _ => Err(ipc::IpcError::UnknownMessageType)
        }
    }
}

impl Drop for RdmGreeter {
    fn drop(&mut self) {
        debug!(self.log, "[RdmGreeter::drop] Dropping RdmGreeter..");
    }
}