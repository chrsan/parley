use swash::{Attributes, CacheKey};

use super::{DataId, FamilyId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FontId(pub(crate) u16);

#[derive(Debug, Clone, Copy)]
pub struct Font {
    pub id: FontId,
    pub family_id: FamilyId,
    pub data_id: DataId,
    pub index: u16,
    pub attributes: Attributes,
    pub cache_key: CacheKey,
}
