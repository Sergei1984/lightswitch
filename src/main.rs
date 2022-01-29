mod app_window;
mod content_view;
mod lang_switch_monitor;
mod switch_lang;

use lang_switch_monitor::lang_switch_monitor;
use std::{io::Read, thread};

fn main() {
    thread::spawn(|| {
        lang_switch_monitor();
    });

    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
