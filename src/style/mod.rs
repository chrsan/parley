//! Rich styling support.

mod font;

pub use font::{
    FontFamily, FontFeature, FontSettings, FontStack, FontStretch, FontStyle, FontVariation,
    FontWeight, ObliqueAngle,
};

/// Properties that define a style.
#[derive(Debug, Clone, PartialEq)]
pub enum StyleProperty<'a> {
    /// Font family stack.
    FontStack(FontStack<'a>),
    /// Font size.
    FontSize(f32),
    /// Font stretch.
    FontStretch(FontStretch),
    /// Font style.
    FontStyle(FontStyle),
    /// Font weight.
    FontWeight(FontWeight),
    /// Font variation settings.
    FontVariations(FontSettings<'a, FontVariation>),
    /// Font feature settings.
    FontFeatures(FontSettings<'a, FontFeature>),
    /// Locale.
    Locale(Option<&'a str>),
    /// Line height multiplier.
    LineHeight(f32),
    /// Extra spacing between words.
    WordSpacing(f32),
    /// Extra spacing between letters.
    LetterSpacing(f32),
}
