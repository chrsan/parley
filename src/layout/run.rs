use std::ops::Range;

use swash::NormalizedCoord;

use crate::util::Synthesis;
use crate::FontHandle;

use super::{Cluster, Run};

impl<'a> Run<'a> {
    /// Returns the font for the run.
    pub fn font(&self) -> &FontHandle {
        self.layout.fonts.get(self.data.font_index).unwrap()
    }

    /// Returns the font size for the run.
    pub fn font_size(&self) -> f32 {
        self.data.font_size
    }

    /// Returns the synthesis suggestions for the font associated with the run.
    pub fn synthesis(&self) -> Synthesis {
        self.data.synthesis
    }

    /// Returns the normalized variation coordinates for the font associated
    /// with the run.
    pub fn normalized_coords(&self) -> &[NormalizedCoord] {
        self.layout
            .coords
            .get(self.data.coords_range.clone())
            .unwrap_or(&[])
    }

    /// Returns metrics for the run.
    pub fn metrics(&self) -> &RunMetrics {
        &self.data.metrics
    }

    /// Returns the advance for the run.
    pub fn advance(&self) -> f32 {
        self.data.advance
    }

    /// Returns the original text range for the run.
    pub fn text_range(&self) -> Range<usize> {
        self.data.text_range.clone()
    }

    /// Returns true if the run has right-to-left directionality.
    pub fn is_rtl(&self) -> bool {
        self.data.bidi_level & 1 != 0
    }

    /// Returns the number of clusters in the run.
    pub fn len(&self) -> usize {
        self.data.cluster_range.len()
    }

    /// Returns true if the run is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the cluster at the specified index.
    pub fn get(&self, index: usize) -> Option<Cluster<'a>> {
        let range = &self.data.cluster_range;
        let index = range.start + index;
        Some(Cluster {
            run: self.clone(),
            data: self.layout.clusters.get(index)?,
        })
    }

    /// Returns an iterator over the clusters in logical order.
    pub fn clusters(&'a self) -> impl Iterator<Item = Cluster<'a>> + 'a + Clone {
        let range = self.data.cluster_range.clone();
        Clusters {
            run: self,
            range,
            rev: false,
        }
    }

    /// Returns an iterator over the clusters in visual order.
    pub fn visual_clusters(&'a self) -> impl Iterator<Item = Cluster<'a>> + 'a + Clone {
        let range = self.data.cluster_range.clone();
        Clusters {
            run: self,
            range,
            rev: self.is_rtl(),
        }
    }
}

struct Clusters<'a> {
    run: &'a Run<'a>,
    range: Range<usize>,
    rev: bool,
}

impl<'a> Clone for Clusters<'a> {
    fn clone(&self) -> Self {
        Self {
            run: self.run,
            range: self.range.clone(),
            rev: self.rev,
        }
    }
}

impl<'a> Iterator for Clusters<'a> {
    type Item = Cluster<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = if self.rev {
            self.range.next_back()?
        } else {
            self.range.next()?
        };
        Some(Cluster {
            run: self.run.clone(),
            data: self.run.layout.clusters.get(index)?,
        })
    }
}

/// Metrics information for a run.
#[derive(Debug, Default, Clone, Copy)]
pub struct RunMetrics {
    /// Typographic ascent.
    pub ascent: f32,
    /// Typographic descent.
    pub descent: f32,
    /// Typographic leading.
    pub leading: f32,
}
