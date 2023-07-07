/*
    STACK

    SP=0xDFFF

    MEMORY:
    0xDFF7: 00
    0xDFF8: 00
    0xDFF9: 00
    0xDFFA: 00
    0xDFFB: 00
    0xDFFC: 00
    0xDFFD: 00
    0xDFFE: 00
    0xDFFF: 00 <- SP

    PUSH 0x55

    SP-- = 0xDFFE
    MEMORY[0xDFFE] = 0x55

    MEMORY:
    0xDFF7: 00
    0xDFF8: 00
    0xDFF9: 00
    0xDFFA: 00
    0xDFFB: 00
    0xDFFC: 00
    0xDFFD: 00
    0xDFFE: 55 <- SP
    0xDFFF: 00

    PUSH 0x77

    SP-- = 0xDFFD
    MEMORY[0xDFFD] = 0x77

    MEMORY:
    0xDFF7: 00
    0xDFF8: 00
    0xDFF9: 00
    0xDFFA: 00
    0xDFFB: 00
    0xDFFC: 00
    0xDFFD: 77 <- SP
    0xDFFE: 55
    0xDFFF: 00

    val = POP

    val = MEMORY[0xDFFD] = 0x77
    SP++ = 0xDFFE

    MEMORY:
    0xDFF7: 00
    0xDFF8: 00
    0xDFF9: 00
    0xDFFA: 00
    0xDFFB: 00
    0xDFFC: 00
    0xDFFD: 77
    0xDFFE: 55 <- SP
    0xDFFF: 00


    PUSH 0x88

    SP-- = 0xDFFD
    MEMORY[0xDFFD] = 0x88

    MEMORY:
    0xDFF7: 00
    0xDFF8: 00
    0xDFF9: 00
    0xDFFA: 00
    0xDFFB: 00
    0xDFFC: 00
    0xDFFD: 88 <- SP
    0xDFFE: 55
    0xDFFF: 00
*/

use super::bus::{bus_read, bus_write};
use super::cpu_util::cpu_get_regs;

pub unsafe fn stack_push(data: u8) {
    let mut regs = cpu_get_regs();
    regs.sp -= 1;
    bus_write(regs.sp, data);
}

pub unsafe fn stack_push16(data: u16) {
    stack_push((data >> 8) as u8);
    stack_push((data) as u8);
}

pub unsafe fn stack_pop() -> u8 {
    let regs = cpu_get_regs();
    let read = bus_read(regs.sp);
    regs.sp += 1;
    read
}

pub unsafe fn stack_pop16() -> u16 {
    let lo = stack_pop();
    let hi = stack_pop();
    (hi as u16) << 8 | (lo as u16)
}
