/**
 * Author:  Raye Lattice
 * Repo:    envision
 * Created: 10/19/2025
 */

fn char_to_8bit_bin(c: char) -> String {
    let ascii = c as u8;
    format!("{:08b}", ascii)
}

fn encrypt_groups(groups: &[String]) -> Vec<String> {
    let mut encrypted = Vec::with_capacity(groups.len());
    if let Some(first) = groups.get(0) {
        encrypted.push(first.clone());
        let mut prev = first.clone();
        for i in 1..groups.len() {
            let current = &groups[i];
            let mut xor_result = String::with_capacity(prev.len());
            for (bit_prev, bit_curr) in prev.chars().zip(current.chars()) {
                let xor_bit = bit_prev != bit_curr;
                xor_result.push(if xor_bit { '1' } else { '0' });
            }
            encrypted.push(xor_result.clone());
            prev = xor_result;
        }
    }
    encrypted
}

fn compress(cipher: &str) -> Vec<usize> {
    let mut compressed = Vec::new();
    if cipher.is_empty() {
        return compressed;
    }

    let mut chars = cipher.chars();
    let first_char = chars.next().unwrap();
    let mut current_char = first_char;
    let mut count = 1;

    compressed.push(if current_char == '0' { 0 } else { 1 });

    for c in chars {
        if c == current_char {
            count += 1;
        } else {
            compressed.push(count);
            current_char = c;
            compressed.push(if current_char == '0' { 0 } else { 1 });
            count = 1;
        }
    }
    compressed.push(count);
    compressed
}

fn encrypt_text(text: &str) -> Vec<usize> {
    let mut all_bits = String::new();
    for c in text.chars() {
        all_bits.push_str(&char_to_8bit_bin(c));
    }

    let groups: Vec<String> = all_bits
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect())
        .collect();

    let encrypted_groups = encrypt_groups(&groups);
    let cipher_bits: String = encrypted_groups.join("");
    compress(&cipher_bits)
}

fn decompress(compressed: &[usize]) -> Result<String, &'static str> {
    if compressed.is_empty() {
        return Ok(String::new());
    }

    if compressed.len() % 2 != 1 {
        return Err("Invalid compressed data format");
    }

    let mut cipher = String::new();
    let mut iter = compressed.iter();
    let first_char_indicator = iter.next().ok_or("Invalid compressed data")?;
    let first_char = match first_char_indicator {
        0 => '0',
        1 => '1',
        _ => return Err("Invalid character indicator, must be 0 or 1"),
    };

    let mut current_char = first_char;

    while let Some(&count) = iter.next() {
        cipher.extend(std::iter::repeat(current_char).take(count));
        if let Some(&next_char_indicator) = iter.next() {
            current_char = match next_char_indicator {
                0 => '0',
                1 => '1',
                _ => return Err("Invalid character indicator, must be 0 or 1"),
            };
        }
    }

    Ok(cipher)
}

fn decrypt_groups(groups: &[String]) -> Vec<String> {
    let mut decrypted = Vec::with_capacity(groups.len());
    if let Some(first) = groups.get(0) {
        decrypted.push(first.clone());
        let mut prev_encrypted = first.clone();

        for i in 1..groups.len() {
            let current_encrypted = &groups[i];
            let mut xor_back = String::with_capacity(prev_encrypted.len());

            for (bit_prev, bit_curr) in prev_encrypted.chars().zip(current_encrypted.chars()) {
                let orig_bit = bit_prev != bit_curr;
                xor_back.push(if orig_bit { '1' } else { '0' });
            }

            decrypted.push(xor_back.clone());
            prev_encrypted = current_encrypted.clone();
        }
    }
    decrypted
}

fn groups_to_text(groups: &[String]) -> Result<String, &'static str> {
    let mut bits = String::new();
    for group in groups {
        bits.push_str(group);
    }

    if bits.len() % 8 != 0 {
        return Err("Bit length is not a multiple of 8");
    }

    let mut text = String::new();
    for byte_bits in bits.chars().collect::<Vec<_>>().chunks(8) {
        let byte_str: String = byte_bits.iter().collect();
        let byte = u8::from_str_radix(&byte_str, 2)
            .map_err(|_| "Failed to parse binary string")?;
        text.push(byte as char);
    }

    Ok(text)
}

fn decrypt_compressed(compressed: &[usize]) -> Result<String, &'static str> {
    let cipher_bits = decompress(compressed)?;

    if cipher_bits.len() % 4 != 0 {
        return Err("Cipher bits length is not a multiple of 4");
    }

    let groups: Vec<String> = cipher_bits
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect())
        .collect();

    let decrypted_groups = decrypt_groups(&groups);
    groups_to_text(&decrypted_groups)
}
