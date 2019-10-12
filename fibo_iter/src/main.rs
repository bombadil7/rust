struct Fibonacci(u32, u32);

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci(0, 1)
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let fnum = self.0;
        let nextnum = self.0 + self.1;
        self.0 = self.1;
        self.1 = nextnum;
        Some(fnum)
    }
}

fn main() {
    let fib = Fibonacci::new();

    for fnum in fib.take(15) {
        println!("{}", fnum);
    }

    println!("Done!");
}
