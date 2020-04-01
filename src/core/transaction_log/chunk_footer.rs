pub const FOOTER_SIZE: usize = 128;
#[derive(Debug)]
pub struct Footer {
    pub flags: u8,
    pub is_completed: bool,
    pub is_map_12_bytes: bool,
    pub physical_data_size: i32,
    pub logical_data_size: i64,
    pub map_size: i32,
    pub hash: [u8; 16],
}