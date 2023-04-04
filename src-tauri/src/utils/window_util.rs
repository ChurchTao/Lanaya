use active_win_pos_rs::get_active_window;
use cocoa::base::{nil};
use cocoa::appkit::{NSRunningApplication, NSApplicationActivateIgnoringOtherApps};

pub fn get_active_process_id() -> i32 {
    match get_active_window() {
        Ok(active_window) => {
            let process_id: i32 = active_window.process_id.try_into().unwrap();
            process_id
        },
        Err(()) => {
            println!("error occurred while getting the active window");
            0
        }
    }
}

pub fn focus_window(process_id: i32) {
    if process_id == 0 {
        return;
    }
    unsafe {
        let current_app = NSRunningApplication::runningApplicationWithProcessIdentifier(nil, process_id);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }
}