use std::collections::HashMap;
use std::time::{SystemTime, Instant, Duration};
use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Add;

#[cfg(test)]
mod tests;

pub struct ExpiringDict<K, V, T=SystemTime> {
  default_ttl: Duration,
  container: HashMap<K, (V, T)>,
}

impl<K: Hash + Eq, V, T: TimeNow> ExpiringDict<K, V, T>
where T: Add<Duration, Output=T> + PartialOrd
{
  pub fn new(default_ttl: Duration) -> Self {
    Self {
      default_ttl,
      container: HashMap::new(),
    }
  }

  pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where K: Borrow<Q>,
          Q: Hash + Eq + ?Sized
  {
    self.container.get(k).map(|(item, _)| item)
  }

  pub fn insert(&mut self, k: K, v: V) -> Option<V> {
    self.container.insert(k, (v, T::now() + self.default_ttl)).map(|(item, _)| item)
  }

  pub fn insert_with_ttl(&mut self, k: K, v: V, ttl: Duration) -> Option<V> {
    self.container.insert(k, (v, T::now() + ttl)).map(|(item, _)| item)
  }

  pub fn expire(&mut self) {
    let now = T::now();
    self.container.retain(|_k, (_v, t)| *t > now);
  }

}

pub trait TimeNow {
  fn now() -> Self;
}

impl TimeNow for SystemTime {
  fn now() -> Self {
    SystemTime::now()
  }
}

impl TimeNow for Instant {
  fn now() -> Self {
    Instant::now()
  }
}
