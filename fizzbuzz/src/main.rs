fn main() {
    for i in 1..101 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(val: u32) -> String {
    let mut str: String = String::new();

    if val % 3 == 0 {
        str = str + &String::from("fizz");
    }

    if val % 5 == 0 {
        str = str + &String::from("buzz");
    }

    if str.len() < 1 {
        str = val.to_string();
    }

    str
}
