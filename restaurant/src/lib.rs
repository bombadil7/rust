mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {

    // relative path
    hosting::add_to_waitlist();
}

fn serve_order() {}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
