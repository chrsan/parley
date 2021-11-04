use std::collections::HashMap;
use std::sync::Arc;

use swash::{FontDataRef, Stretch};

use super::{
    data::FontData,
    family::{FontElement, FontFamily},
    font::Font,
    DataId, FamilyId, FontId,
};

#[derive(Debug, Default, Clone)]
pub struct FontCollection {
    pub families: Vec<Arc<FontFamily>>,
    pub family_map: HashMap<Arc<str>, FamilyId>,
    pub fonts: Vec<Font>,
    pub data: Vec<FontData>,
}

impl FontCollection {
    pub fn family_id(&self, name: &str) -> Option<FamilyId> {
        self.family_map.get(name).map(|id| *id)
    }

    pub fn family(&self, id: FamilyId) -> Option<Arc<FontFamily>> {
        self.families.get(id.0 as usize).cloned()
    }

    pub fn family_by_name(&self, name: &str) -> Option<Arc<FontFamily>> {
        self.family(self.family_id(name)?)
    }

    pub fn font(&self, id: FontId) -> Option<Font> {
        self.fonts.get(id.0 as usize).cloned()
    }

    pub fn data(&self, id: DataId) -> Option<FontData> {
        self.data.get(id.0 as usize).cloned()
    }

    pub fn add_fonts(&mut self, name: &str, data: Vec<u8>) -> Option<usize> {
        let data = FontData(Arc::new(data));
        let font_data = if let Some(font_data) = FontDataRef::new(&data) {
            font_data
        } else {
            return None;
        };
        let num_fonts = font_data.len();
        if num_fonts == 0 {
            return None;
        }
        let data_id = self.data.len();
        assert!(data_id <= u16::MAX as _);
        let data_id = DataId(data_id as _);
        let mut data_added = false;
        let mut count = 0;
        for index in 0..num_fonts {
            assert!(index <= u16::MAX as _);
            let font = if let Some(font) = font_data.get(index) {
                font
            } else {
                continue;
            };
            let font_id = self.fonts.len();
            assert!(font_id <= u16::MAX as _);
            let font_id = FontId(font_id as _);
            let family_id = if let Some(family_id) = self.family_map.get(name) {
                *family_id
            } else {
                let family_id = self.families.len();
                assert!(family_id <= u16::MAX as _);
                let family_id = FamilyId(family_id as _);
                let family = FontFamily {
                    id: family_id,
                    name: name.into(),
                    has_stretch: false,
                    fonts: Vec::new(),
                };
                self.families.push(Arc::new(family));
                self.family_map.insert(name.into(), family_id);
                family_id
            };
            let family = Arc::make_mut(self.families.get_mut(family_id.0 as usize).unwrap());
            let attributes = font.attributes();
            let (stretch, weight, style) = attributes.parts();
            if family
                .fonts
                .iter()
                .any(|e| e.stretch == stretch && e.weight == weight && e.style == style)
            {
                continue;
            }
            if !data_added {
                self.data.push(data.clone());
                data_added = true;
            }
            if stretch != Stretch::NORMAL {
                family.has_stretch = true;
            }
            match family.fonts.binary_search_by(|e| e.weight.cmp(&weight)) {
                Ok(index) | Err(index) => {
                    family.fonts.insert(
                        index,
                        FontElement {
                            font_id,
                            stretch,
                            weight,
                            style,
                        },
                    );
                }
            }
            self.fonts.push(Font {
                id: font_id,
                family_id,
                data_id,
                index: index as u16,
                attributes,
                cache_key: font.key,
            });
            count += 1;
        }
        Some(count)
    }
}
