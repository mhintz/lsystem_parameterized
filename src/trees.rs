use std::{f32, f64};
use cgmath::num_traits::Float;

use lsystem::*;
use rand_util::{random_max, random_lohi};

const PHI: f32 = 1.61803398875;
const PHI_RECIP: f32 = 1.0 / PHI;
const PHI_COMPLEMENT: f32 = 1.0 - PHI_RECIP;
const PI: f32 = f32::consts::PI;

#[derive(Copy, Clone)]
pub struct KochCurve;

impl LSystem for KochCurve {
  type Module = Module;

  fn axiom(& self) -> Vec<Module> {
    vec![
      branch(1.0, 27.0, 1),
      pitch((-120.0_f32).to_radians()),
      branch(1.0, 27.0, 1),
      pitch((-120.0_f32).to_radians()),
      branch(1.0, 27.0, 1),
    ]
  }

  fn produce(& self, module: Module) -> Vec<Module> {
    match module {
      Module::Branch { w, l, life } => {
        vec![
          branch(w, l / 3.0, life),
          pitch(60.0_f32.to_radians()),
          branch(w, l / 3.0, life),
          pitch(-120.0_f32.to_radians()),
          branch(w, l / 3.0, life),
          pitch(60.0_f32.to_radians()),
          branch(w, l / 3.0, life),
        ]
      },
      _ => vec![module],
    }
  }
}

#[derive(Copy, Clone)]
pub struct DragonCurve;

impl LSystem for DragonCurve {
  type Module = Module;

  fn axiom(& self) -> Vec<Module> {
    vec![
      custom(1, segment_cmd(1.0, 1.0))
    ]
  }

  fn produce(& self, module: Module) -> Vec<Module> {
    match module {
      Module::Custom(1, _) => vec![custom(1, segment_cmd(1.0, 1.0)), roll(-90.0_f32.to_radians()), custom(2, segment_cmd(1.0, 1.0))],
      Module::Custom(2, _) => vec![custom(1, segment_cmd(1.0, 1.0)), roll(90.0_f32.to_radians()), custom(2, segment_cmd(1.0, 1.0))],
      _ => vec![module],
    }
  }
}

#[derive(Copy, Clone)]
pub struct BasicTree;

impl LSystem for BasicTree {
  type Module = Module;

  fn axiom(& self) -> Vec<Module> {
    vec![
      branch(1.0, 2.0, 1),
      trunk_apex(0),
    ]
  }

  fn produce(& self, module: Module) -> Vec<Module> {
    match module {
      Module::Branch { w: width, l: length, life } => vec![branch(width, length * 1.15, life)],
      Module::TrunkApex { .. } => { // Trunk apex
        vec![
          // Left branch
          push(),
          roll(30.0_f32.to_radians()),
          branch(1.0, 1.0, 1),
          custom_none(2),
          pop(),
          push(),
          roll(-30.0_f32.to_radians()),
          branch(1.0, 1.0, 1),
          custom_none(3),
          pop(),
          branch(1.0, 1.0, 1),
          yaw(60.0_f32.to_radians()),
          trunk_apex(0),
        ]
      },
      Module::BranchApex { .. } => vec![module],
      Module::Custom(2, _) => { // Branch apex left
        vec![
          roll(25.0_f32.to_radians()),
          branch(1.0, 1.0, 1),
          custom_none(2),
        ]
      },
      Module::Custom(3, _) => { // Branch apex right
        vec![
          roll(-25.0_f32.to_radians()),
          branch(1.0, 1.0, 1),
          custom_none(3),
        ]
      },
      _ => vec![module],
    }
  }
}

#[derive(Copy, Clone)]
pub struct BranchingTree {
  pub base_width: f32,
  pub base_length: f32,
}

impl LSystem for BranchingTree {
  type Module = Module;

  fn axiom(& self) -> Vec<Module> {
    vec![
      trunk(self.base_width, 2.0 * self.base_length, 5),
      trunk_apex(0),
    ]
  }

  fn produce(& self, module: Module) -> Vec<Module> {
      match module {
          Module::Branch { w, l, life } | Module::Trunk { w, l, life } => {
              let (new_width, new_length, new_life) = if life > 0 {
                (w * 1.3, l * 1.3, life - 1)
              } else {
                (w, l, 0)
              };
              vec![branch(new_width, new_length, new_life)]
          },
          Module::TrunkApex { .. } => vec![
            push(),
            roll(-30.0_f32.to_radians()),
            trunk(self.base_width, self.base_length, 3),
            custom_none(1),
            pop(),
            push(),
            roll(30.0_f32.to_radians()),
            branch(self.base_width, self.base_length, 3),
            custom_none(1),
            pop(),
            yaw((PHI_RECIP * 360.0_f32).to_radians()),
            branch(self.base_width, self.base_length, 3),
            trunk_apex(0),
          ],
          Module::BranchApex { .. } => vec![module],
          Module::Custom(1, _) => vec![
            push(),
            roll(-30.0_f32.to_radians()),
            branch(self.base_width, 0.85 * self.base_length, 2),
            pop(),
            push(),
            roll(30.0_f32.to_radians()),
            branch(self.base_width, 0.85 * self.base_length, 2),
            pop(),
            yaw((PHI_COMPLEMENT * 360.0_f32).to_radians()),
            branch(self.base_width, 0.85 * self.base_length, 2),
            custom_none(1),
          ],
          _ => vec![module],
      }
  }
}

#[derive(Copy, Clone)]
pub struct RoundTree {
  pub base_width: f32,
  pub trunk_base_length: f32,
  pub branch_base_length: f32,
}

impl LSystem for RoundTree {
  type Module = Module;

  fn axiom(&self) -> Vec<Module> {
    let max_rot = PI / 32.0;
    vec![
      euler(random_max(max_rot), random_max(max_rot), random_max(max_rot)),
      trunk(self.base_width, self.trunk_base_length, 0),
      trunk_apex(0),
    ]
  }

  fn produce(&self, module: Module) -> Vec<Module> {
    let max_trunk_rot = PI / 200.0;
    let min_branch_rot = -PI / 8.0;
    let max_branch_rot = PI / 8.0;
    let branch_angle_min = PI * 1.0 / 4.0;
    let branch_angle_max = PI * 1.0 / 3.0;
    match module {
      Module::Trunk { w, l, life } => {
        vec![
          trunk(w, l * 1.2, life),
          euler(random_max(max_trunk_rot), random_max(max_trunk_rot), random_max(max_trunk_rot)),
          trunk(w, self.trunk_base_length, 0),
        ]
      },
      Module::Branch { w, l, life } => {
        vec![
          if life > 0 {
            branch(w, l * 1.1, life - 1)
          } else {
            module
          }
        ]
      },
      Module::TrunkApex { .. } => {
        vec![
          yaw((PHI * 30.0_f32).to_radians()),
          // yaw((90.0_f32).to_radians()),
          push(),
          roll(-random_lohi(branch_angle_min, branch_angle_max)),
          branch(self.base_width, self.branch_base_length, 4),
          branch_apex(4),
          pop(),
          push(),
          roll(random_lohi(branch_angle_min, branch_angle_max)),
          branch(self.base_width, self.branch_base_length, 4),
          branch_apex(4),
          pop(),
          trunk(self.base_width, self.trunk_base_length, 0),
          trunk_apex(0)
        ]
      },
      Module::BranchApex { life } => {
        if life > 0 {
          vec![
            yaw((PHI * 360.0_f32).to_radians()),
            // yaw((90.0_f32).to_radians()),
            push(),
            roll(random_lohi(25.0_f32, 30.0_f32).to_radians()),
            euler(random_lohi(min_branch_rot, max_branch_rot), 0.0, random_lohi(min_branch_rot, max_branch_rot)),
            branch(self.base_width, self.branch_base_length, 3),
            branch_apex(2),
            pop(),
            push(),
            roll(-random_lohi(25.0_f32, 30.0_f32).to_radians()),
            euler(random_lohi(min_branch_rot, max_branch_rot), 0.0, random_lohi(min_branch_rot, max_branch_rot)),
            branch(self.base_width, self.branch_base_length, 3),
            branch_apex(2),
            pop(),
            branch(self.base_width, self.branch_base_length, 3),
            branch_apex(life - 1)
          ]
        } else {
          vec![]
        }
      },
      _ => vec![module],
    }
  }
}
