struct OneToTen(u32);

impl OneToTen {
    fn new() -> OneToTen {
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

impl<I> Iterator for Doubler<I> 
    where 
    I: Iterator,
    I::Item: std::ops::Add<Output=I::Item> + Copy,
    {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(x) => Some(x + x),
        }
    }
}

fn main() {
    let orig_iter = 1..11;
    let doubled_iter = Doubler { iter: orig_iter };
    for i in doubled_iter {
        println!("{}", i);
    }
}
