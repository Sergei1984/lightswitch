use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef};
use std::ffi::c_void;

struct __TISInputSource;

#[link(name = "Carbon", kind = "framework")]
extern "C" {

    fn TISCreateInputSourceList(props: Option<CFArrayRef>, includeAllInstalled: bool)
        -> CFArrayRef;

    fn TISSelectInputSource(input_source: *const c_void) -> i32;

}

fn main() {
    println!("Hello, world!");

    unsafe {
        let list = TISCreateInputSourceList(None, false);


        let cnt = CFArrayGetCount(list);
        println!("Count {}", cnt);

        let lang = CFArrayGetValueAtIndex(list, 0);
        let select_result = TISSelectInputSource(lang);

        println!("Result {}", select_result);
    }
}
