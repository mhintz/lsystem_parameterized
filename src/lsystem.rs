use std::thread;

/// An enum for drawing commands using a turtle graphics-style approach
#[derive(Copy, Clone, Debug)]
pub enum DrawCommand {
  Foliage { r: f32, l: f32 },
  /// Make a branch with width w and length l
  Segment { w: f32, l: f32 },
  /// Move forward distance d, without making a branch
  Forward { d: f32 },
  /// Roll by r radians (rotate heading around the z axis)
  Roll { r: f32 },
  /// Pitch by r radians (rotate heading around the x axis)
  Pitch { r: f32 },
  /// Yaw by r radians (rotate heading around the y axis)
  Yaw { r: f32 },
  /// Rotation represented as Euler angles x, y, z
  Euler { x: f32, y: f32, z: f32 },
  /// Push current transformation onto the local pushdown stack
  Push,
  /// Pop the current transformation from the pushdown stack and return to the most recently pushed one
  Pop,
  /// Do Nothing, don't draw
  None,
}

pub fn foliage_cmd(r: f32, l: f32) -> DrawCommand { DrawCommand::Foliage { r: r, l: l } }
pub fn segment_cmd(w: f32, l: f32) -> DrawCommand { DrawCommand::Segment { w: w, l: l } }
pub fn forward_cmd(d: f32) -> DrawCommand { DrawCommand::Forward { d: d } }
pub fn roll_cmd(r: f32) -> DrawCommand { DrawCommand::Roll { r: r } }
pub fn pitch_cmd(r: f32) -> DrawCommand { DrawCommand::Pitch { r: r } }
pub fn yaw_cmd(r: f32) -> DrawCommand { DrawCommand::Yaw { r: r } }
pub fn euler_cmd(x: f32, y: f32, z: f32) -> DrawCommand { DrawCommand::Euler { x: x, y: y, z: z } }
pub fn push_cmd() -> DrawCommand { DrawCommand::Push }
pub fn pop_cmd() -> DrawCommand { DrawCommand::Pop }
pub fn none_cmd() -> DrawCommand { DrawCommand::None }

/// This is a "module", one part of an l-system "word",
/// where a word is a full description of the l-system at a certain
/// level of iteration.
#[derive(Copy, Clone, Debug)]
pub enum Module {
  /// Rotate around the z axis by r radians
  Roll { r: f32 },
  /// Rotate around the x axis by r radians
  Pitch { r: f32 },
  /// Rotate around the y axis by r radians
  Yaw { r: f32 },
  /// An orientation change in Euler angles
  Euler { x: f32, y: f32, z: f32 },
  /// Push the current transform matrix onto a matrix stack
  Push,
  /// Pop the transform matrix off of the matrix stack, returns to the previously pushed matrix, or identity
  Pop,
  /// Generation point for plant organs - on the trunk
  TrunkApex { life: u8 },
  /// Generation point for plant organs - on a branch
  BranchApex { r: f32, l: f32, life: u8 },
  /// Trunk of the tree
  Trunk { w: f32, l: f32, life: u8 },
  /// Creates a straight branch with width w and length l
  Branch { w: f32, l: f32, life: u8 },
  /// Can be used for any custom element
  Custom(u8, DrawCommand),
}

impl Module {
  /// Transform an LSystem module into its corresponding turtle drawing command
  pub fn to_draw_command(& self) -> DrawCommand {
    match * self {
      Module::Roll { r } => roll_cmd(r),
      Module::Pitch { r } => pitch_cmd(r),
      Module::Yaw { r } => yaw_cmd(r),
      Module::Euler { x, y, z } => euler_cmd(x, y, z),
      Module::Push => push_cmd(),
      Module::Pop => pop_cmd(),
      Module::TrunkApex { .. } => none_cmd(),
      Module::BranchApex { r, l, .. } => foliage_cmd(r, l),
      Module::Trunk { w, l, .. } => segment_cmd(w, l),
      Module::Branch { w, l, .. } => segment_cmd(w, l),
      Module::Custom(_, cmd) => cmd,
    }
  }
}

pub fn roll(r: f32) -> Module { Module::Roll { r: r } }
pub fn pitch(r: f32) -> Module { Module::Pitch { r: r } }
pub fn yaw(r: f32) -> Module { Module::Yaw { r: r } }
pub fn euler(x: f32, y: f32, z: f32) -> Module { Module::Euler { x: x, y: y, z: z } }
pub fn push() -> Module { Module::Push }
pub fn pop() -> Module { Module::Pop }
pub fn trunk_apex(life: u8) -> Module { Module::TrunkApex { life: life } }
pub fn branch_apex(r: f32, l: f32, life: u8) -> Module { Module::BranchApex { r: r, l: l, life: life } }
pub fn trunk(w: f32, l: f32, life: u8) -> Module { Module::Trunk { w: w, l: l, life: life } }
pub fn branch(w: f32, l: f32, life: u8) -> Module { Module::Branch { w: w, l: l, life: life } }
pub fn custom(num: u8, cmd: DrawCommand) -> Module { Module::Custom(num, cmd) }
pub fn custom_none(num: u8) -> Module { Module::Custom(num, DrawCommand::None) }

/// A trait which can be implemented by arbitrary structs so that they can be used as an lsystem
pub trait LSystem where Self: Send + Copy + 'static {
  /// The type for the modules of the l-system. These modules are the constituent
  /// parts of the system, which is composed of strings of this type, plus rules for
  /// generation of new strings from the existing modules
  type Module: Copy + Clone + Send;
  /// Provides the initial axiom, the "seed" of the lsystem
  fn axiom(& self) -> Vec<Self::Module>;
  /// Implement custom versions of this function to produce new chains of modules from an existing module
  fn produce(& self, module: Self::Module) -> Vec<Self::Module>;
}

/// Split up a vector into discrete chunks. This function could probably be optimized
fn split_vec<T: Clone>(thevec: Vec<T>, numsplits: usize) -> Vec<Vec<T>> {
  thevec.chunks(numsplits).map(|chunk| { chunk.to_vec() }).collect()
}

/// Iterate over an lsystem word, producing a new vector of modules for each module in the word,
/// then collect these modules together
pub fn iterate_system<T: LSystem>(lsystem: T, word: Vec<T::Module>) -> Vec<T::Module> {
  word.iter().flat_map(|letter| lsystem.produce(* letter)).collect()
}

/// Multi-threaded l-system processing - splits each iteration of the l-system into several chunks,
/// spawns a thread to process each chunk, and then joins all the results.
/// The upside of this is dramatically improved performance.
/// The downside is that under the current implementation, context-sensitive l-systems are not possible.
/// Theoretically, it should be possible to implement these, by including a certain
/// number of "padding" modules on either end of a split chunk. Processing each module would then take into
/// account the contents of this padding, without actually processing it. Modules in the middle of the chunk would be
/// processed with context as usual. This approach is obviously more complex, and not needed for my purposes at the moment.
pub fn run_system<T: LSystem + Send + Copy + 'static>(lsystem: T, iterations: u32) -> Vec<T::Module> {
  // Start with the l-system's axiom
  let mut word = lsystem.axiom();

  for _ in 0..iterations {
    // Could make this configurable; this seemed like a sensible default
    static TARGET_THREAD_NUM: u8 = 8;
    // Calculate an appropriate split size on which to split up the word
    let chunk_size = (word.len() as f32 / TARGET_THREAD_NUM as f32).ceil() as usize;

    // The type of this expression is Vec<thread::JoinHandle<Vec<T::Module>>>
    let threads: Vec<_> = split_vec(word, chunk_size)
      // Take ownership of the split vector's contents
      .into_iter()
      // Grabs the lsystem local variable, spawns a thread to process each chunk of the split up vector
      .map(|chunk| { thread::spawn(move || { iterate_system(lsystem, chunk) }) })
      .collect();

    // iterate over the JoinHandles and join each one, which waits for its thread
    // to finish processing its vector of modules.
    word = threads.into_iter().flat_map(|t| t.join().unwrap()).collect();
  }

  // word.
  word
}
