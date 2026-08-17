//! This module contains the most common traits used in `cgmath`. By
//! glob-importing this module, you can avoid the need to import each trait
//! individually, while still being selective about what types you import.

pub use structure::*;

pub use rotation::Rotation;
pub use rotation::Rotation2;
pub use rotation::Rotation3;

pub use transform::Transform;
pub use transform::Transform2;
pub use transform::Transform3;
