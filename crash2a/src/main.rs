#[derive(Debug, Clone, Copy)]
struct Foobar(i32);

fn uses_foobar(foobar: Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

#[derive(Debug)]
struct Foo(i32);

fn double(foo: Foo) -> Foo {    
    Foo(foo.0 * 2)
}

fn main() {
    let x = Foobar(1);
    uses_foobar(x);
    uses_foobar(x);

    let y: Foo = Foo(1);
    let z: Foo = double(y);
    println!("{}", z.0);
}
