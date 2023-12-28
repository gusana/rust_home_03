mod counters;
mod pair;
mod vector;
use counters::counters::{
    default_signed_counter, default_unsigned_counter, next_signed, next_unsigned, prev_signed,
};

use pair::pair::{default_pair, pair_scalar_sum, pair_vector_sum};

use vector::vector::{default_vec3, vec3_scalar_sum, vec3_vector_sum};

fn main() {
    assert_eq!(0_usize, default_unsigned_counter());
    assert_eq!(0_isize, default_signed_counter());
    assert_eq!(2, next_signed(1));
    assert_eq!(2, next_unsigned(1));
    assert_eq!(2, prev_signed(3));

    let def_vec: Vec<i32> = vec![0, 0, 0];
    assert_eq!(def_vec, default_vec3());
    assert_eq!([2, 2, 3], vec3_vector_sum([1, 1, 2], [1, 1, 1]));
    assert_eq!(8, vec3_scalar_sum([0, 2, 1], [1, 1, 3]));

    let def_pair = (0, 0);
    assert_eq!(def_pair, default_pair());
    assert_eq!((2, 2), pair_vector_sum((1, 1), (1, 1)));
    assert_eq!(4, pair_scalar_sum((1, 1), (1, 1)))
}
