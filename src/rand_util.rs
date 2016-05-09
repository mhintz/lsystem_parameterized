use rand;
use num_traits::Float;

pub use rand::random;

pub fn random_lohi<T: Float + rand::Rand>(lo: T, hi: T) -> T {
  lo + random::<T>() * (hi - lo)
}

pub fn random_max<T: Float + rand::Rand>(hi: T) -> T {
  random::<T>() * hi
}
