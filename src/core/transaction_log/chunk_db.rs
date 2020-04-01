use super::chunk_header;
use super::chunk_footer;
use crate::core::utils::dotnet_conversion;

use std::convert::TryInto;
use std::{fs::File, io::prelude::*};

const CHECK_SUM_SIZE: usize = 16;

pub struct ChunkDb {
    pub header: chunk_header::Header,
    pub footer: chunk_footer::Footer,
    pub location: String,
}

impl ChunkDb {
    pub fn open(location: String) -> ChunkDb {
        let mut file = File::open(location.clone()).unwrap();

        ChunkDb {
            header: ChunkDb::read_header(&mut file),
            footer: ChunkDb::read_footer(&mut file),
            location,
        }
    }

    fn read_header(file: &mut File) -> chunk_header::Header {
        let mut buffer = [0; chunk_header::HEADER_SIZE];
        let _ = file.read(&mut buffer).unwrap();

        return chunk_header::Header {
            file_type: buffer[0],
            version: buffer[1],
            chunk_size: i32::from_le_bytes(buffer[2..6].try_into().unwrap()),
            chunk_start_number: i32::from_le_bytes(buffer[7..11].try_into().unwrap()),
            chunk_end_number: i32::from_le_bytes(buffer[12..16].try_into().unwrap()),
            is_scavenged: buffer[17] == 1u8,
            chunk_id: dotnet_conversion::convert_dotnet_guid(buffer[18..34].try_into().unwrap()),
        };
    }

    fn read_footer(file: &mut File) -> chunk_footer::Footer {
        let mut buffer = [0; chunk_footer::FOOTER_SIZE];
        let _ = file.seek(std::io::SeekFrom::End(-(chunk_footer::FOOTER_SIZE as i64)));
        let _ = file.read(&mut buffer).unwrap();

        let flags: u8 = buffer[0];

        let is_map_12_bytes = (flags & 2) != 0;

        let logical_data_size = if is_map_12_bytes {
            i64::from_le_bytes(buffer[5..13].try_into().unwrap())
        } else {
            i64::from_le_bytes(buffer[5..9].try_into().unwrap())
        };

        let map_size = if is_map_12_bytes {
            i32::from_le_bytes(buffer[14..18].try_into().unwrap())
        } else {
            i32::from_le_bytes(buffer[10..14].try_into().unwrap())
        };

        let physical_data_size = i32::from_le_bytes(buffer[1..5].try_into().unwrap());
        let _ = file.seek(std::io::SeekFrom::End(-(CHECK_SUM_SIZE as i64)));
        let _ = file.read(&mut buffer).unwrap();
        return chunk_footer::Footer {
            flags,
            is_completed: (flags & 1) != 0,
            is_map_12_bytes,
            physical_data_size,
            logical_data_size,
            map_size,
            hash: buffer[0..CHECK_SUM_SIZE].try_into().unwrap(),
        };
    }
}
