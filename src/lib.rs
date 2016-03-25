// These actions define actual mesh-generation actions
pub enum Action {
  Branch { w: f32, l: f32 }, // Creates a straight branch with width w and length l
  Move { d: f32 }, // Move forward distance d, without making a branch
  Roll { r: f32 }, // Roll by r radians (rotate around the local z axis)
  Pitch { r: f32 }, // Pitch by r radians (rotate around the local x axis)
  Yaw { r: f32 }, // Yaw by r radians (rotate around the local y axis)
  Reverse, // Reverse direction, turn around (equivalent to a π radians yaw)
  Push, // Push current transformation onto the local pushdown stack
  Pop, // Pop the current transformation from the pushdown stack and return to the most recently pushed one
  None, // This action does nothing. Used as the action part of a module which is a rule
}

// This enum should be given custom variants that store rule data
pub enum Rule {
  None, // For cases where there is no rule to be implemented. Used as the rule part of a module which is an action
}

// This is a "module", one part of an l-system "word",
// where a word is a full description of the l-system at a certain
// level of iteration. The module can be an action, in which case it grows
pub struct Module {
  action: Action,
  rule: Rule,
}

// Implement custom versions of this function to produce new chains of modules from an existing module
pub fn produce(component: Module) -> Vec<Module> {
  match component.action {
    Action::None => {
      match component.rule {
        Rule::None => vec![component],
      }
    },
    _ => vec![component],
  }
}
