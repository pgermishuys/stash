use uuid::Uuid;
pub fn convert_dotnet_guid(buffer: [u8; 16]) -> Uuid {
    let order = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut uuid: [u8; 16] = [0; 16];
    for n in 0..16 {
        uuid[n] = buffer[order[n]];
    }
    Uuid::from_bytes(uuid)
}

#[cfg(test)]
mod tests {
    use super::convert_dotnet_guid;
    use std::convert::TryInto;
    use uuid::Uuid;

    #[test]
    fn can_convert_dotnet_guid() {
        let expected = Uuid::parse_str("050d9d21-717a-40f5-9037-d455c6af0ebc").unwrap();
        let dotnet_uuid: [u8; 16] = [
            33, 157, 13, 5, 122, 113, 245, 64, 144, 55, 212, 85, 198, 175, 14, 188,
        ];
        let uuid = convert_dotnet_guid(dotnet_uuid.try_into().unwrap());
        assert_eq!(expected, uuid);
    }
}

