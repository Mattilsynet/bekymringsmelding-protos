use prost::Message;
use std::io::Cursor;

pub fn serialize<T: Message>(message: T) -> Vec<u8> {
    let mut buf = Vec::with_capacity(message.encoded_len());
    message.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize<T: Message + Default>(buf: &[u8]) -> Result<T, prost::DecodeError> {
    T::decode(&mut Cursor::new(buf))
}
