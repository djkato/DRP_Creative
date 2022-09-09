use crate::app::{App, Apps};
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    UI::WindowsAndMessaging::{self, GetWindowTextA, GetWindowTextLengthA, WNDENUMPROC},
};
pub fn get_running_program(apps: &Apps) -> Option<(&App, String)> {
    let running_window_names = unsafe { get_running_windows_titles() };
    for window_name in running_window_names {
        //dbg!(&window_name);
        if let Some(app) = apps.find_app(&window_name) {
            if !window_name.contains("- Google Chrome") {
                //So googling it won't affect the DRP lol
                return Some((&app, app.parse(&window_name)));
            }
        }
    }
    return None;
}
pub fn is_program_still_running(app: &App) -> Option<String> {
    let running_window_names = unsafe { get_running_windows_titles() };

    for window_name in running_window_names {
        if window_name.contains(&app.process_search_string) {
            if !window_name.contains("- Google Chrome") {
                //So googling it won't affect the DRP lol
                return Some(app.parse(&window_name));
            }
        }
    }
    return None;
}

pub unsafe fn get_running_windows_titles() -> Vec<String> {
    let mut running_windows_names: Vec<String> = Vec::new();
    unsafe extern "system" fn processhwd(hwnd: HWND, lparam: LPARAM) -> BOOL {
        // Get the length of the window text
        let window_text_len = GetWindowTextLengthA(hwnd);
        if window_text_len < 0 {
            panic!("Uh oh, it went wrong.");
        }

        // Make a buffer for the window text (+ 1 for terminating NUL byte)
        let mut window_text_buffer = vec![0_u8; window_text_len as usize + 1];

        // Get the window text. For understanding how to deal with the value in
        // `result`, see the MSDN documentation.
        GetWindowTextA(hwnd, &mut window_text_buffer);

        //turn ascii into characters
        let mut window_text: Vec<char> = Vec::new();
        for char in window_text_buffer {
            window_text.push(char as char);
        }
        //turn characters into strings, but exclude last weird escape null char "\0"
        window_text.pop();
        let window_text = String::from_iter(window_text.iter());

        //turn vector into pointer and push window_texts to it
        let running_windows_names_pointer = lparam.0 as *mut Vec<String>;

        running_windows_names_pointer
            .as_mut()
            .expect("welp the pointer failed...")
            .push(window_text);

        BOOL(1)
    }
    let lpenumfunc: WNDENUMPROC =
        Some(processhwd as unsafe extern "system" fn(hwnd: HWND, lparam: LPARAM) -> BOOL);

    let windows_names_ptr: *mut Vec<String> = &mut running_windows_names;

    WindowsAndMessaging::EnumWindows(lpenumfunc, LPARAM(windows_names_ptr as isize));

    running_windows_names
}
