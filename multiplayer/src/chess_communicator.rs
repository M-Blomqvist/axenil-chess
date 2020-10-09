fn process_move(input: &[u8; 5]) -> Result<String, String> {
    if input[0] == 0x01 {
        match input[1] {
            0x00 => {
                let pos = bits_to_coord(&input[2]);
                let new_pos = bits_to_coord(&input[3]);
                Ok("reg".to_string())
            }
            0x01 => {
                println!("s");
                Ok("reg".to_string())
            }
            0x02 => {
                println!("s");
                Ok("reg".to_string())
            }
            0x03 => Ok("0-0".to_string()),
            0x04 => Ok("0-0-0".to_string()),
            _ => Err("Error processing move".to_string()),
        }
    } else {
        Err("Tried to process a non-move_message as move".to_string())
    }
}

fn bits_to_coord(byte: &u8) -> (u8, u8) {
    (byte & 0b0000_0111, byte & 0b0011_100)
}
