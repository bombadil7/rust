// Network Programming with Rust
// chapter 2 - closure-borrow

fn main() {
    // closure with two parameters
    let add = |a, b| a + b;
    assert_eq!(add(2, 3), 5);

    // comman use cases are on iterators
    println!("{:?}", (1..10).filter(|x| x % 2 == 0).collect::<Vec<u32>>());

    let times = 2;
    println!("{:?}", (1..10).map(|x| x * times).collect::<Vec<u32>>());
}
