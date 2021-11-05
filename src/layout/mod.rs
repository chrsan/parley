//! Layout types.

use swash::GlyphId;

use super::font::FontHandle;

mod cluster;
mod line;
mod run;

pub(crate) mod data;

use self::data::{ClusterData, LayoutData, LineData, LineRunData, RunData};

pub use self::line::greedy::BreakLines;
pub use self::line::{GlyphRun, LineMetrics};
pub use self::run::RunMetrics;

/// Alignment of a layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Alignment {
    Start,
    Middle,
    End,
    Justified,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Start
    }
}

/// Text layout.
#[derive(Debug, Default, Clone)]
pub struct Layout {
    pub(crate) data: LayoutData,
}

impl Layout {
    /// Creates an empty layout.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the scale factor provided when creating the layout.
    pub fn scale(&self) -> f32 {
        self.data.scale
    }

    /// Returns the style collection for the layout.
    pub fn styles(&self) -> &[Style] {
        &self.data.styles
    }

    /// Returns the width of the layout.
    pub fn width(&self) -> f32 {
        self.data.width
    }

    /// Returns the width of the layout, including the width of any trailing
    /// whitespace.
    pub fn full_width(&self) -> f32 {
        self.data.full_width
    }

    /// Returns the height of the layout.
    pub fn height(&self) -> f32 {
        self.data.height
    }

    /// Returns the number of lines in the layout.
    pub fn len(&self) -> usize {
        self.data.lines.len()
    }

    /// Returns true if the layout is empty.
    pub fn is_empty(&self) -> bool {
        self.data.lines.is_empty()
    }

    /// Returns the line at the specified index.
    pub fn get(&self, index: usize) -> Option<Line> {
        Some(Line {
            layout: &self.data,
            data: self.data.lines.get(index)?,
        })
    }

    /// Returns an iterator over the lines in the layout.
    pub fn lines(&self) -> impl Iterator<Item = Line> + '_ + Clone {
        self.data.lines.iter().map(move |data| Line {
            layout: &self.data,
            data,
        })
    }

    /// Returns line breaker to compute lines for the layout.
    pub fn break_lines(&mut self) -> BreakLines {
        BreakLines::new(&mut self.data)
    }

    /// Breaks all lines with the specified maximum advance and alignment.
    pub fn break_all_lines(&mut self, max_advance: Option<f32>, alignment: Alignment) {
        self.break_lines()
            .break_remaining(max_advance.unwrap_or(f32::MAX), alignment)
    }

    /// Returns an iterator over the runs in the layout.
    pub fn runs(&self) -> impl Iterator<Item = Run> + '_ + Clone {
        self.data.runs.iter().map(move |data| Run {
            layout: &self.data,
            data,
            line_data: None,
        })
    }
}

/// Sequence of clusters with a single font and style.
#[derive(Debug, Clone, Copy)]
pub struct Run<'a> {
    layout: &'a LayoutData,
    data: &'a RunData,
    line_data: Option<&'a LineRunData>,
}

/// Atomic unit of text.
#[derive(Debug, Clone, Copy)]
pub struct Cluster<'a> {
    run: Run<'a>,
    data: &'a ClusterData,
}

/// Glyph with an offset and advance.
#[derive(Default, Debug, Clone, Copy)]
pub struct Glyph {
    pub id: GlyphId,
    pub style_index: u16,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
}

impl Glyph {
    /// Returns the index into the layout style collection.
    pub fn style_index(&self) -> usize {
        self.style_index as usize
    }
}

/// Line in a text layout.
#[derive(Debug, Clone, Copy)]
pub struct Line<'a> {
    layout: &'a LayoutData,
    data: &'a LineData,
}

/// Style properties.
#[derive(Debug, Clone)]
pub struct Style {
    /// Multiplicative line height factor.
    pub(crate) line_height: f32,
}
