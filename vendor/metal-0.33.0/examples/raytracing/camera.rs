use glam::f32::Vec4;

#[repr(C)]
pub struct Camera {
    pub position: Vec4,
    pub right: Vec4,
    pub up: Vec4,
    pub forward: Vec4,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec4::new(0.0, 3.0, 10.0, 0.0),
            right: Vec4::new(1.0, 0.0, 0.0, 0.0),
            up: Vec4::new(0.0, 1.0, 0.0, 0.0),
            forward: Vec4::new(0.0, 0.0, -1.0, 0.0),
        }
    }
}
