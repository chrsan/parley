//! Context for layout.

use std::fmt;
use std::ops::RangeBounds;

use swash::shape::ShapeContext;
use swash::text::cluster::CharInfo;

use crate::bidi;
use crate::font::FontContext;
use crate::layout::Layout;
use crate::resolve::{
    range::{RangedStyle, RangedStyleBuilder},
    ResolveContext,
};
use crate::shape::shape_text;
use crate::style::StyleProperty;

/// Context for building a text layout.
pub struct LayoutContext {
    bidi: bidi::BidiResolver,
    rcx: ResolveContext,
    styles: Vec<RangedStyle>,
    rsb: RangedStyleBuilder,
    info: Vec<(CharInfo, u16)>,
    scx: ShapeContext,
}

impl LayoutContext {
    pub fn new() -> Self {
        Self {
            bidi: bidi::BidiResolver::new(),
            rcx: ResolveContext::default(),
            styles: vec![],
            rsb: RangedStyleBuilder::default(),
            info: vec![],
            scx: ShapeContext::default(),
        }
    }

    pub fn ranged_builder<'a>(
        &'a mut self,
        fcx: &'a mut FontContext,
        text: &'a str,
        scale: f32,
    ) -> RangedBuilder<'a> {
        self.begin(text);
        fcx.cache.reset();
        RangedBuilder {
            text,
            scale,
            // lcx: MaybeShared::Borrowed(self),
            // fcx: MaybeShared::Borrowed(fcx),
            lcx: self,
            fcx,
        }
    }

    fn begin(&mut self, text: &str) {
        self.rcx.clear();
        self.styles.clear();
        self.rsb.begin(text.len());
        self.info.clear();
        self.bidi.clear();
        let text = if text.is_empty() { " " } else { text };
        let mut a = swash::text::analyze(text.chars());
        for x in a.by_ref() {
            self.info.push((CharInfo::new(x.0, x.1), 0));
        }
        if a.needs_bidi_resolution() {
            self.bidi.resolve(
                text.chars()
                    .zip(self.info.iter().map(|info| info.0.bidi_class())),
                Some(0),
            );
        }
    }
}

impl Default for LayoutContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for LayoutContext {
    fn clone(&self) -> Self {
        // None of the internal state is visible so just return a new instance.
        Self::new()
    }
}

impl fmt::Debug for LayoutContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LayoutContext")
            .field("bidi", &self.bidi)
            .field("rcx", &self.rcx)
            .field("styles", &self.styles)
            .field("rsb", &self.rsb)
            .finish()
    }
}

/// Builder for constructing a text layout with ranged attributes.
pub struct RangedBuilder<'a> {
    text: &'a str,
    scale: f32,
    lcx: &'a mut LayoutContext,
    fcx: &'a mut FontContext,
}

impl<'a> RangedBuilder<'a> {
    pub fn push_default(&mut self, property: &StyleProperty) {
        let resolved = self.lcx.rcx.resolve(self.fcx, &property, self.scale);
        self.lcx.rsb.push_default(resolved);
    }

    pub fn push(&mut self, property: &StyleProperty, range: impl RangeBounds<usize>) {
        let resolved = self.lcx.rcx.resolve(self.fcx, &property, self.scale);
        self.lcx.rsb.push(resolved, range);
    }

    pub fn build_into(&mut self, layout: &mut Layout) -> bool {
        layout.data.clear();
        layout.data.scale = self.scale;
        if self.text.is_empty() {
            return false;
        }
        layout.data.has_bidi = !self.lcx.bidi.levels().is_empty();
        layout.data.base_level = !self.lcx.bidi.base_level();
        layout.data.text_len = self.text.len();
        self.lcx.rsb.finish(&mut self.lcx.styles);
        let mut char_index = 0;
        for (i, style) in self.lcx.styles.iter().enumerate() {
            for _ in self.text[style.range.clone()].chars() {
                self.lcx.info[char_index].1 = i as u16;
                char_index += 1;
            }
        }
        shape_text(
            &self.lcx.rcx,
            // &mut fcx,
            self.fcx,
            &self.lcx.styles,
            &self.lcx.info,
            self.lcx.bidi.levels(),
            &mut self.lcx.scx,
            self.text,
            layout,
        );
        layout.data.finish();
        true
    }

    pub fn build(&mut self) -> Option<Layout> {
        let mut layout = Layout::default();
        if self.build_into(&mut layout) {
            Some(layout)
        } else {
            None
        }
    }
}
