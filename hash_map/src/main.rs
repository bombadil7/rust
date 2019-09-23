use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }

    let val = match hm.get(&String::from("random")) {
        Some(&n) => n,
        _ => 42,
    };
    println!("{}", val);

    // casting
    let f = 24.4321_f32;
    let i = f as u8;
    let c = i as char;

    println!("{} {} {}", f, i, c);

    println!("{}", 255 as char);
    println!("\u{256}");
}
