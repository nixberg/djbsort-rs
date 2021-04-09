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
generate_constant_time_sort!(u128);

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
generate_minmax!(u128);

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
generate_gt_mask!(u128, 128);

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::ConstantTimeSort;
    use crate::GreaterThanMask;

    macro_rules! sort_random {
        ($name:ident, $t:ty, $n:expr) => {
            #[test]
            fn $name() {
                for count in 0..$n {
                    let mut vec: Vec<$t> = rand::thread_rng()
                        .sample_iter(rand::distributions::Standard)
                        .take(count)
                        .collect();
                    let mut expected = vec.to_vec();
                    vec.ct_sort();
                    expected.sort();
                    assert_eq!(vec, expected);
                }
            }
        };
    }

    sort_random!(sort_u8_random, u8, 1024);
    sort_random!(sort_u16_random, u16, 1024);
    sort_random!(sort_u32_random, u32, 1024);
    sort_random!(sort_u64_random, u64, 1024);
    sort_random!(sort_u128_random, u128, 1024);

    #[test]
    fn gt_mask_u8_exhaustive() {
        for lhs in 0..=u8::MAX {
            for rhs in 0..=u8::MAX {
                assert_eq!(lhs.gt_mask(rhs), if lhs > rhs { u8::MAX } else { 0 });
            }
        }
    }

    macro_rules! gt_mask_random {
        ($name:ident, $t:ty, $n:expr) => {
            #[test]
            fn $name() {
                for _ in 0..($n) {
                    let lhs: $t = rand::thread_rng().gen();
                    let rhs: $t = rand::thread_rng().gen();
                    assert_eq!(lhs.gt_mask(rhs), if lhs > rhs { <$t>::MAX } else { 0 });
                }
            }
        };
    }

    gt_mask_random!(gt_mask_u16_random, u16, 1024);
    gt_mask_random!(gt_mask_u32_random, u32, 1024);
    gt_mask_random!(gt_mask_u64_random, u64, 1024);
    gt_mask_random!(gt_mask_u128_random, u128, 1024);
}
