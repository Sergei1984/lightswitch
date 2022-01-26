use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use readkey::Keycode;

pub fn start_keypress_watching<THandler>(on_lang_switching: THandler)
where
    THandler: Fn(i32) + Send + 'static,
{
    thread::spawn(move || {
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

                    // App::<LightswitchApp, i32>::dispatch_main(lang_index);
                    on_lang_switching(lang_index);

                    // println!("Switching to {}", lang_index);
                }
            } else {
                is_pressed = false;
            }

            thread::sleep(Duration::from_millis(30));
        }
    });
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
