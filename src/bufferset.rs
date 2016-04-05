use glium::backend::Facade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::vertex::VertexBuffer;

use defs::*;

#[derive(Copy, Clone, Debug)]
#[repr="C"]
pub struct Vert {
  a_pos: [f32; 3],
  a_norm: [f32; 3],
}

implement_vertex!(Vert, a_pos, a_norm);

impl Vert {
  pub fn new(pos: & [f32; 3], norm: & [f32; 3]) -> Vert {
    Vert {
      a_pos: *pos,
      a_norm: *norm,
    }
  }

  pub fn pos_only(pos: & [f32; 3]) -> Vert {
    Vert {
      a_pos: *pos,
      a_norm: [0.0; 3],
    }
  }
}

pub struct BufferSet {
  pub vertices: VertexBuffer<Vert>,
  pub indices: IndexBuffer<u32>
}

impl BufferSet {
  pub fn new <T> (gl: & T, primtype: PrimitiveType) -> BufferSet
  where T: Facade {
    BufferSet {
      indices: IndexBuffer::<u32>::empty(gl, primtype, 0).unwrap(),
      vertices: VertexBuffer::<Vert>::empty(gl, 0).unwrap()
    }
  }
}

#[derive(Copy, Clone, Debug)]
#[repr="C"]
pub struct LineVert {
  a_pos: [f32; 3],
}

implement_vertex!(LineVert, a_pos);

impl LineVert {
  pub fn new(pos: & [f32; 3]) -> LineVert {
    LineVert {
      a_pos: *pos
    }
  }
}

pub struct LineMesh {
  vertices: Option<VertexBuffer<LineVert>>,
  indices: Option<IndexBuffer<u32>>,
  pub points: Vec<Pt>,
}

impl LineMesh {
  pub fn new() -> LineMesh {
    LineMesh {
      vertices: None,
      indices: None,
      points: Vec::new(),
    }
  }

  pub fn append_point(&mut self, pt: Pt) {
    if let Some(& last_point) = self.points.last() {
      self.points.push(last_point);
    } else {
      // Double up the initial point
      self.points.push(pt);
    }
    self.points.push(pt);
  }

  pub fn append_segment(&mut self, pt_a: Pt, pt_b: Pt) {
    self.points.push(pt_a);
    self.points.push(pt_b);
  }

  pub fn move_to(&mut self, pt: Pt) {
    self.points.push(pt);
    self.points.push(pt);
  }

  pub fn to_buffer<T: Facade>(& self, gl: & T) -> LineBuffer {
    let vert_storage: Vec<LineVert> = self.points.iter().map(|v| LineVert::new(v.as_ref())).collect();
    let index_storage: Vec<u32> = (0..(self.points.len() as u32)).collect();

    LineBuffer {
      vertices: VertexBuffer::new(gl, & vert_storage).unwrap(),
      indices: IndexBuffer::new(gl, PrimitiveType::LinesList, & index_storage).unwrap(),
    }
  }
}

pub struct LineBuffer {
  pub vertices: VertexBuffer<LineVert>,
  pub indices: IndexBuffer<u32>,
}

