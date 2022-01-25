mod app_window;
// mod keypress_listener;
mod switch_lang;

use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use readkey::Keycode;
use switch_lang::switch_lang;

fn main() {
    let mut last_key_press_timestamp = get_epoch_ms();
    let mut lang_index = 0;

    loop {
        if Keycode::Function.is_pressed() {
            let now = get_epoch_ms();

            println!(
                "Last press = {}, now = {}, diff = {}",
                last_key_press_timestamp,
                now,
                now - last_key_press_timestamp
            );

            if (now - last_key_press_timestamp) < 300 {
                lang_index += 1;
                println!("Increasing lanugage index to {}", lang_index);
            } else {
                lang_index = 0;

                println!(
                    "Resetting timer: last key press = {}, lang index = {}",
                    last_key_press_timestamp, lang_index
                );
            }

            last_key_press_timestamp = get_epoch_ms();
            unsafe {
                switch_lang(lang_index);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
