pub const VEC3_LEN: usize = 3;
pub type Vec3 = [i32; VEC3_LEN];

pub fn default_vec3() -> Vec3 {
    [0; 3]
}

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_default_vector() {
        let def_vec: Vec<i32> = vec![0, 0, 0];
        assert_eq!(def_vec, default_vec3());
    }
    #[test]
    fn check_vector_sum() {
        assert_eq!([2, 2, 3], vec3_vector_sum([1, 1, 2], [1, 1, 1]));
    }
    #[test]
    fn check_scalar_sum() {
        assert_eq!(8, vec3_scalar_sum([0, 2, 1], [1, 1, 3]));
    }
}
