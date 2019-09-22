use std::os::raw;
use std::ffi;

extern "C" {
    fn reverse(const *c_char) -> const *c_char;
}

fn stuff(value: &str) {
    println!("{:?}",
        unsafe { reverse(CStr::from(value).unwrap()) }
    );
}