fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..6).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let v = 192;
    match v {
        1...100 => println!("Inside"),
        _ => println!("Outside")
    }

    let r = [2,3,4,5,6,9];
    for i in r.iter() {
        println!("num");
    }
}
