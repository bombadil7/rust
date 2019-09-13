use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_searchstring_param() {

    }
}