use crate::emu_components::cart::cart_load;
use crate::emu_components::cpu::{cpu_init, cpu_step};
use sdl2;
use sdl2_sys::SDL_Delay;

#[derive(Debug)]
pub struct EmuContext {
    paused: bool,
    running: bool,
    ticks: u64,
}

/*
  Emu components:

  |Cart|
  |CPU|
  |Address Bus|
  |PPU|
  |Timer|

*/

static mut CTX: EmuContext = EmuContext {
    paused: false,
    running: false,
    ticks: 0,
};

pub unsafe fn emu_get_context() -> &'static mut EmuContext {
    &mut CTX
}

unsafe fn delay(ms: u32) {
    SDL_Delay(ms);
}

pub unsafe fn emu_run(args: Vec<String>) {
    if args.len() < 2 {
        println!("Usage: emu <rom_file>");
        return;
    }

    let rom_file = &args[1];
    cart_load(rom_file.to_owned()).expect("Failed to load cart");

    println!("Cart loaded..");

    let _ = sdl2::init().expect("Expecting SDL2 to work");
    println!("SDL INIT");
    let _ = sdl2::ttf::init().expect("Expecting SDL2 ttf to work");
    println!("TTF INIT");

    cpu_init();

    CTX.running = true;
    CTX.paused = false;
    CTX.ticks = 0;

    while CTX.running {
        if CTX.paused {
            delay(10);
            continue;
        }

        if !cpu_step() {
            println!("CPU Stopped");
            return;
        }

        CTX.ticks += 1;
    }
}

pub fn emu_cycles(cpu_cycles: i32) {
    // todo
}
