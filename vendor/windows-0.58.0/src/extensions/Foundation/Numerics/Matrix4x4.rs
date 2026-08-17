use crate::Foundation::Numerics::Matrix4x4;

impl Matrix4x4 {
    pub const fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            M11: 1.0,
            M12: 0.0,
            M13: 0.0,
            M14: 0.0,
            M21: 0.0,
            M22: 1.0,
            M23: 0.0,
            M24: 0.0,
            M31: 0.0,
            M32: 0.0,
            M33: 1.0,
            M34: 0.0,
            M41: x,
            M42: y,
            M43: z,
            M44: 1.0,
        }
    }
    pub fn rotation_y(degree: f32) -> Self {
        windows_targets::link!("d2d1.dll" "system" fn D2D1SinCos(angle: f32, sin: *mut f32, cos: *mut f32));
        let angle = degree * (3.141592654 / 180.0);
        let mut sin = 0.0;
        let mut cos = 0.0;
        unsafe {
            D2D1SinCos(angle, &mut sin, &mut cos);
        }
        Self {
            M11: cos,
            M12: 0.0,
            M13: -sin,
            M14: 0.0,
            M21: 0.0,
            M22: 1.0,
            M23: 0.0,
            M24: 0.0,
            M31: sin,
            M32: 0.0,
            M33: cos,
            M34: 0.0,
            M41: 0.0,
            M42: 0.0,
            M43: 0.0,
            M44: 1.0,
        }
    }
    pub fn perspective_projection(depth: f32) -> Self {
        let projection = if depth > 0.0 { -1.0 / depth } else { 0.0 };
        Self {
            M11: 1.0,
            M12: 0.0,
            M13: 0.0,
            M14: 0.0,
            M21: 0.0,
            M22: 1.0,
            M23: 0.0,
            M24: 0.0,
            M31: 0.0,
            M32: 0.0,
            M33: 1.0,
            M34: projection,
            M41: 0.0,
            M42: 0.0,
            M43: 0.0,
            M44: 1.0,
        }
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 + rhs.M11,
            M12: self.M12 + rhs.M12,
            M13: self.M13 + rhs.M13,
            M14: self.M14 + rhs.M14,
            M21: self.M21 + rhs.M21,
            M22: self.M22 + rhs.M22,
            M23: self.M23 + rhs.M23,
            M24: self.M24 + rhs.M24,
            M31: self.M31 + rhs.M31,
            M32: self.M32 + rhs.M32,
            M33: self.M33 + rhs.M33,
            M34: self.M34 + rhs.M34,
            M41: self.M41 + rhs.M41,
            M42: self.M42 + rhs.M42,
            M43: self.M43 + rhs.M43,
            M44: self.M44 + rhs.M44,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 - rhs.M11,
            M12: self.M12 - rhs.M12,
            M13: self.M13 - rhs.M13,
            M14: self.M14 - rhs.M14,
            M21: self.M21 - rhs.M21,
            M22: self.M22 - rhs.M22,
            M23: self.M23 - rhs.M23,
            M24: self.M24 - rhs.M24,
            M31: self.M31 - rhs.M31,
            M32: self.M32 - rhs.M32,
            M33: self.M33 - rhs.M33,
            M34: self.M34 - rhs.M34,
            M41: self.M41 - rhs.M41,
            M42: self.M42 - rhs.M42,
            M43: self.M43 - rhs.M43,
            M44: self.M44 - rhs.M44,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 * rhs.M11 + self.M12 * rhs.M21 + self.M13 * rhs.M31 + self.M14 * rhs.M41,
            M12: self.M11 * rhs.M12 + self.M12 * rhs.M22 + self.M13 * rhs.M32 + self.M14 * rhs.M42,
            M13: self.M11 * rhs.M13 + self.M12 * rhs.M23 + self.M13 * rhs.M33 + self.M14 * rhs.M43,
            M14: self.M11 * rhs.M14 + self.M12 * rhs.M24 + self.M13 * rhs.M34 + self.M14 * rhs.M44,
            M21: self.M21 * rhs.M11 + self.M22 * rhs.M21 + self.M23 * rhs.M31 + self.M24 * rhs.M41,
            M22: self.M21 * rhs.M12 + self.M22 * rhs.M22 + self.M23 * rhs.M32 + self.M24 * rhs.M42,
            M23: self.M21 * rhs.M13 + self.M22 * rhs.M23 + self.M23 * rhs.M33 + self.M24 * rhs.M43,
            M24: self.M21 * rhs.M14 + self.M22 * rhs.M24 + self.M23 * rhs.M34 + self.M24 * rhs.M44,
            M31: self.M31 * rhs.M11 + self.M32 * rhs.M21 + self.M33 * rhs.M31 + self.M34 * rhs.M41,
            M32: self.M31 * rhs.M12 + self.M32 * rhs.M22 + self.M33 * rhs.M32 + self.M34 * rhs.M42,
            M33: self.M31 * rhs.M13 + self.M32 * rhs.M23 + self.M33 * rhs.M33 + self.M34 * rhs.M43,
            M34: self.M31 * rhs.M14 + self.M32 * rhs.M24 + self.M33 * rhs.M34 + self.M34 * rhs.M44,
            M41: self.M41 * rhs.M11 + self.M42 * rhs.M21 + self.M43 * rhs.M31 + self.M44 * rhs.M41,
            M42: self.M41 * rhs.M12 + self.M42 * rhs.M22 + self.M43 * rhs.M32 + self.M44 * rhs.M42,
            M43: self.M41 * rhs.M13 + self.M42 * rhs.M23 + self.M43 * rhs.M33 + self.M44 * rhs.M43,
            M44: self.M41 * rhs.M14 + self.M42 * rhs.M24 + self.M43 * rhs.M34 + self.M44 * rhs.M44,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            M11: self.M11 * rhs,
            M12: self.M12 * rhs,
            M13: self.M13 * rhs,
            M14: self.M14 * rhs,
            M21: self.M21 * rhs,
            M22: self.M22 * rhs,
            M23: self.M23 * rhs,
            M24: self.M24 * rhs,
            M31: self.M31 * rhs,
            M32: self.M32 * rhs,
            M33: self.M33 * rhs,
            M34: self.M34 * rhs,
            M41: self.M41 * rhs,
            M42: self.M42 * rhs,
            M43: self.M43 * rhs,
            M44: self.M44 * rhs,
        }
    }
}

impl core::ops::Add<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_add(rhs)
    }
}
impl core::ops::Add<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_add(rhs)
    }
}
impl core::ops::Sub<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Sub<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: f32) -> Matrix4x4 {
        self.impl_mul_f32(rhs)
    }
}
impl core::ops::Mul<f32> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: f32) -> Matrix4x4 {
        self.impl_mul_f32(rhs)
    }
}
