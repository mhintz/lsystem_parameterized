use glium::index::PrimitiveType;
use cgmath::*;

use defs::*;
use matrixstack;
use lsystem::{Module, DrawCommand};
use line_mesh::LineMesh;
use vertex_index_mesh::{VertexIndexMesh, Vertex};

pub fn ls_to_lines(word: &[Module]) -> LineMesh {
  let mut line = LineMesh::new();

  // lsystem moves by default in the positive-y direction
  let base_heading = Vec3::new(0.0, 1.0, 0.0);
  let mut mat_stack: matrixstack::MatrixStack<f32> = matrixstack::MatrixStack::new();

  // Starting point
  line.append_point(Pt::origin());

  for item in word {
    match item.to_draw_command() {
      DrawCommand::Segment { w: _, l: length } => {
        mat_stack.transform(Matrix4::from_translation(base_heading * length));
        line.append_point(mat_stack.origin());
      },
      DrawCommand::Forward { d: distance } => {
        mat_stack.transform(Matrix4::from_translation(base_heading * distance));
        line.move_to(mat_stack.origin());
      },
      DrawCommand::Roll { r } => {
        mat_stack.rotate(Matrix3::from_angle_z(Rad::new(r)));
      },
      DrawCommand::Pitch { r } => {
        mat_stack.rotate(Matrix3::from_angle_x(Rad::new(r)));
      },
      DrawCommand::Yaw { r } => {
        mat_stack.rotate(Matrix3::from_angle_y(Rad::new(r)));
      },
      DrawCommand::Euler { x, y, z } => {
        mat_stack.rotate(Matrix3::from(Euler::new(Rad::new(x), Rad::new(y), Rad::new(z))));
      },
      DrawCommand::Push => {
        mat_stack.push();
        line.move_to(mat_stack.origin());
      },
      DrawCommand::Pop => {
        mat_stack.pop();
        line.move_to(mat_stack.origin());
      },
      DrawCommand::None => (),
    }
  }

  return line;
}

fn cylinder(start: Pt, end: Pt, facets: u32, radius: f32) -> VertexIndexMesh {
  if facets < 2 { return VertexIndexMesh::new(PrimitiveType::TrianglesList); }

  let rot_angle = Rad::full_turn() / (facets as f32);
  let offset_angle = rot_angle / 2.0;

  let stem_axis = (end - start).normalize();
  // If the vector happens to be the x axis, the cross product won't work
  let cross_vec = if stem_axis == Vec3::unit_x() { Vec3::unit_y() } else { Vec3::unit_x() };
  // let perp_vec = stem_axis.cross(cross_vec).normalize_to(radius);
  let perp_vec = stem_axis.cross(cross_vec).normalize_to(radius);

  let mut mesh = VertexIndexMesh::new(PrimitiveType::TrianglesList);

  let half_step = Mat3::from_axis_angle(stem_axis, offset_angle);
  let full_step = Mat3::from_axis_angle(stem_axis, rot_angle);
  let one_and_one_half_step = Mat3::from_axis_angle(stem_axis, rot_angle + offset_angle);

  let base_point = start + perp_vec;
  let top_point = end + (half_step * perp_vec);
  let next_point = start + (full_step * perp_vec);
  let top_next_point = end + (one_and_one_half_step * perp_vec);

  let base_struct = vec![start, base_point, next_point, top_point, top_next_point, end];
  // bottom, left tri, right tri, top
  let base_indices: Vec<usize> = vec![0, 2, 1, 1, 2, 3, 2, 4, 3, 3, 4, 5];

  for base_num in 0..facets {
    let base_mult = base_num as f32;
    let rot_matrix = Mat4::from_translation(start.to_vec()) * Mat4::from_axis_angle(stem_axis, rot_angle * base_mult) * Mat4::from_translation(-start.to_vec());
    let rotated = transform_points(& base_struct, rot_matrix);
    for & idx in & base_indices {
      mesh.add_vertex(Vertex::pos_only(rotated[idx].as_ref()));
    }
  }

  return mesh;
}

fn transform_points(points: & [Pt], transform: Mat4) -> Vec<Pt> {
  points.iter().map(|pt| Pt::from_vec((transform * pt.to_homogeneous()).truncate())).collect()
}

pub fn ls_to_cylinders(word: & [Module]) -> VertexIndexMesh {
  let mut mesh = VertexIndexMesh::new(PrimitiveType::TrianglesList);

  // lsystem moves by default in the positive-y direction
  let base_heading = Vec3::new(0.0, 1.0, 0.0);
  let mut mat_stack: matrixstack::MatrixStack<f32> = matrixstack::MatrixStack::new();

  for item in word {
    match item.to_draw_command() {
      DrawCommand::Segment { w: width, l: length } => {
        let start = mat_stack.origin();
        mat_stack.transform(Matrix4::from_translation(base_heading * length));
        let end = mat_stack.origin();
        mesh.extend_with(& cylinder(start, end, 8, width / 2.0));
      },
      DrawCommand::Forward { d: distance } => {
        mat_stack.transform(Matrix4::from_translation(base_heading * distance));
      },
      DrawCommand::Pitch { r } => {
        mat_stack.rotate(Matrix3::from_angle_x(Rad::new(r)));
      },
      DrawCommand::Yaw { r } => {
        mat_stack.rotate(Matrix3::from_angle_y(Rad::new(r)));
      },
      DrawCommand::Roll { r } => {
        mat_stack.rotate(Matrix3::from_angle_z(Rad::new(r)));
      },
      DrawCommand::Euler { x, y, z } => {
        mat_stack.rotate(Matrix3::from(Euler::new(Rad::new(x), Rad::new(y), Rad::new(z))));
      },
      DrawCommand::Push => {
        mat_stack.push();
      },
      DrawCommand::Pop => {
        mat_stack.pop();
      },
      DrawCommand::None => (),
    }
  }

  return mesh;
}
