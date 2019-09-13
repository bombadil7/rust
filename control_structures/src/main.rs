use std::io::stdin;

fn main() {
    println!("Enter a number between 1 and 10 ");
    let mut num = String::new();
    stdin().read_line(&mut num).expect("Failed to read line");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("You entered {}", num);

    if num < 5 && num > 0{
        println!("Number is less than 5");
    }
    else if num < 10 && num > 0 {
        println!("Number is greater than 5");
    }
    else {
        println!("Number is out of range 1 - 10");
    }

}
