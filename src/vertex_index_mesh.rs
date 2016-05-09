use glium::backend::Facade;
use glium::index::{PrimitiveType};

use cgmath::prelude::*;

use defs::*;
use vertex::Vertex;
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
    self.add_vertex(Vertex::from_pos(point));
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

  pub fn recompute_normals(&mut self) {
    for tri in self.indices.chunks(3) {
      if tri.len() != 3 { continue; }
      let (i0, i1, i2) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);

      let apex = self.vertices[i0].pos();
      let s0 = apex - self.vertices[i1].pos();
      let s1 = apex - self.vertices[i2].pos();

      let normal = s0.cross(s1).normalize();

      for & i in & [i0, i1, i2] {
        let norm_i = self.vertices[i].normal();
        self.vertices[i].set_normal(norm_i + normal);
      }
    }

    for vert in self.vertices.iter_mut() {
      vert.normalize_normal();
    }
  }

  pub fn to_buffer<T: Facade>(& self, gl: & T) -> BufferSet {
    BufferSet::from_vertex_index(gl, self.primtype, self)
  }
}
