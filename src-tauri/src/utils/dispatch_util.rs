use rdev::{simulate, EventType, Key, SimulateError};
use std::{thread, time};

pub fn paste() {
    // Run command + v to paste
    // Same approach as both Maccy and Clipy, reference: https://github.com/p0deje/Maccy/blob/master/Maccy/Clipboard.swift#L101
    dispatch(&EventType::KeyPress(Key::MetaLeft));
    dispatch(&EventType::KeyPress(Key::KeyV));
    dispatch(&EventType::KeyRelease(Key::KeyV));
    dispatch(&EventType::KeyRelease(Key::MetaLeft));
}

pub fn request_permissions() {
    // Simply press and release the shift key. First time the OS will ask for permissions, then do it without asking.
    dispatch(&EventType::KeyPress(Key::ShiftLeft));
    dispatch(&EventType::KeyRelease(Key::ShiftLeft));
}

fn dispatch(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not dispatch {:?}", event_type);
        }
    }
    // Let the OS catchup (at least MacOS)
    sleep(20)
}

fn sleep(ms: u64) {
    let delay = time::Duration::from_millis(ms);
    thread::sleep(delay);
}