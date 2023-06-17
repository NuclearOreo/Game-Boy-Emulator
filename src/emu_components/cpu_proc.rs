use crate::emu_components::common::bit_set;
use crate::emu_components::cpu::cpu_context;
use crate::emu_components::cpu::{cpu_flag_c, cpu_flag_z};
use crate::emu_components::emu::emu_cycles;
use crate::emu_components::instructions::{CondType, InType};

pub type IN_PROC = unsafe fn(&mut cpu_context);

fn proc_none(ctx: &mut cpu_context) {
    panic!("Invalid instructions");
}

fn proc_unknown(ctx: &mut cpu_context) {
    panic!("Unimplemented proc for instruction: {:2X}", ctx.cur_opcode);
}

fn proc_nop(ctx: &mut cpu_context) {}

fn proc_ld(ctx: &mut cpu_context) {
    //Todo
}

fn cpu_set_flags(
    ctx: &mut cpu_context,
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

fn proc_xor(ctx: &mut cpu_context) {
    ctx.regs.a ^= ctx.fetched_data as u8;

    cpu_set_flags(
        ctx,
        Some(ctx.regs.a == 0),
        Some(false),
        Some(false),
        Some(false),
    )
}

unsafe fn check_cond(ctx: &mut cpu_context) -> bool {
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

unsafe fn proc_di(ctx: &mut cpu_context) {
    ctx.int_master_enabled = false;
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
        InType::IN_DI => proc_di,
        InType::IN_XOR => proc_xor,
        _ => proc_unknown,
    }
}
