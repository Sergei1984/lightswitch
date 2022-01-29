mod app_window;
mod content_view;
// mod keypress_watcher;
mod global_keyboard_hook;
mod switch_lang;

use global_keyboard_hook::start_global_keyboard_hook;
// use keypress_watcher::start_keypress_watching;

fn main() {
    unsafe {
        start_global_keyboard_hook(|| {
            println!("Key pressed");
        });
    }
}
