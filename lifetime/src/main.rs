fn main() {
    let v = vec![4, 5, 6, 7, 3, 6, 7, 4, 8, 6, 4, 2, 5, 3, 7, 7];
    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {} times", i, r);
    }
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}
