// mod app_window;
// mod content_view;
// mod keypress_watcher;
// mod global_keyboard_hook;
mod switch_lang;

// use global_keyboard_hook::start_global_keyboard_hook;

use crate::switch_lang::switch_lang;
// use keypress_watcher::start_keypress_watching;

use std::{
    io::Read,
    sync::{Arc, Mutex},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use rdev::{grab, Event, EventType, Key};

fn main() {
    thread::spawn(|| {
        let mut last_key_press_timestamp_arc = Arc::new(Mutex::new(get_epoch_ms()));
        let mut lang_index_arc = Arc::new(Mutex::new(0));

        let callback = move |event: Event| -> Option<Event> {
            match event.event_type {
                EventType::KeyPress(Key::Function) => {
                    let now = get_epoch_ms();

                    let mut last_key_press_timestamp = last_key_press_timestamp_arc.lock().unwrap();
                    let mut lang_index = lang_index_arc.lock().unwrap();

                    println!(
                        "Last press = {}, now = {}, diff = {}",
                        last_key_press_timestamp,
                        now,
                        now - *last_key_press_timestamp
                    );

                    if (now - *last_key_press_timestamp) < 300 {
                        *lang_index += 1;
                        println!("Increasing lanugage index to {}", lang_index);
                    } else {
                        *lang_index = 0;

                        println!(
                            "Resetting timer: last key press = {}, lang index = {}",
                            last_key_press_timestamp, lang_index
                        );
                    }

                    *last_key_press_timestamp = get_epoch_ms();
                    unsafe {
                        switch_lang(*lang_index);
                    }
                }
                EventType::KeyPress(Key::CapsLock) => {
                    println!("CapsLock pressed");
                }
                _ => {}
            }

            Some(event)
        };
        // This will block.
        if let Err(error) = grab(callback) {
            println!("Error: {:?}", error)
        }
    });

    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
