use super::*;

impl Vector4 {
    pub fn new(X: f32, Y: f32, Z: f32, W: f32) -> Self {
        Self { X, Y, Z, W }
    }
    pub fn zero() -> Self {
        Self {
            X: 0f32,
            Y: 0f32,
            Z: 0f32,
            W: 0f32,
        }
    }
    pub fn one() -> Self {
        Self {
            X: 1f32,
            Y: 1f32,
            Z: 1f32,
            W: 1f32,
        }
    }
    pub fn unit_x() -> Self {
        Self {
            X: 1.0,
            Y: 0.0,
            Z: 0.0,
            W: 0.0,
        }
    }
    pub fn unit_y() -> Self {
        Self {
            X: 0.0,
            Y: 1.0,
            Z: 0.0,
            W: 0.0,
        }
    }
    pub fn unit_z() -> Self {
        Self {
            X: 0.0,
            Y: 0.0,
            Z: 1.0,
            W: 0.0,
        }
    }
    pub fn unit_w() -> Self {
        Self {
            X: 0.0,
            Y: 0.0,
            Z: 0.0,
            W: 1.0,
        }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.X * rhs.X + self.Y * rhs.Y + self.Z * rhs.Z + self.W * rhs.W
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
            W: self.W + rhs.W,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            X: self.X - rhs.X,
            Y: self.Y - rhs.Y,
            Z: self.Z - rhs.Z,
            W: self.W - rhs.W,
        }
    }
    fn impl_div(&self, rhs: &Self) -> Self {
        Self {
            X: self.X / rhs.X,
            Y: self.Y / rhs.Y,
            Z: self.Z / rhs.Z,
            W: self.W / rhs.W,
        }
    }
    fn impl_div_f32(&self, rhs: f32) -> Self {
        Self {
            X: self.X / rhs,
            Y: self.Y / rhs,
            Z: self.Z / rhs,
            W: self.W / rhs,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            X: self.X * rhs.X,
            Y: self.Y * rhs.Y,
            Z: self.Z * rhs.Z,
            W: self.W * rhs.W,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            X: self.X * rhs,
            Y: self.Y * rhs,
            Z: self.Z * rhs,
            W: self.W * rhs,
        }
    }
}

impl core::ops::Add<Self> for Vector4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Self> for Vector4 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        self.impl_add(rhs)
    }
}
impl core::ops::Add<Vector4> for &Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Vector4 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn add(self, rhs: &Vector4) -> Vector4 {
        self.impl_add(rhs)
    }
}
impl core::ops::Sub<Self> for Vector4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Self> for Vector4 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        self.impl_sub(rhs)
    }
}
impl core::ops::Sub<Vector4> for &Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Vector4 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: &Vector4) -> Vector4 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Div<Self> for Vector4 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Self> for Vector4 {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<Vector4> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Vector4 {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: &Vector4) -> Vector4 {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<f32> for Vector4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Div<f32> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: f32) -> Vector4 {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Mul<Self> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Self> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<Vector4> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Vector4 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Vector4 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<f32> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        self.impl_mul_f32(rhs)
    }
}
impl core::ops::Mul<f32> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Vector4 {
        self.impl_mul_f32(rhs)
    }
}
