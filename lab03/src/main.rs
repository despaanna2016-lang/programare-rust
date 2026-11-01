fn is_prime(n: u16) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i <= n / i {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut curr = x + 1;
    while curr < u16::MAX {
        if is_prime(curr) {
            return Some(curr);
        }
        curr += 1;
    }
    None
}

fn add_with_panic(a: u32, b: u32) -> u32 {
    match a.checked_add(b) {
        Some(suma) => suma,
        None => panic!("EROARE: Depasire la adunare!"),
    }
}

fn mul_with_panic(a: u32, b: u32) -> u32 {
    match a.checked_mul(b) {
        Some(produs) => produs,
        None => panic!("EROARE: Depasire la inmultire!"),
    }
}

#[derive(Debug)]
enum MathError {
    Overflow,
}

fn add_with_result(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_add(b) {
        Some(suma) => Ok(suma),
        None => Err(MathError::Overflow),
    }
}

fn mul_with_result(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_mul(b) {
        Some(produs) => Ok(produs),
        None => Err(MathError::Overflow),
    }
}

fn calc_complexe(a: u32, b: u32, c: u32) -> Result<u32, MathError> {
    let suma = match add_with_result(a, b) {
        Ok(valoare) => valoare,
        Err(eroare) => return Err(eroare),
    };

    let rezultat_final = match mul_with_result(suma, c) {
        Ok(valoare) => valoare,
        Err(eroare) => return Err(eroare),
    };

    Ok(rezultat_final)
}
enum CharError {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(c.to_ascii_uppercase())
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(c.to_ascii_lowercase())
}

fn print_char(c: char) -> Result<(), CharError> {
    if c == '\n' || c == '\t' || c == '\r' {
        return Err(CharError::NotPrintable);
    }
    println!("Caracter valid: {}", c);
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    match c.to_digit(10) {
        Some(cifra) => Ok(cifra),
        None => Err(CharError::NotDigit),
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    match c.to_digit(16) {
        Some(cifra) => Ok(cifra),
        None => Err(CharError::NotBase16),
    }
}

fn print_error(err: CharError) {
    match err {
        CharError::NotAscii => println!("Eroare: Carac nu e ASCII."),
        CharError::NotDigit => println!("Eroare: Carac nu e o cifra nrml."),
        CharError::NotBase16 => println!("Eroare: Carac nu e o cifra hexazecimala."),
        CharError::NotLetter => println!("Eroare: Carac nu e o litera."),
        CharError::NotPrintable => println!("Eroare: Carac nu poate fi afisat."),
    }
}

fn main() {
    let mut nr = 65500;
    while let Some(prim) = next_prime(nr) {
        println!("Numar prim gasit: {}", prim);
        nr = prim;
    }
    println!("Nu s au mai gasit numere prime in u16!");

    println!("\nTestare Operatii Matematice");
    match calc_complexe(100, 200, 3) {
        Ok(valoare) => println!("Rezultat matematic: {}", valoare),
        Err(MathError::Overflow) => println!("Esuat: Depasire de mem!"),
    }

    println!("\nTestare Operatii Caractere (Succes)");
    let c = 'a';
    match to_uppercase(c) {
        Ok(litera_mare) => println!("Litera mare pentru {} este {}", c, litera_mare),
        Err(_) => println!("A aparut o eroare la transformare."),
    }

    println!("\nTestare Operatii Caractere (Esec)");
    let c_gresit = '5';
    match to_uppercase(c_gresit) {
        Ok(_) => println!("Transformarea a reusit!"),
        Err(eroare) => print_error(eroare),
    }
}
