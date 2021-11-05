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
    pub font_attrs: Vec<FontAttributes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontAttributes {
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
                for font_attrs in &self.font_attrs {
                    let font_stretch = if font_attrs.stretch > Stretch::NORMAL {
                        font_attrs.stretch.raw() as i32 - Stretch::NORMAL.raw() as i32
                            + Stretch::ULTRA_EXPANDED.raw() as i32
                    } else {
                        font_attrs.stretch.raw() as i32
                    };
                    let offset = (font_stretch - stretch.raw() as i32).abs();
                    if offset < min_stretch_dist {
                        min_stretch_dist = offset;
                        matching_stretch = font_attrs.stretch;
                    }
                }
            } else {
                for font_attrs in &self.font_attrs {
                    let font_stretch = if font_attrs.stretch < Stretch::NORMAL {
                        font_attrs.stretch.raw() as i32 - Stretch::NORMAL.raw() as i32
                            + Stretch::ULTRA_EXPANDED.raw() as i32
                    } else {
                        font_attrs.stretch.raw() as i32
                    };
                    let offset = (font_stretch - stretch.raw() as i32).abs();
                    if offset < min_stretch_dist {
                        min_stretch_dist = offset;
                        matching_stretch = font_attrs.stretch;
                    }
                }
            }
        }
        let mut matching_style;
        match style {
            Style::Normal => {
                matching_style = Style::Italic;
                for font_attrs in self
                    .font_attrs
                    .iter()
                    .filter(|a| a.stretch == matching_stretch)
                {
                    match font_attrs.style {
                        Style::Normal => {
                            matching_style = style;
                            break;
                        }
                        Style::Oblique(_) => {
                            matching_style = font_attrs.style;
                        }
                        _ => {}
                    }
                }
            }
            Style::Oblique(_) => {
                matching_style = Style::Normal;
                for font_attrs in self
                    .font_attrs
                    .iter()
                    .filter(|a| a.stretch == matching_stretch)
                {
                    match font_attrs.style {
                        Style::Oblique(_) => {
                            matching_style = style;
                            break;
                        }
                        Style::Italic => {
                            matching_style = font_attrs.style;
                        }
                        _ => {}
                    }
                }
            }
            Style::Italic => {
                matching_style = Style::Normal;
                for font_attrs in self
                    .font_attrs
                    .iter()
                    .filter(|a| a.stretch == matching_stretch)
                {
                    match font_attrs.style {
                        Style::Italic => {
                            matching_style = style;
                            break;
                        }
                        Style::Oblique(_) => {
                            matching_style = font_attrs.style;
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
            for font_attrs in self.font_attrs.iter().filter(|a| {
                a.stretch == matching_stretch
                    && a.style == matching_style
                    && a.weight >= weight
                    && a.weight <= Weight(500)
            }) {
                return Some(font_attrs.font_id);
            }
            // Followed by weights less than the target weight in descending
            // order.
            for font_attrs in self.font_attrs.iter().rev().filter(|a| {
                a.stretch == matching_stretch && a.style == matching_style && a.weight < weight
            }) {
                return Some(font_attrs.font_id);
            }
            // Followed by weights greater than 500, until a match is found.
            return self
                .font_attrs
                .iter()
                .filter(|a| {
                    a.stretch == matching_stretch
                        && a.style == matching_style
                        && a.weight > Weight(500)
                })
                .map(|a| a.font_id)
                .next();
        // If the desired weight is less than 400.
        } else if weight < Weight(400) {
            // Weights less than or equal to the desired weight are checked in
            // descending order.
            for font_attrs in self.font_attrs.iter().rev().filter(|a| {
                a.stretch == matching_stretch && a.style == matching_style && a.weight <= weight
            }) {
                return Some(font_attrs.font_id);
            }
            // Followed by weights above the desired weight in ascending order
            // until a match is found.
            return self
                .font_attrs
                .iter()
                .filter(|a| {
                    a.stretch == matching_stretch && a.style == matching_style && a.weight > weight
                })
                .map(|a| a.font_id)
                .next();
        // If the desired weight is greater than 500.
        } else {
            // Weights greater than or equal to the desired weight are checked
            // in ascending order.
            for font_attrs in self.font_attrs.iter().filter(|a| {
                a.stretch == matching_stretch && a.style == matching_style && a.weight >= weight
            }) {
                return Some(font_attrs.font_id);
            }
            // Followed by weights below the desired weight in descending order
            // until a match is found.
            return self
                .font_attrs
                .iter()
                .rev()
                .filter(|a| {
                    a.stretch == matching_stretch && a.style == matching_style && a.weight < weight
                })
                .map(|a| a.font_id)
                .next();
        }
    }
}
