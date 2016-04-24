pub use glm::*;
pub use glm::ext::*;
use defs::GVec3;

pub fn x_axis() -> GVec3 { GVec3::new(1.0, 0.0, 0.0) }

pub fn y_axis() -> GVec3 { GVec3::new(0.0, 1.0, 0.0) }

pub fn z_axis() -> GVec3 { GVec3::new(0.0, 0.0, 1.0) }
