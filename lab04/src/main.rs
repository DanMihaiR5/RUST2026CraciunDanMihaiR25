use std::{fs, io};

// P1 - Longest line
fn p1() -> Result<(), io::Error> {
    let content = fs::read_to_string("p1.txt")?;

    let mut longest_bytes = "";
    let mut longest_chars = "";

    for line in content.lines() {
        if line.len() > longest_bytes.len() {
            longest_bytes = line;
        }

        if line.chars().count() > longest_chars.chars().count() {
            longest_chars = line;
        }
    }

    println!("Longest line in bytes: {}", longest_bytes);
    println!("Longest line in characters: {}", longest_chars);

    Ok(())
}

// P2 - ROT13 Cipher
fn p2() -> Result<(), io::Error> {
    let content = fs::read_to_string("p2.txt")?;

    let mut result = String::new();

    for c in content.chars() {
        if !c.is_ascii() {
            println!("Error: Non-ASCII character encountered!");
            return Ok(());
        }

        if c.is_ascii_lowercase() {
            let code = c as u8;
            let new_code = (code - b'a' + 13) % 26 + b'a';
            result.push(new_code as char);
        } else if c.is_ascii_uppercase() {
            let code = c as u8;
            let new_code = (code - b'A' + 13) % 26 + b'A';
            result.push(new_code as char);
        } else {
            result.push(c);
        }
    }

    println!("ROT13 result: {}", result);

    Ok(())
}

// P3 - Replace abbreviations
fn p3() -> Result<(), io::Error> {
    let content = fs::read_to_string("p3.txt")?;

    let abbreviations = vec![
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    let mut result = String::new();

    for word in content.split_whitespace() {
        let mut found = false;

        for abbreviation in &abbreviations {
            if word == abbreviation.0 {
                result.push_str(abbreviation.1);
                found = true;
                break;
            }
        }

        if !found {
            result.push_str(word);
        }

        result.push(' ');
    }

    println!("Result: {}", result.trim());

    Ok(())
}

// P4 - Read hosts file
fn p4() -> Result<(), io::Error> {
    let path = if cfg!(target_os = "windows") {
        r"C:\Windows\System32\drivers\etc\hosts"
    } else {
        "/etc/hosts"
    };

    let content = fs::read_to_string(path)?;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('#') {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() >= 2 {
            println!("{} => {}", words[1], words[0]);
        }
    }

    Ok(())
}

fn main() {
    println!("========== P1 ==========");
    if let Err(e) = p1() {
        println!("Error: {}", e);
    }

    println!("\n========== P2 ==========");
    if let Err(e) = p2() {
        println!("Error: {}", e);
    }

    println!("\n========== P3 ==========");
    if let Err(e) = p3() {
        println!("Error: {}", e);
    }

    println!("\n========== P4 ==========");
    if let Err(e) = p4() {
        println!("Error: {}", e);
    }
}
