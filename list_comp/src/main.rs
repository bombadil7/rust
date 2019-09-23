fn main() {
    //    let mut numbers = vec![];
    //    for i in 1..1000 {
    //        numbers.push(i)
    //    }

    let numbers: Vec<i32> = (1..1000).collect();
    println!("{:?}", numbers)
}
