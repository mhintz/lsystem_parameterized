use glium::index::PrimitiveType;

use defs::*;
use vertex::*;

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
}
