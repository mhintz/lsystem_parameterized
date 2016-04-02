extern crate lsystem;

use lsystem::*;

const NUM_ITERATIONS: u32 = 1;

fn main() {
  let mut word: Vec<Module> = vec![
    Module::action(branch(1.0, 27.0)),
    Module::action(pitch((-120.0_f32).to_radians())),
    Module::action(branch(1.0, 27.0)),
    Module::action(pitch((-120.0_f32).to_radians())),
    Module::action(branch(1.0, 27.0)),
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
