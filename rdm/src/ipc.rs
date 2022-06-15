use std::io::{self, Read, Write};
use std::net::Shutdown;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::sync::Arc;
use std::thread;

use futures::prelude::*;
use slog::Logger;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio_util::codec::Framed;

use rdmcommon::{ipc, util};

pub struct IpcService(Logger);

pub struct IpcManager {
    log: Logger,
    listener: UnixListener,
}

impl IpcManager {
    pub fn new<L: Into<Option<Logger>>>(logger: L) -> Result<Self, ipc::IpcError> {
        let log = logger.into().unwrap_or_else(util::plain_logger);

        debug!(log, "[IpcManager::new] Binding IPC socket");
        let listener = UnixListener::bind("/home/florian/tmp/sock")?;

        Ok(IpcManager { log, listener })
    }

    pub async fn run(&mut self) -> Result<(), ipc::IpcError> {
        match self.listener.accept().await {
            Ok((stream, _addr)) => self.handle(stream).await,
            Err(e) => {
                error!(&self.log, "[IpcManager::run] Error accepting unix listener");
                Err(ipc::IpcError::IO(e))
            }
        }
    }

    async fn handle(&mut self, stream: UnixStream) -> Result<(), ipc::IpcError> {
        debug!(&self.log, "[IpcManager::handle] New client");
        let (mut writer, mut reader) = Framed::new(stream, ipc::IpcMessageCodec).split();

        let req = match reader.next().await {
            Some(m) => m,
            // FIXME: extend IpcError with something like premature termination
            None => return Ok(()),
        };
        let resp = match req? {
            ipc::IpcMessage::ClientHello => ipc::IpcMessage::ServerHello,
            _ => return Err(ipc::IpcError::UnknownMessageType),
        };
        writer.send(resp).await
    }
}

impl Drop for IpcManager {
    fn drop(&mut self) {
        debug!(&self.log, "Dropping IpcManager");
    }
}
