use defs::*;

#[derive(Copy, Clone, Debug)]
#[repr="C"]
pub struct Vertex {
  a_color: [f32; 4],
  a_pos: [f32; 3],
  a_norm: [f32; 3],
  a_tex: [f32; 2],
}

implement_vertex!(Vertex, a_color, a_pos, a_norm, a_tex);

impl Vertex {
  pub fn new(pos: & [f32; 3], norm: & [f32; 3], color: & [f32; 4], tex: & [f32; 2]) -> Vertex {
    Vertex {
      a_color: *color,
      a_pos: *pos,
      a_norm: *norm,
      a_tex: *tex,
    }
  }

  pub fn pos_only(pos: & [f32; 3]) -> Vertex {
    Vertex {
      a_color: [0.0; 4],
      a_pos: *pos,
      a_norm: [0.0; 3],
      a_tex: [0.0; 2],
    }
  }

  pub fn from(pos: Pt, norm: Vec3, color: Vec4, tex: Vec2) -> Vertex {
    Vertex {
      a_color: color.into(),
      a_pos: pos.into(),
      a_norm: norm.into(),
      a_tex: tex.into(),
    }
  }

  pub fn from_pos(pos: Pt) -> Vertex {
    Vertex {
      a_color: [0.0; 4],
      a_pos: pos.into(),
      a_norm: [0.0; 3],
      a_tex: [0.0; 2],
    }
  }
}

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

  pub fn from(pos: Pt, color: Vec4) -> LineVertex {
    LineVertex {
      a_color: color.into(),
      a_pos: pos.into(),
    }
  }
}
