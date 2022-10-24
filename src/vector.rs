use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vect2d(pub f64, pub f64);

impl Vect2d {
    pub fn new() -> Vect2d {
      Vect2d(0.0, 0.0)
    }
    fn dot(a: &Vect2d, b: &Vect2d) -> f64 {
        a.0 * b.0 + a.1 * b.1
    }
    fn cross(a: &Vect2d, b: &Vect2d) -> f64 {
        a.0 * b.1 - a.1 * b.0
    }
    fn length(v: &Vect2d) -> f64 {
        v.0.hypot(v.1)
    }
    // TODO: check if this is legit
    fn to_radians(v: &Vect2d) -> f64 {
        v.0.atan2(0.0 - v.1)
    }
    fn from_radians(rad: f64) -> Vect2d {
        Vect2d(rad.sin(), rad.cos())
    }
    fn from_degrees(angle: f64) -> Vect2d {
        let theta = (angle * std::f64::consts::PI) / 180.0;
        Vect2d(theta.cos(), theta.sin())
    }
    fn normalise(v: &Vect2d) -> Vect2d {
        let l = Vect2d::length(&v);
        if l == 0.0 {
            return Vect2d(v.0, v.1);
        }
        let factor = 1.0 / l;
        let v2 = v.clone();
        v2 * factor
    }
    fn distance(a: Vect2d, b: Vect2d) -> f64 {
        let v = a - b;
        Vect2d::length(&v)
    }
}

impl ops::Add<Vect2d> for Vect2d {
    type Output = Vect2d;
    fn add(self, _rhs: Vect2d) -> Vect2d {
        Vect2d(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}
impl ops::Sub<Vect2d> for Vect2d {
    type Output = Vect2d;
    fn sub(self, _rhs: Vect2d) -> Vect2d {
        Vect2d(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}
impl ops::Mul<f64> for Vect2d {
    type Output = Vect2d;
    fn mul(self, _rhs: f64) -> Vect2d {
        Vect2d(self.0 * _rhs, self.1 * _rhs)
    }
}
impl ops::MulAssign<f64> for Vect2d {
    fn mul_assign(&mut self, _rhs: f64) {
        self.0 *= _rhs;
        self.1 *= _rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vect2d(1.0, 2.0);
        let b = Vect2d(3.0, 4.5);

        let left = a + b;
        let right = Vect2d(4.0, 6.5);

        assert_eq!(left, right);
    }
    #[test]
    fn test_subtract() {
        let a = Vect2d(1.0, 2.0);
        let b = Vect2d(3.0, 4.5);

        let left = a - b;
        let right = Vect2d(-2.0, -2.5);

        assert_eq!(left, right);
    }
    #[test]
    fn test_mult() {
        let a = Vect2d(1.0, 2.0);

        let left = a * 2.0;
        let right = Vect2d(2.0, 4.0);

        assert_eq!(left, right);
    }
    #[test]
    fn test_dot() {
        let a = Vect2d(1.0, 2.0);
        let b = Vect2d(3.0, 4.5);
        assert_eq!(Vect2d::dot(&a, &b), 12.0);
    }
    #[test]
    fn test_cross() {
        let a = Vect2d(1.0, 2.0);
        let b = Vect2d(3.0, 4.5);
        assert_eq!(Vect2d::cross(&a, &b), -1.5);
    }
    #[test]
    fn test_length() {
        let v = Vect2d(3.0, 4.0);
        assert_eq!(Vect2d::length(&v), 5.0);
    }
    #[test]
    fn test_to_radians() {
        let v = Vect2d(3.0, 4.0);
        // same as node
        assert_eq!(Vect2d::to_radians(&v), 2.4980917);
    }
    #[test]
    fn test_from_radians() {
        // same as node
        let v = Vect2d(-0.9589243, 0.2836622);
        assert_eq!(Vect2d::from_radians(5.0), v);
    }
    /*
    #[test]
    fn test_to_from_radians() {
        let v = Vect2d::from_radians(4.0);
        let r = Vect2d::to_radians(&v);
        assert_eq!(r, 4.0);
    }
    */
    #[test]
    fn test_from_degrees() {
        let v = Vect2d(0.9961947, 0.087155744);
        assert_eq!(Vect2d::from_degrees(5.0), v);
    }
    #[test]
    fn test_normalise() {
        let v = Vect2d(5.0, 5.0);
        assert_eq!(Vect2d::normalise(&v), Vect2d(0.7071068, 0.7071068));
    }
    #[test]
    fn test_distance() {
        let a = Vect2d(1.0, 1.0);
        let b = Vect2d(4.0, 5.0);
        assert_eq!(Vect2d::distance(a, b), 5.0);
    }
}
