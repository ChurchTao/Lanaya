use rdev::{simulate, EventType, Key, SimulateError};
use std::{thread, time};

pub fn paste_in_previous_window() {
    focus_previous_window();
    sleep(100);
    paste();
}

pub fn focus_previous_window() {
    // Ideally there's some native function to return focus to previous window,
    // haven't found it yet though.

    // First run command + tab
    dispatch(&EventType::KeyPress(Key::MetaLeft));
    dispatch(&EventType::KeyPress(Key::Tab));
    dispatch(&EventType::KeyRelease(Key::Tab));
    sleep(100);
    // While pressing command key, also run shift + command + tab
    dispatch(&EventType::KeyPress(Key::ShiftLeft));
    dispatch(&EventType::KeyPress(Key::Tab));
    dispatch(&EventType::KeyRelease(Key::Tab));
    dispatch(&EventType::KeyRelease(Key::ShiftLeft));
    dispatch(&EventType::KeyRelease(Key::MetaLeft));
}

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
    sleep(40)
}

fn sleep(ms: u64) {
    let delay = time::Duration::from_millis(ms);
    thread::sleep(delay);
}