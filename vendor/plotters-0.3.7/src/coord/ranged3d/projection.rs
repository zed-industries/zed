use std::f64::consts::PI;
use std::ops::Mul;

/// The projection matrix which is used to project the 3D space to the 2D display panel
#[derive(Clone, Debug, Copy)]
pub struct ProjectionMatrix([[f64; 4]; 4]);

impl AsRef<[[f64; 4]; 4]> for ProjectionMatrix {
    fn as_ref(&self) -> &[[f64; 4]; 4] {
        &self.0
    }
}

impl AsMut<[[f64; 4]; 4]> for ProjectionMatrix {
    fn as_mut(&mut self) -> &mut [[f64; 4]; 4] {
        &mut self.0
    }
}

impl From<[[f64; 4]; 4]> for ProjectionMatrix {
    fn from(data: [[f64; 4]; 4]) -> Self {
        ProjectionMatrix(data)
    }
}

impl Default for ProjectionMatrix {
    fn default() -> Self {
        ProjectionMatrix::rotate(PI, 0.0, 0.0)
    }
}

impl Mul<ProjectionMatrix> for ProjectionMatrix {
    type Output = ProjectionMatrix;
    fn mul(self, other: ProjectionMatrix) -> ProjectionMatrix {
        let mut ret = ProjectionMatrix::zero();
        for r in 0..4 {
            for c in 0..4 {
                for k in 0..4 {
                    ret.0[r][c] += other.0[r][k] * self.0[k][c];
                }
            }
        }
        ret.normalize();
        ret
    }
}

impl Mul<(i32, i32, i32)> for ProjectionMatrix {
    type Output = (i32, i32);
    fn mul(self, (x, y, z): (i32, i32, i32)) -> (i32, i32) {
        let (x, y, z) = (x as f64, y as f64, z as f64);
        let m = self.0;
        (
            (x * m[0][0] + y * m[0][1] + z * m[0][2] + m[0][3]) as i32,
            (x * m[1][0] + y * m[1][1] + z * m[1][2] + m[1][3]) as i32,
        )
    }
}

impl Mul<(f64, f64, f64)> for ProjectionMatrix {
    type Output = (i32, i32);
    fn mul(self, (x, y, z): (f64, f64, f64)) -> (i32, i32) {
        let m = self.0;
        (
            (x * m[0][0] + y * m[0][1] + z * m[0][2] + m[0][3]) as i32,
            (x * m[1][0] + y * m[1][1] + z * m[1][2] + m[1][3]) as i32,
        )
    }
}

impl ProjectionMatrix {
    /// Returns the identity matrix
    pub fn one() -> Self {
        ProjectionMatrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    /// Returns the zero maxtrix
    pub fn zero() -> Self {
        ProjectionMatrix([[0.0; 4]; 4])
    }
    /// Returns the matrix which shift the coordinate
    pub fn shift(x: f64, y: f64, z: f64) -> Self {
        ProjectionMatrix([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    /// Returns the matrix which rotates the coordinate
    #[allow(clippy::many_single_char_names)]
    pub fn rotate(x: f64, y: f64, z: f64) -> Self {
        let (c, b, a) = (x, y, z);
        ProjectionMatrix([
            [
                a.cos() * b.cos(),
                a.cos() * b.sin() * c.sin() - a.sin() * c.cos(),
                a.cos() * b.sin() * c.cos() + a.sin() * c.sin(),
                0.0,
            ],
            [
                a.sin() * b.cos(),
                a.sin() * b.sin() * c.sin() + a.cos() * c.cos(),
                a.sin() * b.sin() * c.cos() - a.cos() * c.sin(),
                0.0,
            ],
            [-b.sin(), b.cos() * c.sin(), b.cos() * c.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    /// Returns the matrix that applies a scale factor
    pub fn scale(factor: f64) -> Self {
        ProjectionMatrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0 / factor],
        ])
    }
    /// Normalize the matrix, this will make the metric unit to 1
    pub fn normalize(&mut self) {
        if self.0[3][3] > 1e-20 {
            for r in 0..4 {
                for c in 0..4 {
                    self.0[r][c] /= self.0[3][3];
                }
            }
        }
    }

    /// Get the distance of the point in guest coordinate from the screen in pixels
    pub fn projected_depth(&self, (x, y, z): (i32, i32, i32)) -> i32 {
        let r = &self.0[2];
        (r[0] * x as f64 + r[1] * y as f64 + r[2] * z as f64 + r[3]) as i32
    }
}

/// The helper struct to build a projection matrix
#[derive(Copy, Clone)]
pub struct ProjectionMatrixBuilder {
    /// Specifies the yaw of the 3D coordinate system
    pub yaw: f64,
    /// Specifies the pitch of the 3D coordinate system
    pub pitch: f64,
    /// Specifies the scale of the 3D coordinate system
    pub scale: f64,
    pivot_before: (i32, i32, i32),
    pivot_after: (i32, i32),
}

impl Default for ProjectionMatrixBuilder {
    fn default() -> Self {
        Self {
            yaw: 0.5,
            pitch: 0.15,
            scale: 1.0,
            pivot_after: (0, 0),
            pivot_before: (0, 0, 0),
        }
    }
}

impl ProjectionMatrixBuilder {
    /// Creates a new, default projection matrix builder object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the pivot point, which means the 3D coordinate "before" should be mapped into
    /// the 2D coordinatet "after"
    pub fn set_pivot(&mut self, before: (i32, i32, i32), after: (i32, i32)) -> &mut Self {
        self.pivot_before = before;
        self.pivot_after = after;
        self
    }

    /// Build the matrix based on the configuration
    pub fn into_matrix(self) -> ProjectionMatrix {
        let mut ret = if self.pivot_before == (0, 0, 0) {
            ProjectionMatrix::default()
        } else {
            let (x, y, z) = self.pivot_before;
            ProjectionMatrix::shift(-x as f64, -y as f64, -z as f64) * ProjectionMatrix::default()
        };

        if self.yaw.abs() > 1e-20 {
            ret = ret * ProjectionMatrix::rotate(0.0, self.yaw, 0.0);
        }

        if self.pitch.abs() > 1e-20 {
            ret = ret * ProjectionMatrix::rotate(self.pitch, 0.0, 0.0);
        }

        if (self.scale - 1.0).abs() > 1e-20 {
            ret = ret * ProjectionMatrix::scale(self.scale);
        }

        if self.pivot_after != (0, 0) {
            let (x, y) = self.pivot_after;
            ret = ret * ProjectionMatrix::shift(x as f64, y as f64, 0.0);
        }

        ret
    }
}
