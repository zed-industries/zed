/*!
Shaper that partitions by font using trait based per-cluster selection.

*/

use super::{Direction, ShapeContext, Shaper};
use crate::text::cluster::{CharCluster, Parser, Token};
use crate::text::{Codepoint as _, Language, Script};
use crate::{FontRef, Setting, Synthesis};
use core::iter::Copied;
use core::slice;

/// Trait for types that can select appropriate fonts for character clusters.
pub trait Selector {
    /// Type of font returned by the provider.
    type SelectedFont: SelectedFont;

    /// Selects a font for the specified character cluster.
    fn select_font(&mut self, cluster: &mut CharCluster) -> Option<Self::SelectedFont>;
}

/// Trait for a font provided by a font selector.
pub trait SelectedFont: PartialEq {
    /// Returns a reference to the underlying font.
    fn font(&self) -> FontRef<'_>;

    fn id_override(&self) -> Option<[u64; 2]> {
        None
    }

    /// Returns an optional synthesis state for the font.
    fn synthesis(&self) -> Option<Synthesis> {
        None
    }
}

/// Trait for types that specify shaping options.
pub trait ShapeOptions {
    /// Iterator over the feature settings for a fragment.
    type Features: Iterator<Item = Setting<u16>>;

    /// Iterator over the variation settings for a fragment.
    type Variations: Iterator<Item = Setting<f32>>;

    /// Returns the script of the text in the fragment.
    fn script(&self) -> Script {
        Script::Latin
    }

    /// Returns the language of the text in the fragment.
    fn language(&self) -> Option<Language> {
        None
    }

    /// Returns the direction for the fragment.
    fn direction(&self) -> Direction {
        Direction::LeftToRight
    }

    /// Returns the font size for the fragment.
    fn size(&self) -> f32 {
        0.
    }

    /// Returns an iterator over the feature settings for the fragment.
    fn features(&self) -> Self::Features;

    /// Returns an iterator over the variation settings for the fragment.
    fn variations(&self) -> Self::Variations;

    /// Returns true if the shaper should insert dotted circles for broken
    /// clusters in the fragment.
    fn insert_dotted_circles(&self) -> bool {
        false
    }
}

/// Simple implementation of the shape options trait.
#[derive(Copy, Clone)]
pub struct SimpleShapeOptions<'a> {
    /// Script for the fragment.
    pub script: Script,
    /// Language for the fragment.
    pub language: Option<Language>,
    /// Text direction of the fragment.
    pub direction: Direction,
    /// Font size for the fragment.
    pub size: f32,
    /// Feature settings for the fragment.
    pub features: &'a [Setting<u16>],
    /// Variation settings for the fragment.
    pub variations: &'a [Setting<f32>],
    /// True if the shaper should insert dotted circles for broken clusters.
    pub insert_dotted_circles: bool,
}

impl Default for SimpleShapeOptions<'_> {
    fn default() -> Self {
        Self {
            script: Script::Latin,
            language: None,
            direction: Direction::LeftToRight,
            size: 0.,
            features: &[],
            variations: &[],
            insert_dotted_circles: false,
        }
    }
}

impl<'a> ShapeOptions for SimpleShapeOptions<'a> {
    type Features = Copied<slice::Iter<'a, Setting<u16>>>;
    type Variations = Copied<slice::Iter<'a, Setting<f32>>>;

    fn script(&self) -> Script {
        self.script
    }

    fn language(&self) -> Option<Language> {
        self.language
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn size(&self) -> f32 {
        self.size
    }

    fn features(&self) -> Self::Features {
        self.features.iter().copied()
    }

    fn variations(&self) -> Self::Variations {
        self.variations.iter().copied()
    }

    fn insert_dotted_circles(&self) -> bool {
        self.insert_dotted_circles
    }
}

/// Shapes a run of text (provided as a [`Token`] iterator) using the specified [`Selector`] to assign
/// per-cluster fonts. Invokes the specified closure `f` for each contiguous fragment with the chosen font
/// and a prepared [`Shaper`] that contains the cluster content of the fragment.
pub fn shape<S, T>(
    context: &mut ShapeContext,
    selector: &mut S,
    options: &impl ShapeOptions,
    tokens: T,
    mut f: impl FnMut(&S::SelectedFont, Shaper),
) where
    S: Selector,
    T: IntoIterator<Item = Token>,
    T::IntoIter: Clone,
{
    let mut cluster = CharCluster::new();
    if options.direction() == Direction::RightToLeft {
        let mut parser = Parser::new(
            options.script(),
            tokens.into_iter().map(|mut t| {
                t.ch = t.ch.mirror().unwrap_or(t.ch);
                t
            }),
        );
        if !parser.next(&mut cluster) {
            return;
        }
        let mut font = selector.select_font(&mut cluster);
        while shape_clusters(
            context,
            selector,
            options,
            &mut parser,
            &mut cluster,
            &mut font,
            &mut f,
        ) {}
    } else {
        let mut parser = Parser::new(options.script(), tokens.into_iter());
        if !parser.next(&mut cluster) {
            return;
        }
        let mut font = selector.select_font(&mut cluster);
        while shape_clusters(
            context,
            selector,
            options,
            &mut parser,
            &mut cluster,
            &mut font,
            &mut f,
        ) {}
    }
}

fn shape_clusters<S, T>(
    context: &mut ShapeContext,
    selector: &mut S,
    options: &impl ShapeOptions,
    parser: &mut Parser<T>,
    cluster: &mut CharCluster,
    current_font: &mut Option<S::SelectedFont>,
    f: &mut impl FnMut(&S::SelectedFont, Shaper),
) -> bool
where
    S: Selector,
    T: Iterator<Item = Token> + Clone,
{
    if current_font.is_none() {
        return false;
    }
    let font = current_font.as_ref().unwrap();
    let font_ref = font.font();
    let id = font
        .id_override()
        .unwrap_or([font_ref.key.value(), u64::MAX]);
    let mut shaper = context
        .builder_with_id(font.font(), id)
        .script(options.script())
        .language(options.language())
        .direction(options.direction())
        .size(options.size())
        .features(options.features())
        .variations(
            font.synthesis()
                .as_ref()
                .map(|s| s.variations())
                .unwrap_or(&[])
                .iter()
                .copied(),
        )
        .variations(options.variations())
        .insert_dotted_circles(options.insert_dotted_circles())
        .build();
    loop {
        shaper.add_cluster(cluster);
        if !parser.next(cluster) {
            f(font, shaper);
            return false;
        }
        if let Some(next_font) = selector.select_font(cluster) {
            if next_font != *font {
                f(font, shaper);
                *current_font = Some(next_font);
                return true;
            }
        } else {
            return false;
        }
    }
}
