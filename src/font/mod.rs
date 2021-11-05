//! Font management.

use swash::proxy::CharmapProxy;
use swash::text::cluster::{CharCluster, Status};
use swash::{Attributes, CacheKey, FontRef, Synthesis};

use crate::style::FontFamily;

mod collection;
mod data;
mod family;
mod font;

use self::collection::FontCollection;

pub use self::{data::DataId, family::FamilyId, font::FontId};

/// Shared handle to a font.
#[derive(Debug, Clone)]
pub struct FontHandle {
    data: data::FontData,
    offset: u32,
    key: CacheKey,
}

impl FontHandle {
    /// Returns a reference to the font.
    pub fn as_ref(&self) -> FontRef {
        FontRef {
            data: &*self.data,
            offset: self.offset,
            key: self.key,
        }
    }
}

impl PartialEq for FontHandle {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

/// Context for font selection and fallback.
#[derive(Debug, Default, Clone)]
pub struct FontContext {
    pub(crate) cache: FontCache,
}

impl FontContext {
    /// Returns true if a family of the specified name exists in the context.
    pub fn has_family(&self, name: &str) -> bool {
        self.cache.collection.family_by_name(name).is_some()
    }

    pub fn fonts(&mut self, family: FontFamily<'_>) -> Vec<(FontHandle, Attributes)> {
        let collection = &self.cache.collection;
        if let Some(family) = collection.family_by_name(family.name) {
            family
                .font_attrs
                .iter()
                .flat_map(|a| {
                    let font = collection.font(a.font_id)?;
                    let data = collection.data(font.data_id)?;
                    let attributes = font.attributes;
                    let font_ref = FontRef::from_index(&data, font.index as _)?;
                    let offset = font_ref.offset;
                    let font = FontHandle {
                        data,
                        offset,
                        key: font.cache_key,
                    };
                    Some((font, attributes))
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn register_fonts(&mut self, name: &str, data: Vec<u8>) -> Option<usize> {
        self.cache.collection.add_fonts(name, data)
    }
}

#[derive(Debug, Default, Clone)]
pub struct FontCache {
    collection: FontCollection,
    selected_params: Option<(usize, Attributes)>,
    selected_fonts: Vec<CachedFont>,
    attrs: Attributes,
}

impl FontCache {
    pub fn collection(&self) -> &FontCollection {
        &self.collection
    }

    pub fn reset(&mut self) {
        self.selected_params = None;
        self.selected_fonts.clear();
        self.attrs = Attributes::default();
    }

    pub fn select_families(&mut self, id: usize, families: &[FamilyId], attrs: Attributes) {
        if self.selected_params != Some((id, attrs)) {
            self.selected_params = Some((id, attrs));
            self.selected_fonts.clear();
            let collection = &self.collection;
            self.selected_fonts.extend(
                families
                    .iter()
                    .filter_map(|id| collection.family(*id))
                    .filter_map(|family| family.query(attrs))
                    .map(CachedFont::new),
            );
            self.attrs = attrs;
        }
    }

    pub fn map_cluster(&mut self, cluster: &mut CharCluster) -> Option<(FontHandle, Synthesis)> {
        let mut best = None;
        map_cluster(
            &self.collection,
            &mut self.selected_fonts,
            cluster,
            &mut best,
        );
        best.map(|(font, attrs)| (font, attrs.synthesize(self.attrs)))
    }
}

fn map_cluster(
    collection: &FontCollection,
    fonts: &mut [CachedFont],
    cluster: &mut CharCluster,
    best: &mut Option<(FontHandle, Attributes)>,
) -> bool {
    for font in fonts {
        if font.map_cluster(collection, cluster, best) {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone)]
struct CachedFont {
    id: FontId,
    font: Option<(FontHandle, CharmapProxy)>,
    attrs: Attributes,
    error: bool,
}

impl CachedFont {
    fn new(id: FontId) -> Self {
        Self {
            id,
            font: None,
            attrs: Attributes::default(),
            error: false,
        }
    }

    fn get_font(&self, collection: &FontCollection) -> Option<(FontHandle, Attributes)> {
        let font = collection.font(self.id)?;
        let data = collection.data(font.data_id)?;
        let font_ref = FontRef::from_index(&data, font.index as usize)?;
        let offset = font_ref.offset;
        Some((
            FontHandle {
                data,
                offset,
                key: font.cache_key,
            },
            font.attributes,
        ))
    }

    fn map_cluster(
        &mut self,
        collection: &FontCollection,
        cluster: &mut CharCluster,
        best: &mut Option<(FontHandle, Attributes)>,
    ) -> bool {
        if self.error {
            return false;
        }
        let (font, charmap_proxy) = if let Some(font) = &self.font {
            (&font.0, font.1)
        } else if let Some((font, attrs)) = self.get_font(collection) {
            self.font = Some((font.clone(), CharmapProxy::from_font(&font.as_ref())));
            self.attrs = attrs;
            let (font, charmap_proxy) = self.font.as_ref().unwrap();
            (font, *charmap_proxy)
        } else {
            self.error = true;
            return false;
        };
        let charmap = charmap_proxy.materialize(&font.as_ref());
        match cluster.map(|ch| charmap.map(ch)) {
            Status::Complete => {
                *best = Some((font.clone(), self.attrs));
                return true;
            }
            Status::Keep => {
                *best = Some((font.clone(), self.attrs));
            }
            Status::Discard => {}
        }
        false
    }
}
