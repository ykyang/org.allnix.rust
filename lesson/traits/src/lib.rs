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

pub trait HasArea {
    fn area(&self) -> f64;
    fn is_larger(&self, &Self) -> bool;
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
    
    fn is_larger(&self, rhs: &Self) -> bool {
        self.area() > rhs.area()
    }
}

pub fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
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
        let circle = Circle {x: 0., y: 0., r: 2.,};
        print_area(circle);
    }
}


