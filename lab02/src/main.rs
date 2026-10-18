fn add_chars_borrow(s: &mut String, c: char, n: u32) {
    let mut j = 0;
    while j < n {
        s.push(c);
        j += 1;
    }
}

fn add_chars_ownership(mut s: String, c: char, n: u32) -> String {
    let mut j = 0;
    while j < n {
        s.push(c);
        j += 1;
    }
    s
}

fn add_space(s: &mut String, n: usize) {
    let mut j = 0;
    while j < n {
        s.push(' ');
        j += 1;
    }
}

fn add_str(s: &mut String, text: &str) {
    s.push_str(text);
}

fn add_integer(s: &mut String, mut n: u32) {
    if n == 0 {
        s.push('0');
        return;
    }
    let mut digits = [0; 20];
    let mut len = 0;
    while n > 0 {
        digits[len] = n % 10;
        len += 1;
        n /= 10;
    }
    let mut i = len;
    while i > 0 {
        i -= 1;
        s.push((digits[i] as u8 + b'0') as char);
        if i > 0 && i % 3 == 0 {
            s.push('_');
        }
    }
}

fn add_float(s: &mut String, f: f32) {
    let int_part = f as u32;
    add_integer(s, int_part);
    s.push('.');
    let mut frac = f - (int_part as f32);
    frac += 0.0001;
    let mut count = 0;
    while count < 3 {
        frac *= 10.0;
        let digit = frac as u32;
        s.push((digit as u8 + b'0') as char);
        frac -= digit as f32;
        count += 1;
    }
}

fn main() {
    let mut s1 = String::from("");
    let mut i1 = 0;
    while i1 < 26 {
        let c = (i1 as u8 + b'a') as char;
        add_chars_borrow(&mut s1, c, 26 - i1);
        i1 += 1;
    }
    println!("{}\n", s1);

    let mut s2 = String::from("");
    let mut i2 = 0;
    while i2 < 26 {
        let c = (i2 as u8 + b'a') as char;
        s2 = add_chars_ownership(s2, c, 26 - i2);
        i2 += 1;
    }
    println!("{}\n", s2);

    let mut text = String::new();

    add_space(&mut text, 40);
    add_str(&mut text, "I 💚\n");

    add_space(&mut text, 40);
    add_str(&mut text, "RUST.\n\n");

    add_space(&mut text, 4);
    add_str(&mut text, "Most");
    add_space(&mut text, 12);
    add_str(&mut text, "crate");
    add_space(&mut text, 6);
    add_integer(&mut text, 306437968);
    add_space(&mut text, 11);
    add_str(&mut text, "and");
    add_space(&mut text, 4);
    add_str(&mut text, "lastest");
    add_space(&mut text, 8);
    add_str(&mut text, "is\n");

    add_space(&mut text, 9);
    add_str(&mut text, "downloaded");
    add_space(&mut text, 8);
    add_str(&mut text, "has");
    add_space(&mut text, 13);
    add_str(&mut text, "downloads");
    add_space(&mut text, 5);
    add_str(&mut text, "the");
    add_space(&mut text, 8);
    add_str(&mut text, "version");
    add_space(&mut text, 4);
    add_float(&mut text, 2.038);
    add_str(&mut text, ".");

    println!("{}", text);
}
