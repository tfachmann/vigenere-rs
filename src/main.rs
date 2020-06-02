/// Transforms character to numeric value.
/// !Very hacky should not be used in production
fn get_numeric(c: char) -> Option<u32> {
    Some(c.to_digit(36)? - 10)
}

/// Transforms numeric value to character.
/// !Very hacky should not be used in production
fn get_character(n: u32) -> Option<char> {
    Some(std::char::from_digit(n + 10, 36)?.to_ascii_uppercase())
}

/// Decodes message with key regarding vingenere algorithm.
fn decode(msg: &str, key: &str) -> Option<String> {
    let mut new_msg = String::from("");
    let key_it = key.chars().cycle();                       // cyclic iterator of key
    for (char_msg, char_key) in msg.chars().zip(key_it) {   // loop over each character of msg
        let num_msg = get_numeric(char_msg)?;               // transform to numeric
        let num_key = get_numeric(char_key)?;               // transform to numeric
        let num_new = (26 + num_msg - num_key) % 26;        // decrypt char
        new_msg.push(get_character(num_new)?);
    }
    Some(new_msg)
}

/// Encodes message with key regarding vingenere algorithm.
fn encode(msg: &str, key: &str) -> Option<String> {
    let mut new_msg = String::from("");
    let key_it = key.chars().cycle();                       // cyclic iterator of key
    for (char_msg, char_key) in msg.chars().zip(key_it) {   // loop over each character of msg
        let num_msg = get_numeric(char_msg)?;               // transform to numeric
        let num_key = get_numeric(char_key)?;               // transform to numeric
        let num_new = (num_msg + num_key) % 26;             // encrypt char
        new_msg.push(get_character(num_new)?);
    }
    Some(new_msg)
}

fn main() {
    let message = "MEINPASSWORTISTGEHEIM";
    let key = "CODE";

    let encoded_msg = encode(message, key)
        .expect("Encoding has failed. Only letters from A-Z are allowed");
    let decoded_msg = decode(&encoded_msg, key)
        .expect("Decoding has failed.");

    println!("{} - {}", encoded_msg, decoded_msg);

    assert_eq!(message, &decoded_msg);
}
