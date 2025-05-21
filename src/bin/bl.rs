use std::env;
use std::process::Command;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    let mut args = env::args_os();
    args.next();

    let program = match args.next() {
        Some(program) => program,
        None => {
            eprintln!("Minimal background launcher for CLI programs.");
            eprintln!("Usage: bl.exe <program.exe> [arg1 arg2 ...]");
            std::process::exit(0);
        }
    };

    let child = Command::new(&program)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();

    match child {
        Ok(child_proc) => {
            println!("Launched {:?} in background with PID {}.", program, child_proc.id());
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to start process {:?}: {}", program, e);
            std::process::exit(1);
        }
    };
}
