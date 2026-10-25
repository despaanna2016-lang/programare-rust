struct Note {
    title: String,
    body: String,
}

fn count_char(s: &str, c: char) -> usize {
    let mut count = 0;
    for ch in s.chars() {
        if ch == c {
            count += 1;
        }
    }
    count
}

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

fn reverse(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        result.insert(0, c);
    }
    result
}

fn is_palindrome(s: &str) -> bool {
    s == reverse(s)
}

fn repeat_str(s: &str, n: usize) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < n {
        result.push_str(s);
        i += 1;
    }
    result
}

fn add_tag(s: &mut String, tag: &str) {
    s.push('[');
    s.push_str(tag);
    s.push(']');
}

fn censor(s: &mut String, c: char) {
    let mut aux = String::new();
    for ch in s.chars() {
        if ch == c {
            aux.push('*');
        } else {
            aux.push(ch);
        }
    }
    *s = aux;
}

fn shout(s: String) -> String {
    let mut upper_s = s;
    upper_s.make_ascii_uppercase();
    upper_s
}

fn longer_of(a: String, b: String) -> String {
    if a.len() >= b.len() { a } else { b }
}

fn make_note(title: &str, body: &str) -> Note {
    Note {
        title: String::from(title),
        body: String::from(body),
    }
}

fn word_count(note: &Note) -> usize {
    if note.body.is_empty() {
        return 0;
    }
    let mut count = 1;
    for c in note.body.chars() {
        if c == ' ' {
            count += 1;
        }
    }
    count
}

fn append(note: &mut Note, text: &str) {
    if note.body.is_empty() {
        note.body.push_str(text);
    } else {
        note.body.push(' ');
        note.body.push_str(text);
    }
}

fn archive(note: Note) -> String {
    let mut result = String::from("[ARCHIVED] ");
    result.push_str(&note.title);
    result.push_str(": ");
    result.push_str(&note.body);
    result
}

fn merge(a: Note, b: &Note) -> Note {
    let mut merged = a;
    merged.title.push('+');
    merged.title.push_str(&b.title);
    if !merged.body.is_empty() && !b.body.is_empty() {
        merged.body.push(' ');
    }
    merged.body.push_str(&b.body);
    merged
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
