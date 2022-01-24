use cacao::core_graphics::event::{
    CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType, KeyCode,
};
use core_foundation::{
    array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef},
    base::CFIndexConvertible,
    boolean::kCFBooleanTrue,
    dictionary::{CFDictionaryCreate, CFDictionaryRef},
    runloop::{kCFRunLoopDefaultMode, CFRunLoop},
    string::CFStringRef,
};
use std::{
    ffi::c_void,
    time::{SystemTime, UNIX_EPOCH},
};

struct __TISInputSource;

#[link(name = "Carbon", kind = "framework")]
extern "C" {

    fn TISCreateInputSourceList(props: CFDictionaryRef, includeAllInstalled: bool) -> CFArrayRef;

    fn TISSelectInputSource(input_source: *const c_void) -> i32;

    static kTISPropertyInputSourceType: CFStringRef;
    static kTISPropertyInputSourceIsEnabled: CFStringRef;
    static kTISTypeKeyboardLayout: CFStringRef;

}

struct KeyPressInfo {
    pub last_key_press_timestamp: u128,
    pub lang_index: isize,
}

unsafe impl Send for KeyPressInfo {}

static mut INFO: KeyPressInfo = KeyPressInfo {
    last_key_press_timestamp: 0,
    lang_index: 0,
};

fn main() {
    unsafe {
        let current = CFRunLoop::get_current();

        let tap_result = CGEventTap::new(
            CGEventTapLocation::HID,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::ListenOnly,
            vec![CGEventType::FlagsChanged],
            move |_a, _b, d| {
                let key_code = d.get_integer_value_field(9);
                let flags = d.get_flags();

                if key_code == KeyCode::FUNCTION.into() && flags.bits() != 256 {
                    println!(
                        "Caps Lock pressed, flags = {}, key_code = {}",
                        flags.bits(),
                        key_code
                    );
                    let now = get_epoch_ms();

                    println!(
                        "Last press = {}, now = {}, diff = {}",
                        INFO.last_key_press_timestamp,
                        now,
                        now - INFO.last_key_press_timestamp
                    );

                    if (now - INFO.last_key_press_timestamp) < 600 {
                        INFO.lang_index += 1;
                        println!("Increasing lanugage index to {}", INFO.lang_index);
                    } else {
                        INFO.lang_index = 0;
                        INFO.last_key_press_timestamp = get_epoch_ms();

                        println!(
                            "Resetting timer: last key press = {}, lang index = {}",
                            INFO.last_key_press_timestamp, INFO.lang_index
                        );
                    }

                    switch_lang(INFO.lang_index);
                }

                None
            },
        );

        match tap_result {
            Ok(tap) => {
                let loop_source = tap
                    .mach_port
                    .create_runloop_source(0)
                    .expect("Somethings is bad ");

                current.add_source(&loop_source, kCFRunLoopDefaultMode);
                tap.enable();
                CFRunLoop::run_current();
            }
            Err(_) => {
                println!("Error create event listener");
            }
        }
    }
}

unsafe fn switch_lang(language_index: isize) {
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

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
