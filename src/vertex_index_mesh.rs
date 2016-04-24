use glium::backend::Facade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::vertex::VertexBuffer;

use defs::*;
use vertex::*;
use bufferset::BufferSet;

pub struct VertexIndexMesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<Idx>,
  pub primtype: PrimitiveType,
}

impl VertexIndexMesh {
  pub fn new(primtype: PrimitiveType) -> VertexIndexMesh {
    VertexIndexMesh {
      vertices: Vec::new(),
      indices: Vec::new(),
      primtype: primtype,
    }
  }

  pub fn add_point(&mut self, point: Pt) {
    self.add_vertex(Vertex::pos_only(point.as_ref()));
  }

  pub fn add_vertex(&mut self, vert: Vertex) {
    self.vertices.push(vert);
    let last = self.indices.len() as u32;
    self.indices.push(last);
  }

  /// Assumes that the primitive type is the same
  pub fn extend_with(&mut self, other: & VertexIndexMesh) {
    let num_verts = self.vertices.len() as u32;
    for vert in & other.vertices {
      self.vertices.push(* vert);
    }
    for index in & other.indices {
      self.indices.push(num_verts + index);
    }
  }

  pub fn to_buffer<T: Facade>(& self, gl: & T) -> BufferSet {
    BufferSet::from_vertex_index(gl, self.primtype, self)
  }
}
