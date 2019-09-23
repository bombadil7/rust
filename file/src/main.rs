use std::fs::File;

fn main() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            match std::env::current_dir() {
                Ok(dir) =>  panic!("There was a problem opening the file: {:?}. CWD: {:?}", error, dir),
                Err(er) => panic!("Problem reading file: {:?}.  Can't even read current directory!!!{:?}", error, er),
            }
            
        },
    };
}
