pub fn add_nums(x: i32, y: i32) -> i32 {
    x + y
}

pub fn mul_nums(x: i32, y: i32) -> i32 {
    x * y
}

pub fn power(num: i32, k: u8) -> i32 {
    match k {
        0 => 0,
        1 => num,
        _ => num.checked_pow(k as u32).unwrap_or(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_nums() {
        assert_eq!(add_nums(5, 10), 15);
    }
    #[test]
    fn test_mul_nums() {
        assert_eq!(mul_nums(5, 10), 50);
    }
    #[test]
    fn test_power() {
        assert_eq!(power(2, 5), 32);
        assert_eq!(power(4, 2), 16);
        assert_eq!(power(9, 2), 81);
        assert_eq!(power(9, 0), 0);
        assert_eq!(power(9, 1), 9);
    }
}
