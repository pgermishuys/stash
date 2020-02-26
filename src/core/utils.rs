pub fn convert_dotnet_guid(buffer: [u8; 128]) -> [u8; 16]{
    let order = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut uuid: [u8; 16] = [0; 16];
    for n in 0..16 {
        uuid[n] = buffer[18 + order[n]];
    }
    return uuid;
}
