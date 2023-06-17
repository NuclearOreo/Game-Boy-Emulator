use crate::emu_components::bus::bus_read;
use crate::emu_components::common::bit;
use crate::emu_components::cpu_proc::inst_get_processor;
use crate::emu_components::emu::emu_cycles;
use crate::emu_components::instructions::{instruction_by_opcode, set_instuctions};
use crate::emu_components::instructions::{AddrMode, CondType, InType, Instruction, RegType};

#[derive(Debug)]
pub struct cpu_registers {
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
pub struct cpu_context {
    pub regs: cpu_registers,

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

static mut CTX: cpu_context = cpu_context {
    regs: cpu_registers {
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

pub unsafe fn cpu_init() {
    set_instuctions();
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

unsafe fn fetch_data() {
    CTX.mem_dest = 0;
    CTX.dest_is_mem = false;

    match CTX.cur_inst.mode {
        AddrMode::AM_IMP => (),
        AddrMode::AM_R => {
            CTX.fetched_data = cpu_read_reg(CTX.cur_inst.reg_1);
        }
        AddrMode::AM_R_D8 => {
            CTX.fetched_data = bus_read(CTX.regs.pc) as u16;
            emu_cycles(1);
            CTX.regs.pc += 1;
        }
        AddrMode::AM_D16 => {
            let lo = bus_read(CTX.regs.pc) as u16;
            emu_cycles(1);

            let hi = bus_read(CTX.regs.pc + 1) as u16;
            emu_cycles(1);

            CTX.fetched_data = lo | (hi << 8);

            CTX.regs.pc += 2;
        }
        _ => panic!("Unknown Addressing mode"),
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

pub unsafe fn cpu_flag_z() -> bool {
    bit(CTX.regs.f, 7)
}

pub unsafe fn cpu_flag_c() -> bool {
    bit(CTX.regs.f, 4)
}

fn reverse(n: u16) -> u16 {
    ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)
}

unsafe fn cpu_read_reg(rt: RegType) -> u16 {
    match rt {
        RegType::RT_A => CTX.regs.a as u16,
        RegType::RT_F => CTX.regs.f as u16,
        RegType::RT_B => CTX.regs.b as u16,
        RegType::RT_C => CTX.regs.c as u16,
        RegType::RT_D => CTX.regs.d as u16,
        RegType::RT_E => CTX.regs.e as u16,
        RegType::RT_H => CTX.regs.h as u16,
        RegType::RT_L => CTX.regs.l as u16,
        RegType::RT_AF => reverse(CTX.regs.a as u16),
        RegType::RT_BC => reverse(CTX.regs.b as u16),
        RegType::RT_DE => reverse(CTX.regs.d as u16),
        RegType::RT_HL => reverse(CTX.regs.h as u16),
        RegType::RT_PC => CTX.regs.pc,
        RegType::RT_SP => CTX.regs.sp,
        _ => 0,
    }
}
