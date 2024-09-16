use std::thread::sleep;

use super::*;

#[test]
fn it_works() {
  let dur = Duration::from_millis(10);
  let mut d = ExpiringDict::<&str, i32>::new(dur);
  assert_eq!(d.insert("a", 1), None);
  assert_eq!(d.get("a"), Some(&1));
  sleep(dur*2);
  d.expire();
  assert_eq!(d.get("a"), None);
}

#[test]
fn insert_with_ttl() {
  let dur = Duration::from_millis(10);
  let mut d = ExpiringDict::<&str, i32>::new(dur);
  assert_eq!(d.insert("a", 1), None);
  assert_eq!(d.insert_with_ttl("b", 2, dur*3), None);
  assert_eq!(d.get("a"), Some(&1));
  sleep(dur*2);
  d.expire();
  assert_eq!(d.get("a"), None);
  assert_eq!(d.get("b"), Some(&2));
  sleep(dur);
  d.expire();
  assert_eq!(d.get("b"), None);
}

#[test]
fn with_instant() {
  let dur = Duration::from_millis(10);
  let mut d = ExpiringDict::<&str, i32, Instant>::new(dur);
  assert_eq!(d.insert("a", 1), None);
  assert_eq!(d.get("a"), Some(&1));
  sleep(dur*2);
  d.expire();
  assert_eq!(d.get("a"), None);
}
