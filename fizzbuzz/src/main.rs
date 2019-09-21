fn main() {
    for i in 1..16 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz<'a>(val: u32) -> &'a str {
    if val % 3 == 0 && val % 5 == 0 {
        "fizzbuzz"
    } else if val % 3 == 0 {
        "fizz"
    } else if val % 5 == 0 {
        "buzz"
    } else {
        let s: String = val.to_string();
        //let ss = &s[..];
        println!("{}", s);
        //ss.clone()
        ""
    }
}
