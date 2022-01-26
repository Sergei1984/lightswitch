use cacao::macos::window::Window;
use cacao::macos::AppDelegate;
use cacao::notification_center::Dispatcher;

use crate::switch_lang::switch_lang;

#[derive(Default)]
pub struct LightswitchApp {
    window: Window,
}

impl AppDelegate for LightswitchApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(200., 100.);
        self.window
            .set_title("Lightswitch Keyboard Layout Switcher");
        self.window.show();
    }
}

impl Dispatcher for LightswitchApp {
    type Message = i32;

    fn on_ui_message(&self, message: Self::Message) {
        unsafe {
            switch_lang(message as isize);
        }
    }

    fn on_background_message(&self, message: Self::Message) {
        println!("Background Message: {}", message);
    }
}
