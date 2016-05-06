const PHI: f32 = 1.61803398875;
const PHI_RECIP: f32 = 1.0 / PHI;
const PHI_COMPLEMENT: f32 = 1.0 - PHI_RECIP;

use lsystem::*;

#[derive(Copy, Clone)]
pub struct KochCurve;

impl LSystem for KochCurve {
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
  fn axiom(& self) -> Vec<Module> {
    vec![
      branch(1.0, 2.0, 1),
      trunk_apex(),
    ]
  }

  fn produce(& self, module: Module) -> Vec<Module> {
    match module {
      Module::Branch { w: width, l: length, life } => vec![branch(width, length * 1.15, life)],
      Module::TrunkApex => { // Trunk apex
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
          trunk_apex(),
        ]
      },
      Module::BranchApex => vec![module],
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
  fn axiom(& self) -> Vec<Module> {
    vec![
      trunk(self.base_width, 2.0 * self.base_length, 5),
      trunk_apex(),
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
          Module::TrunkApex => vec![
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
            trunk_apex(),
          ],
          Module::BranchApex => vec![module],
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
