mod app_window;
mod content_view;
// mod keypress_watcher;
mod switch_lang;
mod global_keyboard_hook;

use app_window::LightswitchApp;
use cacao::{
    macos::{
        window::{Window, WindowConfig},
        App,
    },
    view::View,
};
use content_view::ContentView;
// use keypress_watcher::start_keypress_watching;

fn main() {
    let app = App::new(
        "com.serhii-tokariev.lightswitch",
        LightswitchApp {
            window: Window::new(WindowConfig::default()),
            content: View::with(ContentView::default()),
        },
    );

    // start_keypress_watching(|lang_index| {
    //     App::<LightswitchApp, i32>::dispatch_main(lang_index);
    // });

    app.run();
}
