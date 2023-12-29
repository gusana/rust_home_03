pub type SignedCounter = isize;
pub type UnsignedCounter = usize;
pub fn default_signed_counter() -> SignedCounter {
    0
}
pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}
pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}
pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}
pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_default_unsigned_counter() {
        assert_eq!(0_usize, default_unsigned_counter());
    }
    #[test]
    fn check_default_signed_counter() {
        assert_eq!(0_isize, default_signed_counter());
    }
    #[test]
    fn check_next_signed() {
        assert_eq!(2, next_signed(1));
    }
    #[test]
    fn check_next_unsigned() {
        assert_eq!(2, next_unsigned(1));
    }
    #[test]
    fn check_prev_signed() {
        assert_eq!(2, prev_signed(3));
    }
}
