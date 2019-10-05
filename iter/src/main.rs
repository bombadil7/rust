//struct Empty;
//
//impl Iterator for Empty {
//    type Item = u32;
//
//    fn next(&mut self) -> Option<<Self>::Item> {
//        None
//    }
//}

//struct TheAnswer;
//
//impl Iterator for TheAnswer {
//    type Item = u32;
//
//    fn next(&mut self) -> Option<u32> {
//        Some(42)
//    }
//}
//
struct OneToTen(u32);

impl OneToTen {
    fn new() -> OneToTen {
        OneToTen {0: 0 }
    }
}

impl Iterator for OneToTen {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        OneToTen.0 += 1;
        Some( OneToTen.0 )
    }
}

fn main() {
    let num = OneToTen::new();

    for i in OneToTen.take(10) {
        println!("The answer to life, the universe, and everything is {}", i);
    }
    println!("All done!");
}
