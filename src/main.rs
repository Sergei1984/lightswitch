use core_foundation::{
    array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef},
    base::CFIndexConvertible,
    boolean::{kCFBooleanTrue, CFBooleanRef},
    dictionary::{CFDictionaryCreate, CFDictionaryRef},
    string::kCFStringEncodingUTF8,
    string::{CFStringGetCStringPtr, CFStringRef},
};
use std::ffi::{c_void, CStr};

struct __TISInputSource;

#[link(name = "Carbon", kind = "framework")]
extern "C" {

    fn TISCreateInputSourceList(props: CFDictionaryRef, includeAllInstalled: bool) -> CFArrayRef;

    fn TISGetInputSourceProperty(
        inputSource: *const c_void,
        propertyKey: CFStringRef,
    ) -> *const c_void;

    fn TISSelectInputSource(input_source: *const c_void) -> i32;

    static kTISPropertyInputSourceID: CFStringRef;
    static kTISPropertyInputSourceCategory: CFStringRef;
    static kTISPropertyInputSourceType: CFStringRef;
    static kTISPropertyInputSourceIsEnabled: CFStringRef;
    static kTISPropertyInputSourceIsSelected: CFStringRef;

    static kTISTypeKeyboardLayout: CFStringRef;

}

fn main() {
    unsafe {
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

        println!("Dictionary created");

        println!("Querying languages");

        let list = TISCreateInputSourceList(filter, false);

        let cnt = CFArrayGetCount(list);
        println!("Count {}", cnt);

        let cfstr_props = vec![
            kTISPropertyInputSourceID,
            kTISPropertyInputSourceCategory,
            kTISPropertyInputSourceType,
        ];

        let cfbool_props = vec![
            kTISPropertyInputSourceIsEnabled,
            kTISPropertyInputSourceIsSelected,
        ];

        for i in 0..cnt {
            let lang = CFArrayGetValueAtIndex(list, i);

            println!("Language #{}", i);
            for p in &cfstr_props {
                let prop_void = TISGetInputSourceProperty(lang, *p);
                let prop: CFStringRef = std::mem::transmute(prop_void);

                println!("{} = {}", from_cf_string_ref(*p), from_cf_string_ref(prop));
            }

            for p in &cfbool_props {
                let prop_void = TISGetInputSourceProperty(lang, *p);
                let prop_bool: CFBooleanRef = std::mem::transmute(prop_void);

                println!(
                    "{} = {}",
                    from_cf_string_ref(*p),
                    prop_bool == kCFBooleanTrue
                );
            }
            println!("\n\n");
        }

        let lang = CFArrayGetValueAtIndex(list, 1);
        let select_result = TISSelectInputSource(lang);

        println!("Result {}", select_result);
    }
}

unsafe fn from_cf_string_ref(r: CFStringRef) -> String {
    let c_ptr = CFStringGetCStringPtr(r, kCFStringEncodingUTF8);
    let prop_val = CStr::from_ptr(c_ptr);

    String::from(prop_val.to_str().unwrap())
}
