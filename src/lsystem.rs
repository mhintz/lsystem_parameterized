use std::thread;

#[derive(Copy, Clone, Debug)]
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

pub fn segment_cmd(w: f32, l: f32) -> DrawCommand { DrawCommand::Segment { w: w, l: l } }
pub fn forward_cmd(d: f32) -> DrawCommand { DrawCommand::Forward { d: d } }
pub fn roll_cmd(r: f32) -> DrawCommand { DrawCommand::Roll { r: r } }
pub fn pitch_cmd(r: f32) -> DrawCommand { DrawCommand::Pitch { r: r } }
pub fn yaw_cmd(r: f32) -> DrawCommand { DrawCommand::Yaw { r: r } }
pub fn push_cmd() -> DrawCommand { DrawCommand::Push }
pub fn pop_cmd() -> DrawCommand { DrawCommand::Pop }
pub fn none_cmd() -> DrawCommand { DrawCommand::None }

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
  TrunkApex, // Generation point for plant organs - on the trunk
  BranchApex, // Generation point for plant organs - on a branch
  Trunk { w: f32, l: f32, life: u8 }, // Trunk of the tree
  Branch { w: f32, l: f32, life: u8 }, // Creates a straight branch with width w and length l
  Custom(u8, DrawCommand), // Can be used for any custom element
}

impl Module {
  pub fn to_draw_command(& self) -> DrawCommand {
    match * self {
      Module::Roll { r } => roll_cmd(r),
      Module::Pitch { r } => pitch_cmd(r),
      Module::Yaw { r } => yaw_cmd(r),
      Module::Push => push_cmd(),
      Module::Pop => pop_cmd(),
      Module::TrunkApex => none_cmd(),
      Module::BranchApex => none_cmd(),
      Module::Trunk { w, l, .. } => segment_cmd(w, l),
      Module::Branch { w, l, .. } => segment_cmd(w, l),
      Module::Custom(_, cmd) => cmd,
    }
  }
}

pub fn roll(r: f32) -> Module { Module::Roll { r: r } }
pub fn pitch(r: f32) -> Module { Module::Pitch { r: r } }
pub fn yaw(r: f32) -> Module { Module::Yaw { r: r } }
pub fn push() -> Module { Module::Push }
pub fn pop() -> Module { Module::Pop }
pub fn trunk_apex() -> Module { Module::TrunkApex }
pub fn branch_apex() -> Module { Module::BranchApex }
pub fn trunk(w: f32, l: f32, life: u8) -> Module { Module::Trunk { w: w, l: l, life: life } }
pub fn branch(w: f32, l: f32, life: u8) -> Module { Module::Branch { w: w, l: l, life: life } }
pub fn custom(num: u8, cmd: DrawCommand) -> Module { Module::Custom(num, cmd) }
pub fn custom_none(num: u8) -> Module { Module::Custom(num, DrawCommand::None) }

pub trait LSystem {
  // / The type for the modules of the l-system. These modules are the constituent
  // / parts of the system, which is composed of strings of this type, plus rules for
  // / generation of new strings from the existing modules
  type Module: Copy + Clone + Send;
  /// Provides the initial axiom, the "seed" of the lsystem
  fn axiom(& self) -> Vec<Self::Module>;
  /// Implement custom versions of this function to produce new chains of modules from an existing module
  fn produce(& self, module: Self::Module) -> Vec<Self::Module>;
}

pub fn iterate_system<T: LSystem>(lsystem: T, word: Vec<T::Module>) -> Vec<T::Module> {
  word.iter().flat_map(|letter| lsystem.produce(* letter)).collect()
}

fn split_vec<T: Clone>(thevec: Vec<T>, numsplits: usize) -> Vec<Vec<T>> {
  thevec.chunks(numsplits).map(|chunk| { chunk.to_vec() }).collect()
}

pub fn run_system<T: LSystem + Send + Copy + 'static>(lsystem: T, iterations: u32) -> Vec<T::Module> {
  let mut word = lsystem.axiom();

  for _ in 0..iterations {
    static TARGET_THREAD_NUM: u8 = 8;
    let chunk_size = (word.len() as f32 / TARGET_THREAD_NUM as f32).ceil() as usize;

    // The type here is Vec<thread::JoinHandle<Vec<T::Module>>>
    let threads: Vec<_> = split_vec(word, chunk_size)
      .into_iter()
      .map(|chunk| { thread::spawn(move || { iterate_system(lsystem, chunk) }) })
      .collect();

    word = threads.into_iter().flat_map(|t| t.join().unwrap()).collect();
  }

  word
}

