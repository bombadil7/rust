struct Empty;

impl Iterator for Empty {
    type Item = u32;

    fn next(&mut self) -> Option<<Self>::Item> {
        None
    }
}

struct TheAnswer;

impl Iterator for TheAnswer {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(42)
    }
}

fn main() {
    for i in TheAnswer.take(10) {
        panic!("The answer to life, the universe, and everything is {}", i);
    }
    println!("All done!");
}
