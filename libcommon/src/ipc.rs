use std::error::Error;
use std::fmt;
use std::io;
use std::str;

use bytes::BytesMut;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_proto::pipeline::ServerProto;

pub const MAGIC: i16 = ((b'1' as i16) << 8) as i16 + b'w' as i16;
pub const HEADER_SIZE: usize = 8;

#[derive(Debug)]
pub enum IpcError {
    IoError(io::Error),
    HeaderTooShort,
    WrongMagic,
    InvalidMessageKind
}

#[derive(Debug, Copy, Clone)]
pub enum IpcMessageKind {
    ClientHello,
    ServerHello,
    ClientBye,
    RequestAuthentication
}

#[derive(Debug)]
pub struct IpcMessage {
    kind: IpcMessageKind,
    payload: Option<Vec<u8>>
}

#[derive(Debug)]
pub struct IpcMessageCodec {
    to_read: usize,
    kind: Option<IpcMessageKind>
}

#[derive(Debug)]
pub struct IpcProto;

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        write!(f, "test")
    }
}

impl Error for IpcError {
    fn description(&self) -> &str {
        "test"
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &IpcError::IoError(ref err)  => Some(err),
            _   => None
        }
    }
}

impl From<io::Error> for IpcError {
    fn from(error: io::Error) -> Self {
        IpcError::IoError(error)
    }
}

impl IpcMessageKind {
    pub fn from_bytes(bytes: &[u8]) -> Option<IpcMessageKind> {
        match str::from_utf8(bytes) {
            Ok(s)   => match s {
                "CH" => Some(IpcMessageKind::ClientHello),
                "SH" => Some(IpcMessageKind::ServerHello),
                "CB" => Some(IpcMessageKind::ClientBye),
                "RA" => Some(IpcMessageKind::RequestAuthentication),
                _    => None
            },
            Err(_)  => None
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match *self {
            IpcMessageKind::ClientHello => b"CH",
            IpcMessageKind::ServerHello => b"SH",
            IpcMessageKind::ClientBye => b"CB",
            IpcMessageKind::RequestAuthentication => b"RA"
        }
    }
}

impl IpcMessage {
    pub fn assemble(kind: IpcMessageKind, payload: Option<&str>) -> IpcMessage {
        IpcMessage {
            kind: kind,
            payload: payload.map(|s| Vec::from(s.as_bytes()))
        }
    }

    pub fn length(&self) -> usize {
        match self.payload {
            Some(ref p) => p.len(),
            None => 0
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length = self.length();
        let mut buf = Vec::with_capacity(length as usize);

        write_i16(&mut buf, MAGIC);
        buf.extend(self.kind.as_bytes());
        write_i32(&mut buf, length as i32);

        if let Some(ref p) = self.payload {
            buf.extend(p)
        }
        buf
    }
}

impl IpcMessageCodec {
    pub fn new() -> IpcMessageCodec {
        IpcMessageCodec { to_read: HEADER_SIZE, kind: None }
    }
}

impl Decoder for IpcMessageCodec {
    type Item = IpcMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<IpcMessage>> {
        if buf.len() >= self.to_read {
            // We have a valid chunk to read
            let chunk = buf.split_to(self.to_read);

            if let Some(k) = self.kind {
                // We previously parsed a message kind and now have a payload
                self.kind = None;
                self.to_read = HEADER_SIZE;

                Ok(Some(IpcMessage {kind: k, payload: Some(chunk.to_vec())}))
            } else {
                // We have a header to parse
                match validate_header(&chunk) {
                    // Zero length means there is no payload
                    Ok((kind, size)) if size == 0 => Ok(Some(IpcMessage {kind: kind, payload: None})),
                    // Save the kind and rerun for payload
                    //TODO: Optimize when we already have enough data?
                    //TODO: Should also probably add some sanity checks
                    Ok((kind, size)) => {
                        self.kind = Some(kind);
                        self.to_read = size;
                        Ok(None)
                    }
                    Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, Box::new(err)))
                }
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for IpcMessageCodec {
    type Item = IpcMessage;
    type Error = io::Error;

    fn encode(&mut self, msg: IpcMessage, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        Ok(())
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for IpcProto {
    /// For this protocol style, `Request` matches the `Item` type of the codec's `Encoder`
    type Request = IpcMessage;

    /// For this protocol style, `Response` matches the `Item` type of the codec's `Decoder`
    type Response = IpcMessage;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, IpcMessageCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(IpcMessageCodec::new()))
    }
}

fn validate_header(msg: &[u8]) -> Result<(IpcMessageKind, usize), IpcError> {
    if msg.len() < 8 {
        println!("[validate_message]: Wrong length: {}", msg.len());
        return Err(IpcError::HeaderTooShort);
    }

    if msg[0] != 0x31 || msg[1] != 0x77 {
        println!("[validate_message]: Wrong magic");
        return Err(IpcError::WrongMagic);
    }
    let kind = match IpcMessageKind::from_bytes(&msg[2..4]) {
        Some(k) => k,
        None => {
            println!("[validate_message]: Invalid kind");
            return Err(IpcError::InvalidMessageKind);
        }
    };

    Ok((kind, read_i32(&msg[5..8]) as usize))
}

fn write_i16(msg: &mut Vec<u8>, val: i16) {
    msg.push(((val >> 8) & 0xFF) as u8);
    msg.push((val & 0xFF) as u8);
}

fn write_i32(msg: &mut Vec<u8>, val: i32) {
    msg.push((val >> 24) as u8);
    msg.push(((val >> 16) & 0xFF) as u8);
    msg.push(((val >> 8) & 0xFF) as u8);
    msg.push((val & 0xFF) as u8);
}

fn read_i32(msg: &[u8]) -> i32 {
    if msg.len() < 4 {
        panic!("[read_i32] buffer too small..");
    }
    (msg[0] as i32) << 24 + (msg[1] as i32) << 16 + (msg[2] as i32) << 8 + (msg[3] as i32)
}