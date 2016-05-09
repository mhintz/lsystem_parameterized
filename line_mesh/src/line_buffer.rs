use glium::backend::Facade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::vertex::VertexBuffer;

use line_vertex::LineVertex;

pub struct LineBuffer {
  pub vertices: VertexBuffer<LineVertex>,
  pub indices: IndexBuffer<u32>,
}

impl LineBuffer {
  pub fn new <T: Facade> (gl: & T, primtype: PrimitiveType) -> LineBuffer {
    LineBuffer {
      vertices: VertexBuffer::<LineVertex>::empty(gl, 0).unwrap(),
      indices: IndexBuffer::<u32>::empty(gl, primtype, 0).unwrap(),
    }
  }
}
