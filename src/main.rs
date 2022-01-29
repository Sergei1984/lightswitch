// mod app_window;
// mod content_view;
// mod lang_switch_monitor;
// mod switch_lang;

// use app_window::LightswitchApp;
// use cacao::{
//     macos::{
//         window::{Window, WindowConfig},
//         App,
//     },
//     view::View,
// };
// use content_view::ContentView;

// fn main() {
//     let app = App::new(
//         "com.serhii-tokariev.lightswitch",
//         LightswitchApp {
//             window: Window::new(WindowConfig::default()),
//             content: View::with(ContentView::default()),
//         },
//     );

//     app.run();
// }

mod lang_switch_monitor;
mod switch_lang;

use lang_switch_monitor::lang_switch_monitor;

fn main() {
    lang_switch_monitor();
}
