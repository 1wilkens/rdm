#![feature(futures_api, async_await, await_macro)]
#![allow(clippy::redundant_field_names)]

#[macro_use]
extern crate slog;

#[macro_use]
extern crate tokio;

use slog::Logger;
use tokio::codec::Decoder;
use tokio::net::UnixStream;
use tokio::prelude::*;

use std::io;

use rdmcommon::ipc::{IpcError, IpcMessage, IpcMessageCodec};
use rdmcommon::util;

pub struct RdmGreeter {
    log: Logger,
    receiver: Box<Stream<Item = IpcMessage, Error = IpcError>>,
    sender: Box<Sink<SinkItem = IpcMessage, SinkError = IpcError>>,
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
        let log = logger.into().unwrap_or_else(util::plain_logger);
        debug!(log, "[RdmGreeter::new] Connecting server socket");
        let codec = IpcMessageCodec;
        let sock = await!(UnixStream::connect("/home/florian/tmp/sock"))?;
        let (tx, rx) = codec.framed(sock).split();

        debug!(log, "[RdmGreeter::new] Sending ClientHello");
        let tx = await!(tx.send(IpcMessage::ClientHello))?;

        debug!(log, "[RdmGreeter::new] Reading server response");
        let (resp, rx) = await!(rx.take(1).into_future().map_err(|(err, _)| err))?;
        debug!(log, "[RdmGreeter::new] Got server response"; "response" => ?resp);

        match resp {
            Some(IpcMessage::ServerHello) => Ok(RdmGreeter {
                receiver: Box::new(rx),
                sender: Box::new(tx),
                log: log,
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
