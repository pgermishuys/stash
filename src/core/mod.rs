pub mod utils;
use std::convert::TryInto;
use uuid::Uuid;
use std::{
    fs::File,
    io::prelude::*
};

const HEADER_SIZE: usize = 128;
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

const FOOTER_SIZE: usize = 128;
const CHECK_SUM_SIZE: usize = 16;
#[derive(Debug)]
pub struct Footer {
    flags: u8,
    is_completed: bool,
    is_map_12_bytes: bool,
    physical_data_size: i32,
    logical_data_size: i64,
    map_size: i32,
}

pub struct Chunk {
    pub header: Header,
    pub footer: Footer,
    pub location: String
}

impl Chunk {
    pub fn open(location: String) -> Chunk {
        let mut file = File::open(location.clone()).unwrap();
        let mut buffer = [0; 128];
        file.read(&mut buffer).unwrap();

        let header = Header {
            file_type: buffer[0],
            version: buffer[1],
            chunk_size: i32::from_le_bytes(buffer[2..6].try_into().unwrap()),
            chunk_start_number: i32::from_le_bytes(buffer[7..11].try_into().unwrap()),
            chunk_end_number: i32::from_le_bytes(buffer[12..16].try_into().unwrap()),
            is_scavenged: buffer[17],
            chunk_id: utils::convert_dotnet_guid(buffer[18..34].try_into().unwrap()),
        };

        file.seek(std::io::SeekFrom::End(-(FOOTER_SIZE as i64)));
        file.read(&mut buffer).unwrap();

        let flags: u8 = buffer[0];

        let is_map_12_bytes = (flags & 2) != 0;
        let mut logical_size: i64 = 0;
        if is_map_12_bytes {
            logical_size = i64::from_le_bytes(buffer[5..13].try_into().unwrap());
        } else {
            logical_size = i64::from_le_bytes(buffer[5..9].try_into().unwrap());
        }

        let mut map_size: i32 = 0;
        if is_map_12_bytes {
            map_size = i32::from_le_bytes(buffer[14..18].try_into().unwrap());
        } else {
            map_size = i32::from_le_bytes(buffer[10..14].try_into().unwrap());
        }

        let footer = Footer {
            flags,
            is_completed: (flags & 1) != 0,
            is_map_12_bytes,
            physical_data_size: i32::from_le_bytes(buffer[1..5].try_into().unwrap()),
            logical_data_size: logical_size,
            map_size,
        };

        return Chunk {
            header,
            footer,
            location,
        }
    }
}
