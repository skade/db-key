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
  fn compare(&self, _other: &Key) -> Ordering {
    Equal
  }
}

#[test]
fn test() {
  let key = from_u8("test".as_bytes());
}
