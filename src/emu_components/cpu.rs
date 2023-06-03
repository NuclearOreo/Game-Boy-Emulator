use crate::emu_components::instructions::set_instuctions;
use crate::emu_components::instructions::{AddrMode, CondType, InType, Instruction, RegType};

#[derive(Debug)]
struct cpu_registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

#[derive(Debug)]
struct cpu_context {
    regs: cpu_registers,

    // Current fetch
    fetched_data: u16,
    mem_dest: u16,
    cur_opcode: u8,

    halted: bool,
    stepping: bool,

    cur_inst: Instruction,
}

static mut CTX: cpu_context = cpu_context {
    regs: cpu_registers {
        a: 0,
        f: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        pc: 0,
        sp: 0,
    },
    fetched_data: 0,
    mem_dest: 0,
    cur_opcode: 0,
    halted: false,
    stepping: false,
    cur_inst: Instruction {
        i_type: InType::IN_NONE,
        mode: AddrMode::AM_IMP,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    },
};

pub fn cpu_init() {
    set_instuctions();
}

fn fetch_instruction() {
    todo!()
}

fn fetch_data() {
    todo!()
}

fn execute() {
    todo!()
}

pub fn cpu_step() -> bool {
    unsafe {
        if !CTX.halted {
            fetch_instruction();
            fetch_data();
            execute();
        }
    }
    false
}
