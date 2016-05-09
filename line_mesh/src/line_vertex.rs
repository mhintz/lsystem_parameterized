use cgmath::{Point3, Vector4};

#[derive(Copy, Clone, Debug)]
#[repr="C"]
pub struct LineVertex {
  a_color: [f32; 4],
  a_pos: [f32; 3],
}

implement_vertex!(LineVertex, a_pos, a_color);

impl LineVertex {
  pub fn new(pos: & [f32; 3], color: & [f32; 4]) -> LineVertex {
    LineVertex {
      a_color: *color,
      a_pos: *pos,
    }
  }

  pub fn from(pos: Point3<f32>, color: Vector4<f32>) -> LineVertex {
    LineVertex {
      a_color: color.into(),
      a_pos: pos.into(),
    }
  }
}
