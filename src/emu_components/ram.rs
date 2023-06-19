struct RamContext {
    wram: [u8; 0x2000],
    hram: [u8; 0x80],
}

static mut CTX: RamContext = RamContext {
    wram: [0; 0x2000],
    hram: [0; 0x80],
};

pub unsafe fn wram_read(mut address: u16) -> u8 {
    address -= 0xC000;

    if address >= 0x2000 {
        println!("INVALID WRAM ({:04X})", address);
    }

    CTX.wram[address as usize]
}

pub unsafe fn wram_write(mut address: u16, value: u8) {
    address -= 0xC000;
    CTX.wram[address as usize] = value;
}

pub unsafe fn hram_read(mut address: u16) -> u8 {
    address -= 0xFF80;
    CTX.hram[address as usize]
}

pub unsafe fn hram_write(mut address: u16, value: u8) {
    address -= 0xFF80;
    CTX.hram[address as usize] = value;
}
