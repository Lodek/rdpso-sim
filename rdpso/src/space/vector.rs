use crate::wasm_bindgen;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
#[derive(Serialize, Deserialize)]
/// Vector implements a simple R3 vector object
pub struct Vector
 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Vector
 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{
            x,
            y,
            z,
        }
    }

    /// unit_x returns a unit vector parallel to the x axis
    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// unit_y returns a unit vector parallel to the y axis
    pub fn unit_y() -> Self {
        Self {
            y: 1.0,
            x: 0.0,
            z: 0.0,
        }
    }

    /// unit_z returns a unit vector parallel to the z axis
    pub fn unit_z() -> Self {
        Self {
            z: 1.0,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn add(&self, other: &Vector
    ) -> Vector
     {
        Vector
         {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn mult(&self, c: f64) -> Vector
     {
        Vector
         {
            x: c * self.x,
            y: c * self.y,
            z: c * self.z,
        }
    }

    pub fn sub(&self, other: &Vector
    ) -> Vector
     {
        Vector
         {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }

    /// rotate rotates a vector by the given angle (in radians)
    pub fn rotate_xy(&self, angle: f64) -> Vector
     {
        let (sin, cos) = (angle.sin(), angle.cos());
        Vector
         {
            x: self.x * cos + self.y * sin,
            y: (-self.x * sin) + self.y * cos,
            z: self.z,
        }
    }

    /// rotate rotates a vector by the given angle (in radians)
    pub fn rotate_xz(&self, angle: f64) -> Vector
     {
        let (sin, cos) = (angle.sin(), angle.cos());
        Vector
         {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: (-self.x * sin) + self.z * cos,
        }
    }
    
    pub fn component_mult(&self, other: &Vector) -> Vector
     {
        Self {
            x: self.x * self.x,
            y: self.y * self.y,
            z: self.z * self.z,
        }
    }

    /// magnitude returns the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// unit transforms self into a unit vector (ie a vec with magnitude 1)
    pub fn unit(&self) -> Vector {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z /magnitude,
        }
    }
}


impl std::ops::Add<Vector
> for Vector
 {
    type Output = Self;

    fn add(self, rhs: Vector
    ) -> Self::Output {
        Vector
        ::add(&self, &rhs)
    }
}

impl std::ops::Sub<Vector
> for Vector
 {
    type Output = Self;

    fn sub(self, rhs: Vector
    ) -> Self::Output {
        Vector
        ::sub(&self, &rhs)
    }
}

impl std::ops::Mul<Vector
> for f64 {
    type Output = Vector
    ;

    fn mul(self, vec: Vector
    ) -> Self::Output{
        Vector
        ::mult(&vec, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_sum() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(3.0, 4.0, 6.0);

        let c = a + b;

        assert_eq!(c, Vector::new(4.0, 6.0, 9.0))
    }

    fn vec_sub() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(3.0, 4.0, 1.0);

        let c = a - b;

        assert_eq!(c, Vector::new(-2.0, -2.0, 2.0))
    }

    fn scalar_mult() {
        let a = Vector::new(1.0, 2.0, 3.0);

        let b = 3.0 * a;

        assert_eq!(b, Vector::new(3.0, 6.0, 9.0))
    }
}
