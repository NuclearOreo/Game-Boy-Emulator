use crate::emu_components::common::bit;
use crate::emu_components::cpu::cpu_get_context;
use crate::emu_components::instructions::RegType;

pub unsafe fn cpu_flag_z() -> bool {
    let ctx = cpu_get_context();
    bit(ctx.regs.f, 7)
}

pub unsafe fn cpu_flag_c() -> bool {
    let ctx = cpu_get_context();
    bit(ctx.regs.f, 4)
}

fn reverse(n: u16) -> u16 {
    ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)
}

pub unsafe fn cpu_read_reg(rt: RegType) -> u16 {
    let ctx = cpu_get_context();
    match rt {
        RegType::RT_A => ctx.regs.a as u16,
        RegType::RT_F => ctx.regs.f as u16,
        RegType::RT_B => ctx.regs.b as u16,
        RegType::RT_C => ctx.regs.c as u16,
        RegType::RT_D => ctx.regs.d as u16,
        RegType::RT_E => ctx.regs.e as u16,
        RegType::RT_H => ctx.regs.h as u16,
        RegType::RT_L => ctx.regs.l as u16,
        RegType::RT_AF => reverse(ctx.regs.a as u16),
        RegType::RT_BC => reverse(ctx.regs.b as u16),
        RegType::RT_DE => reverse(ctx.regs.d as u16),
        RegType::RT_HL => reverse(ctx.regs.h as u16),
        RegType::RT_PC => ctx.regs.pc,
        RegType::RT_SP => ctx.regs.sp,
        _ => 0,
    }
}

// pub unsafe fn cpu_set_reg(rt: RegType, val: u16) {}
