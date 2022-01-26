mod app_window;
mod keypress_watcher;
mod switch_lang;

use app_window::LightswitchApp;
use cacao::macos::App;
use keypress_watcher::start_keypress_watching;

fn main() {
    let app = App::new("com.serhii-tokariev.lightswitch", LightswitchApp::default());

    start_keypress_watching(|lang_index| {
        App::<LightswitchApp, i32>::dispatch_main(lang_index);
    });

    app.run();
}
