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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping_inc_no_wrap() {
        assert_eq!(wrapping_inc(0, 4), (1, false));
        assert_eq!(wrapping_inc(2, 4), (3, false));
        assert_eq!(wrapping_inc(7, 9), (8, false));
    }

    #[test]
    fn test_wrapping_inc_wrap() {
        assert_eq!(wrapping_inc(3, 4), (0, true));
        assert_eq!(wrapping_inc(0, 1), (0, true));
        assert_eq!(wrapping_inc(90, 91), (0, true));
    }

    #[test]
    fn test_wrapping_dec_no_wrap() {
        assert_eq!(wrapping_dec(1, 4), (0, false));
        assert_eq!(wrapping_dec(2, 4), (1, false));
        assert_eq!(wrapping_dec(7, 9), (6, false));
    }

    #[test]
    fn test_wrapping_dec_wrap() {
        assert_eq!(wrapping_dec(0, 4), (3, true));
        assert_eq!(wrapping_dec(0, 1), (0, true));
        assert_eq!(wrapping_dec(0, 91), (90, true));
    }
}
