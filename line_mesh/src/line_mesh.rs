use glium::backend::Facade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::vertex::VertexBuffer;

use cgmath::prelude::*;
use cgmath::{Point3, Vector4};

use line_vertex::LineVertex;
use line_buffer::LineBuffer;

pub struct LineMesh {
  pub points: Vec<Point3<f32>>,
  pub colors: Vec<Vector4<f32>>,
  pub color: Vector4<f32>,
}

impl LineMesh {
  pub fn new() -> LineMesh {
    LineMesh {
      points: Vec::new(),
      colors: Vec::new(),
      color: Vector4::zero(),
    }
  }

  pub fn set_color(&mut self, color: Vector4<f32>) {
    self.color = color;
  }

  pub fn add_point(&mut self, pt: Point3<f32>) {
    self.points.push(pt);
    self.colors.push(self.color);
  }

  pub fn append_point(&mut self, pt: Point3<f32>) {
    if let Some(& last_point) = self.points.last() {
      self.add_point(last_point);
    } else {
      // Double up the initial point
      self.add_point(pt);
    }
    self.add_point(pt);
  }

  pub fn append_segment(&mut self, pt_a: Point3<f32>, pt_b: Point3<f32>) {
    self.add_point(pt_a);
    self.add_point(pt_b);
  }

  pub fn move_to(&mut self, pt: Point3<f32>) {
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
