#[macro_use]
extern crate slog;

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_uds;

extern crate rdmcommon;

use std::io::{Read, Write};

use slog::Logger;

use futures::{Stream, Sink};
use tokio_core::reactor::Handle;
use tokio_io::AsyncRead;
use tokio_uds::UnixStream;

use rdmcommon::ipc;
use rdmcommon::util;

pub struct RdmGreeter {
    receiver: Box<Stream<Item = ipc::IpcMessage, Error = ipc::IpcError>>,
    sender: Box<Sink<SinkItem = ipc::IpcMessage, SinkError = ipc::IpcError>>,
    log: Logger
}

impl RdmGreeter {
    pub fn new<L : Into<Option<Logger>>>(handle: &Handle, logger: L) -> Option<RdmGreeter> {
        let log = logger.into().unwrap_or(util::plain_logger());
        let greeter = UnixStream::connect("/home/florian/tmp/sock", handle).map(move |sock| {
            let (send, recv) = sock.framed(ipc::IpcMessageCodec).split();
            /*let resp = send.send(IpcMessage::ClientHello).map(move |r| {
                // perform handshake here?
            });*/
            RdmGreeter {
                receiver: Box::new(recv),
                sender: Box::new(send),
                log: log,
            }
        });

        //debug!(log, "Writing ClientHello");
        /*match sock.write_all(&msg) {
            Ok(())  => debug!(log, "Wrote ClientHello"),
            Err(e)  => debug!(log, "Error during ServerHello: {:?}", e)
        }

        match sock.flush() {
            Ok(())  => debug!(log, "Successful flush"),
            Err(e)  => debug!(log, "Error during flush: {:?}", e)
        }*/

        Some(greeter.ok().unwrap())
    }

    /*pub fn request_authentication(&mut self, user: &str, password: &str) {
        let mut msg = Vec::with_capacity(8);
        msg.extend(b"1wRA\0\0\0\0");
        debug!(self.log, "Writing RA: {:?}", &msg);
        /*match self.sock.write_all(&msg) {
            Ok(())  => debug!(self.log, "Wrote RA"),
            Err(e)  => debug!(self.log, "Error during ServerHello: {:?}", e)
        }

        match self.sock.flush() {
            Ok(())  => debug!(self.log, "Successful flush"),
            Err(e)  => debug!(self.log, "Error during flush: {:?}", e)
        }*/
    }*/
}

impl Drop for RdmGreeter {
    fn drop(&mut self) {
        println!("Dropping RdmGreeter..");
        //self.sock.shutdown(Shutdown::Both).unwrap();
    }
}