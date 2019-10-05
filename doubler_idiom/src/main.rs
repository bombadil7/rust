fn main() {
    for i in (1..11).map(|x| x + 1).filter(|x| x % 2 == 0) {
        println!("{}", i);
    }

    let my_vec: Vec<u32> = (1..11).collect();
    println!("{:?}", my_vec);

    let other_vec: Vec<u32> = my_vec.iter().map(|x| x * x).filter(|x| x % 2 != 0).collect();
    println!("{:?}", other_vec);
}
