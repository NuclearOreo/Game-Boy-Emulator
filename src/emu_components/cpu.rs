use crate::emu_components::bus::bus_read;
use crate::emu_components::common::bit;
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
pub struct cpu_context {
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

unsafe fn CPU_FLAG_Z() -> bool {
    bit(CTX.regs.f, 7)
}

unsafe fn CPU_FLAG_C() -> bool {
    bit(CTX.regs.f, 4)
}

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

pub type IN_PROC = unsafe fn(&mut cpu_context);

fn proc_none(ctx: &mut cpu_context) {
    panic!("Invalid instructions");
}

fn proc_unknown(ctx: &mut cpu_context) {
    panic!("Unimplemented proc for instruction: {:2X}", ctx.cur_opcode);
}

fn proc_nop(ctx: &mut cpu_context) {}

fn proc_ld(ctx: &mut cpu_context) {
    todo!();
}

unsafe fn check_cond(ctx: &mut cpu_context) -> bool {
    let z = CPU_FLAG_Z();
    let c = CPU_FLAG_C();

    match ctx.cur_inst.cond {
        CondType::CT_NONE => true,
        CondType::CT_C => c,
        CondType::CT_NC => !c,
        CondType::CT_Z => z,
        CondType::CT_NZ => !z,
    }
}

unsafe fn proc_jp(ctx: &mut cpu_context) {
    if check_cond(ctx) {
        ctx.regs.pc = ctx.fetched_data;
        emu_cycles(1);
    }
}

pub fn inst_get_processor(i_type: InType) -> IN_PROC {
    match i_type {
        InType::IN_NONE => proc_none,
        InType::IN_NOP => proc_nop,
        InType::IN_LD => proc_ld,
        InType::IN_JP => proc_jp,
        _ => proc_unknown,
    }
}
