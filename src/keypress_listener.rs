use std::time::{SystemTime, UNIX_EPOCH};

use core_foundation::runloop::{kCFRunLoopDefaultMode, CFRunLoop};
use core_graphics::event::{
    CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType, KeyCode,
};

use crate::switch_lang::*;

struct KeyPressInfo {
    pub last_key_press_timestamp: u128,
    pub lang_index: isize,
}

unsafe impl Send for KeyPressInfo {}

static mut INFO: KeyPressInfo = KeyPressInfo {
    last_key_press_timestamp: 0,
    lang_index: 0,
};

pub unsafe fn start_switching_kbd_on_keypress() {
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

                if (now - INFO.last_key_press_timestamp) < 300 {
                    INFO.lang_index += 1;
                    println!("Increasing lanugage index to {}", INFO.lang_index);
                } else {
                    INFO.lang_index = 0;

                    println!(
                        "Resetting timer: last key press = {}, lang index = {}",
                        INFO.last_key_press_timestamp, INFO.lang_index
                    );
                }

                INFO.last_key_press_timestamp = get_epoch_ms();

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

            let current = CFRunLoop::get_current();

            current.add_source(&loop_source, kCFRunLoopDefaultMode);
            tap.enable();
            CFRunLoop::run_current();
        }
        Err(_) => {
            println!("Error create event listener");
        }
    }
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
