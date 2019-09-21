fn main() {
    for i in 1..116 {
        println!("{}", fizzbizz(i));
    }
}

fn fizzbizz(val: u32) -> String {
    let mut str = String::new();

    match (val % 3, val % 5) {
        (0, 0) => str = String::from("fizzbizz"),
        (0, _) => str = String::from("fizz"),
        (_, 0) => str = String::from("bizz"),
        (_, _) => str = val.to_string(),
    }

    str
}
