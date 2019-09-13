use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_searchstring_param() {

    }
}