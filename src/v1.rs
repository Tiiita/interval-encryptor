use rand::random_range;

use crate::{Encryptor, DISRUPTIONS};

pub struct EncryptorV1;
impl Encryptor for EncryptorV1 {
    fn encrypt(interval: u16, input: &str) -> String {
        let input = input.replace(" ", "");
    
        let mut encrypted = interval.to_string();
        for (i, ele) in input.chars().enumerate() {
            encrypted.push(ele);
    
            if i + 1 != input.len() {
                for _ in 1..interval {
                    let random_disruption = DISRUPTIONS[random_range(0..DISRUPTIONS.len())];
                    encrypted.push(random_disruption);
                }
            }
        }
    
        encrypted
    }
    
    fn decrypt(encrypted: String) -> Option<String> {
        if encrypted.len() < 2 {
            return None;
        }
    
        let interval_str = encrypted.get(0..1).unwrap();
        let interval = interval_str.parse::<usize>().ok()?;
    
        let mut decrypted = String::new();
        for (i, ele) in encrypted.chars().enumerate() {
            if i == 0 {
                continue;
            };
            if i == 1 {
                decrypted.push(ele);
                continue;
            }
    
            if (i - 1) % interval == 0 {
                decrypted.push(ele);
            }
        }
    
        Some(decrypted)
    }
}