use cacao::macos::window::Window;
use cacao::macos::{App, AppDelegate};

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
