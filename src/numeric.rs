pub fn wrapping_inc(n: usize, max: usize) -> (usize, bool) {
    if n == max - 1 {
        (0, true)
    } else {
        (n + 1, false)
    }
}

pub fn wrapping_dec(n: usize, max: usize) -> (usize, bool) {
    if n == 0 {
        (max - 1, true)
    } else {
        (n - 1, false)
    }
}
