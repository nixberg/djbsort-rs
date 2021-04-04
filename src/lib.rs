pub trait ConstantTimeSort {
    fn ct_sort(&mut self);
}

macro_rules! generate_constant_time_sort {
    ($t:ty) => {
        impl ConstantTimeSort for [$t] {
            fn ct_sort(&mut self) {
                use std::iter::successors;

                if let Some(top) = successors(Some(1usize), |t| t.checked_mul(2))
                    .take_while(|t| *t < self.len())
                    .last()
                {
                    for p in
                        successors(Some(top), |p| Some(p.wrapping_shr(1))).take_while(|p| *p > 0)
                    {
                        for i in 0..(self.len() - p) {
                            if i & p == 0 {
                                self.minmax_at(i, i + p);
                            }
                        }
                        successors(Some(top), |t| Some(t.wrapping_shr(1)))
                            .take_while(|q| *q > p)
                            .fold(0usize, |offset, q| {
                                for i in offset..(self.len() - q) {
                                    if i & p == 0 {
                                        self[i + p] =
                                            successors(Some(q), |r| Some(r.wrapping_shr(1)))
                                                .take_while(|r| *r > p)
                                                .fold(self[i + p], |a, r| self.minmax(a, i + r));
                                    }
                                }
                                self.len() - q
                            });
                    }
                }
            }
        }
    };
}

generate_constant_time_sort!(u8);
generate_constant_time_sort!(u16);
generate_constant_time_sort!(u32);
generate_constant_time_sort!(u64);

trait MinMax<T> {
    fn minmax_at(&mut self, i: usize, j: usize);
    fn minmax(&mut self, a: T, j: usize) -> T;
}

macro_rules! generate_minmax {
    ($t:ty) => {
        impl MinMax<$t> for [$t] {
            #[inline(always)]
            fn minmax_at(&mut self, i: usize, j: usize) {
                let a = self[i];
                let b = self[j];
                let swap_operator = (a ^ b) & a.gt_mask(b);
                self[i] = a ^ swap_operator;
                self[j] = b ^ swap_operator;
            }

            #[inline(always)]
            fn minmax(&mut self, a: $t, j: usize) -> $t {
                let b = self[j];
                let swap_operator = (a ^ b) & a.gt_mask(b);
                self[j] = b ^ swap_operator;
                a ^ swap_operator
            }
        }
    };
}

generate_minmax!(u8);
generate_minmax!(u16);
generate_minmax!(u32);
generate_minmax!(u64);

trait GreaterThanMask {
    fn gt_mask(self, other: Self) -> Self;
}

macro_rules! generate_gt_mask {
    ($t:ty, $bits:expr) => {
        impl GreaterThanMask for $t {
            #[inline(never)]
            fn gt_mask(self, other: Self) -> Self {
                let mut result = other.wrapping_sub(self);
                result ^= self;
                result |= other ^ self;
                result ^= other;
                result >>= $bits - 1;
                result.wrapping_neg()
            }
        }
    };
}

generate_gt_mask!(u8, 8);
generate_gt_mask!(u16, 16);
generate_gt_mask!(u32, 32);
generate_gt_mask!(u64, 64);

#[cfg(test)]
mod tests {
    use crate::ConstantTimeSort;
    use crate::GreaterThanMask;
    use rand::Rng;

    #[test]
    fn test_sort_u8() {
        for count in 0..1024 {
            let mut vec: Vec<u8> = rand::thread_rng()
                .sample_iter(rand::distributions::Standard)
                .take(count)
                .collect();
            let mut expected = vec.to_vec();
            vec.ct_sort();
            expected.sort();
            assert_eq!(vec, expected);
        }
    }

    #[test]
    fn test_sort_u16() {
        for count in 0..1024 {
            let mut vec: Vec<u16> = rand::thread_rng()
                .sample_iter(rand::distributions::Standard)
                .take(count)
                .collect();
            let mut expected = vec.to_vec();
            vec.ct_sort();
            expected.sort();
            assert_eq!(vec, expected);
        }
    }

    #[test]
    fn test_sort_u32() {
        for count in 0..1024 {
            let mut vec: Vec<u32> = rand::thread_rng()
                .sample_iter(rand::distributions::Standard)
                .take(count)
                .collect();
            let mut expected = vec.to_vec();
            vec.ct_sort();
            expected.sort();
            assert_eq!(vec, expected);
        }
    }

    #[test]
    fn test_sort_u64() {
        for count in 0..1024 {
            let mut vec: Vec<u64> = rand::thread_rng()
                .sample_iter(rand::distributions::Standard)
                .take(count)
                .collect();
            let mut expected = vec.to_vec();
            vec.ct_sort();
            expected.sort();
            assert_eq!(vec, expected);
        }
    }

    #[test]
    fn test_gt_mask_u8() {
        for lhs in 0..=u8::MAX {
            for rhs in 0..=u8::MAX {
                assert_eq!(lhs.gt_mask(rhs), if lhs > rhs { 0xff } else { 0 });
            }
        }
    }

    #[test]
    fn test_gt_mask_u16() {
        let mut rng = rand::thread_rng();
        for _ in 0..1024 {
            let lhs: u16 = rng.gen();
            let rhs: u16 = rng.gen();
            assert_eq!(lhs.gt_mask(rhs), if lhs > rhs { 0xffff } else { 0 });
        }
    }

    #[test]
    fn test_gt_mask_u32() {
        let mut rng = rand::thread_rng();
        for _ in 0..1024 {
            let lhs: u32 = rng.gen();
            let rhs: u32 = rng.gen();
            assert_eq!(lhs.gt_mask(rhs), if lhs > rhs { 0xffff_ffff } else { 0 });
        }
    }

    #[test]
    fn test_gt_mask_u64() {
        let mut rng = rand::thread_rng();
        for _ in 0..1024 {
            let lhs: u64 = rng.gen();
            let rhs: u64 = rng.gen();
            assert_eq!(
                lhs.gt_mask(rhs),
                if lhs > rhs { 0xffff_ffff_ffff_ffff } else { 0 }
            );
        }
    }
}
