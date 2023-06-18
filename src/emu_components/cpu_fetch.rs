use crate::emu_components::bus::bus_read;
use crate::emu_components::cpu::cpu_get_context;
use crate::emu_components::cpu_util::cpu_read_reg;
use crate::emu_components::emu::emu_cycles;
use crate::emu_components::instructions::{AddrMode, RegType};

pub unsafe fn fetch_data() {
    let ctx = cpu_get_context();

    ctx.mem_dest = 0;
    ctx.dest_is_mem = false;

    match ctx.cur_inst.mode {
        AddrMode::AM_IMP => (),
        AddrMode::AM_R => {
            ctx.fetched_data = cpu_read_reg(ctx.cur_inst.reg_1);
        }
        AddrMode::AM_R_R => {
            ctx.fetched_data = cpu_read_reg(ctx.cur_inst.reg_2);
        }
        AddrMode::AM_R_D8 => {
            ctx.fetched_data = bus_read(ctx.regs.pc) as u16;
            emu_cycles(1);
            ctx.regs.pc += 1;
        }
        AddrMode::AM_D16 | AddrMode::AM_R_D16 => {
            let lo = bus_read(ctx.regs.pc) as u16;
            emu_cycles(1);

            let hi = bus_read(ctx.regs.pc + 1) as u16;
            emu_cycles(1);

            ctx.fetched_data = lo | (hi << 8);

            ctx.regs.pc += 2;
        }
        AddrMode::AM_MR_R => {
            ctx.fetched_data = cpu_read_reg(ctx.cur_inst.reg_2);
            ctx.mem_dest = cpu_read_reg(ctx.cur_inst.reg_1);
            ctx.dest_is_mem = true;

            if ctx.cur_inst.reg_1 == RegType::RT_C {
                ctx.mem_dest |= 0xFF00;
            }
        }
        AddrMode::AM_R_MR => {
            let addr = cpu_read_reg(ctx.cur_inst.reg_2);

            if ctx.cur_inst.reg_2 == RegType::RT_C {
                ctx.mem_dest |= 0xFF00;
            }

            ctx.fetched_data = bus_read(addr) as u16;
            emu_cycles(1);
        }
        AddrMode::AM_R_HLI => {
            ctx.fetched_data = bus_read(cpu_read_reg(ctx.cur_inst.reg_2)) as u16;
            emu_cycles(1);
            // Need cpu_set_reg
        }
        _ => panic!("Unknown Addressing mode"),
    }
}
