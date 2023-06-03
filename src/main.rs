mod emu_components;

use crate::emu_components::emu::emu_run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    unsafe {
        emu_run(args);
    }
}
