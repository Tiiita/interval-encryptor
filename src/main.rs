use colored::Colorize;
use v2::EncryptorV2;

const DISRUPTIONS: [char; 74] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '!', '/', '(', ')', '#', ':', '<', '>', '|', '^', '~', '@',
];

mod v1;
mod v2;
fn main() {
    // V2 Example
    let text = "This is an example text";
    println!("Original: {}", text.green());

    let encrypted = EncryptorV2::encrypt(3, text);    
    println!("Encrypted: {}", encrypted.red());

    let decrypted = EncryptorV2::decrypt(encrypted);
    println!("Decrypted: {}", decrypted.unwrap().green());
}

pub trait Encryptor {
    fn encrypt(interval: u16, input: &str) -> String;
    fn decrypt(encrypted: String) -> Option<String>;
}

