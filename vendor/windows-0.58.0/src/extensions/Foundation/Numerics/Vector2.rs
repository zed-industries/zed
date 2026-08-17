use crate::Foundation::Numerics::Vector2;

impl Vector2 {
    pub fn new(X: f32, Y: f32) -> Self {
        Self { X, Y }
    }
    pub fn zero() -> Self {
        Self { X: 0f32, Y: 0f32 }
    }
    pub fn one() -> Self {
        Self { X: 1f32, Y: 1f32 }
    }
    pub fn unit_x() -> Self {
        Self { X: 1.0, Y: 0.0 }
    }
    pub fn unit_y() -> Self {
        Self { X: 0.0, Y: 1.0 }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.X * rhs.X + self.Y * rhs.Y
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn distance(&self, value: &Self) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Self) -> f32 {
        (self - value).length_squared()
    }
    pub fn normalize(&self) -> Self {
        self / self.length()
    }

    fn impl_add(&self, rhs: &Self) -> Self {
        Self { X: self.X + rhs.X, Y: self.Y + rhs.Y }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self { X: self.X - rhs.X, Y: self.Y - rhs.Y }
    }
    fn impl_div(&self, rhs: &Self) -> Self {
        Self { X: self.X / rhs.X, Y: self.Y / rhs.Y }
    }
    fn impl_div_f32(&self, rhs: f32) -> Self {
        Self { X: self.X / rhs, Y: self.Y / rhs }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self { X: self.X * rhs.X, Y: self.Y * rhs.Y }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self { X: self.X * rhs, Y: self.Y * rhs }
    }
}

impl core::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: &Vector2) -> Vector2 {
        self.impl_add(rhs)
    }
}
impl core::ops::Add<Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: &Vector2) -> Vector2 {
        self.impl_add(rhs)
    }
}
impl core::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: &Vector2) -> Vector2 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Sub<Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: &Vector2) -> Vector2 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Div<Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: &Vector2) -> Vector2 {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<Vector2> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: &Vector2) -> Vector2 {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Div<f32> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: &Vector2) -> Vector2 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<Vector2> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: &Vector2) -> Vector2 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        self.impl_mul_f32(rhs)
    }
}
impl core::ops::Mul<f32> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        self.impl_mul_f32(rhs)
    }
}
