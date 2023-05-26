use crate::emu_components::cpu::{cpu_init, cpu_step};
use sdl2;

pub struct EmuContext {
    paused: bool,
    running: bool,
    ticks: u64,
}

static mut CTX: EmuContext = EmuContext {
    paused: false,
    running: false,
    ticks: 0,
};

pub fn emu_get_context() -> &'static mut EmuContext {
    unsafe { &mut CTX }
}

fn delay(ms: u32) {
    todo!();
}

pub fn emu_run(args: Vec<String>) {
    if args.len() < 2 {
        panic!("Usage: emu <rom_file>");
    }

    let rom_file = &args[1];
    if rom_file.len() == 0 {
        panic!("Failed to load ROM file: {}", rom_file);
    }

    println!("Cart Loaded..");

    sdl2::init().expect("Expecting SDL2 to work");
    println!("SDL INIT");
    sdl2::ttf::init().expect("Expecting SDL2 ttf to work");
    println!("TTF INIT");

    cpu_init();

    unsafe {
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
}
