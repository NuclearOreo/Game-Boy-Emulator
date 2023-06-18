use crate::emu_components::common::bit_set;
use crate::emu_components::cpu::CpuContext;
use crate::emu_components::cpu_util::{cpu_flag_c, cpu_flag_z};
use crate::emu_components::emu::emu_cycles;
use crate::emu_components::instructions::{CondType, InType};

pub type InProc = unsafe fn(&mut CpuContext);

fn proc_none(ctx: &mut CpuContext) {
    panic!("Invalid instructions");
}

fn proc_unknown(ctx: &mut CpuContext) {
    panic!("Unimplemented proc for instruction: {:2X}", ctx.cur_opcode);
}

fn proc_nop(ctx: &mut CpuContext) {}

fn proc_ld(ctx: &mut CpuContext) {
    //Todo
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
