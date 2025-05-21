use std::env;
use std::process;
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, BOOL},
        UI::WindowsAndMessaging::*,
    },
};

static mut WINDOW_MATCHED: bool = false;

unsafe fn restore_window_from_tray(hwnd: HWND) {
    let exstyle = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
    if exstyle & WS_EX_MDICHILD.0 != 0 {
        return;
    }

    let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
    let target_hwnd = if style & WS_CHILD.0 != 0 {
        GetAncestor(hwnd, GA_ROOT)
    } else {
        hwnd
    };

    ShowWindow(target_hwnd, SW_RESTORE);
    SetForegroundWindow(target_hwnd);
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));
    let target_pid = lparam.0 as u32;

    if pid == target_pid {
        restore_window_from_tray(hwnd);
        WINDOW_MATCHED = true;
    }

    true.into()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Restores and brings to front all windows belonging to the given process ID.");
        println!("Usage: blf.exe <pid>");
        process::exit(0);
    }

    let pid: u32 = match args[1].parse() {
        Ok(pid) => pid,
        Err(_) => {
            eprintln!("Invalid PID: '{}'", args[1]);
            process::exit(1);
        }
    };

    unsafe {
        EnumWindows(Some(enum_windows_proc), LPARAM(pid as isize));

        if WINDOW_MATCHED {
            println!("Successfully restored and focused windows for PID {}.", pid);
            process::exit(0);
        } else {
            println!("No windows found for PID {}.", pid);
            process::exit(1);
        }
    }
}
