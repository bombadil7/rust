use std::env::args;

fn main() {
    //let mut args = args();
    //    println!("{:?}", args.next());
    //    println!("{:?}", args.next());
    //    println!("{:?}", args.next());
    //    println!("{:?}", args.next());

    //    loop {
    //        match args.next() {
    //            Some(a) => println!("{:?}", a),
    //            None => break,
    //        }
    //    }

    //    loop {
    //        if let Some(arg) = args.next() {
    //            println!("{}", arg);
    //        } else {
    //            break;
    //        }
    //    }

    //    while let Some(arg) = args.next() {
    //        println!("{:}", arg);
    //    }
    for arg in args().skip(1) {
        println!("{}", arg);
    }
}
