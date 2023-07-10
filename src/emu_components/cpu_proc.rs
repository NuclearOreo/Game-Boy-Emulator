use std::num::IntErrorKind;

use super::bus::{bus_read, bus_write, bus_write16};
use super::common::bit_set;
use super::cpu::CpuContext;
use super::cpu_util::{cpu_flag_c, cpu_flag_z, cpu_read_reg, cpu_set_reg};
use super::emu::emu_cycles;
use super::instructions::{AddrMode, CondType, InType, RegType};
use super::stack::{stack_pop, stack_push, stack_push16};

pub type InProc = unsafe fn(&mut CpuContext);

fn proc_none(ctx: &mut CpuContext) {
    panic!("Invalid instructions");
}

fn proc_unknown(ctx: &mut CpuContext) {
    panic!("Unimplemented proc for instruction: {:02X}", ctx.cur_opcode);
}

fn proc_nop(ctx: &mut CpuContext) {}

unsafe fn proc_di(ctx: &mut CpuContext) {
    ctx.int_master_enabled = false;
}

unsafe fn proc_ld(ctx: &mut CpuContext) {
    if ctx.dest_is_mem {
        if ctx.cur_inst.reg_2.is_16bit() {
            emu_cycles(1);
            bus_write16(ctx.mem_dest, ctx.fetched_data);
        } else {
            bus_write(ctx.mem_dest, ctx.fetched_data as u8);
        }
    }

    if ctx.cur_inst.mode == AddrMode::AM_HL_SPR {
        let hflag = (cpu_read_reg(ctx.cur_inst.reg_2) & 0xF) + (ctx.fetched_data & 0xF) >= 0x10;
        let cflag = (cpu_read_reg(ctx.cur_inst.reg_2) & 0xFF) + (ctx.fetched_data & 0xFF) >= 0x100;

        cpu_set_flags(ctx, Some(false), Some(false), Some(hflag), Some(cflag));

        cpu_set_reg(
            ctx.cur_inst.reg_1,
            cpu_read_reg(ctx.cur_inst.reg_2) + (ctx.fetched_data as u8) as u16,
        )
    }
    cpu_set_reg(ctx.cur_inst.reg_1, ctx.fetched_data);
}

fn cpu_set_flags(
    ctx: &mut CpuContext,
    z: Option<bool>,
    n: Option<bool>,
    h: Option<bool>,
    c: Option<bool>,
) {
    if let Some(z) = z {
        bit_set(&mut ctx.regs.f, 7, z);
    }

    if let Some(n) = n {
        bit_set(&mut ctx.regs.f, 6, n);
    }

    if let Some(h) = h {
        bit_set(&mut ctx.regs.f, 5, h);
    }

    if let Some(c) = c {
        bit_set(&mut ctx.regs.f, 4, c);
    }
}

unsafe fn proc_ldh(ctx: &mut CpuContext) {
    if ctx.cur_inst.reg_1 == RegType::RT_A {
        cpu_set_reg(
            ctx.cur_inst.reg_1,
            bus_read(0xFF00 | ctx.fetched_data) as u16,
        );
    } else {
        bus_write(0xFF00 | ctx.fetched_data, ctx.regs.a);
    }

    emu_cycles(1);
}

fn proc_xor(ctx: &mut CpuContext) {
    ctx.regs.a ^= ctx.fetched_data as u8;

    cpu_set_flags(
        ctx,
        Some(ctx.regs.a == 0),
        Some(false),
        Some(false),
        Some(false),
    )
}

unsafe fn check_cond(ctx: &mut CpuContext) -> bool {
    let z = cpu_flag_z();
    let c = cpu_flag_c();

    match ctx.cur_inst.cond {
        CondType::CT_NONE => true,
        CondType::CT_C => c,
        CondType::CT_NC => !c,
        CondType::CT_Z => z,
        CondType::CT_NZ => !z,
    }
}

unsafe fn goto_addr(ctx: &mut CpuContext, addr: u16, pushpc: bool) {
    if check_cond(ctx) {
        if pushpc {
            emu_cycles(2);
            stack_push16(ctx.regs.pc);
        }

        ctx.regs.pc = addr;
        emu_cycles(1);
    }
}

unsafe fn proc_jp(ctx: &mut CpuContext) {
    goto_addr(ctx, ctx.fetched_data, false);
}

unsafe fn proc_jr(ctx: &mut CpuContext) {
    let rel = (ctx.fetched_data as u8) as u16;
    let addr = ctx.regs.pc + rel;
    goto_addr(ctx, addr, false);
}

unsafe fn proc_call(ctx: &mut CpuContext) {
    goto_addr(ctx, ctx.fetched_data, true);
}

unsafe fn proc_ret(ctx: &mut CpuContext) {
    if ctx.cur_inst.cond != CondType::CT_NONE {
        emu_cycles(1);
    }

    if check_cond(ctx) {
        let lo = stack_pop();
        emu_cycles(1);
        let hi = stack_pop();
        emu_cycles(1);

        let n = ((hi as u16) << 8) | (lo as u16);
        ctx.regs.pc = n;

        emu_cycles(1);
    }
}

unsafe fn proc_reti(ctx: &mut CpuContext) {
    ctx.int_master_enabled = true;
    proc_ret(ctx);
}

unsafe fn proc_pop(ctx: &mut CpuContext) {
    let lo = stack_pop() as u16;
    emu_cycles(1);

    let hi = stack_pop() as u16;
    emu_cycles(1);

    let n = (hi << 8) | lo;

    cpu_set_reg(ctx.cur_inst.reg_1, n);

    if ctx.cur_inst.reg_1 == RegType::RT_AF {
        cpu_set_reg(ctx.cur_inst.reg_1, n & 0xFFF0);
    }
}

unsafe fn proc_push(ctx: &mut CpuContext) {
    let hi = (cpu_read_reg(ctx.cur_inst.reg_1) >> 8) as u8;
    emu_cycles(1);
    stack_push(hi);

    let lo = cpu_read_reg(ctx.cur_inst.reg_1) as u8;
    emu_cycles(1);
    stack_push(lo);

    emu_cycles(1)
}

pub fn inst_get_processor(i_type: InType) -> InProc {
    match i_type {
        InType::IN_NONE => proc_none,
        InType::IN_NOP => proc_nop,
        InType::IN_LDH => proc_ldh,
        InType::IN_LD => proc_ld,
        InType::IN_JP => proc_jp,
        InType::IN_POP => proc_pop,
        InType::IN_PUSH => proc_push,
        InType::IN_CALL => proc_call,
        InType::IN_JR => proc_jr,
        InType::IN_DI => proc_di,
        InType::IN_RET => proc_ret,
        InType::IN_RETI => proc_reti,
        InType::IN_XOR => proc_xor,
        _ => proc_unknown,
    }
}
