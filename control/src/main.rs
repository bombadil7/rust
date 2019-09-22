fn main() {
    let n = 15;
    match n {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime!"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let p = 5;

    let n = match p {
        n @ 1..=12 => n,
        n @ 13..=19 => n,
        _ => 0,
    };

    println!("n: {}", n);
}
