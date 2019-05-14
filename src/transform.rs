use num_traits::{Float};

use std::marker::PhantomData;

use crate::traits::{Algebra};


pub trait Transform<T: Float, A: Algebra<T>> {
    fn apply(&self, x: A) -> A;
}

pub trait Chain<T: Float, A: Algebra<T>> {
    fn chain(&self, other: &Self) -> Self;
}

pub mod prelude {
    pub use crate::transform::{Transform, Chain};
}

#[derive(Clone, Debug)]
pub struct Moebius<T: Float, A: Algebra<T>> {
    pub a: A,
    pub b: A,
    pub c: A,
    pub d: A,
    pd: PhantomData<T>,
}

impl<T: Float, A: Algebra<T>> Moebius<T, A> {
    pub fn new(a: A, b: A, c: A, d: A) -> Self {
        Self { a, b, c, d, pd: PhantomData }
    }
}

impl<T: Float, A: Algebra<T>> Chain<T, A> for Moebius<T, A> {
    fn chain(&self, other: &Self) -> Self {
        Self::new(
            self.a*other.a + self.b*other.c,
            self.a*other.b + self.b*other.d,
            self.c*other.a + self.d*other.c,
            self.c*other.b + self.d*other.d,
        )
    }
}

impl<T: Float, A: Algebra<T>> Transform<T, A> for Moebius<T, A> {
    fn apply(&self, x: A) -> A {
        (self.a*x + self.b)/(self.c*x + self.d)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Complex, Quaternion, Octonion};

    use rand::{prelude::*, Rng};
    use rand::distributions::StandardNormal;
    use rand_xorshift::XorShiftRng;

    use assert_approx_eq::assert_approx_eq;

    const TRANSFORM_ATTEMPTS: usize = 64;
    const POINT_ATTEMPTS: usize = 16;


    struct TestRng {
        base: XorShiftRng,
    }
    impl TestRng {
        fn new() -> Self {
            Self { base: XorShiftRng::seed_from_u64(0xdeadbeef) }
        }
        fn sample(&mut self) -> f64 {
            self.base.sample(StandardNormal)
        }
    }

    trait TestRand {
        fn random(rng: &mut TestRng) -> Self;
    }
    impl TestRand for f64 {
        fn random(rng: &mut TestRng) -> Self {
            rng.sample()
        }
    }
    impl TestRand for Complex<f64> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new2(f64::random(rng), f64::random(rng))
        }
    }
    impl TestRand for Quaternion<f64> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new2(Complex::random(rng), Complex::random(rng))
        }
    }
    impl TestRand for Octonion<f64> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new2(Quaternion::random(rng), Quaternion::random(rng))
        }
    }
    impl TestRand for Moebius<f64, Complex<f64>> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new(
                Complex::random(rng),
                Complex::random(rng),
                Complex::random(rng),
                Complex::random(rng),
            )
        }
    }
    impl TestRand for Moebius<f64, Quaternion<f64>> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new(
                Quaternion::random(rng),
                Quaternion::random(rng),
                Quaternion::random(rng),
                Quaternion::random(rng),
            )
        }
    }
    impl TestRand for Moebius<f64, Octonion<f64>> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new(
                Octonion::random(rng),
                Octonion::random(rng),
                Octonion::random(rng),
                Octonion::random(rng),
            )
        }
    }

    #[test]
    fn moebius2() {
        let mut rng = TestRng::new();
        for _ in 0..TRANSFORM_ATTEMPTS {
            let a = Moebius::random(&mut rng);
            let b = Moebius::random(&mut rng);
            let c = a.chain(&b);
            for _ in 0..POINT_ATTEMPTS {
                let x = Complex::random(&mut rng);
                let y = a.apply(b.apply(x));
                let z = c.apply(x);
                assert_approx_eq!(y, z);
            }
        }
    }

    #[test]
    fn moebius4() {
        let mut rng = TestRng::new();
        for _ in 0..TRANSFORM_ATTEMPTS {
            let a = Moebius::random(&mut rng);
            let b = Moebius::random(&mut rng);
            let c = a.chain(&b);
            for _ in 0..POINT_ATTEMPTS {
                let x = Quaternion::random(&mut rng);
                let y = a.apply(b.apply(x));
                let z = c.apply(x);
                assert_approx_eq!(y, z);
            }
        }
    }

    /// Moebuis transform over octonions isn't chainable and therefore should fail
    #[test]
    #[should_panic]
    fn moebius8() {
        let mut rng = TestRng::new();
        for _ in 0..TRANSFORM_ATTEMPTS {
            let a = Moebius::random(&mut rng);
            let b = Moebius::random(&mut rng);
            let c = a.chain(&b);
            for _ in 0..POINT_ATTEMPTS {
                let x = Octonion::random(&mut rng);
                let y = a.apply(b.apply(x));
                let z = c.apply(x);
                assert_approx_eq!(y, z);
            }
        }
    }
}