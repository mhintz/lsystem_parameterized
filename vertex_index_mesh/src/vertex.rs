use cgmath::*;

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

  pub fn pos_and_color(pos: & [f32; 3], color: & [f32; 4]) -> Vertex {
    Vertex {
      a_color: *color,
      a_pos: *pos,
      a_norm: [0.0; 3],
      a_tex: [0.0; 2],
    }
  }

  pub fn pos_and_tex(pos: & [f32; 3], tex: & [f32; 2]) -> Vertex {
    Vertex {
      a_color: [0.0; 4],
      a_pos: *pos,
      a_norm: [0.0; 3],
      a_tex: *tex,
    }
  }

  pub fn from(pos: Point3<f32>, norm: Vector3<f32>, color: Vector4<f32>, tex: Vector2<f32>) -> Vertex {
    Vertex {
      a_color: color.into(),
      a_pos: pos.into(),
      a_norm: norm.into(),
      a_tex: tex.into(),
    }
  }

  pub fn from_pos(pos: Point3<f32>) -> Vertex {
    Vertex {
      a_color: [0.0; 4],
      a_pos: pos.into(),
      a_norm: [0.0; 3],
      a_tex: [0.0; 2],
    }
  }

  pub fn from_pos_and_color(pos: Point3<f32>, color: Vector4<f32>) -> Vertex {
    Vertex {
      a_color: color.into(),
      a_pos: pos.into(),
      a_norm: [0.0; 3],
      a_tex: [0.0; 2],
    }
  }

  pub fn from_pos_and_tex(pos: Point3<f32>, tex: Vector2<f32>) -> Vertex {
    Vertex {
      a_color: [0.0; 4],
      a_pos: pos.into(),
      a_norm: [0.0; 3],
      a_tex: tex.into(),
    }
  }

  pub fn pos(&self) -> Point3<f32> { Point3::from(self.a_pos) }

  pub fn color(&self) -> Vector4<f32> { Vector4::from(self.a_color) }

  pub fn normal(&self) -> Vector3<f32> { Vector3::from(self.a_norm) }

  pub fn tex(&self) -> Vector2<f32> { Vector2::from(self.a_tex) }

  pub fn set_pos(&mut self, pos: Point3<f32>) { self.a_pos = pos.into() }

  pub fn set_color(&mut self, color: Vector4<f32>) { self.a_color = color.into() }

  pub fn set_normal(&mut self, norm: Vector3<f32>) { self.a_norm = norm.into() }

  pub fn normalize_normal(&mut self) { self.a_norm = Vector3::from(self.a_norm).normalize().into() }

  pub fn set_tex(&mut self, tex: Vector2<f32>) { self.a_tex = tex.into() }
}
