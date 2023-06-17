use crate::emu_components::bus::bus_read;
use crate::emu_components::cpu::cpu_get_context;
use crate::emu_components::cpu_util::cpu_read_reg;
use crate::emu_components::emu::emu_cycles;
use crate::emu_components::instructions::AddrMode;

pub unsafe fn fetch_data() {
    let CTX = cpu_get_context();

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
