pub trait Key {
  fn from_u8(key: &[u8]) -> Self;
  fn as_slice<T>(&self, f: |v: &[u8]| -> T) -> T;
}

pub fn from_u8<K: Key>(key: &[u8]) -> K {
  Key::from_u8(key)
}

impl Key for int {
  fn from_u8(key: &[u8]) -> int {
    assert!(key.len() == 4);

    (key[0] as int) << 24 |
    (key[1] as int) << 16 |
    (key[2] as int) << 8 |
    (key[3] as int)
  }

  fn as_slice<T>(&self, f: |v: &[u8]| -> T) -> T {
    let mut dst = [0u8, ..4];
    dst[0] = (*self >> 24) as u8;
    dst[1] = (*self >> 16) as u8;
    dst[2] = (*self >> 8) as u8;
    dst[3] = *self as u8;
    f(&dst)
  }
}
