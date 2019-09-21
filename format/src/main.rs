struct Person {
    name: String,
    age: u32,
}

impl std::fmt::Display for Person {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} ({} years old)", self.name, self.age)
    }
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{}", alice);

    let val: String = String::from("Hello, World!");
    printer(&val);
    printer(&val);

    printer2(val.clone());
    printer2(val);

    // looping
    let mut i = 1;

    loop {
        println!("i = {}", i);
        if i >= 10 {
            break;
        } else {
            i += 1;
        }
    }

    let mut i = 1;
    while i <= 10 {
        println!("i = {}", i);
        i += 1;
    }

    for i in 1..11 {
        println!("i = {}", i);
    }
}

fn printer(val: &String) {
    println!("{}", val);
}

fn printer2(val: String) {
    println!("{}", val);
}
