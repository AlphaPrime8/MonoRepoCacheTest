use rmp_serde::Serializer;
use serde::Serialize;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  ClientRequests {
    pub data: String,
}

impl ClientRequests {
    pub fn new(s: String) -> Self {
        Self{
            data: s,
        }
    }
}

pub fn serialize_client_request(client_request: ClientRequests) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    client_request.serialize(&mut Serializer::new(&mut buf)).unwrap();
    buf
}

pub fn deserialize_client_request(buf: Vec<u8>) -> ClientRequests {
    rmp_serde::decode::from_slice::<ClientRequests>(buf.as_slice()).unwrap()
}
