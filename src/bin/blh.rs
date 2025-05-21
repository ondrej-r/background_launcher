fn main() {
    println!(
"Background Launcher Toolset Help
================================

This toolset provides utilities for launching, hiding, and managing background 
processes on Windows. Each tool is named with a short prefix:

    bl   - Background Launch (CLI-only)
    blm  - Background Launch with PID output (for scripting)
    blg  - Background Launch graphical applications
    blb  - Bring existing window to the background
    blf  - Bring existing window to the foreground

USAGE
-----

1. bl.exe <program.exe> [args...]
   --------------------------------
   Launches a command-line program in the background (no window).
   Ideal for CLI tools or daemons.
   Example:
       bl.exe my_script.bat

2. blm.exe <program.exe> [args...]
   --------------------------------
   Launches a CLI program in the background like 'bl', but olny prints the PID 
   of the process on success. Prints 'error' on failure. Useful for scripting 
   and monitoring.
   Example:
       blm.exe my_script.bat

3. blg.exe <program.exe> [args...]
   --------------------------------
   Launches a GUI program and immediately hides its window.
   The window is minimized and hidden from view.
   Example:
       blg.exe notepad.exe

4. blb.exe <PID>
   ----------------
   Hides or minimizes the window of a running process by its PID.
   Example:
       blb.exe 1234

5. blf.exe <PID>
   ----------------
   Brings the window of a backgrounded process to the foreground.
   Makes it visible and focused.
   Example:
       blf.exe 1234


To get the PID of a running process, use Task Manager (Ctrl+Shift+Esc) or:
    tasklist | findstr <program name>

"
    );
}
