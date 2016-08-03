use glium::index::PrimitiveType;
use cgmath::*;
use rand;

use defs::*;
use matrixstack;
use lsystem::{Module, DrawCommand};
use line_mesh::LineMesh;
use vertex_index_mesh::{self, VertexIndexMesh, Vertex};
use rand_util;
use convex_hull;
use half_edge_mesh::{HalfEdgeMesh, ToPtrVec};

pub fn ls_to_lines(word: &[Module]) -> LineMesh {
  let mut line = LineMesh::new();

  // lsystem moves by default in the positive-y direction
  let base_heading = Vec3::new(0.0, 1.0, 0.0);
  let mut mat_stack: matrixstack::MatrixStack<f32> = matrixstack::MatrixStack::new();

  // Starting point
  line.append_point(Pt::origin());

  for item in word {
    match item.to_draw_command() {
      DrawCommand::Foliage { .. } => (), // Don't draw foliage in the line version
      DrawCommand::Segment { w: _, l: length } => {
        mat_stack.transform(Matrix4::from_translation(base_heading * length));
        line.append_point(mat_stack.origin());
      },
      DrawCommand::Forward { d: distance } => {
        mat_stack.transform(Matrix4::from_translation(base_heading * distance));
        line.move_to(mat_stack.origin());
      },
      DrawCommand::Roll { r } => {
        mat_stack.rotate(Matrix3::from_angle_z(Rad(r)));
      },
      DrawCommand::Pitch { r } => {
        mat_stack.rotate(Matrix3::from_angle_x(Rad(r)));
      },
      DrawCommand::Yaw { r } => {
        mat_stack.rotate(Matrix3::from_angle_y(Rad(r)));
      },
      DrawCommand::Euler { x, y, z } => {
        mat_stack.rotate(Matrix3::from(Euler::new(Rad(x), Rad(y), Rad(z))));
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

pub fn generate_branch(start: Pt, end: Pt, facets: u32, radius: f32) -> VertexIndexMesh {
  let mut branch_body = cylinder(start, end, facets, radius);

  branch_body = vertex_index_mesh::assign_colors(branch_body, |_, _| {
    let redval = rand_util::random_lohi(25.0_f32, 70.0_f32);
    let ratio_grn = rand_util::random_lohi(1.45_f32, 1.65_f32);
    let ratio_blu = rand_util::random_lohi(3.0_f32, 3.4_f32);
    let (r, g, b) = (redval, redval / ratio_grn, redval / ratio_blu);
    let v = Vec4::new(r / 255.0, g / 255.0, b / 255.0, 1.0);
    [v, v, v]
  });

  branch_body
}

fn to_vertex_index_mesh(half_edge: HalfEdgeMesh) -> VertexIndexMesh {
  let mut mesh = VertexIndexMesh::new(PrimitiveType::TrianglesList);

  for face in half_edge.faces.values() {
    let face_borrow = face.borrow();
    for vert in face_borrow.adjacent_verts().to_ptr_vec() {
      mesh.add_vertex(Vertex::pos_only(vert.borrow().pos.as_ref()));
    }
  }

  mesh
}

pub fn generate_foliage(start: Pt, end: Pt, radius: f32) -> VertexIndexMesh {
  let midpoint = (end.to_vec() + start.to_vec()) / 2.0_f32;
  let points = rand_util::rand_points_in_sphere(&mut rand::thread_rng(), 200, radius);
  let translation = Matrix4::from_translation(start.to_vec());
  let transformed_points = transform_points(& points, translation);
  let hull = convex_hull::get_convex_hull(transformed_points);
  let mut hull_mesh = to_vertex_index_mesh(hull);
  hull_mesh = vertex_index_mesh::assign_colors(hull_mesh, |_, _| {
    let green = Vec4::new(62.0 / 255.0, 117.0 / 255.0, 31.0 / 255.0, 1.0);
    [green, green, green]
  });

  hull_mesh
}

pub fn ls_to_cylinders(word: & [Module]) -> VertexIndexMesh {
  let mut mesh = VertexIndexMesh::new(PrimitiveType::TrianglesList);

  // lsystem moves by default in the positive-y direction
  let base_heading = Vec3::new(0.0, 1.0, 0.0);
  let mut mat_stack: matrixstack::MatrixStack<f32> = matrixstack::MatrixStack::new();

  for item in word {
    match item.to_draw_command() {
      DrawCommand::Foliage { l: length, r: radius } => {
        let start = mat_stack.origin();
        mat_stack.transform(Matrix4::from_translation(base_heading * length));
        let end = mat_stack.origin();
        mesh.extend_with(& generate_foliage(start, end, radius));
      },
      DrawCommand::Segment { w: width, l: length } => {
        let start = mat_stack.origin();
        mat_stack.transform(Matrix4::from_translation(base_heading * length));
        let end = mat_stack.origin();
        mesh.extend_with(& generate_branch(start, end, 8, width / 2.0));
      },
      DrawCommand::Forward { d: distance } => {
        mat_stack.transform(Matrix4::from_translation(base_heading * distance));
      },
      DrawCommand::Pitch { r } => {
        mat_stack.rotate(Matrix3::from_angle_x(Rad(r)));
      },
      DrawCommand::Yaw { r } => {
        mat_stack.rotate(Matrix3::from_angle_y(Rad(r)));
      },
      DrawCommand::Roll { r } => {
        mat_stack.rotate(Matrix3::from_angle_z(Rad(r)));
      },
      DrawCommand::Euler { x, y, z } => {
        mat_stack.rotate(Matrix3::from(Euler::new(Rad(x), Rad(y), Rad(z))));
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

  mesh = vertex_index_mesh::recompute_normals(mesh);

  return mesh;
}
