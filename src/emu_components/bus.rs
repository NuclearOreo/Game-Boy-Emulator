use super::cart::{cart_read, cart_write};
use super::cpu::{cpu_get_ie_register, cpu_set_ie_register};
use super::ram::{hram_read, hram_write, wram_read, wram_write};
// 0x0000 - 0x3FFF : ROM Bank 0
// 0x4000 - 0x7FFF : ROM Bank 1 - Switchable
// 0x8000 - 0x97FF : CHR RAM
// 0x9800 - 0x9BFF : BG Map 1
// 0x9C00 - 0x9FFF : BG Map 2
// 0xA000 - 0xBFFF : Cartridge RAM
// 0xC000 - 0xCFFF : RAM Bank 0
// 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
// 0xE000 - 0xFDFF : Reserved - Echo RAM
// 0xFE00 - 0xFE9F : Object Attribute Memory
// 0xFEA0 - 0xFEFF : Reserved - Unusable
// 0xFF00 - 0xFF7F : I/O Registers
// 0xFF80 - 0xFFFE : Zero Page

pub unsafe fn bus_read(address: u16) -> u8 {
    if address < 0x8000 {
        //ROM Data
        return cart_read(address);
    } else if address < 0xA000 {
        //Char/Map Data
        //TODO
        panic!("UNSUPPORTED  write read ({:04X})", address);
    } else if address < 0xC000 {
        //Cartridge RAM
        return cart_read(address);
    } else if address < 0xE000 {
        //WRAM (Working RAM)
        return wram_read(address);
    } else if address < 0xFE00 {
        //reserved echo ram...
        return 0;
    } else if address < 0xFEA0 {
        //OAM
        //TODO
        panic!("UNSUPPORTED  write read ({:04X})", address);
    } else if address < 0xFF00 {
        //reserved unusable...
        return 0;
    } else if address < 0xFF80 {
        //IO Registers...
        //TODO
        panic!("UNSUPPORTED  write read ({:04X})", address);
    } else if address < 0xFFFF {
        //CPU ENABLE REGISTER...
        return cpu_get_ie_register();
    }

    //NO_IMPL
    return hram_read(address);
}

pub unsafe fn bus_write(address: u16, value: u8) {
    if address < 0x8000 {
        //ROM Data
        cart_write(address, value);
    } else if address < 0xA000 {
        //Char/Map Data
        //TODO
        println!("UNSUPPORTED  write read ({:04X})", address);
        // panic!("UNSUPPORTED  write read ({:04X})", address);
    } else if address < 0xC000 {
        //WRAM
        wram_write(address, value);
    } else if address < 0xFE00 {
        //reserved echo ram
        return;
    } else if address < 0xFEA0 {
        //OAM
        //TODO
        println!("UNSUPPORTED  write read ({:04X})", address);
        // panic!("UNSUPPORTED  write read ({:04X})", address);
    } else if address < 0xFF00 {
        //unusable reserved
        return;
    } else if address < 0xFF80 {
        //IO Registers...
        //TODO
        println!("UNSUPPORTED  write read ({:04X})", address);
        // panic!("UNSUPPORTED  write read ({:04X})", address);
    } else if address == 0xFFFF {
        //CPU SET ENABLE REGISTER
        cpu_set_ie_register(value);
    } else {
        hram_write(address, value);
    }
}

pub unsafe fn bus_read16(address: u16) -> u16 {
    let lo = bus_read(address) as u16;
    let hi = bus_read(address + 1) as u16;
    lo | (hi << 8)
}

pub unsafe fn bus_write16(address: u16, value: u16) {
    bus_write(address + 1, (value >> 8) as u8);
    bus_write(address, value as u8);
}
