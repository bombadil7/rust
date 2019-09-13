const MAX_NUM: u32 = 1 << 16;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", MAX_NUM);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let y = "324";
    let y: u32 = y.trim().parse().expect("Not a number");
    let y = y + 1;
    println!("The value of x is: {}", y);

    another_function();

    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    if x == 5 {
        println!("The number is indeed 5");
    }
    else {
        println!("The number is no 5 at all");
    }
}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    5
}
