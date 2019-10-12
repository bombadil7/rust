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
        //OneToTen {0: 0 }
        OneToTen(0)
    }
}

impl Iterator for OneToTen {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.0 += 1;
        Some(self.0)
    }
}

struct Doubler<I> {
    iter: I,
}

impl<I: Iterator<Item=u32>> Iterator for Doubler<I> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match self.iter.next() {
            None => None,
            Some(x) => Some(x * 2),
        }
    }
}

fn main() {
    //    let num = OneToTen::new();
    //
    //    for i in num.take(10) {
    //        println!("The answer to life, the universe, and everything is {}", i);
    //    }
    //    println!("All done!");

    let orig_iter = 1..11;
    let doubled_iter = Doubler {
        iter: orig_iter,
    };
    for i in doubled_iter {
        println!("{}", i);
    }
}
