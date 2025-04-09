use rand::random_range;

const DISRUPTIONS: [char; 74] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', // Not every symbol included, excluded the ugly ones
    '!', '/', '(', ')', '#', ':', '<', '>', '|', '^', '~', '@',
];
fn main() {
    let text = "Leni ist cool!";
    println!("Text: {text}");
    let text = encrypt(3, text);
    println!("Interval-Verschlüsselt: {text}");
    let text = decrypt(text).unwrap();
    println!("Interval-Entschlüsselt: {text}");
}

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
        if i == 0 { continue };
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
