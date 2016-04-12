// This is a "module", one part of an l-system "word",
// where a word is a full description of the l-system at a certain
// level of iteration.
#[derive(Copy, Clone, Debug)]
pub enum Module {
  Branch { w: f32, l: f32 }, // Creates a straight branch with width w and length l
  Forward { d: f32 }, // Move forward distance d, without making a branch
  Roll { r: f32 }, // Roll by r radians (rotate heading around the z axis)
  Pitch { r: f32 }, // Pitch by r radians (rotate heading around the x axis)
  Yaw { r: f32 }, // Yaw by r radians (rotate heading around the y axis)
  Reverse, // Reverse direction, turn around (equivalent to a π radians yaw)
  Push, // Push current transformation onto the local pushdown stack
  Pop, // Pop the current transformation from the pushdown stack and return to the most recently pushed one
  Apex, // Generation point for plant organs
  Custom(u8), // Custom rules
}

pub fn branch(w: f32, l: f32) -> Module { Module::Branch { w: w, l: l } }
pub fn forward(d: f32) -> Module { Module::Forward { d: d } }
pub fn roll(r: f32) -> Module { Module::Roll { r: r } }
pub fn pitch(r: f32) -> Module { Module::Pitch { r: r } }
pub fn yaw(r: f32) -> Module { Module::Yaw { r: r } }
pub fn reverse() -> Module { Module::Reverse }
pub fn push() -> Module { Module::Push }
pub fn pop() -> Module { Module::Pop }
pub fn custom(num: u8) -> Module { Module::Custom(num) }

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
