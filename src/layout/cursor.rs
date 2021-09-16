//! Hit testing.

use super::*;

/// Represents a position within a layout.
#[derive(Copy, Clone, Default, Debug)]
pub struct Cursor {
    path: CursorPath,
    baseline: f32,
    offset: f32,
    advance: f32,
    text_start: usize,
    text_end: usize,
    is_rtl: bool,
    is_leading: bool,
    is_inside: bool,
}

impl Cursor {
    /// Creates a new cursor from the specified layout and point.
    pub fn from_point<B: Brush>(layout: &Layout<B>, mut x: f32, y: f32) -> Self {
        let mut result = Self {
            is_inside: x >= 0. && y >= 0.,
            .. Default::default()
        };
        let last_line = layout.data.lines.len().saturating_sub(1);
        for (line_index, line) in layout.lines().enumerate() {
            let line_metrics = line.metrics();
            if y <= line_metrics.baseline || line_index == last_line {
                if y > line_metrics.baseline + line_metrics.leading * 0.5 {
                    result.is_inside = false;
                    x = f32::MAX;
                } else if y < 0. {
                    x = 0.;
                }
                result.baseline = line_metrics.baseline;
                result.path.line_index = line_index;
                let mut last_edge = line_metrics.offset;
                for (run_index, run) in line.runs().enumerate() {
                    result.path.run_index = run_index;
                    let cluster_range = run.data().cluster_range.clone();
                    for (cluster_index, cluster) in run.visual_clusters().enumerate() {
                        let range = cluster.text_range();
                        result.text_start = range.start;
                        result.text_end = range.end;
                        if run.is_rtl() {
                            result.path.cluster_index = cluster_range.end - cluster_index - 1;
                        } else {
                            result.path.cluster_index = cluster_index;
                        }
                        let advance = cluster.advance();
                        if x >= last_edge {
                            let far_edge = last_edge + advance;
                            if x < far_edge {
                                result.is_leading = false;
                                let middle = (last_edge + far_edge) * 0.5;
                                result.advance = advance;
                                if x <= middle {
                                    result.is_leading = true;
                                    result.offset = last_edge;
                                } else {
                                    result.is_leading = false;
                                    result.offset = far_edge;
                                }
                                return result;
                            }
                            last_edge = far_edge;
                        } else {
                            result.is_inside = false;
                            result.is_leading = true;
                            result.offset = line_metrics.offset;
                            return result;
                        }
                    }
                }
                break;
            }
        }
        result
    }

    /// Creates a new cursor for the specified layout and text position.
    pub fn from_position<B: Brush>(layout: &Layout<B>, mut position: usize) -> Self {
        let mut result = Self {
            is_leading: true,
            is_inside: true,
            .. Default::default()
        };
        if position >= layout.data.text_len {
            result.is_inside = false;
            result.is_leading = false;
            position = layout.data.text_len.saturating_sub(1);
        }
        let last_line = layout.data.lines.len().saturating_sub(1);
        for (line_index, line) in layout.lines().enumerate() {
            let line_metrics = line.metrics();
            result.baseline = line_metrics.baseline;
            result.path.line_index = line_index;
            if !line.text_range().contains(&position) && line_index != last_line {
                continue;
            }
            let mut last_edge = line_metrics.offset;
            result.offset = last_edge;
            for (run_index, run) in line.runs().enumerate() {
                result.path.run_index = run_index;
                if !run.text_range().contains(&position) {
                    continue;
                }
                let cluster_range = run.data().cluster_range.clone();
                for (cluster_index, cluster) in run.visual_clusters().enumerate() {
                    let range = cluster.text_range();
                    result.text_start = range.start;
                    result.text_end = range.end;
                    result.offset = last_edge;
                    if run.is_rtl() {
                        result.path.cluster_index = cluster_range.end - cluster_index - 1;
                    } else {
                        result.path.cluster_index = cluster_index;
                    }
                    let advance = cluster.advance();
                    if range.contains(&position) {
                        if !result.is_inside {
                            result.offset += advance;
                        }
                        result.advance = advance;
                        return result;
                    }
                    last_edge += advance;
                }
            }
            result.offset = last_edge;
            break;
        }
        result.is_leading = false;
        result.is_inside = false;
        result
    }

    /// Returns the path to the target cluster.
    pub fn path(&self) -> &CursorPath {
        &self.path
    }

    /// Returns the offset to the baseline.
    pub fn baseline(&self) -> f32 {
        self.baseline
    }

    /// Returns the offset to the target cluster along the baseline.
    pub fn offset(&self) -> f32 {
        self.offset
    }

    /// Returns the advance of the target cluster.
    pub fn advance(&self) -> f32 {
        self.advance
    }

    /// Returns the range of source text for the target cluster.
    pub fn text_range(&self) -> Range<usize> {
        self.text_start..self.text_end
    }

    /// Returns true if the cursor is on the leading edge of the target
    /// cluster.
    pub fn is_leading(&self) -> bool {
        self.is_leading
    }

    /// Returns true if the target cluster is part of a right-to-left run.
    pub fn is_rtl(&self) -> bool {
        self.is_rtl
    }

    /// Returns true if the cursor was created from a point or position
    /// that is inside the layout.
    pub fn is_inside(&self) -> bool {
        self.is_inside
    }
}

/// Index based path to a cluster.
#[derive(Copy, Clone, Default, Debug)]
pub struct CursorPath {
    /// Index of the containing line.
    pub line_index: usize,
    /// Index of the run within the containing line.
    pub run_index: usize,
    /// Index of the cluster within the containing run.
    pub cluster_index: usize,
}

impl CursorPath {
    /// Returns the line for this path and the specified layout.
    pub fn line<'a, B: Brush>(&self, layout: &'a Layout<B>) -> Option<Line<'a, B>> {
        layout.get(self.line_index)
    }

    /// Returns the run for this path and the specified layout.
    pub fn run<'a, B: Brush>(&self, layout: &'a Layout<B>) -> Option<Run<'a, B>> {
        self.line(layout)?.get(self.run_index)
    }

    /// Returns the cluster for this path and the specified layout.
    pub fn cluster<'a, B: Brush>(&self, layout: &'a Layout<B>) -> Option<Cluster<'a, B>> {
        self.run(layout)?.get(self.cluster_index)
    }
}