#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {
    let x = vec![1, 2, 3, 4];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{}", i);
    }

    println!("{:?} {} {}", &v, v.len(), v.capacity());

    let r = vec![
        Example::Int(142),
        Example::Float(12.32),
        Example::Text(String::from("string")),
    ];
    println!("{:?}", &r);
}
