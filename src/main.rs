#[macro_use]
extern crate glium;
extern crate cgmath;

mod lsystem;
mod bufferset;
mod defs;

use lsystem::*;
use bufferset::*;
use cgmath::{Point, Basis3, Angle, Rad, Rotation, Rotation3};
use defs::*;

const NUM_ITERATIONS: u32 = 1;
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

  let mut pos = Pt::origin();
  // lsystem moves by default in the positive-z direction
  let base_heading = Vec3::new(0.0, 0.0, 1.0);
  let mut matrix: Basis3<f32> = Basis3::one();

  line.append_point(pos);

  for item in word {
    match *item {
      Module::Branch{ w: _, l } => {
        let heading = matrix.rotate_vector(base_heading);
        pos = pos + heading * l;
        line.append_point(pos);
      },
      Module::Forward{ d } => {
        let heading = matrix.rotate_vector(base_heading);
        pos = pos + heading * d;
        line.move_to(pos);
      },
      Module::Roll{ r } => {
        matrix = matrix.concat(& Basis3::from_angle_z(Rad::new(r)));
      },
      Module::Pitch{ r } => {
        matrix = matrix.concat(& Basis3::from_angle_x(Rad::new(r)));
      },
      Module::Yaw{ r } => {
        matrix = matrix.concat(& Basis3::from_angle_y(Rad::new(r)));
      },
      _ => (),
    }
  }

  return line;
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

  println!("{:?}", word);
}
