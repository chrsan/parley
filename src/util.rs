//! Misc helpers.

use std::fmt;
use std::ops::Deref;

pub fn nearly_eq(x: f32, y: f32) -> bool {
    (x - y).abs() < f32::EPSILON
}

pub fn nearly_zero(x: f32) -> bool {
    nearly_eq(x, 0.)
}

#[derive(Default, Clone, Copy)]
pub struct Synthesis(swash::Synthesis);

impl From<swash::Synthesis> for Synthesis {
    fn from(synthesis: swash::Synthesis) -> Self {
        Self(synthesis)
    }
}

impl From<Synthesis> for swash::Synthesis {
    fn from(synthesis: Synthesis) -> Self {
        synthesis.0
    }
}

impl AsRef<swash::Synthesis> for Synthesis {
    fn as_ref(&self) -> &swash::Synthesis {
        &self.0
    }
}

impl Deref for Synthesis {
    type Target = swash::Synthesis;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Synthesis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Synthesis")
            .field("variations", &self.0.variations())
            .field("embolden", &self.0.embolden())
            .field("skew", &self.0.skew())
            .finish()
    }
}
