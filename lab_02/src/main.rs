// P1
/*
fn add_chars_n(mut s: String, c: char, n: usize) -> String {
    for _ in 0..n {
        s.push(c);
    }
    s
}
*/

// P2

fn add_chars_n(s: &mut String, c: char, n: usize) {
    for _ in 0..n {
        s.push(c);
    }
}

//P3

fn add_space(s: &mut String, n: usize) {
    for _ in 0..n {
        s.push(' ');
    }
}

fn add_str(s: &mut String, text: &str) {
    s.push_str(text);
}

fn add_integer(s: &mut String, mut n: i32) {
    if n < 0 {
        s.push('-');
        n = -n;
    }

    if n == 0 {
        s.push('0');
        return;
    }

    let mut digits: [i32; 10] = [0; 10];
    let mut count = 0;

    while n != 0 {
        digits[count] = n % 10;
        n = n / 10;
        count += 1;
    }

    let mut i = count - 1;

    while i >= 0 {
        s.push((digits[i] as u8 + b'0') as char);

        if i > 0 && i % 3 == 0 {
            s.push('_');
        }

        if i == 0 {
            break;
        }

        i -= 1;
    }
}

fn add_float(s: &mut String, f: f64) {
    let mut whole = f as i32;
    let mut frac = f - whole as f64;

    if f < 0.0 {
        s.push('-');
        whole = -whole;
        frac = -frac;
    }

    add_integer(s, whole);

    if frac == 0.0 {
        return;
    }

    s.push('.');

    
    let mut i = 0;

    while i < 15 {
        frac = frac * 10.0;
        let digit = frac as i32;

        s.push((digit as u8 + b'0') as char);

        frac = frac - digit as f64;
        i += 1;

        if frac < 0.000000000000001 && frac > -0.000000000000001 {
            break;
        }
    }
}

fn main() {
    let mut s = String::new();

    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n(&mut s, c, 26 - i);
        i += 1;
    }

    println!("{}", s);

    let mut s1 = String::new();

    // First line
    add_space(&mut s1, 40);
    add_str(&mut s1, "I 💚");
    add_str(&mut s1, "\n");

    // Second line
    add_space(&mut s1, 40);
    add_str(&mut s1, "RUST.");
    add_str(&mut s1, "\n\n");

    // Third line
    add_space(&mut s1, 4);
    add_str(&mut s1, "Most");
    add_space(&mut s1, 12);
    add_str(&mut s1, "crate");
    add_space(&mut s1, 6);
    add_integer(&mut s1, 306437968);
    add_space(&mut s1, 11);
    add_str(&mut s1, "and");
    add_space(&mut s1, 5);
    add_str(&mut s1, "lastest");
    add_space(&mut s1, 9);
    add_str(&mut s1, "is");
    add_str(&mut s1, "\n");

    // Fourth line
    add_space(&mut s1, 9);
    add_str(&mut s1, "downloaded");
    add_space(&mut s1, 8);
    add_str(&mut s1, "has");
    add_space(&mut s1, 13);
    add_str(&mut s1, "downloads");
    add_space(&mut s1, 5);
    add_str(&mut s1, "the");
    add_space(&mut s1, 9);
    add_str(&mut s1, "version");
    add_space(&mut s1, 4);
    add_float(&mut s1, 2.038);
    add_str(&mut s1, "\n\n");

    println!("{}", s1);
}

