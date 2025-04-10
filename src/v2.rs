use crate::{DISRUPTIONS, Encryptor};
use rand::random_range;

pub struct EncryptorV2;
impl Encryptor for EncryptorV2 {
    fn encrypt(interval: u16, input: &str) -> String {
        let random_interval_minuend = random_range((interval + 4)..99);
        let interval_subtrahend = random_interval_minuend - interval;

        let input = input.replace(" ", "");
        let mut encrypted = random_interval_minuend.to_string();
        for _ in 0..interval {
            let random_disruption_no_nums: Vec<char> = DISRUPTIONS
                .into_iter()
                .filter(|item| !item.is_ascii_digit())
                .collect();

            encrypted
                .push(random_disruption_no_nums[random_range(0..random_disruption_no_nums.len())]);
        }

        for (i, ele) in input.chars().enumerate() {
            encrypted.push(ele);

            if i + 1 != input.len() {
                for _ in 1..interval {
                    let random_disruption = DISRUPTIONS[random_range(0..DISRUPTIONS.len())];
                    encrypted.push(random_disruption);
                }
            }
        }

        encrypted.push_str(interval_subtrahend.to_string().as_str());
        encrypted
    }

    fn decrypt(encrypted: String) -> Option<String> {
        let interval = find_interval(&encrypted)? as usize;
        let encrypted = remove_interval_parts(&encrypted);

        let mut decrypted = String::new();
        for (i, ele) in encrypted.chars().enumerate() {
            if i == 0 {
                continue;
            }

            if i % interval == 0 {
                decrypted.push(ele);
            }
        }

        Some(decrypted)
    }
}

fn remove_interval_parts(encrypted: &str) -> String {
    let mut stripped_encrypted: String = encrypted.into();

    for ele in encrypted.chars() {
        if !ele.is_ascii_digit() { break }
        stripped_encrypted.remove(0);
    }

    for ele in encrypted.chars().rev() {
        if !ele.is_ascii_digit() { break }
        stripped_encrypted.pop();
    }
 
    stripped_encrypted
}

fn find_interval(encrypted: &str) -> Option<u16> {
    let mut start_num_str = String::new();
    let mut last_num_str = String::new();
    for ele in encrypted.chars() {
        if !ele.is_ascii_digit() { break }
        start_num_str.push(ele);
    }

    for ele in encrypted.chars().rev() {
        if !ele.is_ascii_digit() { break }
        last_num_str.push(ele);
    }

    let start = start_num_str.parse::<u16>().ok()?;
    let last = last_num_str.chars().rev().collect::<String>().parse::<u16>().ok()?;

    Some(start - last)
}
