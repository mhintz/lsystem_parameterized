use rand;
use num_traits::Float;

pub fn random01<T: Float>() -> T {
  rand::random() / T::max_value()
}

pub fn random<T: Float>(lo: T, hi: T) -> T {
  lo + random01() * (hi - lo)
}

pub fn randomMax<T: Float>(hi: T) -> T {
  random01() * hi
}
