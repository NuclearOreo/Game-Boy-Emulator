pub fn bit(a: u8, n: u8) -> bool {
    if a & (1 << n) != 0 {
        true
    } else {
        false
    }
}

pub fn bit_set(a: &mut u8, n: u8, on: bool) {
    if on {
        *a |= 1 << n;
    } else {
        *a &= !(1 << n);
    }
}

pub fn between(a: u8, b: u8, c: u8) -> bool {
    a >= b && a <= c
}

pub fn convert_to_u16(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) | (b as u16)
}
