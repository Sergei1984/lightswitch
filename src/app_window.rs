use cacao::macos::menu::{Menu, MenuItem};
use cacao::macos::window::Window;
use cacao::macos::{App, AppDelegate};
use cacao::notification_center::Dispatcher;

use crate::switch_lang::switch_lang;

#[derive(Default)]
pub struct LightswitchApp {
    window: Window,
}

impl AppDelegate for LightswitchApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![Menu::new("", vec![MenuItem::Quit])]);

        App::activate();

        let width = 400.;
        let height = 200.;

        self.window.set_content_size(width, height);
        self.window.set_minimum_content_size(width, height);
        self.window.set_maximum_content_size(width, height);

        self.window
            .set_title("Lightswitch Keyboard Layout Switcher");


        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
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