struct Note {
    title: String,
    body: String,
}

// ---------- Part 1: borrow to read (&str) ----------

// How many times does c appear in s?
fn count_char(s: &str, c: char) -> usize {
    let mut count = 0;

    for ch in s.chars() {
        if ch == c {
            count += 1;
        }
    }

    count
}

// The first word of s, as a slice of s.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b' ' {
            return &s[..i];
        }
        i += 1;
    }

    s
}

// A new String with the characters of s in reverse order.
fn reverse(s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        result.insert(0, c);
    }

    result
}

// True if s reads the same backwards.
fn is_palindrome(s: &str) -> bool {
    s == reverse(s)
}

// A new String made of s repeated n times.
fn repeat_str(s: &str, n: usize) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < n {
        result += s;
        i += 1;
    }

    result
}

// ---------- Part 2: mutable borrow (&mut String) ----------

// Append "[tag]" to the caller's string.
fn add_tag(s: &mut String, tag: &str) {
    s.push('[');
    s.push_str(tag);
    s.push(']');
}

// Replace every occurrence of c with '*'.
fn censor(s: &mut String, c: char) {
    let mut result = String::new();

    for ch in s.chars() {
        if ch == c {
            result.push('*');
        } else {
            result.push(ch);
        }
    }

    *s = result;
}

// ---------- Part 3: ownership in, ownership out (String) ----------

// Take a string, make it uppercase, and give it back.
fn shout(mut s: String) -> String {
    s.make_ascii_uppercase();
    s
}

// Take two strings and return the longer one.
// The first is returned on a tie.
fn longer_of(a: String, b: String) -> String {
    if a.len() >= b.len() { a } else { b }
}

// ---------- Part 4: Notebook ----------

// Build a Note that owns its text.
fn make_note(title: &str, body: &str) -> Note {
    let mut owned_title = String::new();
    owned_title += title;

    let mut owned_body = String::new();
    owned_body += body;

    Note {
        title: owned_title,
        body: owned_body,
    }
}

// Number of words in the body, counted by hand with chars().
fn word_count(note: &Note) -> usize {
    let mut count = 0;
    let mut inside_word = false;

    for c in note.body.chars() {
        if c == ' ' {
            inside_word = false;
        } else {
            if !inside_word {
                count += 1;
                inside_word = true;
            }
        }
    }

    count
}

// Add text to the end of the body, separated by one space.
fn append(note: &mut Note, text: &str) {
    if !note.body.is_empty() {
        note.body.push(' ');
    }

    note.body.push_str(text);
}

// Consume the note and return its archived representation.
fn archive(note: Note) -> String {
    let mut result = String::new();

    result += "[ARCHIVED] ";
    result += &note.title;
    result += ": ";
    result += &note.body;

    result
}

// Combine two notes. Reuse a and only borrow b.
fn merge(mut a: Note, b: &Note) -> Note {
    a.title.push('+');
    a.title.push_str(&b.title);

    if !a.body.is_empty() && !b.body.is_empty() {
        a.body.push(' ');
    }

    a.body.push_str(&b.body);

    a
}

fn main() {
    // Part 1
    let text = String::from("level up your rust");
    println!("{}", count_char(&text, 'e'));
    println!("{}", count_char("banana", 'a'));
    println!("{}", first_word(&text));
    println!("{}", first_word("rust"));
    println!("{}", reverse(&text));
    println!("{} {}", is_palindrome("level"), is_palindrome(&text));
    println!("{}", repeat_str("ab", 3));

    // Part 2
    let mut s = String::from("hello world");
    add_tag(&mut s, "v2");
    println!("{s}");
    censor(&mut s, 'o');
    println!("{s}");

    // Part 3
    let loud = shout(s);
    println!("{loud}");
    let winner = longer_of(String::from("tiny"), loud);
    println!("{winner}");

    // Part 4
    let title = String::from("shopping");
    let mut n1 = make_note(&title, "milk");
    let n2 = make_note("ideas", "learn rust borrowing");
    println!("{title}");
    println!("{} word(s)", word_count(&n1));
    append(&mut n1, "eggs");
    append(&mut n1, "bread");
    println!("{}: {}", n1.title, n1.body);
    let merged = merge(n1, &n2);
    println!("{}: {}", merged.title, merged.body);
    println!("{} word(s)", word_count(&merged));
    println!("{}", archive(n2));
}
