pub fn bit(a: u32, n: u32) -> u32 {
    if a & (1 << n) != 0 {
        1
    } else {
        0
    }
}

pub fn bit_set(a: &mut u32, n: u32, on: bool) {
    if on {
        *a |= 1 << n;
    } else {
        *a &= !(1 << n);
    }
}

pub fn between(a: u32, b: u32, c: u32) -> bool {
    a >= b && a <= c
}
