use std::io::{self, Read, Write};
use std::os::unix::net::{UnixStream};
use std::str;


pub const MAGIC: i16 = ((b'1' as i16) << 8) as i16 + b'w' as i16;
pub const HEADER_SIZE: usize = 8;

#[derive(Debug, Copy, Clone)]
pub enum IpcMessageKind {
    ClientHello,
    ServerHello,
    ClientBye,
    RequestAuthentication
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

#[derive(Debug)]
pub enum IpcParseError {
    IoError(io::Error),
    HeaderTooShort,
    WrongMagic,
    InvalidMessageKind
}

impl From<io::Error> for IpcParseError {
    fn from(error: io::Error) -> Self {
        IpcParseError::IoError(error)
    }
}

#[derive(Debug)]
pub struct IpcMessage {
    kind: IpcMessageKind,
    payload: Option<Vec<u8>>
}

impl IpcMessage {
    pub fn assemble(kind: IpcMessageKind, payload: Option<&str>) -> IpcMessage {
        IpcMessage {
            kind: kind,
            payload: payload.map(|s| Vec::from(s.as_bytes()))
        }
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Result<IpcMessage, IpcParseError> {
        let mut buf = Vec::with_capacity(HEADER_SIZE);
        reader.read_exact(&mut buf)?;
        let (kind, length) = validate_header(&buf)?;

        if length > HEADER_SIZE {
            buf.clear();
            buf.reserve(length - HEADER_SIZE);
            reader.read_exact(&mut buf)?;
        }

        Ok(IpcMessage {
            kind: kind,
            payload: if length > HEADER_SIZE { Some(buf) } else { None }
        })
    }

    pub fn write_to_stream(&self, stream: &mut UnixStream) {
        let msg = self.to_bytes();
        println!("[write_to_stream]: Writing message {:?}", &msg);

        match stream.write_all(&msg) {
            Ok(())  => println!("[write_to_stream]: Successfully wrote msg"),
            Err(e)  => {
                println!("[write_to_stream]: Error while writing msg: {:?}", e);
                return;
            }
        }
        match stream.flush() {
            Ok(())  => println!("[write_to_stream]: Successfully flushed socket"),
            Err(e)  => println!("[write_to_stream]: Error while flushing socket: {:?}", e)
        }
    }

    pub fn length(&self) -> usize {
        match self.payload {
            Some(ref p) => HEADER_SIZE + p.len(),
            None    => HEADER_SIZE
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
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

fn validate_header(msg: &[u8]) -> Result<(IpcMessageKind, usize), IpcParseError> {
    if msg.len() < 8 {
        println!("[validate_message]: Wrong length: {}", msg.len());
        return Err(IpcParseError::HeaderTooShort);
    }

    if msg[0] != 0x31 || msg[1] != 0x77 {
        println!("[validate_message]: Wrong magic");
        return Err(IpcParseError::WrongMagic);
    }
    let kind = match IpcMessageKind::from_bytes(&msg[2..4]) {
        Some(k) => k,
        None    => {
            println!("[validate_message]: Invalid kind");
            return Err(IpcParseError::InvalidMessageKind);
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