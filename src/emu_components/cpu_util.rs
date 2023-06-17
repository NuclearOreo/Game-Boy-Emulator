use crate::emu_components::common::bit;
use crate::emu_components::cpu::cpu_get_context;
use crate::emu_components::instructions::RegType;

pub unsafe fn cpu_flag_z() -> bool {
    let CTX = cpu_get_context();
    bit(CTX.regs.f, 7)
}

pub unsafe fn cpu_flag_c() -> bool {
    let CTX = cpu_get_context();
    bit(CTX.regs.f, 4)
}

fn reverse(n: u16) -> u16 {
    ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)
}

pub unsafe fn cpu_read_reg(rt: RegType) -> u16 {
    let CTX = cpu_get_context();
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
