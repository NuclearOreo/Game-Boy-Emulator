use crate::emu_components::common::convert_to_u16;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct RomHeader {
    entry: [u8; 4],
    logo: [u8; 48],
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
pub struct CartContext {
    pub filename: String,
    pub rom_size: u64,
    pub rom_data: Vec<u8>,
    pub header: RomHeader,
}

static mut CTX: CartContext = CartContext {
    filename: String::new(),
    rom_size: 0,
    rom_data: Vec::new(),
    header: RomHeader {
        entry: [0; 4],
        logo: [0; 48],
        title: [0; 16],
        new_lic_code: 0,
        sgb_flag: 0,
        c_type: 0,
        rom_size: 0,
        ram_size: 0,
        dest_code: 0,
        lic_code: 0,
        version: 0,
        checksum: 0,
        global_checksum: 0,
    },
};

static ROM_TYPES: [&str; 35] = [
    "ROM ONLY",
    "MBC1",
    "MBC1+RAM",
    "MBC1+RAM+BATTERY",
    "0x04 ???",
    "MBC2",
    "MBC2+BATTERY",
    "0x07 ???",
    "ROM+RAM 1",
    "ROM+RAM+BATTERY 1",
    "0x0A ???",
    "MMM01",
    "MMM01+RAM",
    "MMM01+RAM+BATTERY",
    "0x0E ???",
    "MBC3+TIMER+BATTERY",
    "MBC3+TIMER+RAM+BATTERY 2",
    "MBC3",
    "MBC3+RAM 2",
    "MBC3+RAM+BATTERY 2",
    "0x14 ???",
    "0x15 ???",
    "0x16 ???",
    "0x17 ???",
    "0x18 ???",
    "MBC5",
    "MBC5+RAM",
    "MBC5+RAM+BATTERY",
    "MBC5+RUMBLE",
    "MBC5+RUMBLE+RAM",
    "MBC5+RUMBLE+RAM+BATTERY",
    "0x1F ???",
    "MBC6",
    "0x21 ???",
    "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
];

fn LIC_CODE(code: u16) -> String {
    let lic_code = vec![
        (0x00, "None"),
        (0x01, "Nintendo R&D1"),
        (0x08, "Capcom"),
        (0x13, "Electronic Arts"),
        (0x18, "Hudson Soft"),
        (0x19, "b-ai"),
        (0x20, "kss"),
        (0x22, "pow"),
        (0x24, "PCM Complete"),
        (0x25, "san-x"),
        (0x28, "Kemco Japan"),
        (0x29, "seta"),
        (0x30, "Viacom"),
        (0x31, "Nintendo"),
        (0x32, "Bandai"),
        (0x33, "Ocean/Acclaim"),
        (0x34, "Konami"),
        (0x35, "Hector"),
        (0x37, "Taito"),
        (0x38, "Hudson"),
        (0x39, "Banpresto"),
        (0x41, "Ubi Soft"),
        (0x42, "Atlus"),
        (0x44, "Malibu"),
        (0x46, "angel"),
        (0x47, "Bullet-Proof"),
        (0x49, "irem"),
        (0x50, "Absolute"),
        (0x51, "Acclaim"),
        (0x52, "Activision"),
        (0x53, "American sammy"),
        (0x54, "Konami"),
        (0x55, "Hi tech entertainment"),
        (0x56, "LJN"),
        (0x57, "Matchbox"),
        (0x58, "Mattel"),
        (0x59, "Milton Bradley"),
        (0x60, "Titus"),
        (0x61, "Virgin"),
        (0x64, "LucasArts"),
        (0x67, "Ocean"),
        (0x69, "Electronic Arts"),
        (0x70, "Infogrames"),
        (0x71, "Interplay"),
        (0x72, "Broderbund"),
        (0x73, "sculptured"),
        (0x75, "sci"),
        (0x78, "THQ"),
        (0x79, "Accolade"),
        (0x80, "misawa"),
        (0x83, "lozc"),
        (0x86, "Tokuma Shoten Intermedia"),
        (0x87, "Tsukuda Original"),
        (0x91, "Chunsoft"),
        (0x92, "Video system"),
        (0x93, "Ocean/Acclaim"),
        (0x95, "Varie"),
        (0x96, "Yonezawa/s’pal"),
        (0x97, "Kaneko"),
        (0x99, "Pack in soft"),
        (0xA4, "Konami (Yu-Gi-Oh!)"),
    ];

    match lic_code.iter().find(|(lic_code, _)| lic_code == &code) {
        Some((_, string)) => string.to_string(),
        None => "UNKNOWN".to_string(),
    }
}

fn populate_header() {
    unsafe {
        CTX.header.entry.copy_from_slice(&CTX.rom_data[256..260]);
        CTX.header.logo.copy_from_slice(&CTX.rom_data[260..308]);
        CTX.header
            .title
            .copy_from_slice(&CTX.rom_data[0x134..=0x143]);
        CTX.header.title[15] = 0;
        CTX.header.new_lic_code = convert_to_u16(CTX.rom_data[324], CTX.rom_data[325]);
        CTX.header.sgb_flag = CTX.rom_data[326];
        CTX.header.c_type = CTX.rom_data[326]
    }
}

pub fn cart_load(cart: String) {
    unsafe {
        CTX.filename = cart.to_owned();

        let mut fp = File::open(&CTX.filename).expect("Failed to open Cart");

        println!("Opened: {}", &CTX.filename);

        let rom_size = fp.metadata().expect("Expected to grab metadata").len();
        CTX.rom_size = rom_size;

        let mut rom_in_memory = Vec::new();
        fp.read_to_end(&mut rom_in_memory)
            .expect("Expected to read entire ROM");

        CTX.rom_data = rom_in_memory.to_owned();

        populate_header();

        println!();
        println!("\t{:?}", CTX.header.new_lic_code);
        println!("\t{:?}", String::from_utf8_lossy(&CTX.header.title));
        println!();

        println!("Cartridge Loaded:");
    }
}
