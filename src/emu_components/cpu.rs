use crate::emu_components::bus::bus_read;
use crate::emu_components::cpu_fetch::fetch_data;
use crate::emu_components::cpu_proc::inst_get_processor;
use crate::emu_components::instructions::{instruction_by_opcode, set_instructions};
use crate::emu_components::instructions::{AddrMode, CondType, InType, Instruction, RegType};

#[derive(Debug)]
pub struct CpuRegisters {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

#[derive(Debug)]
pub struct CpuContext {
    pub regs: CpuRegisters,

    // Current fetch
    pub fetched_data: u16,
    pub mem_dest: u16,
    pub dest_is_mem: bool,
    pub cur_opcode: u8,

    pub halted: bool,
    pub stepping: bool,

    pub int_master_enabled: bool,

    pub cur_inst: Instruction,
}

static mut CTX: CpuContext = CpuContext {
    regs: CpuRegisters {
        a: 1,
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
    dest_is_mem: false,
    cur_opcode: 0,
    halted: false,
    stepping: false,
    int_master_enabled: true,
    cur_inst: Instruction {
        i_type: InType::IN_NONE,
        mode: AddrMode::AM_IMP,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    },
};

pub unsafe fn cpu_get_context() -> &'static mut CpuContext {
    &mut CTX
}

pub unsafe fn cpu_init() {
    set_instructions();
    CTX.regs.pc = 0x100;
}

unsafe fn fetch_instruction() {
    CTX.cur_opcode = bus_read(CTX.regs.pc);
    CTX.regs.pc += 1;

    CTX.cur_inst = match instruction_by_opcode(CTX.cur_opcode) {
        Some(x) => x,
        _ => panic!("Unknown instruction: {:2X}", CTX.cur_opcode),
    }
}

unsafe fn execute() {
    let proc = inst_get_processor(CTX.cur_inst.i_type);
    proc(&mut CTX);
}

pub unsafe fn cpu_step() -> bool {
    let pc = CTX.regs.pc;
    if !CTX.halted {
        fetch_instruction();
        fetch_data();

        println!(
            "{:2X}: {} ({:2X}, {:2X}, {:2X}) A: {:2X} B: {:2X} C: {:2X}",
            pc,
            CTX.cur_inst.i_type,
            CTX.cur_opcode,
            bus_read(pc + 1),
            bus_read(pc + 2),
            CTX.regs.a,
            CTX.regs.b,
            CTX.regs.c
        );

        execute();
    }
    true
}
