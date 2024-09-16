//! A dict / HashMap whose items expire over time
//!
//! This is a simple wrapper over [`HashMap`] and when you call its [`expire()`](ExpiringDict::expire) method, items
//! inserted early expire and disappear.
//!
//! There is no external dependency like tokio or the like, but you need to call [`expire()`](ExpiringDict::expire)
//! at appropriate time.
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

  /// Get an item from the dict.
  ///
  /// Remember to call [`expire()`](Self::expire) before calling this method unless you don't care
  /// expired items being returned.
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

  /// Remove expired items.
  ///
  /// Remember to call this method before use!
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
