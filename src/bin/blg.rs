use std::{env, thread::sleep, time::Duration};
use std::process::{Command, Stdio};

use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, BOOL},
        UI::WindowsAndMessaging::*,
    },
};

unsafe fn minimize_window_to_tray(hwnd: HWND) {
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

    ShowWindow(target_hwnd, SW_MINIMIZE);
    ShowWindow(target_hwnd, SW_HIDE);
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));
    let target_pid = lparam.0 as u32;

    if pid == target_pid {
        minimize_window_to_tray(hwnd);
    }

    true.into()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Launches a GUI program and minimizes its window to the tray.\nUsage: blg.exe <program.exe> [args...]");
        std::process::exit(0);
    }

    let program = &args[1];
    let program_args = &args[2..];

    let child = Command::new(program)
        .args(program_args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    let child = match child {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to spawn process \"{}\": {}", program, e);
            std::process::exit(1);
        }
    };

    let pid = child.id();

    sleep(Duration::from_millis(500));

    unsafe {
        EnumWindows(Some(enum_windows_proc), LPARAM(pid as isize));
    }

    println!("Launched \"{}\" in background with PID {}.", program, pid);
    std::process::exit(0);
}
