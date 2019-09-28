use std::env::args;

fn main() {
    let args = args().skip(1);
    for arg in args {
        println!("{}", arg.parse::<i32>().unwrap());
    }
}
