pub trait Key<'a> : From<&'a [u8]> + AsRef<[u8]> { }

impl<'a> Key<'a> for &'a [u8] { }
impl<'a> Key<'a> for Vec<u8> { }
