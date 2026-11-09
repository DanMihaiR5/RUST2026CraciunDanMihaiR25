
use std::{fs, error::Error};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

// P1 - Oldest and youngest student
fn p1() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("students.txt")?;

    let mut students = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 3 {
            continue;
        }

        let student = Student {
            name: parts[0].trim().to_string(),
            phone: parts[1].trim().to_string(),
            age: parts[2].trim().parse()?,
        };

        students.push(student);
    }

    print_oldest_youngest(&students);

    Ok(())
}

// P2 - Canvas
struct Canvas {
    pixels: [[u8; 100]; 55],
}

fn new_canvas() -> Canvas {
    Canvas {
        pixels: [[b' '; 100]; 55],
    }
}

fn set_pixels(canvas: &mut Canvas, pixels: &[(usize, usize, u8)]) {
    for pixel in pixels {
        let (x, y, value) = *pixel;

        if x < 100 && y < 55 {
            canvas.pixels[y][x] = value;
        }
    }
}

fn print(canvas: Canvas) {
    for row in &canvas.pixels {
        for pixel in row {
            print!("{}", *pixel as char);
        }
        println!();
    }
}

fn p2() {
    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(c, &[(2, 16, 95), (5, 35, 92), (6, 3, 95), (2, 11, 95), (5, 3, 95)]);
    set_pixels(c, &[(2, 38, 95), (4, 9, 40), (3, 41, 124), (2, 37, 95), (2, 25, 124)]);
    set_pixels(c, &[(5, 27, 124), (2, 27, 124), (4, 0, 124), (3, 35, 47), (2, 18, 95)]);
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(c, &[(6, 30, 95), (5, 45, 95), (4, 31, 124), (4, 7, 124), (3, 43, 39)]);
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(c, &[(3, 44, 95), (3, 19, 92), (5, 23, 95), (3, 8, 47), (2, 10, 95)]);
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(c, &[(3, 10, 95), (4, 44, 95), (2, 9, 95), (0, 32, 95), (5, 2, 95)]);
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(c, &[(3, 46, 92), (5, 25, 44), (1, 43, 124), (5, 46, 47), (3, 15, 47)]);
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(c, &[(5, 44, 95), (2, 2, 95), (5, 10, 95), (5, 9, 95), (4, 43, 124)]);
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(c, &[(4, 40, 124), (3, 3, 95), (4, 4, 124), (6, 31, 47), (3, 4, 96)]);
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(c, &[(3, 12, 92), (2, 32, 95), (5, 37, 95), (5, 26, 95), (5, 39, 47)]);
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(c, &[(5, 36, 95), (7, 30, 95), (6, 4, 47), (4, 24, 95), (1, 32, 95)]);
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(canvas);
}

// P3 - Oldest and youngest student using JSON
fn p3() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("students.jsonl")?;

    let mut students = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let student: Student = serde_json::from_str(line)?;

        students.push(student);
    }

    print_oldest_youngest(&students);

    Ok(())
}

// Common function for P1 and P3
fn print_oldest_youngest(students: &[Student]) {
    if students.is_empty() {
        println!("No students found!");
        return;
    }

    let oldest = students.iter().max_by_key(|s| s.age).unwrap();
    let youngest = students.iter().min_by_key(|s| s.age).unwrap();

    println!("Oldest student:");
    println!("Name: {}", oldest.name);
    println!("Phone: {}", oldest.phone);
    println!("Age: {}", oldest.age);

    println!("\nYoungest student:");
    println!("Name: {}", youngest.name);
    println!("Phone: {}", youngest.phone);
    println!("Age: {}", youngest.age);
}

fn main() {
    println!("========== P1 ==========");
    if let Err(e) = p1() {
        println!("Error: {}", e);
    }

    println!("\n========== P2 ==========");
    p2();

    println!("\n========== P3 ==========");
    if let Err(e) = p3() {
        println!("Error: {}", e);
    }
}
