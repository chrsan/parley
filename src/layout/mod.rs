//! Layout types.

use swash::GlyphId;

mod cluster;
mod run;

pub(crate) mod data;

use self::data::{ClusterData, LayoutData, RunData};

pub use self::run::RunMetrics;

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

    /// Returns the run at the given index.
    pub fn run(&self, index: usize) -> Option<Run> {
        self.data.runs.get(index).map(|data| Run {
            layout: &self.data,
            data,
        })
    }

    /// Returns an iterator over the runs in the layout.
    pub fn runs(&self) -> impl Iterator<Item = Run> + '_ + Clone {
        self.data.runs.iter().map(move |data| Run {
            layout: &self.data,
            data,
        })
    }
}

/// Sequence of clusters with a single font and style.
#[derive(Debug, Clone, Copy)]
pub struct Run<'a> {
    layout: &'a LayoutData,
    data: &'a RunData,
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
