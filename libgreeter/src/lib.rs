#![allow(clippy::redundant_field_names)]

#[macro_use]
extern crate slog;

extern crate tokio;

use futures::prelude::*;
use slog::Logger;
use tokio::net::UnixStream;
use tokio_util::codec::Framed;

use std::io;

use rdmcommon::ipc::{IpcError, IpcMessage, IpcMessageCodec};
use rdmcommon::util;

pub struct RdmGreeter {
    log: Logger,
    rx: Box<dyn Stream<Item = Result<IpcMessage, IpcError>>>,
    tx: Box<dyn Sink<IpcMessage, Error = IpcError>>,
}

#[derive(Debug)]
pub enum RdmGreeterError {
    Ipc(IpcError),
    Io(io::Error),
    FailedHandshake,
}

impl From<io::Error> for RdmGreeterError {
    fn from(err: io::Error) -> RdmGreeterError {
        RdmGreeterError::Io(err)
    }
}

impl From<IpcError> for RdmGreeterError {
    fn from(err: IpcError) -> RdmGreeterError {
        RdmGreeterError::Ipc(err)
    }
}

impl RdmGreeter {
    pub async fn new<L: Into<Option<Logger>>>(logger: L) -> Result<RdmGreeter, RdmGreeterError> {
        // XXX: Reenable this when tokio-async-await is updated
        let log = logger.into().unwrap_or_else(util::plain_logger);
        debug!(log, "[RdmGreeter::new] Connecting server socket");
        let sock = UnixStream::connect("/home/florian/tmp/sock").await?;
        let (mut tx, mut rx) = Framed::new(sock, IpcMessageCodec).split();

        debug!(log, "[RdmGreeter::new] Sending ClientHello");
        tx.send(IpcMessage::ClientHello).await?;

        debug!(log, "[RdmGreeter::new] Reading server response");
        let resp = match rx.next().await {
            Some(m) => m,
            // FIXME: extend IpcError with something like premature termination
            None => return Err(RdmGreeterError::FailedHandshake),
        };
        debug!(log, "[RdmGreeter::new] Got server response"; "response" => ?resp);

        match resp? {
            IpcMessage::ServerHello => Ok(RdmGreeter {
                rx: Box::new(rx),
                tx: Box::new(tx),
                log,
            }),
            _ => Err(RdmGreeterError::FailedHandshake),
        }
    }
}

impl Drop for RdmGreeter {
    fn drop(&mut self) {
        debug!(self.log, "[RdmGreeter::drop] Dropping RdmGreeter..");
    }
}
