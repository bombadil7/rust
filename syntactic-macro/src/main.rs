// Network Programming with Rust
// Chapter 2 / syntactic_macros

macro_rules! factorial {
    ($x:expr) => {
        {
            let mut result = 1;

            for i in 1..($x+1) {
                result = result * i;
            }
            result
        }
    };
}

fn main() {
    match std::env::args().nth(2) {
        Some(_) => {
            println!("Error: too many arguments.  Just one will do ;)");
            return 
        },
        None => {
            let arg = std::env::args().nth(1).expect("Please provide at least one argument");
            println!("{:?}", factorial!(arg.parse::<u64>().expect("Could not parse  
                to an integer")));
        },
    }
}
