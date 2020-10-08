use crate::message;
fn process_move(input: &[u8; 5]) -> Result<String, String> {
    if input[0] == 0x01 {
        Ok("is move".to_string())
    } else {
        Err("Tried to process a non-move_message as move".to_string())
    }
}
