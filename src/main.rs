mod app_window;
// mod keypress_listener;
mod switch_lang;

use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use app_window::LightswitchApp;
use cacao::macos::App;
use readkey::Keycode;
use switch_lang::switch_lang;

fn main() {
    let app = App::new("com.serhii-tokariev.lightswitch", LightswitchApp::default());
    thread::spawn(|| {
        let mut last_key_press_timestamp = get_epoch_ms();
        let mut lang_index = 0;

        let mut is_pressed = false;

        loop {
            if Keycode::Function.is_pressed() || Keycode::CapsLock.is_pressed() {
                if !is_pressed {
                    println!("--- pressed");

                    is_pressed = true;

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
                    // unsafe {
                    //     switch_lang(lang_index);
                    // }

                    App::<LightswitchApp, i32>::dispatch_main(lang_index);

                    // println!("Switching to {}", lang_index);
                }
            } else {
                is_pressed = false;
            }

            thread::sleep(Duration::from_millis(30));
        }
    });

    app.run();
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
