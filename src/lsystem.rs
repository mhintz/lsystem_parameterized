// This is a "module", one part of an l-system "word",
// where a word is a full description of the l-system at a certain
// level of iteration.
#[derive(Copy, Clone, Debug)]
pub enum Module {
  None, // This action does nothing. Used as the action part of a module which is a rule
  Branch { w: f32, l: f32 }, // Creates a straight branch with width w and length l
  Forward { d: f32 }, // Move forward distance d, without making a branch
  Roll { r: f32 }, // Roll by r radians (rotate heading around the z axis)
  Pitch { r: f32 }, // Pitch by r radians (rotate heading around the x axis)
  Yaw { r: f32 }, // Yaw by r radians (rotate heading around the y axis)
  Reverse, // Reverse direction, turn around (equivalent to a π radians yaw)
  Push, // Push current transformation onto the local pushdown stack
  Pop, // Pop the current transformation from the pushdown stack and return to the most recently pushed one
}

pub fn none() -> Module { Module::None }
pub fn branch(w: f32, l: f32) -> Module { Module::Branch { w: w, l: l } }
pub fn forward(d: f32) -> Module { Module::Forward { d: d } }
pub fn roll(r: f32) -> Module { Module::Roll { r: r } }
pub fn pitch(r: f32) -> Module { Module::Pitch { r: r } }
pub fn yaw(r: f32) -> Module { Module::Yaw { r: r } }
pub fn reverse() -> Module { Module::Reverse }
pub fn push() -> Module { Module::Push }
pub fn pop() -> Module { Module::Pop }
