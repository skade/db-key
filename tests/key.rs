extern crate db_key as key;

use key::Key;

#[derive(PartialEq,Eq,PartialOrd,Ord,Debug,Copy,Clone)]
enum MyValues {
  One
}

#[derive(Copy,Clone)]
struct MyKey {
  #[allow(dead_code)]
  val: MyValues
}

impl<'a> From<&'a [u8]> for MyKey {
    fn from(key: &'a [u8]) -> MyKey {
        use std::intrinsics::transmute;

        let key: &MyKey = unsafe { transmute(key.as_ptr()) };
        *key
    }
}

impl AsRef<[u8]> for MyKey {
    fn as_ref(&self) -> &[u8] {
        use std::intrinsics::transmute;
        use std::slice::from_raw_parts;
        use std::mem::size_of;

        unsafe { from_raw_parts(transmute(self), size_of::<MyKey>()) }
    }
}

impl<'a> Key<'a> for MyKey {}

#[test]
fn roundtrip() {
    let key = MyKey { val: MyValues::One };
    let reference = key.as_ref();
    let key2: MyKey = From::from(reference);
    assert_eq!(key2.val, MyValues::One);
}
