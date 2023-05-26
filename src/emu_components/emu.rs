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

pub fn emu_run() -> i32 {
    0
}
