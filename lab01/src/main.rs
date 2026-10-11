fn este_prim(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn cmmdc(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    return a;
}

fn sunt_coprime(a: u32, b: u32) -> bool {
    return cmmdc(a, b) == 1;
}

fn main() {
    println!("nr prime de la 1..100");
    let mut i: u32 = 0;
    while i <= 100 {
        if este_prim(i) {
            println!("{} este prim", i);
        }
        i += 1;
    }

    println!("\nperechile de nr coprime intre 0 si 15");
    let mut x: u32 = 0;
    while x <= 15 {
        let mut y: u32 = 0;
        while y <= 15 {
            if sunt_coprime(x, y) {
                println!("{} si {} sunt coprime", x, y);
            }
            y += 1;
        }
        x += 1;
    }

    println!("\n 99 Bottles of Beer ");
    let mut sticle = 99;
    while sticle >= 1 {
        if sticle > 1 {
            println!("{} bottles of beer on the wall, {} bottles of beer.", sticle, sticle);
            if sticle - 1 == 1 {
                println!("Take one down and pass it around, 1 bottle of beer on the wall.\n");
            } else {-
                println!("Take one down and pass it around, {} bottles of beer on the wall.\n", sticle - 1);
            }
        } else {
            println!("1 bottle of beer on the wall, 1 bottle of beer.");
            println!("Take one down and pass it around, no more bottles of beer on the wall.\n");
        }
        sticle -= 1;
    }
    println!("No more bottles of beer on the wall, no more bottles of beer.");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
}
