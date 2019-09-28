#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
}

fn parse_args() -> Result<Frame, ParseError> {
    use self::ParseError::*;

    let mut args = std::env::args().skip(1);

    match args.next() {
        None => Err(TooFewArgs),
        Some(width_str) => {
            match args.next() {
                None => Err(TooFewArgs),
                Some(height_str) => {
                    match args.next() {
                        Some(_) => Err(TooManyArgs),
                        None => {
                            match width_str.parse() {
                                Err(_) => Err(InvalidInteger(width_str)),
                                Ok(width) => {
                                    match height_str.parse() {
                                        Err(_) => Err(InvalidInteger(height_str)),
                                        Ok(height) => Ok(Frame {
                                            width,
                                            height,
                                        }),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    println!("{:?}", parse_args());
}
