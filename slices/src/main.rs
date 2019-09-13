fn main() {
    let s = String::from("Hello, world!");

    let first = &s[0..5];
    let second = &s[7..13];

    println!("{} {}", first, second);

    let _word = first_word(&s);

    let my_string_literal = "hello wordl";
    let _word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}