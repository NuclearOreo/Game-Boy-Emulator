use super::bus::bus_read;
use super::cpu::cpu_get_context;
use super::cpu_util::{cpu_read_reg, cpu_set_reg};
use super::emu::emu_cycles;
use super::instructions::{AddrMode, RegType};

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
            cpu_set_reg(RegType::RT_HL, cpu_read_reg(RegType::RT_HL) + 1);
        }
        AddrMode::AM_R_HLD => {
            ctx.fetched_data = bus_read(cpu_read_reg(ctx.cur_inst.reg_2)) as u16;
            emu_cycles(1);
            cpu_set_reg(RegType::RT_HL, cpu_read_reg(RegType::RT_HL) - 1);
        }
        AddrMode::AM_HLI_R => {
            ctx.fetched_data = cpu_read_reg(ctx.cur_inst.reg_2);
            ctx.mem_dest = cpu_read_reg(ctx.cur_inst.reg_1);
            ctx.dest_is_mem = true;
            cpu_set_reg(RegType::RT_HL, cpu_read_reg(RegType::RT_HL) + 1);
        }
        AddrMode::AM_HLD_R => {
            ctx.fetched_data = cpu_read_reg(ctx.cur_inst.reg_2);
            ctx.mem_dest = cpu_read_reg(ctx.cur_inst.reg_1);
            ctx.dest_is_mem = true;
            cpu_set_reg(RegType::RT_HL, cpu_read_reg(RegType::RT_HL) - 1);
        }
        AddrMode::AM_R_A8 => {
            ctx.fetched_data = bus_read(ctx.regs.pc) as u16;
            emu_cycles(1);
            ctx.regs.pc += 1;
        }
        AddrMode::AM_A8_R => {
            ctx.mem_dest = bus_read(ctx.regs.pc) as u16 | 0xFF00;
            ctx.dest_is_mem = true;
            emu_cycles(1);
            ctx.regs.pc += 1;
        }
        AddrMode::AM_HL_SPR => {
            ctx.fetched_data = bus_read(ctx.regs.pc) as u16;
            emu_cycles(1);
            ctx.regs.pc += 1;
        }
        AddrMode::AM_D8 => {
            ctx.fetched_data = bus_read(ctx.regs.pc) as u16;
            emu_cycles(1);
            ctx.regs.pc += 1;
        }
        AddrMode::AM_A16_R | AddrMode::AM_D16_R => {
            let lo = bus_read(ctx.regs.pc);
            emu_cycles(1);

            let hi = bus_read(ctx.regs.pc + 1);
            emu_cycles(1);

            ctx.mem_dest = lo as u16 | ((hi as u16) << 8);
            ctx.dest_is_mem = true;

            ctx.regs.pc += 2;
            ctx.fetched_data = cpu_read_reg(ctx.cur_inst.reg_2);
        }
        AddrMode::AM_MR_D8 => {
            ctx.fetched_data = bus_read(ctx.regs.pc) as u16;
            emu_cycles(1);
            ctx.regs.pc += 1;
            ctx.mem_dest = cpu_read_reg(ctx.cur_inst.reg_1);
            ctx.dest_is_mem = true;
        }
        AddrMode::AM_MR => {
            ctx.mem_dest = cpu_read_reg(ctx.cur_inst.reg_1);
            ctx.dest_is_mem = true;
            ctx.fetched_data = bus_read(cpu_read_reg(ctx.cur_inst.reg_1)) as u16;
            emu_cycles(1);
        }
        AddrMode::AM_R_A16 => {
            let lo = bus_read(ctx.regs.pc);
            emu_cycles(1);

            let hi = bus_read(ctx.regs.pc + 1);
            emu_cycles(1);

            let addr = lo as u16 | ((hi as u16) << 8);

            ctx.regs.pc += 2;
            ctx.fetched_data = bus_read(addr) as u16;
            emu_cycles(1);
        }
        _ => panic!("Unknown Addressing mode"),
    }
}
