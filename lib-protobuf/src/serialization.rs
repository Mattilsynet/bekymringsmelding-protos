use bytes::Bytes;
use prost::Message;
pub use prost::{DecodeError, EncodeError};

pub fn serialize<T: Message>(message: T) -> Vec<u8> {
    let mut buf = Vec::with_capacity(message.encoded_len());
    message.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize<T: Message + Default>(buf: &[u8]) -> Result<T, prost::DecodeError> {
    let bytes = Bytes::copy_from_slice(buf);
    T::decode(bytes)
}
