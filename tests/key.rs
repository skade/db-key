extern crate db_key as key;
extern crate libc;

use libc::{c_void};

use key::Key;

#[derive(Debug, Eq, PartialEq)]
enum MyValues {
  One
}

#[test]
fn test() {
    let key : &i32 = &1234413;
    let (raw, len) = key.to_raw::<*mut c_void>();
    let key2 : &i32 = Key::from_raw(raw, len);
    assert_eq!(&1234413, key2);
}

#[test]
fn test2() {
  let key = &MyValues::One;
  let (raw, len) = key.to_raw::<*mut c_void>();
  let key2 : &MyValues = Key::from_raw(raw, len);
  assert_eq!(key2, &MyValues::One);
}
