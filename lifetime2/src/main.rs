fn longer_vector<'a>(x: &'a[i32], y: &'a[i32]) -> &'a[i32] {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![1, 2];

    println!("{:?}", longer_vector(&vec1, &vec2));

    let vec: Vec<std::String> = "127.0.0.1:8.8.8.8".parse().unwrap();
    println!("{} {}", a, b);
}
