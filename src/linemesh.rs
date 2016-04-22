use glium::backend::Facade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::vertex::VertexBuffer;

use cgmath::*;

use defs::*;
use vertex::LineVertex;
use bufferset::LineBuffer;

pub struct LineMesh {
  vertices: Option<VertexBuffer<LineVertex>>,
  indices: Option<IndexBuffer<u32>>,
  pub points: Vec<Pt>,
  pub colors: Vec<Vec4>,
  pub color: Vec4,
}

impl LineMesh {
  pub fn new() -> LineMesh {
    LineMesh {
      vertices: None,
      indices: None,
      points: Vec::new(),
      colors: Vec::new(),
      color: Vec4::zero(),
    }
  }

  pub fn set_color(&mut self, color: Vec4) {
    self.color = color;
  }

  pub fn add_point(&mut self, pt: Pt) {
    self.points.push(pt);
    self.colors.push(self.color);
  }

  pub fn append_point(&mut self, pt: Pt) {
    if let Some(& last_point) = self.points.last() {
      self.add_point(last_point);
    } else {
      // Double up the initial point
      self.add_point(pt);
    }
    self.add_point(pt);
  }

  pub fn append_segment(&mut self, pt_a: Pt, pt_b: Pt) {
    self.add_point(pt_a);
    self.add_point(pt_b);
  }

  pub fn move_to(&mut self, pt: Pt) {
    self.add_point(pt);
    self.add_point(pt);
  }

  pub fn to_buffer<T: Facade>(& self, gl: & T) -> LineBuffer {
    let vert_storage: Vec<LineVertex> = self.points.iter()
      .enumerate()
      .map(|(i, v)| LineVertex::from(* v, self.colors[i]))
      .collect();
    let index_storage: Vec<u32> = (0..(self.points.len() as u32)).collect();

    LineBuffer {
      vertices: VertexBuffer::new(gl, & vert_storage).unwrap(),
      indices: IndexBuffer::new(gl, PrimitiveType::LinesList, & index_storage).unwrap(),
    }
  }
}
