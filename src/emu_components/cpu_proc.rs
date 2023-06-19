use super::bus::{bus_write, bus_write16};
use super::common::bit_set;
use super::cpu::CpuContext;
use super::cpu_util::{cpu_flag_c, cpu_flag_z, cpu_read_reg, cpu_set_reg};
use super::emu::emu_cycles;
use super::instructions::{AddrMode, CondType, InType};

pub type InProc = unsafe fn(&mut CpuContext);

fn proc_none(ctx: &mut CpuContext) {
    panic!("Invalid instructions");
}

fn proc_unknown(ctx: &mut CpuContext) {
    panic!("Unimplemented proc for instruction: {:2X}", ctx.cur_opcode);
}

fn proc_nop(ctx: &mut CpuContext) {}

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

unsafe fn proc_di(ctx: &mut CpuContext) {
    ctx.int_master_enabled = false;
}

unsafe fn proc_jp(ctx: &mut CpuContext) {
    if check_cond(ctx) {
        ctx.regs.pc = ctx.fetched_data;
        emu_cycles(1);
    }
}

pub fn inst_get_processor(i_type: InType) -> InProc {
    match i_type {
        InType::IN_NONE => proc_none,
        InType::IN_NOP => proc_nop,
        InType::IN_LD => proc_ld,
        InType::IN_JP => proc_jp,
        InType::IN_DI => proc_di,
        InType::IN_XOR => proc_xor,
        _ => proc_unknown,
    }
}
