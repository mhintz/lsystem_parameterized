#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate rand;

extern crate arcball_cgmath;
extern crate matrixstack;
extern crate vertex_index_mesh;
extern crate line_mesh;
extern crate convex_hull;
extern crate half_edge_mesh;

mod lsystem;
mod defs;
mod trees;
mod rand_util;
mod draw_helpers;

use std::fs::File;
use std::io::Read;

use glium::glutin;
use glium::glutin::{Event, ElementState};
use glium::{DisplayBuild, Surface};
use glium::backend::Facade;

use cgmath::*;

use lsystem::{run_system};
use trees::*;
use defs::*;
use draw_helpers::{ls_to_lines, ls_to_cylinders};
use line_mesh::LineBuffer;
use vertex_index_mesh::BufferSet;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const ASPECT_RATIO: f32 = (WINDOW_WIDTH as f32) / (WINDOW_HEIGHT as f32);
const NEAR_PLANE_Z: f32 = 0.001;
const FAR_PLANE_Z: f32 = 10000.0;

fn get_file_string(filename: & str) -> String {
  let mut file_obj = File::open(filename).unwrap();
  let mut storage = String::new();
  file_obj.read_to_string(&mut storage).unwrap();
  storage
}

fn gen_new_tree<T: Facade>(gl: & T) -> (LineBuffer, BufferSet) {
  let tree_system = RoundTree {
    base_width: 0.15,
    trunk_base_length: 0.1,
    branch_base_length: 1.0,
    base_foliage_radius: 0.5,
    base_foliage_length: 1.0,
  };
  let tree_produced = run_system(tree_system, 5);
  (ls_to_lines(& tree_produced).to_buffer(gl), ls_to_cylinders(& tree_produced).to_buffer(gl))
}

fn main() {
  // OpenGL setup
  let window = glutin::WindowBuilder::new()
    .with_depth_buffer(24)
    .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
    .with_title("L System".to_string())
    .build_glium().unwrap();

  let (_, mut mesh_buffer) = gen_new_tree(& window);

  // Shader Program
  // let basic_program = glium::Program::from_source(& window, & get_file_string("src/shader/base.vs"), & get_file_string("src/shader/base.fs"), None).unwrap();
  let flat_shaded_program = glium::Program::from_source(& window, & get_file_string("src/shader/base.vs"), & get_file_string("src/shader/flatshaded.fs"), None).unwrap();

  // Matrices
  let mut camera: arcball_cgmath::ArcballCamera<f32> = arcball_cgmath::ArcballCamera::new();
  camera.set_distance(30.0)
    .set_spin_speed(5.0);
  let model_position = Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0));
  let perspective_projection: Mat4 = cgmath::perspective(cgmath::Deg::new(36.0), ASPECT_RATIO, NEAR_PLANE_Z, FAR_PLANE_Z);

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

  // Controls
  let mut mouse_pos: Vector2<f32> = Vector2::new(0.0, 0.0);
  let mut pan_button_pressed: bool = false;
  let dpifactor = window.get_window().unwrap().hidpi_factor();

  loop {
    let mut target = window.draw();

    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    // Uniforms
    let basic_uniforms = uniform! {
      u_model_world: conv::array4x4(model_position),
      u_world_cam: conv::array4x4(camera.get_transform_mat()),
      u_projection: conv::array4x4(perspective_projection),
      u_cam_pos: conv::array3(camera.get_position()),
    };

    // Draw

    // target.draw(& line_buffer.vertices, & line_buffer.indices, & basic_program, & basic_uniforms, & draw_params).unwrap();

    target.draw(& mesh_buffer.vertices, & mesh_buffer.indices, & flat_shaded_program, & basic_uniforms, & draw_params).unwrap();

    target.finish().unwrap();

    for event in window.poll_events() {
      match event {
        Event::Closed => return,
        Event::KeyboardInput(ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => return,
        Event::KeyboardInput(ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Space)) => {
          mesh_buffer = gen_new_tree(& window).1;
        },
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
