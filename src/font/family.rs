use swash::{Attributes, Stretch, Style, Weight};

use super::FontId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FamilyId(pub(crate) u16);

#[derive(Debug, Clone)]
pub struct FontFamily {
    pub id: FamilyId,
    pub name: String,
    pub has_stretch: bool,
    pub fonts: Vec<FontElement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontElement {
    pub font_id: FontId,
    pub stretch: Stretch,
    pub weight: Weight,
    pub style: Style,
}

impl FontFamily {
    pub fn query(&self, attributes: Attributes) -> Option<FontId> {
        let style = attributes.style();
        let weight = attributes.weight();
        let stretch = attributes.stretch();
        let mut min_stretch_dist = i32::MAX;
        let mut matching_stretch = Stretch::NORMAL;
        if self.has_stretch {
            if stretch <= Stretch::NORMAL {
                for font in &self.fonts {
                    let font_stretch = if font.stretch > Stretch::NORMAL {
                        font.stretch.raw() as i32 - Stretch::NORMAL.raw() as i32
                            + Stretch::ULTRA_EXPANDED.raw() as i32
                    } else {
                        font.stretch.raw() as i32
                    };
                    let offset = (font_stretch - stretch.raw() as i32).abs();
                    if offset < min_stretch_dist {
                        min_stretch_dist = offset;
                        matching_stretch = font.stretch;
                    }
                }
            } else {
                for font in &self.fonts {
                    let font_stretch = if font.stretch < Stretch::NORMAL {
                        font.stretch.raw() as i32 - Stretch::NORMAL.raw() as i32
                            + Stretch::ULTRA_EXPANDED.raw() as i32
                    } else {
                        font.stretch.raw() as i32
                    };
                    let offset = (font_stretch - stretch.raw() as i32).abs();
                    if offset < min_stretch_dist {
                        min_stretch_dist = offset;
                        matching_stretch = font.stretch;
                    }
                }
            }
        }
        let mut matching_style;
        match style {
            Style::Normal => {
                matching_style = Style::Italic;
                for font in self.fonts.iter().filter(|f| f.stretch == matching_stretch) {
                    match font.style {
                        Style::Normal => {
                            matching_style = style;
                            break;
                        }
                        Style::Oblique(_) => {
                            matching_style = font.style;
                        }
                        _ => {}
                    }
                }
            }
            Style::Oblique(_) => {
                matching_style = Style::Normal;
                for font in self.fonts.iter().filter(|f| f.stretch == matching_stretch) {
                    match font.style {
                        Style::Oblique(_) => {
                            matching_style = style;
                            break;
                        }
                        Style::Italic => {
                            matching_style = font.style;
                        }
                        _ => {}
                    }
                }
            }
            Style::Italic => {
                matching_style = Style::Normal;
                for font in self.fonts.iter().filter(|f| f.stretch == matching_stretch) {
                    match font.style {
                        Style::Italic => {
                            matching_style = style;
                            break;
                        }
                        Style::Oblique(_) => {
                            matching_style = font.style;
                        }
                        _ => {}
                    }
                }
            }
        }
        // If the desired weight is inclusively between 400 and 500.
        if weight >= Weight(400) && weight <= Weight(500) {
            // Weights greater than or equal to the target weight are checked
            // in ascending order until 500 is hit and checked.
            for font in self.fonts.iter().filter(|f| {
                f.stretch == matching_stretch
                    && f.style == matching_style
                    && f.weight >= weight
                    && f.weight <= Weight(500)
            }) {
                return Some(font.font_id);
            }
            // Followed by weights less than the target weight in descending
            // order.
            for font in self.fonts.iter().rev().filter(|f| {
                f.stretch == matching_stretch && f.style == matching_style && f.weight < weight
            }) {
                return Some(font.font_id);
            }
            // Followed by weights greater than 500, until a match is found.
            return self
                .fonts
                .iter()
                .filter(|f| {
                    f.stretch == matching_stretch
                        && f.style == matching_style
                        && f.weight > Weight(500)
                })
                .map(|f| f.font_id)
                .next();
        // If the desired weight is less than 400.
        } else if weight < Weight(400) {
            // Weights less than or equal to the desired weight are checked in
            // descending order.
            for font in self.fonts.iter().rev().filter(|f| {
                f.stretch == matching_stretch && f.style == matching_style && f.weight <= weight
            }) {
                return Some(font.font_id);
            }
            // Followed by weights above the desired weight in ascending order
            // until a match is found.
            return self
                .fonts
                .iter()
                .filter(|f| {
                    f.stretch == matching_stretch && f.style == matching_style && f.weight > weight
                })
                .map(|f| f.font_id)
                .next();
        // If the desired weight is greater than 500.
        } else {
            // Weights greater than or equal to the desired weight are checked
            // in ascending order.
            for font in self.fonts.iter().filter(|f| {
                f.stretch == matching_stretch && f.style == matching_style && f.weight >= weight
            }) {
                return Some(font.font_id);
            }
            // Followed by weights below the desired weight in descending order
            // until a match is found.
            return self
                .fonts
                .iter()
                .rev()
                .filter(|f| {
                    f.stretch == matching_stretch && f.style == matching_style && f.weight < weight
                })
                .map(|f| f.font_id)
                .next();
        }
    }
}
