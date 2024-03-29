use std::ffi::c_void;

use cacao::core_foundation::{dictionary::{CFDictionaryCreate, CFDictionaryRef}, array::{CFArrayRef, CFArrayGetCount, CFArrayGetValueAtIndex}, string::CFStringRef, boolean::kCFBooleanTrue, base::CFIndexConvertible};


#[link(name = "Carbon", kind = "framework")]
extern "C" {

    fn TISCreateInputSourceList(props: CFDictionaryRef, includeAllInstalled: bool) -> CFArrayRef;

    fn TISSelectInputSource(input_source: *const c_void) -> i32;

    static kTISPropertyInputSourceType: CFStringRef;
    static kTISPropertyInputSourceIsEnabled: CFStringRef;
    static kTISTypeKeyboardLayout: CFStringRef;

}

pub unsafe fn switch_lang(language_index: isize) {
    let keys: Vec<*const c_void> = vec![
        std::mem::transmute(kTISPropertyInputSourceIsEnabled),
        std::mem::transmute(kTISPropertyInputSourceType),
    ];

    let values: Vec<*const c_void> = vec![
        std::mem::transmute(kCFBooleanTrue),
        std::mem::transmute(kTISTypeKeyboardLayout),
    ];

    let filter = CFDictionaryCreate(
        std::ptr::null(),
        keys.as_ptr(),
        values.as_ptr(),
        keys.len().to_CFIndex(),
        std::ptr::null(),
        std::ptr::null(),
    );

    let list = TISCreateInputSourceList(filter, false);

    let cnt = CFArrayGetCount(list);

    let lang = CFArrayGetValueAtIndex(list, language_index.clamp(0, cnt - 1));
    let select_result = TISSelectInputSource(lang);

    println!(
        "Setting language at {}, result {}",
        language_index.clamp(0, cnt - 1),
        select_result
    );
}
