// PROBLEMA 1

fn is_prime(n: u16) -> bool {
    if n < 2 {
        return false;
    }

    let mut i = 2;

    while i <= n / i {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut n = x.checked_add(1)?;

    loop {
        if is_prime(n) {
            return Some(n);
        }

        match n.checked_add(1) {
            Some(next) => n = next,
            None => return None,
        }
    }
}

// PROBLEMA 2

fn checked_addition(a: u32, b: u32) -> u32 {
    match a.checked_add(b) {
        Some(result) => result,
        None => panic!("Overflow la adunare!"),
    }
}

fn checked_multiplication(a: u32, b: u32) -> u32 {
    match a.checked_mul(b) {
        Some(result) => result,
        None => panic!("Overflow la inmultire!"),
    }
}

// PROBLEMA 3

#[derive(Debug)]
enum MathError {
    Overflow,
}

fn checked_addition_result(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_add(b) {
        Some(result) => Ok(result),
        None => Err(MathError::Overflow),
    }
}

fn checked_multiplication_result(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_mul(b) {
        Some(result) => Ok(result),
        None => Err(MathError::Overflow),
    }
}

fn calculate(a: u32, b: u32, c: u32) -> Result<u32, MathError> {
    let sum = checked_addition_result(a, b)?;
    let result = checked_multiplication_result(sum, c)?;

    Ok(result)
}

// PROBLEMA 4

#[derive(Debug)]
enum CharError {
    NotAscii,
    NotDigit,
    NotHexDigit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<String, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }

    Ok(c.to_uppercase().collect())
}

fn to_lowercase(c: char) -> Result<String, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }

    Ok(c.to_lowercase().collect())
}

fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        return Err(CharError::NotPrintable);
    }

    println!("{}", c);
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }

    match c.to_digit(10) {
        Some(number) => Ok(number),
        None => Err(CharError::NotDigit),
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }

    match c.to_digit(16) {
        Some(number) => Ok(number),
        None => Err(CharError::NotHexDigit),
    }
}

fn print_error(error: CharError) {
    match error {
        CharError::NotAscii => println!("Caracterul nu este ASCII!"),
        CharError::NotDigit => println!("Caracterul nu este o cifra!"),
        CharError::NotHexDigit => println!("Caracterul nu este o cifra hexazecimala!"),
        CharError::NotLetter => println!("Caracterul nu este o litera!"),
        CharError::NotPrintable => println!("Caracterul nu este imprimabil!"),
    }
}

// PROBLEMA 5

#[derive(Debug)]
enum FactorialError {
    Overflow,
}

fn factorial(n: u32) -> Result<u32, FactorialError> {
    let mut result: u32 = 1;

    for i in 1..=n {
        result = match result.checked_mul(i) {
            Some(value) => value,
            None => return Err(FactorialError::Overflow),
        };
    }

    Ok(result)
}

fn main() {
    // PROBLEMA 1
    println!("\n--- PROBLEMA 1 ---");

    let mut n = 0;

    loop {
        match next_prime(n) {
            Some(p) => {
                println!("{}", p);
                n = p;
            }
            None => break,
        }
    }

    // PROBLEMA 2
    println!("\n--- PROBLEMA 2 ---");

    println!("Adunare: {}", checked_addition(100, 200));
    println!("Inmultire: {}", checked_multiplication(10, 20));

    // PROBLEMA 3
    println!("\n--- PROBLEMA 3 ---");

    match calculate(10, 20, 3) {
        Ok(result) => println!("Rezultat: {}", result),
        Err(error) => println!("Eroare: {:?}", error),
    }

    match calculate(u32::MAX, 1, 2) {
        Ok(result) => println!("Rezultat: {}", result),
        Err(error) => println!("Eroare: {:?}", error),
    }

    // PROBLEMA 4
    println!("\n--- PROBLEMA 4 ---");

    match to_uppercase('a') {
        Ok(c) => println!("Uppercase: {}", c),
        Err(e) => print_error(e),
    }

    match to_uppercase('5') {
        Ok(c) => println!("Uppercase: {}", c),
        Err(e) => print_error(e),
    }

    match to_lowercase('B') {
        Ok(c) => println!("Lowercase: {}", c),
        Err(e) => print_error(e),
    }

    match print_char('X') {
        Ok(()) => println!("Afisare reusita!"),
        Err(e) => print_error(e),
    }

    match print_char('\n') {
        Ok(()) => println!("Afisare reusita!"),
        Err(e) => print_error(e),
    }

    match char_to_number('7') {
        Ok(n) => println!("Numar: {}", n),
        Err(e) => print_error(e),
    }

    match char_to_number('A') {
        Ok(n) => println!("Numar: {}", n),
        Err(e) => print_error(e),
    }

    match char_to_number('ă') {
        Ok(n) => println!("Numar: {}", n),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('F') {
        Ok(n) => println!("Hex: {}", n),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('G') {
        Ok(n) => println!("Hex: {}", n),
        Err(e) => print_error(e),
    }

    // PROBLEMA 5
    println!("\n--- PROBLEMA 5 ---");

    match factorial(5) {
        Ok(result) => println!("Factorial: {}", result),
        Err(error) => println!("Eroare: {:?}", error),
    }

    match factorial(100) {
        Ok(result) => println!("Factorial: {}", result),
        Err(error) => println!("Eroare: {:?}", error),
    }
}
