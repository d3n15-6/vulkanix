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

    pub const UP: Self = Self::new(0.0, 1.0, 0.0);
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0);
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0);
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0);
    pub const FORWARD: Self = Self::new(0.0, 0.0, -1.0);
    pub const BACKWARD: Self = Self::new(0.0, 0.0, 1.0);

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalized(self) -> Self {
        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            return Self::ZERO;
        }

        Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            return;
        }

        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
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

    #[test]
    fn normalizes_vector() {
        let vector = Vector3::new(3.0, 4.0, 0.0);

        let normalized = vector.normalized();

        assert_eq!(normalized, Vector3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn normalize_modifies_vector() {
        let mut vector = Vector3::new(3.0, 4.0, 0.0);

        vector.normalize();

        assert_eq!(vector, Vector3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn normalizing_zero_returns_zero() {
        assert_eq!(Vector3::ZERO.normalized(), Vector3::ZERO);
    }

    #[test]
    fn normalize_zero_does_nothing() {
        let mut vector = Vector3::ZERO;
        vector.normalize();

        assert_eq!(vector, Vector3::ZERO);
    }

    #[test]
    fn calculates_dot_product() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);

        assert_eq!(a.dot(b), 32.0);
    }

    #[test]
    fn dot_product_of_perpendicular_vectors_is_zero() {
        let right = Vector3::RIGHT;
        let up = Vector3::UP;

        assert_eq!(right.dot(up), 0.0);
    }

    #[test]
    fn dot_producto_of_same_unit_vector_is_one() {
        let forward = Vector3::FORWARD;

        assert_eq!(forward.dot(forward), 1.0);
    }

    #[test]
    fn dot_product_of_opposite_unit_vectors_is_negative_one() {
        let forward = Vector3::FORWARD;
        let backward = Vector3::BACKWARD;

        assert_eq!(forward.dot(backward), -1.0);
    }
}
