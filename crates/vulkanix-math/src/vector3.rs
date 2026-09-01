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
}
