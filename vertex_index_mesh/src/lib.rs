#[macro_use]
extern crate glium;
extern crate cgmath;

mod vertex_index_mesh;
mod bufferset;
mod vertex;

pub use vertex_index_mesh::VertexIndexMesh;
pub use bufferset::BufferSet;
pub use vertex::Vertex;

use cgmath::prelude::*;
use cgmath::Vector4;

pub fn recompute_normals(mut mesh: VertexIndexMesh) -> VertexIndexMesh {
  for tri in mesh.indices.chunks(3) {
    if tri.len() != 3 { continue; }
    let (i0, i1, i2) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);

    let apex = mesh.vertices[i0].pos();
    let s0 = apex - mesh.vertices[i1].pos();
    let s1 = apex - mesh.vertices[i2].pos();

    let normal = s0.cross(s1).normalize();

    for & i in & [i0, i1, i2] {
      let norm_i = mesh.vertices[i].normal();
      mesh.vertices[i].set_normal(norm_i + normal);
    }
  }

  for vert in mesh.vertices.iter_mut() {
    vert.normalize_normal();
  }

  mesh
}

pub fn assign_colors<F>(mut mesh: VertexIndexMesh, get_colors: F) -> VertexIndexMesh where
F: Fn((usize, usize, usize), (& Vertex, & Vertex, & Vertex)) -> [Vector4<f32>; 3]
 {
  for tri in mesh.indices.chunks(3) {
    if tri.len() != 3 { continue; }
    let (i0, i1, i2) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);

    let colors = get_colors((i0, i1, i2), (& mesh.vertices[i0], & mesh.vertices[i1], & mesh.vertices[i2]));

    for (i, & vindex) in [i0, i1, i2].into_iter().enumerate() {
      mesh.vertices[vindex].set_color(colors[i]);
    }
  }

  mesh
}
