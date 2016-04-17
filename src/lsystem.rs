// This is a "module", one part of an l-system "word",
// where a word is a full description of the l-system at a certain
// level of iteration.
#[derive(Copy, Clone, Debug)]
pub enum Module {
  Roll { r: f32 },
  Pitch { r: f32 },
  Yaw { r: f32 },
  Push,
  Pop,
  Apex, // Generation point for plant organs
  Trunk { w: f32, l: f32, life: u8 }, // Trunk of the tree
  Branch { w: f32, l: f32, life: u8 }, // Creates a straight branch with width w and length l
  Custom(u8, DrawCommand), // Can be used for any custom element
}

pub enum DrawCommand {
  Segment { w: f32, l: f32 },
  Forward { d: f32 }, // Move forward distance d, without making a branch
  Roll { r: f32 }, // Roll by r radians (rotate heading around the z axis)
  Pitch { r: f32 }, // Pitch by r radians (rotate heading around the x axis)
  Yaw { r: f32 }, // Yaw by r radians (rotate heading around the y axis)
  Push, // Push current transformation onto the local pushdown stack
  Pop, // Pop the current transformation from the pushdown stack and return to the most recently pushed one
  None, // Do Nothing, don't draw
}

impl Module {
  fn to_draw_command(& self) -> DrawCommand {
    match self {
      Module::Roll { r: r } => DrawCommand::Roll { r: r },
      Module::Pitch { r: r } => DrawCommand::Pitch { r: r },
      Module::Yaw { r: r } => DrawCommand::Yaw { r: r },
      Module::Push => DrawCommand::Push,
      Module::Pop => DrawCommand::Pop,
      Module::Apex => DrawCommand::None,
      Module::Trunk { w: w, l: l, .. } => DrawCommand::Segment { w: w, l: l },
      Module::Branch { w: w, l: l, .. } => DrawCommand::Segment { w: w, l: l },
      Module::Custom(_, cmd) => cmd,
    }
  }
}

pub fn roll(r: f32) -> Module { Module::Roll { r: r } }
pub fn pitch(r: f32) -> Module { Module::Pitch { r: r } }
pub fn yaw(r: f32) -> Module { Module::Yaw { r: r } }
pub fn push() -> Module { Module::Push }
pub fn pop() -> Module { Module::Pop }
pub fn apex() -> Module { Module::Apex }
pub fn trunk(w: f32, l: f32, life: u8) -> Module { Module::Trunk { w: w, l: l, life: life } }
pub fn branch(w: f32, l: f32, life: u8) -> Module { Module::Branch { w: w, l: l, life: life } }
pub fn custom(num: u8, cmd: DrawCommand) -> Module { Module::Custom(num, cmd) }
pub fn custom_none(num: u8) -> Module { Module::Custom(num, DrawCommand::None) }

pub trait LSystem {
  /// Provides the initial axiom, the "seed" of the lsystem
  fn axiom(& self) -> Vec<Module>;
  /// Implement custom versions of this function to produce new chains of modules from an existing module
  fn produce(& self, module: Module) -> Vec<Module>;
}

pub fn run_system(lsystem: & LSystem, iterations: u32) -> Vec<Module> {
  let mut word = lsystem.axiom();
  for _ in 0..iterations {
    let mut collector: Vec<Module> = vec![];
    for letter in word {
      collector.extend(lsystem.produce(letter));
    }
    word = collector;
  }
  word
}

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
          pitch((60.0).to_radians()),
          branch(w, l / 3.0, life),
          pitch(-(120.0).to_radians()),
          branch(w, l / 3.0, life),
          pitch((60.0).to_radians()),
          branch(w, l / 3.0, life),
        ]
      },
      _ => vec![module],
    }
  }
}

pub struct DragonCurve;

impl LSystem for DragonCurve {
  fn axiom(& self) -> Vec<Module> {
    vec![
      custom(1)
    ]
  }

  fn produce(& self, module: Module) -> Vec<Module> {
    match module {
      Module::Custom(1) => vec![custom(1), roll(-(90.0).to_radians()), custom(2)],
      Module::Custom(2) => vec![custom(1), roll((90.0).to_radians()), custom(2)],
      _ => vec![module],
    }
  }
}

pub struct BasicTree;

impl LSystem for BasicTree {
  fn axiom(& self) -> Vec<Module> {
    vec![
      branch(1.0, 2.0),
      custom(1),
    ]
  }

  fn produce(& self, module: Module) -> Vec<Module> {
    match module {
      Module::Branch { w: width, l: length } => vec![branch(width, length * 1.15)],
      Module::Custom(1) => { // Trunk apex
        vec![
          // Left branch
          push(),
          roll(rad(30.0)),
          branch(1.0, 1.0),
          custom(2),
          pop(),
          push(),
          roll(-rad(30.0)),
          branch(1.0, 1.0),
          custom(3),
          pop(),
          branch(1.0, 1.0),
          yaw(rad()),
          custom(1),
        ]
      },
      Module::Custom(2) => { // Branch apex left
        vec![
          roll(25.0 * DEG_TO_RAD),
          branch(1.0, 1.0),
          custom(2),
        ]
      },
      Module::Custom(3) => { // Branch apex right
        vec![
          roll(-25.0 * DEG_TO_RAD),
          branch(1.0, 1.0),
          custom(3),
        ]
      },
      _ => vec![module],
    }
  }
}
