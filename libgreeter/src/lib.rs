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
    pub fn new<L : Into<Option<Logger>>>(logger: L) -> Option<RdmGreeter> {
        let log = logger.into().unwrap_or(util::plain_logger());
        let mut core = Core::new()
            .expect("[rdmgreeter] Failed to instantiate new Core");
        let handle = core.handle();
        // TODO: move this into private fn
        let sock = UnixStream::connect("/home/florian/tmp/sock", &handle)
            .expect("[rdmgreeter] Failed to connect to socket");
        let (tx, rx) = sock.framed(ipc::IpcMessageCodec).split();
        let handshake = tx.send(ipc::IpcMessage::ClientHello).and_then(move |tx| {
            Ok(rx.take(1).into_future().map(|(res, rx)| (res, (tx, rx))))
        });

        let hs = core.run(handshake);
        let res = core.run(hs.unwrap());
        if res.is_ok() {
            let (res, (tx, rx)) = res.unwrap();
            match res {
                Some(ipc::IpcMessage::ServerHello) => Some(RdmGreeter {
                                                    core: core,
                                                    receiver: Box::new(rx),
                                                    sender: Box::new(tx),
                                                    log: log,
                                                }),
                _ => None
            }
        }
        else {
            None
        }
    }
}

impl Drop for RdmGreeter {
    fn drop(&mut self) {
        println!("Dropping RdmGreeter..");
    }
}