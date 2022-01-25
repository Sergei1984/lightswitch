mod keypress_listener;
mod switch_lang;

use keypress_listener::*;

fn main() {
    unsafe {
        start_switching_kbd_on_keypress();
    }
}
