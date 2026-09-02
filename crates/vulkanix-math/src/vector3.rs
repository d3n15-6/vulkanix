use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);


}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(
            - self.x,
            - self.y,
            - self.z,
        )
    }
}

impl Mul<f32>for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }
}

#[cfg(test)]
mod test {

use super::*;

    #[test]
    fn creates_vector3() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
    }

    #[test]
    fn zero_vector() {
        assert_eq!(Vector3::ZERO, Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn one_vector() {
        assert_eq!(Vector3::ONE, Vector3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn adds_vectors() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);

        let result = a + b;

        assert_eq!(result, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn subtracts_vectors() {
        let a = Vector3::new(10.0, 8.0, 6.0);
        let b = Vector3::new(4.0, 3.0, 2.0);

        let result = a - b;

        assert_eq!(result, Vector3::new(6.0, 5.0, 4.0));
    }

    #[test]
    fn negates_vectors() {
        let vector = Vector3::new(1.0, -2.0, 3.0);

        assert_eq!(-vector, Vector3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplie_vector_by_scalar() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        let result = vector * 5.0;

        assert_eq!(result, Vector3::new(5., 10.0, 15.0));
    }

    #[test]
    fn multiplying_by_zero_produces_zero() {
        let vector = Vector3::new(10.0, 20.0, 30.20);

        assert_eq!(vector * 0.0, Vector3::ZERO);
    }

    #[test]
    fn multiplying_by_one_preserves_vector() {
        let vector = Vector3::new(10.0, 20.0, 30.0);

        assert_eq!(vector * 1.0, vector);
    }
}
