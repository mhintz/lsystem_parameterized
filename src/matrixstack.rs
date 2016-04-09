extern crate cgmath;
extern crate num as rust_num;

use std::collections::VecDeque;
use cgmath::*;
use self::rust_num::Float;

pub struct MatrixStack<T: BaseFloat + Float> {
  current: Matrix4<T>,
  stack: VecDeque<Matrix4<T>>,
}

impl<T: BaseFloat + Float> MatrixStack<T> {
  pub fn new() -> MatrixStack<T> {
    MatrixStack {
      current: Matrix4::identity(),
      stack: VecDeque::new(),
    }
  }

  pub fn transform(&mut self, transformation: Matrix4<T>) {
    self.current = transformation * self.current;
  }

  pub fn rotate(&mut self, rotation: Matrix3<T>) {
    self.current = Matrix4::from(rotation) * self.current;
  }

  pub fn push(&mut self) {
    self.stack.push_back(self.current);
  }

  pub fn pop(&mut self) -> Matrix4<T> {
    if let Some(stack_top) = self.stack.pop_back() {
      self.current = stack_top;
    } else {
      // At the bottom of the stack you find the identity matrix
      self.current = Matrix4::identity();
    }
    return self.current;
  }

  pub fn transform_vector(& self, target: Vector3<T>) -> Vector3<T> { (self.current * target.extend(T::zero())).truncate() }

  pub fn transform_point(& self, target: Point3<T>) -> Point3<T> { Point3::from_homogeneous(self.current * target.to_homogeneous()) }

  pub fn get_matrix(& self) -> Matrix4<T> { self.current }
}
