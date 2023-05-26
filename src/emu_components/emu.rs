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

pub fn emu_run(args: Vec<String>) -> i32 {
    if args.len() < 2 {
        println!("Usage: emu <rom_file>");
        return -1;
    }

    let rom_file = &args[1];
    if rom_file.len() > 0 {
        println!("Failed to load ROM file: {}", rom_file);
        return -2;
    }

    0
}
