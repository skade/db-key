extern crate key;

use key::Key;
use key::from_u8;

enum MyValues {
  One,
  Two
}

struct MyKey {
  val: MyValues
}

impl Key for MyKey {
  fn from_u8(key: &[u8]) -> MyKey {
    MyKey { val: One }
  }

  fn as_slice<T>(self, f: |v: &[u8]| -> T) -> T {
    f("test".as_bytes())
  }

  fn compare(&self, _other: &MyKey) -> Ordering {
    Equal
  }
}

#[test]
fn test() {
  let key: MyKey = from_u8("test".as_bytes());
}

#[test]
fn test2() {
  let key = MyKey { val: One };
  key.as_slice(|k| {
    assert_eq!(k, "test".as_bytes())
  })
}
