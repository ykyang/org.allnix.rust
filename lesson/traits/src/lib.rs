use std::f64;

#[macro_export]
macro_rules! assert_eqf64 {
    ($a: expr,$b: expr) => ({
        match (&($a), &($b)) {
            (l,r) => {
                let abs_l = l.abs();
                let abs_r = r.abs();
                let diff = (*l - *r).abs();

                let ans: bool;

                if *l == *r {
                    ans = true;
                } else if *l == 0.0 || *r == 0.0 || diff < f64::MIN_POSITIVE {
                    // One of a or b is zero (or both are extremely close to it,) use absolute error.
                    ans = diff < (f64::EPSILON * f64::MIN_POSITIVE);
                } else {
                    // Use relative error.
                    ans = (diff / f64::min(abs_l + abs_r, f64::MAX)) < f64::EPSILON;
                }

                if !ans {
                    panic!("assertion failed: `(left == right)` (left: `{}`, right: `{}`)", *l, *r);
                }
            }
        }
    })
}

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(PartialEq, Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        f64::consts::PI * self.r.powi(2)
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        self.area()
    }
}

pub fn eq(a: f64, b: f64) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    let ans: bool;

    if a == b {
        // Handle infinities.
        ans = true
    } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        ans = diff < (f64::EPSILON * f64::MIN_POSITIVE);
    } else {
        // Use relative error.
        ans = (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON;
    }

    if !ans {
        println!("Not equal: {} != {}", a, b);
    }

    ans
}

pub fn assert_eq(a: f64, b: f64) {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    let ans: bool;

    if a == b {
        // Handle infinities.
        ans = true
    } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        ans = diff < (f64::EPSILON * f64::MIN_POSITIVE);
    } else {
        // Use relative error.
        ans = (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON;
    }

    if !ans {
        panic!("assertion failed: `(left == right)` (left: `{}`, right: `{}`)", a, b)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);
    }

    #[test]
    fn circle() {
        let circle = Circle {x: 0., y: 0., r: 1.,};
        assert_eqf64!(circle.area(), f64::consts::PI);
        let circle = Circle {x: 0., y: 0., r: 2.,};
        assert_eqf64!(circle.area(), f64::consts::PI * 2. * 2.);
    }
}


