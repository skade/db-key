pub trait Key {
  fn from_u8(key: &[u8]) -> Self;
  fn as_slice<T>(self, f: |v: &[u8]| -> T) -> T;
  fn compare(&self, _other: &Key) -> Ordering {
    Equal
  }
}

pub fn from_u8<K: Key>(key: &[u8]) -> K {
  Key::from_u8(key)
}
