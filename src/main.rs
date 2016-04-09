#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate arcball_cgmath;

mod lsystem;
mod bufferset;
mod defs;
mod matrixstack;

use std::fs::File;
use std::io::Read;

use glium::glutin;
use glium::glutin::{Event, ElementState};
use glium::{DisplayBuild, Surface};

use cgmath::*;

use lsystem::*;
use bufferset::*;
use defs::*;
use matrixstack::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const ASPECT_RATIO: f32 = (WINDOW_WIDTH as f32) / (WINDOW_HEIGHT as f32);
const NEAR_PLANE_Z: f32 = 0.001;
const FAR_PLANE_Z: f32 = 1000.0;

const NUM_ITERATIONS: u32 = 3;
static RAD_60: f32 = 60.0 / 180.0 * std::f32::consts::PI;
static RAD_120: f32 = 120.0 / 180.0 * std::f32::consts::PI;

/// Implement custom versions of this function to produce new chains of modules from an existing module
pub fn rule_produce(component: Module) -> Vec<Module> {
  match component {
    Module::Branch { w, l } => {
      vec![
        branch(w, l / 3.0),
        pitch(RAD_60),
        branch(w, l / 3.0),
        pitch(-RAD_120),
        branch(w, l / 3.0),
        pitch(RAD_60),
        branch(w, l / 3.0),
      ]
    },
    Module::None => vec![component],
    _ => vec![component],
  }
}

pub fn ls_to_lines(word: &[Module]) -> LineMesh {
  let mut line = LineMesh::new();

  // lsystem moves by default in the positive-y direction
  let base_heading = Vec3::new(0.0, 1.0, 0.0);
  let mut mat_stack: MatrixStack<f32> = MatrixStack::new();

  // Starting point
  line.append_point(Pt::origin());

  for item in word {
    match *item {
      Module::Branch { w: _, l: length } => {
        mat_stack.transform(Matrix4::from_translation(base_heading * length));
        line.append_point(mat_stack.transform_point(Point3::new(0.0, 0.0, 0.0)));
      },
      Module::Forward { d: distance } => {
        mat_stack.transform(Matrix4::from_translation(base_heading * distance));
        line.move_to(mat_stack.transform_point(Point3::new(0.0, 0.0, 0.0)));
      },
      Module::Roll { r } => {
        mat_stack.rotate(Matrix3::from_angle_z(Rad::new(r)));
      },
      Module::Pitch { r } => {
        mat_stack.rotate(Matrix3::from_angle_x(Rad::new(r)));
      },
      Module::Yaw { r } => {
        mat_stack.rotate(Matrix3::from_angle_y(Rad::new(r)));
      },
      Module::Reverse => {
        mat_stack.rotate(Matrix3::from_angle_y(Rad::new(180.0_f32.to_radians())));
      },
      Module::Push => {
        mat_stack.push();
      },
      Module::Pop => {
        mat_stack.pop();
      },
      Module::None => (),
    }
  }

  return line;
}

fn mat4_uniform(mat: & Mat4) -> [[f32; 4]; 4] {
  return mat.clone().into();
}

fn main() {
  let mut word: Vec<Module> = vec![
    branch(1.0, 27.0),
    pitch((-120.0_f32).to_radians()),
    branch(1.0, 27.0),
    pitch((-120.0_f32).to_radians()),
    branch(1.0, 27.0),
  ];

  for _ in 0..NUM_ITERATIONS {
    let mut collector: Vec<Module> = vec![];
    for letter in word {
      collector.extend(rule_produce(letter));
    }
    word = collector;
  }

  let line_struct = ls_to_lines(& word);

  let mut cube_struct = LineMesh::new();

  cube_struct.append_point(Pt::new(0.0, 0.0, 0.0));
  cube_struct.append_point(Pt::new(1.0, 0.0, 0.0));
  cube_struct.append_point(Pt::new(1.0, 1.0, 0.0));
  cube_struct.append_point(Pt::new(0.0, 1.0, 0.0));
  cube_struct.append_point(Pt::new(0.0, 1.0, -1.0));
  cube_struct.append_point(Pt::new(1.0, 1.0, -1.0));
  cube_struct.append_point(Pt::new(1.0, 0.0, -1.0));
  cube_struct.append_point(Pt::new(0.0, 0.0, -1.0));

  // OpenGL setup
  let window = glutin::WindowBuilder::new()
    .with_depth_buffer(24)
    .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
    .with_title("L System".to_string())
    .build_glium().unwrap();

  let mut mouse_pos: Vector2<f32> = Vector2::new(0.0, 0.0);
  let mut pan_button_pressed: bool = false;

  let line_buffer = line_struct.to_buffer(& window);

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
      u_model_world: mat4_uniform(& model_position),
      u_world_cam: mat4_uniform(& arc_camera_mat),
      u_projection: mat4_uniform(& perspective_projection),
    };

    // Draw

    target.draw(& line_buffer.vertices, & line_buffer.indices, & basic_program, & basic_uniforms, & draw_params).unwrap();

    // line_struct.draw(& target, & basic_program, & basic_uniforms, & draw_params);

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
