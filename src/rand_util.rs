use rand;
use rand::distributions::Sample;
use cgmath::num_traits::Float;
use cgmath::prelude::*;

use defs::*;

// Re export the random function
pub use rand::random;

pub fn random_lohi<T: Float + rand::Rand>(lo: T, hi: T) -> T {
  lo + random::<T>() * (hi - lo)
}

pub fn random_max<T: Float + rand::Rand>(hi: T) -> T {
  random::<T>() * hi
}

pub fn rand_points_in_sphere<R: rand::Rng>(num_gen: &mut R, num: usize, radius: f32) -> Vec<Pt> {
  let mut vec_range = rand::distributions::Range::new(-1.0, 1.0);
  let mut radius_range = rand::distributions::Range::new(0.0, radius);
  let mut points = Vec::with_capacity(num);
  for idx in 0..num {
    let point_vec = Vec3::new(vec_range.sample(num_gen), vec_range.sample(num_gen), vec_range.sample(num_gen));
    let radius = radius_range.sample(num_gen);
    let rand_point = Pt::from_vec(point_vec.normalize() * radius);
    points.insert(idx, rand_point);
  }
  deduplicate_points_list(&mut points);
  return points;
}

// sorts and deduplicates a list of input points
fn deduplicate_points_list(list: &mut Vec<Pt>) {
  use std::cmp::Ordering::{Less, Equal};

  list.sort_by(|a, b| {
    match a.x.partial_cmp(& b.x).unwrap_or(Less) {
      Equal => {
        match a.y.partial_cmp(& b.y).unwrap_or(Less) {
          Equal => a.z.partial_cmp(& b.z).unwrap_or(Less),
          y => y,
        }
      },
      x => x,
    }
  });

  list.dedup();
}
