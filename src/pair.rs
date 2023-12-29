pub type Pair = (i32, i32);

pub fn default_pair() -> Pair {
    (0, 0)
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_default_pair() {
        let def_pair = (0, 0);
        assert_eq!(def_pair, default_pair());
    }
    #[test]
    fn check_pair_vector_sum() {
        assert_eq!((2, 2), pair_vector_sum((1, 1), (1, 1)));
    }
    #[test]
    fn check_pair_scalar_sum() {
        assert_eq!(4, pair_scalar_sum((1, 1), (1, 1)));
    }
}
