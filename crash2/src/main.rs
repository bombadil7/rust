#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        println!("Dropping a Foobar: {:?}", self);
    }
}

impl Foobar {
    fn use_it(self) {
        println!("I consumed a Foobar");
    }
}
fn uses_foobar(foobar: &Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

fn main() {
    let x = Foobar(1);
    println!("Before uses_foobar");
    uses_foobar(&x);
    println!("After uses_foobar");
    x.use_it();
}
