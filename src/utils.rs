pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}