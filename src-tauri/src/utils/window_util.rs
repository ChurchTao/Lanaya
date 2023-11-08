#[cfg(target_os = "macos")]
use active_win_pos_rs::get_active_window;

#[cfg(target_os = "macos")]
use cocoa::base::{nil};

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSRunningApplication, NSApplicationActivateIgnoringOtherApps};

#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetForegroundWindow, SetForegroundWindow};

pub fn get_active_process_id() -> i32 {
    #[cfg(target_os = "macos")]
    {
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

    #[cfg(target_os = "windows")]
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            println!("Could not get active window");
            return 0;
        }

        let mut process_id: u32 = 0;

        GetWindowThreadProcessId(hwnd, &mut process_id as LPDWORD);

        if process_id == 0 {
            println!("Could not get process id of active window");
            return 0;
        }

        process_id as i32
    }
}

pub fn focus_window(process_id: i32) {
    if process_id == 0 {
        return;
    }
    #[cfg(target_os = "macos")]
    unsafe {
        let current_app = NSRunningApplication::runningApplicationWithProcessIdentifier(nil, process_id);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }

    #[cfg(target_os = "windows")]
    unsafe {
        let hwnd = GetForegroundWindow();

        if hwnd.is_null() {
            println!("Could not get active window");
            return;
        }

        if SetForegroundWindow(hwnd) == 0 {
            println!("Could not focus on window");
        }
    }
} 