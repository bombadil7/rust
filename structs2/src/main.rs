use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// methods
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!(
            "Width {}, height {}, area {}",
            self.width,
            self.height,
            self.area()
        );
    }
}

// Related functions
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) and Area: {}", 
            self.width, self.height, self.area())
    }
}

fn main() {
    let o = Object {
        width: 35,
        height: 55,
    };

    let obj = Object::new(12, 5);
    o.show();
    obj.show();

    println!("{:#?}", o);
    println!("{:#?}", obj);

    println!("{} {}", o, obj);
}
