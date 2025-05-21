use std::env;
use std::process::Command;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    let mut args = env::args_os();
    args.next(); // skip program name

    let program = match args.next() {
        Some(p) => p,
        None => {
            println!("error");
            std::process::exit(1);
        }
    };

    let child = Command::new(&program)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();

    match child {
        Ok(child_proc) => {
            println!("{}", child_proc.id());
            std::process::exit(0);
        }
        Err(_) => {
            println!("error");
            std::process::exit(1);
        }
    }
}
