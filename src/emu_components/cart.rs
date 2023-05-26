#[derive(Debug)]
struct RomHeader {
    entry: [u8; 4],
    logo: [u8; 0x30],
    title: [u8; 16],
    new_lic_code: u16,
    sgb_flag: u8,
    c_type: u8,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    lic_code: u8,
    version: u8,
    checksum: u8,
    global_checksum: u16,
}

#[derive(Debug)]
struct CartContext {
    filename: [u8; 1024],
    rom_size: u32,
    rom_data: *mut u8,
    header: *mut RomHeader,
}

static mut CTX: CartContext = CartContext {
    filename: [0; 1024],
    rom_size: 0,
    rom_data: std::ptr::null_mut(),
    header: std::ptr::null_mut(),
};
