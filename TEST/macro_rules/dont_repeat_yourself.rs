// Don't Repeat Yourself
use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // tt: token tree (operators and tokens)
    ($a: expr, $b: expr, $func: expr, $op: tt) => {
        assert!($a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($v.len(),));
    };
}
macro_rules! op {
    ($func: ident, $bound: ident, $op: tt, $method: ident) => {
        fn $func<T: $bound<T, Ouput=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}