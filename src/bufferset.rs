use glium::backend::Facade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::vertex::VertexBuffer;

use defs::*;
use vertex::{Vertex, LineVertex};
use vertex_index_mesh::VertexIndexMesh;

pub struct BufferSet {
  pub vertices: VertexBuffer<Vertex>,
  pub indices: IndexBuffer<Idx>
}

impl BufferSet {
  pub fn new <T: Facade> (gl: & T, primtype: PrimitiveType) -> BufferSet {
    BufferSet {
      vertices: VertexBuffer::<Vertex>::empty(gl, 0).unwrap(),
      indices: IndexBuffer::<u32>::empty(gl, primtype, 0).unwrap(),
    }
  }

  pub fn from_vertex_index <T: Facade> (gl: & T, primtype: PrimitiveType, mesh: & VertexIndexMesh) -> BufferSet {
      BufferSet {
        vertices: VertexBuffer::<Vertex>::new(gl, & mesh.vertices).unwrap(),
        indices: IndexBuffer::<u32>::new(gl, primtype, & mesh.indices).unwrap(),
      }
  }
}

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
