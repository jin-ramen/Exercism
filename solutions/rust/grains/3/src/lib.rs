const BASE: u64 = 2;

pub fn square(s: u32) -> u64 {
    if s < 1 && s > 64 {
        panic!("yo s is not in the chessboard")
    }
    BASE.pow(s-1)
}

pub fn total() -> u64 {
    u64::MAX
}
