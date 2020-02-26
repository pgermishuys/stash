pub mod utils;
use std::convert::TryInto;
use uuid::Uuid;
use std::{
    fs::File,
    io::prelude::*
};

#[derive(Debug)]
pub struct Header {
    file_type: u8,
    version: u8,
    chunk_size: i32,
    chunk_start_number: i32,
    chunk_end_number: i32,
    is_scavenged: u8,
    chunk_id: Uuid
}

pub struct Chunk {
    pub header: Header,
    pub location: String
}

impl Chunk {
    pub fn open(location: String) -> Chunk {
        let mut file = File::open(location.clone()).unwrap();
        let mut header_buffer = [0; 128];
        file.read(&mut header_buffer).unwrap();
        let x: [u8; 16] = header_buffer[18..34].try_into().unwrap();
        println!("{:?}", x);
        let header = Header {
            file_type: header_buffer[0],
            version: header_buffer[1],
            chunk_size: i32::from_le_bytes(header_buffer[2..6].try_into().unwrap()),
            chunk_start_number: i32::from_le_bytes(header_buffer[7..11].try_into().unwrap()),
            chunk_end_number: i32::from_le_bytes(header_buffer[12..16].try_into().unwrap()),
            is_scavenged: header_buffer[17],
            chunk_id: utils::convert_dotnet_guid(header_buffer[18..34].try_into().unwrap()),
        };
        return Chunk {
            header,
            location: location.clone()
        }
    }
}
