use crate::emu_components::bus::bus_read;
use crate::emu_components::emu::emu_cycles;
use crate::emu_components::instructions::{instruction_by_opcode, set_instuctions};
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
    dest_is_mem: bool,
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
    dest_is_mem: false,
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

pub unsafe fn cpu_init() {
    set_instuctions();
    CTX.regs.pc = 0x100;
}

fn fetch_instruction() {
    unsafe {
        CTX.cur_opcode = bus_read(CTX.regs.pc);
        CTX.regs.pc += 1;

        CTX.cur_inst = match instruction_by_opcode(CTX.cur_opcode) {
            Some(x) => x,
            _ => panic!("Unknown instruction: {:#x}", CTX.cur_opcode),
        }
    }
}

fn fetch_data() {
    unsafe {
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
}

unsafe fn execute() {
    println!("\tNot executing yet...");
}

pub unsafe fn cpu_step() -> bool {
    let pc = CTX.regs.pc;
    if !CTX.halted {
        fetch_instruction();
        fetch_data();

        println!(
            "Executing Instruction: {:#x}   PC: {:#x}",
            CTX.cur_opcode, pc
        );

        execute();
    }
    true
}

fn reverse(n: u16) -> u16 {
    ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)
}

fn cpu_read_reg(rt: RegType) -> u16 {
    unsafe {
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
}
