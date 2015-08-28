pub unsafe trait Key {
    fn from_raw<T>(key: *mut T, len: usize) -> Self;
    fn to_raw<T>(self) -> (*mut T, usize);
}

unsafe impl<'a, K: Sized> Key for &'a K {
    fn from_raw<T>(key: *mut T, len: usize) -> &'a K {
        use std::mem::transmute;
        use std::mem::size_of;

        assert!(len == size_of::<K>());
        unsafe { transmute(key) }
    }

    fn to_raw<T>(self) -> (*mut T, usize) {
        use std::mem::transmute;
        use std::mem::size_of;

        let val = unsafe { transmute(self) };
        let len = size_of::<K>();
        (val, len)
    }
}