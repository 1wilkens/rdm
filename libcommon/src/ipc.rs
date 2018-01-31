use bytes::{BufMut, BytesMut, BigEndian};
use tokio_io::codec::{Encoder, Decoder};

pub use super::error::IpcError;

pub const MAGIC: i16 = ((b'1' as i16) << 8) as i16 + b'w' as i16;
pub const HEADER_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub enum IpcMessage {
    ClientHello,
    ServerHello,
    ClientBye,
    RequestAuthentication(String, String)
}

#[derive(Debug)]
pub struct IpcMessageCodec;

type DecodeResult = Result<Option<(IpcMessage, usize)>, IpcError>;

impl IpcMessage {
    pub fn message_type(&self) -> &[u8; 2] {
        match *self {
            IpcMessage::ClientHello => b"CH",
            IpcMessage::ServerHello => b"SH",
            IpcMessage::ClientBye => b"CB",
            IpcMessage::RequestAuthentication(ref _a, ref _b) => b"RA"
        }
    }
}

impl Encoder for IpcMessageCodec {
    type Item = IpcMessage;
    type Error = IpcError;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let len = match msg {
            IpcMessage::ClientHello => 0,
            IpcMessage::ServerHello => 0,
            IpcMessage::ClientBye => 0,
            IpcMessage::RequestAuthentication(ref user, ref secret) => user.len() + secret.len() + 8,
            _   => return Err(IpcError::UnknownMessageType)
        };

        buf.reserve(HEADER_SIZE + len);
        buf.put_i16::<BigEndian>(MAGIC);
        buf.extend(msg.message_type());
        buf.put_u32::<BigEndian>(len as u32);

        match msg {
            IpcMessage::RequestAuthentication(ref user, ref secret) => {
                buf.put_u32::<BigEndian>(user.len() as u32);
                buf.extend(user.as_bytes());
                buf.put_u32::<BigEndian>(secret.len() as u32);
                buf.extend(secret.as_bytes());
            },
            _   => ()
        };

        Ok(())
    }
}

impl Decoder for IpcMessageCodec {
    type Item = IpcMessage;
    type Error = IpcError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match decode(buf, 0) {
            Ok(None) => Ok(None),
            Ok(Some((item ,pos))) => {
                buf.split_to(pos);
                Ok(Some(item))
            }
            Err(e) => Err(e)
        }
    }
}

fn decode(buf: &mut BytesMut, idx: usize) -> DecodeResult {
    let length = buf.len();
    if length <= idx {
        return Ok(None);
    }

    if length < HEADER_SIZE {
        return Err(IpcError::HeaderTooShort);
    }

    let magic = &buf[idx..idx + 2];
    if magic[0] != b'1' || magic[1] != b'w' {
        return Err(IpcError::InvalidMagic);
    }

    let message_type = &buf[idx + 2..idx + 4];
    match message_type {
        b"CH" => Ok(Some((IpcMessage::ClientHello, idx + HEADER_SIZE))),
        b"SH" => Ok(Some((IpcMessage::ServerHello, idx + HEADER_SIZE))),
        b"CB" => Ok(Some((IpcMessage::ClientBye, idx + HEADER_SIZE))),
        _ => Err(IpcError::UnknownMessageType)
    }
}