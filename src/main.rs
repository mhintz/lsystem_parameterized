#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate arcball_cgmath;
extern crate matrixstack;
extern crate glm;

mod lsystem;
mod bufferset;
mod defs;
mod vertex;
mod linemesh;
mod vertex_index_mesh;
mod math;

use std::fs::File;
use std::io::Read;

use glium::glutin;
use glium::glutin::{Event, ElementState};
use glium::{DisplayBuild, Surface};
use glium::index::PrimitiveType;

use cgmath::*;

use lsystem::*;
use bufferset::*;
use defs::*;
use linemesh::LineMesh;
use vertex::Vertex;
use vertex_index_mesh::VertexIndexMesh;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const ASPECT_RATIO: f32 = (WINDOW_WIDTH as f32) / (WINDOW_HEIGHT as f32);
const NEAR_PLANE_Z: f32 = 0.001;
const FAR_PLANE_Z: f32 = 1000.0;

const NUM_ITERATIONS: u32 = 6;

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

pub fn cylinder(start: Pt, end: Pt, facets: u32, radius: f32) -> VertexIndexMesh {
  if facets < 2 { return VertexIndexMesh::new(PrimitiveType::TrianglesList); }

  let rot_angle = Rad::full_turn() / (facets as f32);
  let offset_angle = rot_angle / 2.0;

  let stem_vec = end - start;
  // If the vector happens to be the x axis, the cross product won't work
  let cross_vec = if stem_vec == Vec3::unit_x() { Vec3::unit_y() } else { Vec3::unit_x() };
  let perp_vec = stem_vec.cross(cross_vec).normalize_to(radius);

  let mut mesh = VertexIndexMesh::new(PrimitiveType::TrianglesList);

  let stem_axis = stem_vec.normalize();
  let full_step = Mat3::from_axis_angle(stem_axis, rot_angle);
  let half_step = Mat3::from_axis_angle(stem_axis, offset_angle);
  let one_and_one_half_step = Mat3::from_axis_angle(stem_axis, rot_angle + offset_angle);

  let base_point = start + perp_vec;
  let top_point = end + half_step * perp_vec;
  let next_point = start + full_step * perp_vec;
  let top_next_point = end + one_and_one_half_step * perp_vec;

  let base_struct = vec![base_point, next_point, top_point, top_next_point];
  let base_indexes = vec![0, 1, 2, 1, 3, 2]; // Two triangles

  for base_num in 0..facets {
    let base_mult = base_num as f32;
    let rot_matrix = Mat3::from_axis_angle(stem_axis, rot_angle * base_mult);
    for point in rotate_points(& base_struct, rot_matrix) {
      mesh.add_vertex(Vertex::pos_only(point.as_ref()));
    }
  }

  return mesh;
}

fn rotate_points(points: & [Pt], transform: Mat3) -> Vec<Pt> {
  points.iter().map(|& pt| Pt::from_vec(transform * pt.to_vec())).collect()
}

fn transform_points(points: & [Pt], transform: Mat4) -> Vec<Pt> {
  points.iter().map(|pt| Pt::from_vec((transform * pt.to_homogeneous()).truncate())).collect()
}

fn mat4_uniform(mat: & Mat4) -> [[f32; 4]; 4] {
  return mat.clone().into();
}

fn main() {
  // let koch_system = KochCurve {};
  // let koch_produced = run_system(& koch_system, NUM_ITERATIONS);
  // let koch_line_struct = ls_to_lines(& koch_produced);
  //
  // let dragon_system = DragonCurve {};
  // let dragon_produced = run_system(& dragon_system, NUM_ITERATIONS);
  // let dragon_line_struct = ls_to_lines(& dragon_produced);

  // let tree_system = BasicTree {};
  // let tree_produced = run_system(& tree_system, NUM_ITERATIONS);
  // let tree_line_struct = ls_to_lines(& tree_produced);

  let tree_system = BranchingTree {};
  let tree_produced = run_system(& tree_system, 10);
  let tree_line_struct = ls_to_lines(& tree_produced);

  // OpenGL setup
  let window = glutin::WindowBuilder::new()
    .with_depth_buffer(24)
    .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
    .with_title("L System".to_string())
    .build_glium().unwrap();

  let mut mouse_pos: Vector2<f32> = Vector2::new(0.0, 0.0);
  let mut pan_button_pressed: bool = false;

  // let line_buffer = koch_line_struct.to_buffer(& window);
  // let line_buffer = dragon_line_struct.to_buffer(& window);
  let line_buffer = tree_line_struct.to_buffer(& window);

  // Vertex Shader
  let mut vert_shader_file = File::open("src/shader/base.vs").unwrap();
  let mut vert_shader = String::new();
  vert_shader_file.read_to_string(&mut vert_shader).unwrap();

  // Fragment Shader
  let mut frag_shader_file = File::open("src/shader/base.fs").unwrap();
  let mut frag_shader = String::new();
  frag_shader_file.read_to_string(&mut frag_shader).unwrap();

  // Shader Program
  let basic_program = glium::Program::from_source(& window, & vert_shader, & frag_shader, None).unwrap();

  // Matrices
  let mut camera: arcball_cgmath::ArcballCamera<f32> = arcball_cgmath::ArcballCamera::new();
  camera.set_distance(30.0)
    .set_spin_speed(5.0);
  let model_position = Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0));
  let perspective_projection: Mat4 = cgmath::perspective(cgmath::Deg::new(40.0), ASPECT_RATIO, NEAR_PLANE_Z, FAR_PLANE_Z);

  let draw_params = glium::draw_parameters::DrawParameters {
    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
    depth: glium::Depth {
      test: glium::DepthTest::IfLess,
      write: true,
      ..Default::default()
    },
    line_width: Some(5.0),
    .. Default::default()
  };

  loop {
    let mut target = window.draw();

    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    let arc_camera_mat = camera.get_transform_mat();

    let basic_uniforms = uniform! {
      u_model_world: conv::array4x4(model_position),
      u_world_cam: conv::array4x4(arc_camera_mat),
      u_projection: conv::array4x4(perspective_projection),
    };

    // Draw

    target.draw(& line_buffer.vertices, & line_buffer.indices, & basic_program, & basic_uniforms, & draw_params).unwrap();

    // koch_line_struct.draw(& target, & basic_program, & basic_uniforms, & draw_params);

    target.finish().unwrap();

    for event in window.poll_events() {
      match event {
        Event::Closed => return,
        Event::KeyboardInput(ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => return,
        Event::MouseInput(ElementState::Pressed, glutin::MouseButton::Left) => {
          if pan_button_pressed {
            camera.pan_start(mouse_pos);
          } else {
            camera.rotate_start(mouse_pos);
          }
        },
        Event::MouseInput(ElementState::Released, glutin::MouseButton::Left) => {
          camera.rotate_end();
          camera.pan_end();
        },
        Event::KeyboardInput(ElementState::Pressed, _, Some(glutin::VirtualKeyCode::LAlt))
        | Event::KeyboardInput(ElementState::Pressed, _, Some(glutin::VirtualKeyCode::RAlt)) => {
          pan_button_pressed = true;
        },
        Event::KeyboardInput(ElementState::Released, _, Some(glutin::VirtualKeyCode::LAlt))
        | Event::KeyboardInput(ElementState::Released, _, Some(glutin::VirtualKeyCode::RAlt)) => {
          pan_button_pressed = false;
          camera.pan_end();
        },
        Event::MouseMoved((mousex, mousey)) => {
          // let dpifactor = window.get_window().unwrap().hidpi_factor();
          let dpifactor = 2.0;

          let norm_x = 2.0 * ((mousex as f32) / dpifactor / (WINDOW_WIDTH as f32)) - 1.0;
          // Note that the sign is reversed
          let norm_y = -(2.0 * ((mousey as f32) / dpifactor / (WINDOW_HEIGHT as f32)) - 1.0);
          mouse_pos = Vector2::new(norm_x, norm_y);

          camera.update(mouse_pos);
        },
        Event::MouseWheel(glutin::MouseScrollDelta::PixelDelta(_, dy)) => {
          camera.zoom(dy);
        },
        Event::MouseWheel(glutin::MouseScrollDelta::LineDelta(_, dy)) => {
          camera.zoom(dy * 10.0);
        },
        _ => (),
      }
    }
  }
}
