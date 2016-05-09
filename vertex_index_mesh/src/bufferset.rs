use glium::backend::Facade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::vertex::VertexBuffer;

use vertex::Vertex;
use vertex_index_mesh::VertexIndexMesh;

pub struct BufferSet {
  pub vertices: VertexBuffer<Vertex>,
  pub indices: IndexBuffer<u32>
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
