use cacao::{
    core_foundation::runloop::{kCFRunLoopDefaultMode, CFRunLoop},
    core_graphics::event::{
        CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType,
        KeyCode,
    },
};

pub unsafe fn start_global_keyboard_hook<TCallback>(switch_lang_key_press: TCallback)
where
    TCallback: Fn() + Send + 'static,
{
    let tap_result = CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::ListenOnly,
        vec![CGEventType::FlagsChanged],
        |_a, _b, d| {
            let key_code = d.get_integer_value_field(9);
            let flags = d.get_flags();

            println!("Key pressed: code = {}, flags = {}", key_code, flags.bits());

            if key_code == KeyCode::FUNCTION.into() && flags.bits() != 256 {
                println!(
                    "Caps Lock pressed, flags = {}, key_code = {}",
                    flags.bits(),
                    key_code
                );

                switch_lang_key_press();
            }

            None
        },
    );

    match tap_result {
        Ok(tap) => {
            let run_loop = CFRunLoop::get_current();
            let loop_source = tap
                .mach_port
                .create_runloop_source(0)
                .expect("Somethings is bad ");

            run_loop.add_source(&loop_source, kCFRunLoopDefaultMode);
            tap.enable();

            CFRunLoop::run_current();
        }
        Err(_) => {
            println!("Error creating tap source");
        }
    }
}
