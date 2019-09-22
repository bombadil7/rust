use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<(), Error> {
    //    let mut numbers = vec![];
    //    for i in 1..1000 {
    //        numbers.push(i)
    //    }

    //    let numbers: Vec<i64> = (1..1000).collect();
    //    println!("{:?}", numbers);

    let path = "/usr/share/dict/words";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    //    for line in buffered.lines() {
    //        println!("{}", line?);
    //    }
    let words: Vec<String> = buffered.lines().filter_map(Result::ok).collect();

//    let word_length: HashMap<_, usize> = words.iter().map(|word| (word, word.len())).collect();
    let mut word_length: HashMap<usize, _> = HashMap::new();
    for word in words {
        word_length.entry(word.len()).or_insert(HashSet::new()).insert(word);
    }
    println!("{:?}", word_length[&3]);

    Ok(())
}
