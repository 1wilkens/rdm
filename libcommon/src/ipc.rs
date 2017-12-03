use bytes::{BufMut, BytesMut, BigEndian};
use tokio_io::codec::{Encoder, Decoder};

use super::error::IpcError;

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
pub struct IpcMessageCodec {
    to_read: usize
}

type DecodeResult = Result<Option<IpcMessage>, IpcError>;

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

impl IpcMessageCodec {
    pub fn new() -> IpcMessageCodec {
        IpcMessageCodec { to_read: HEADER_SIZE }
    }
}

impl Encoder for IpcMessageCodec {
    type Item = IpcMessage;
    type Error = IpcError;

    fn encode(&mut self, msg: IpcMessage, buf: &mut BytesMut) -> Result<(), IpcError> {
        let len = match msg {
            IpcMessage::ClientHello => 0,
            IpcMessage::ServerHello => 0,
            IpcMessage::ClientBye => 0,
            IpcMessage::RequestAuthentication(ref user, ref secret) => user.len() + secret.len(),
            _   => return Err(IpcError::UnknownMessageType)
        };

        buf.reserve(HEADER_SIZE + len);
        buf.put_i16::<BigEndian>(MAGIC);
        buf.extend(msg.message_type());
        buf.put_u32::<BigEndian>(len as u32);

        //TODO: write payload

        Ok(())
    }
}

impl Decoder for IpcMessageCodec {
    type Item = IpcMessage;
    type Error = IpcError;

    fn decode(&mut self, buf: &mut BytesMut) -> DecodeResult {
        if buf.len() >= HEADER_SIZE {
            let msg = buf.split_to(8);

            if msg[0] != 0x31 || msg[1] != 0x77 {
                println!("[validate_message]: Wrong magic");
                return Err(IpcError::WrongMagic);
            }

            Ok(Some(IpcMessage::ClientHello))
        }
        else {
            Ok(None)
        }
    }
}

fn validate_header(msg: &[u8]) -> Result<(IpcMessage, u32), IpcError> {
    if msg.len() < 8 {
        println!("[validate_message]: Wrong length: {}", msg.len());
        return Err(IpcError::HeaderTooShort);
    }

    if msg[0] != 0x31 || msg[1] != 0x77 {
        println!("[validate_message]: Wrong magic");
        return Err(IpcError::WrongMagic);
    }
    /*let kind = match IpcMessage::from_bytes(&msg[2..4]) {
        Some(k) => k,
        None => {
            println!("[validate_message]: Invalid kind");
            return Err(IpcError::UnknownMessageType);
        }
    };

    Ok((kind, BigEndian::read_u32(&msg[5..8])))*/
    Err(IpcError::UnknownMessageType)
}
