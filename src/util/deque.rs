
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapacityDeque<T> {
  inner: VecDeque<T>,
  capacity: usize,
}

impl<T> CapacityDeque<T> {
  pub fn new(capacity: usize) -> Self {
    Self {
      inner: VecDeque::new(),
      capacity,
    }
  }

  pub fn push_back(&mut self, item: T) {
    while self.len() >= self.capacity {
      self.inner.pop_front();
    }
    self.inner.push_back(item);
  }

  pub fn len(&self) -> usize {
    self.inner.len()
  }

  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.inner.iter()
  }
}
