# Background Launcher

A set of small Windows tools to launch and manage apps in background/foreground.

This is a **very minimal toolset** for managing Windows processes. It only provides functionality to:
- Launch an application into the **background**.
- Restore a backgrounded process to the **foreground**.

It **does not** offer any advanced process management.  
### ⚠️ Warning:  
Once a process is hidden in the background, you'll need to use **Task Manager** or `tasklist` to find and terminate it — unless you bring it back to the foreground first and close it normally.

## Included Tools

- `bl.exe`  – Launch a script or terminal app in the background.
- `blg.exe` – Launch and minimize a GUI app to background.
- `blb.exe` – Hide a process window to the background tray (by PID).
- `blf.exe` – Restore a previously backgrounded window (by PID).
- `blh.exe` – Print detailed help and usage info for all tools.

## Building

To build all the utilities, you need to have [Rust](https://rustup.rs/) installed with the latest stable toolchain.

Run the following command in the root directory of the project:

```
cargo build --release
```

This will compile all the binaries and place the executables in:

```
target/release/
```

## Usage

Examples:
```
target/release/bl cmd.exe
target/release/blg notepad.exe
target/release/blb 1234
target/release/blf 1234
```

For more detailed help and descriptions of all utilities, run:

```
target/release/blh
```
