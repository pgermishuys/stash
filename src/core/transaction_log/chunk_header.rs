use uuid::Uuid;
pub const HEADER_SIZE: usize = 128;
#[derive(Debug)]
pub struct Header {
    pub file_type: u8,
    pub version: u8,
    pub chunk_size: i32,
    pub chunk_start_number: i32,
    pub chunk_end_number: i32,
    pub is_scavenged: bool,
    pub chunk_id: Uuid,
}