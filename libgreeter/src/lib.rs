#[macro_use]
extern crate slog;

extern crate rdmcommon;

use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

use rdmcommon::ipc;
use rdmcommon::util;

use slog::Logger;

#[derive(Debug)]
pub struct RdmGreeter {
    sock: UnixStream,
    log: Logger
}

impl RdmGreeter {
    pub fn new<L : Into<Option<Logger>>>(logger: L) -> Option<RdmGreeter> {
        let mut sock = match UnixStream::connect("/home/florian/tmp/sock") {
            Ok(s)   => s,
            Err(_)  => return None
        };
        let log = logger.into().unwrap_or(util::plain_logger());
        ::std::thread::sleep_ms(200);

        let mut msg = Vec::with_capacity(8);
        msg.extend(b"1wCH\0\0\0\0");
        debug!(log, "Writing ClientHello: {:?}", &msg);
        match sock.write_all(&msg) {
            Ok(())  => debug!(log, "Wrote ClientHello"),
            Err(e)  => debug!(log, "Error during ServerHello: {:?}", e)
        }

        match sock.flush() {
            Ok(())  => debug!(log, "Successful flush"),
            Err(e)  => debug!(log, "Error during flush: {:?}", e)
        }

        msg.clear();
        unsafe { msg.set_len(8) };
        debug!(log, "reading...");
        sock.read_exact(&mut msg);
        debug!(log, "reading done..");
        match ipc::IpcMessage::from_bytes(&msg[2..4]) {
            Some(ipc::IpcMessage::ServerHello) => (),
            m => {
                debug!(log, "Did not get ServerHello: {:?}", m);
                return None;
            }
        }

        Some(RdmGreeter {
            sock: sock,
            log: log,
        })
    }

    pub fn request_authentication(&mut self, user: &str, password: &str) {
        let mut msg = Vec::with_capacity(8);
        msg.extend(b"1wRA\0\0\0\0");
        debug!(self.log, "Writing RA: {:?}", &msg);
        match self.sock.write_all(&msg) {
            Ok(())  => debug!(self.log, "Wrote RA"),
            Err(e)  => debug!(self.log, "Error during ServerHello: {:?}", e)
        }

        match self.sock.flush() {
            Ok(())  => debug!(self.log, "Successful flush"),
            Err(e)  => debug!(self.log, "Error during flush: {:?}", e)
        }
    }
}

impl Drop for RdmGreeter {
    fn drop(&mut self) {
        println!("Dropping RdmGreeter..");
        self.sock.shutdown(Shutdown::Both).unwrap();
    }
}