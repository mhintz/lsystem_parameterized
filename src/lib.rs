// These actions define actual mesh-generation actions
#[derive(Copy, Clone, Debug)]
pub enum Action {
  Branch { w: f32, l: f32 }, // Creates a straight branch with width w and length l
  Forward { d: f32 }, // Move forward distance d, without making a branch
  Roll { r: f32 }, // Roll by r radians (rotate around the local z axis)
  Pitch { r: f32 }, // Pitch by r radians (rotate around the local x axis)
  Yaw { r: f32 }, // Yaw by r radians (rotate around the local y axis)
  Reverse, // Reverse direction, turn around (equivalent to a π radians yaw)
  Push, // Push current transformation onto the local pushdown stack
  Pop, // Pop the current transformation from the pushdown stack and return to the most recently pushed one
  None, // This action does nothing. Used as the action part of a module which is a rule
}

impl Action {
  pub fn branch(w: f32, l: f32) -> Action { Action::Branch { w: w, l: l } }
  pub fn forward(d: f32) -> Action { Action::Forward { d: d } }
  pub fn roll(r: f32) -> Action { Action::Roll { r: r } }
  pub fn pitch(r: f32) -> Action { Action::Pitch { r: r } }
  pub fn yaw(r: f32) -> Action { Action::Yaw { r: r } }
  pub fn reverse() -> Action { Action::Reverse }
  pub fn push() -> Action { Action::Push }
  pub fn pop() -> Action { Action::Pop }
  pub fn none() -> Action { Action::None }
}

// This is a "module", one part of an l-system "word",
// where a word is a full description of the l-system at a certain
// level of iteration. The module can be an action, in which case it grows
#[derive(Copy, Clone, Debug)]
pub struct Module {
  action: Action,
  rule: Rule,
}

impl Module {
  pub fn none() -> Module {
    Module {
      action: Action::None,
      rule: Rule::None,
    }
  }

  pub fn action(action: Action) -> Module {
    Module {
      action: action,
      rule: Rule::None,
    }
  }

  pub fn rule(rule: Rule) -> Module {
    Module {
      action: Action::None,
      rule: rule,
    }
  }
}

// This enum should be given custom variants that store rule data
#[derive(Copy, Clone, Debug)]
pub enum Rule {
  None, // For cases where there is no rule to be implemented. Used as the rule part of a module which is an action
}

// Implement custom versions of this function to produce new chains of modules from an existing module
pub fn rule_produce(component: Module) -> Vec<Module> {
  match component.action {
    Action::Branch { w, l } => {
      vec![
        Module::action(Action::branch(w, l / 3.0)),
        Module::action(Action::pitch((60.0_f32).to_radians())),
        Module::action(Action::branch(w, l / 3.0)),
        Module::action(Action::pitch((-120.0_f32).to_radians())),
        Module::action(Action::branch(w, l / 3.0)),
        Module::action(Action::pitch((60.0_f32).to_radians())),
        Module::action(Action::branch(w, l / 3.0)),
      ]
    },
    Action::None => {
      match component.rule {
        _ => vec![component],
      }
    },
    _ => vec![component],
  }
}
