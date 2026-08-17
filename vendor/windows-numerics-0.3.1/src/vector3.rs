use super::*;

impl Vector3 {
    pub fn new(X: f32, Y: f32, Z: f32) -> Self {
        Self { X, Y, Z }
    }
    pub fn zero() -> Self {
        Self {
            X: 0f32,
            Y: 0f32,
            Z: 0f32,
        }
    }
    pub fn one() -> Self {
        Self {
            X: 1f32,
            Y: 1f32,
            Z: 1f32,
        }
    }
    pub fn unit_x() -> Self {
        Self {
            X: 1.0,
            Y: 0.0,
            Z: 0.0,
        }
    }
    pub fn unit_y() -> Self {
        Self {
            X: 0.0,
            Y: 1.0,
            Z: 0.0,
        }
    }
    pub fn unit_z() -> Self {
        Self {
            X: 0.0,
            Y: 0.0,
            Z: 1.0,
        }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.X * rhs.X + self.Y * rhs.Y + self.Z * rhs.Z
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    #[cfg(feature = "std")]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    #[cfg(feature = "std")]
    pub fn distance(&self, value: &Self) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Self) -> f32 {
        (self - value).length_squared()
    }
    #[cfg(feature = "std")]
    pub fn normalize(&self) -> Self {
        self / self.length()
    }

    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            X: self.X + rhs.X,
            Y: self.Y + rhs.Y,
            Z: self.Z + rhs.Z,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            X: self.X - rhs.X,
            Y: self.Y - rhs.Y,
            Z: self.Z - rhs.Z,
        }
    }
    fn impl_div(&self, rhs: &Self) -> Self {
        Self {
            X: self.X / rhs.X,
            Y: self.Y / rhs.Y,
            Z: self.Z / rhs.Z,
        }
    }
    fn impl_div_f32(&self, rhs: f32) -> Self {
        Self {
            X: self.X / rhs,
            Y: self.Y / rhs,
            Z: self.Z / rhs,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            X: self.X * rhs.X,
            Y: self.Y * rhs.Y,
            Z: self.Z * rhs.Z,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            X: self.X * rhs,
            Y: self.Y * rhs,
            Z: self.Z * rhs,
        }
    }
}

impl core::ops::Add<Self> for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Self> for Vector3 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        self.impl_add(rhs)
    }
}
impl core::ops::Add<Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, rhs: &Vector3) -> Vector3 {
        self.impl_add(rhs)
    }
}
impl core::ops::Sub<Self> for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Self> for Vector3 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        self.impl_sub(rhs)
    }
}
impl core::ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: &Vector3) -> Vector3 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Div<Self> for Vector3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Self> for Vector3 {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<Vector3> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Vector3 {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: &Vector3) -> Vector3 {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Div<f32> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Vector3 {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Mul<Self> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Self> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<Vector3> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: &Vector3) -> Vector3 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        self.impl_mul_f32(rhs)
    }
}
impl core::ops::Mul<f32> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        self.impl_mul_f32(rhs)
    }
}
