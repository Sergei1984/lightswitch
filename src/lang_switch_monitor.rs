use rdev::{listen, Event};

pub fn lang_switch_monitor() {
    let callback = move |_event: Event| {};

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
