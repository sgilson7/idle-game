// Desktop entry point; the phone starts from `idlewright_lib::run` too.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    idlewright_lib::run()
}
