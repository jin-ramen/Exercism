const BASE: u64 = 2;

pub fn square(s: u32) -> u64 {
    if s < 1 && s > 64 {
        panic!("yo s is not in the chessboard")
    }
    BASE.pow(s-1)
}

pub fn total() -> u64 {
    // Geometric series with a = 1, r = 2 (i.e. BASE)
    // if r != 1 -> S_n = a * ((1 - r^n) / (1 - r)) 
    // else -> S_n = n * a 
    // this doenst work?
    (BASE.pow(64) - 1) / (BASE - 1)
}
