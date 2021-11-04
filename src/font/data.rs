use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DataId(pub(crate) u16);

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct FontData(pub Arc<Vec<u8>>);

impl AsRef<[u8]> for FontData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for FontData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
