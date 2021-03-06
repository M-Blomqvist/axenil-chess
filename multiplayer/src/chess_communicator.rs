use rust_chess::board::{position_to_string, string_to_position};
pub fn process_move(input: &[u8; 5]) -> Result<String, String> {
    if input[0] == 0x01 {
        println!("{:?}", input);
        match input[1] {
            0x00 => {
                let pos = bits_to_coord(&input[2]);
                let new_pos = bits_to_coord(&input[3]);
                let pos = position_to_string(pos.1, pos.0);
                let new_pos = position_to_string(new_pos.1, new_pos.0);
                let mov = pos + " " + new_pos.as_str();
                println!("{}", mov);
                Ok(mov)
            }
            0x01 => {
                let pos = bits_to_coord(&input[2]);
                let new_pos = bits_to_coord(&input[3]);
                let pos = position_to_string(pos.1, pos.0);
                let new_pos = position_to_string(new_pos.1, new_pos.0);
                let mov = pos + " " + new_pos.as_str();
                Ok(mov)
            }
            0x02 => {
                let pos = bits_to_coord(&input[2]);
                let new_pos = bits_to_coord(&input[3]);
                let pos = position_to_string(pos.1, pos.0);
                let new_pos = position_to_string(new_pos.1, new_pos.0);
                let mov = pos + " " + new_pos.as_str();
                Ok(mov)
            }
            0x03 => Ok("0-0".to_string()),
            0x04 => Ok("0-0-0".to_string()),
            _ => Err("Error processing move".to_string()),
        }
    } else {
        Err("Tried to process a non-move_message as move".to_string())
    }
}

pub fn move_to_bytes(input: String) -> [u8; 5] {
    if input == "0-0-0" {
        [0x01, 0x4, 255, 255, 255]
    } else if input == "0-0" {
        [0x01, 0x3, 255, 255, 255]
    } else {
        let (input, _) = input.split_at(5);
        let mut input = input.split_whitespace();
        let (pos_x, pos_y) = string_to_position(input.next().unwrap());
        let (new_x, new_y) = string_to_position(input.next().unwrap());
        let pos_bin = pos_x as u8 + (pos_y << 3) as u8;
        let new_bin = new_x as u8 + (new_y << 3) as u8;
        [0x01, 0x00, pos_bin, new_bin, 255]
    }
}

fn bits_to_coord(byte: &u8) -> (u8, u8) {
    let (x, y) = (byte & 0b0000_0111, (byte & 0b0_111000) >> 3);
    println!("{},{}", x, y);
    (x, 7 - y)
}
