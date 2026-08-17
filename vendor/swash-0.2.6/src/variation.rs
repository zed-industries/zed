use super::internal::{var::*, RawFont};
use super::{
    setting::Setting,
    string::{LocalizedString, StringId},
    FontRef, NormalizedCoord, Tag,
};

/// Proxy for rematerializing variations collections.
#[derive(Copy, Clone)]
pub struct VariationsProxy {
    fvar: u32,
    avar: u32,
    len: usize,
}

impl VariationsProxy {
    /// Creates a variations proxy from the specified font.
    pub fn from_font(font: &FontRef) -> Self {
        let fvar = font.table_offset(FVAR);
        let table = Fvar::from_font(font).unwrap_or_else(|| Fvar::new(&[]));
        let avar = font.table_offset(AVAR);
        let len = table.axis_count() as usize;
        Self { fvar, avar, len }
    }

    /// Materializes variations from the specified font. This proxy must have
    /// been created by the same font.
    pub fn materialize<'a>(&self, font: &FontRef<'a>) -> Variations<'a> {
        let data = if self.fvar != 0 {
            font.data.get(self.fvar as usize..).unwrap_or(&[])
        } else {
            &[]
        };
        Variations {
            font: *font,
            fvar: Fvar::new(data),
            avar: self.avar,
            len: self.len,
            pos: 0,
        }
    }
}

/// Iterator over a collection of font variations.
#[derive(Copy, Clone)]
pub struct Variations<'a> {
    font: FontRef<'a>,
    fvar: Fvar<'a>,
    avar: u32,
    len: usize,
    pos: usize,
}

impl<'a> Variations<'a> {
    pub(crate) fn from_font(font: &FontRef<'a>) -> Self {
        let fvar = Fvar::from_font(font).unwrap_or_else(|| Fvar::new(&[]));
        let avar = font.table_offset(AVAR);
        let len = fvar.axis_count() as usize;
        Self {
            font: *font,
            fvar,
            avar,
            len,
            pos: 0,
        }
    }

    fn get(&self, index: usize) -> Option<Variation<'a>> {
        let axis = self.fvar.get_axis(index as u16)?;
        Some(Variation {
            font: self.font,
            axis,
            avar: self.avar,
        })
    }

    /// Searches for a variation with the specified tag.
    ///
    /// ## Iteration behavior
    /// This function searches the entire variation collection without regard
    /// for the current state of the iterator.
    pub fn find_by_tag(&self, tag: Tag) -> Option<Variation<'a>> {
        for i in 0..self.len {
            if let Some(var) = self.get(i) {
                if var.tag() == tag {
                    return Some(var);
                }
            }
        }
        None
    }

    /// Returns an iterator over the set of normalized coordinates
    /// corresponding to the specified variation settings.
    pub fn normalized_coords<I>(&self, settings: I) -> impl Iterator<Item = NormalizedCoord> + Clone
    where
        I: IntoIterator,
        I::Item: Into<Setting<f32>>,
    {
        let mut copy = *self;
        copy.pos = 0;
        let mut coords = [0i16; 32];
        let len = self.len.min(32);
        for setting in settings {
            let val = setting.into();
            let tag = val.tag;
            for (var, coord) in copy.take(len).zip(coords.iter_mut()) {
                if var.axis.tag == tag {
                    *coord = var.normalize(val.value);
                }
            }
        }
        (0..len).map(move |i| coords[i])
    }
}

impl_iter!(Variations, Variation);

/// Axis of variation in a variable font.
#[derive(Copy, Clone)]
pub struct Variation<'a> {
    font: FontRef<'a>,
    axis: VarAxis,
    avar: u32,
}

impl<'a> Variation<'a> {
    /// Returns the index of the variation.
    pub fn index(&self) -> usize {
        self.axis.index as usize
    }

    /// Returns the tag that identifies the variation.
    pub fn tag(&self) -> Tag {
        self.axis.tag
    }

    /// Returns the name identifier for the variation.
    pub fn name_id(&self) -> StringId {
        StringId::Other(self.axis.name_id)
    }

    /// Returns the name for the variation, optionally for a
    /// particular language.
    pub fn name(&self, language: Option<&str>) -> Option<LocalizedString<'a>> {
        self.font
            .localized_strings()
            .find_by_id(self.name_id(), language)
    }

    /// Returns true if the variation should be hidden from users.
    pub fn is_hidden(&self) -> bool {
        self.axis.is_hidden()
    }

    /// Returns the minimum value of the variation.
    pub fn min_value(&self) -> f32 {
        self.axis.min.to_f32()
    }

    /// Returns the maximum value of the variation.
    pub fn max_value(&self) -> f32 {
        self.axis.max.to_f32()
    }

    /// Returns the default value of the variation.
    pub fn default_value(&self) -> f32 {
        self.axis.default.to_f32()
    }

    /// Computes a normalized coordinate for the specified value.
    pub fn normalize(&self, value: f32) -> NormalizedCoord {
        let avar = if self.avar != 0 {
            Some((self.font.data, self.avar))
        } else {
            None
        };
        self.axis.normalized_coord(value.into(), avar)
    }
}

/// Iterator over a collection of named variation instances.
#[derive(Copy, Clone)]
pub struct Instances<'a> {
    font: FontRef<'a>,
    fvar: Fvar<'a>,
    avar: u32,
    len: usize,
    pos: usize,
}

impl<'a> Instances<'a> {
    pub(crate) fn from_font(font: &FontRef<'a>) -> Self {
        let fvar = Fvar::from_font(font).unwrap_or_else(|| Fvar::new(&[]));
        let avar = font.table_offset(AVAR);
        Self {
            font: *font,
            fvar,
            avar,
            len: fvar.instance_count() as usize,
            pos: 0,
        }
    }

    fn get(&self, index: usize) -> Option<Instance<'a>> {
        let inner = self.fvar.get_instance(index as u16)?;
        Some(Instance {
            parent: *self,
            inner,
        })
    }

    /// Searches for an instance with the specified name.
    ///
    /// ## Iteration behavior
    /// This function searches the entire instance collection without regard
    /// for the current state of the iterator.
    pub fn find_by_name(&self, name: &str) -> Option<Instance<'a>> {
        let strings = self.font.localized_strings();
        for i in 0..self.len {
            if let Some(instance) = self.get(i) {
                let id = instance.name_id();
                for instance_name in strings.filter(|s| s.id() == id) {
                    if instance_name.chars().eq(name.chars()) {
                        return Some(instance);
                    }
                }
            }
        }
        None
    }
    /// Searches for an instance with the specified PostScript name.
    ///
    /// ## Iteration behavior
    /// This function searches the entire instance collection without regard
    /// for the current state of the iterator.
    pub fn find_by_postscript_name(&self, name: &str) -> Option<Instance<'a>> {
        let strings = self.font.localized_strings();
        for i in 0..self.len {
            if let Some(instance) = self.get(i) {
                if let Some(id) = instance.postscript_name_id() {
                    for instance_name in strings.filter(|s| s.id() == id) {
                        if instance_name.chars().eq(name.chars()) {
                            return Some(instance);
                        }
                    }
                }
            }
        }
        None
    }
}

impl_iter!(Instances, Instance);

/// Named instance in a variable font.
#[derive(Copy, Clone)]
pub struct Instance<'a> {
    parent: Instances<'a>,
    inner: VarInstance<'a>,
}

impl<'a> Instance<'a> {
    /// Returns the index of the instance.
    pub fn index(&self) -> usize {
        self.inner.index as usize
    }

    /// Returns the name identifier for the instance.
    pub fn name_id(&self) -> StringId {
        StringId::Other(self.inner.name_id)
    }

    /// Returns the name for the instance, optionally for a
    /// particular language.
    pub fn name(&self, language: Option<&str>) -> Option<LocalizedString<'a>> {
        self.parent
            .font
            .localized_strings()
            .find_by_id(self.name_id(), language)
    }

    /// Returns the PostScript name identifier for the instance.
    pub fn postscript_name_id(&self) -> Option<StringId> {
        self.inner.postscript_name_id.map(StringId::Other)
    }

    /// Returns the PostScript name for the instance, optionally for a
    /// particular language.
    pub fn postscript_name(&self, language: Option<&str>) -> Option<LocalizedString<'a>> {
        self.parent
            .font
            .localized_strings()
            .find_by_id(self.postscript_name_id()?, language)
    }

    /// Returns an iterator over the variation values of the instance.
    pub fn values(&self) -> impl Iterator<Item = f32> + 'a {
        self.inner.values.iter().map(|v| v.to_f32())
    }

    /// Returns an iterator over the normalized coordinates for the instance.
    pub fn normalized_coords(&self) -> impl Iterator<Item = NormalizedCoord> + 'a {
        let avar = if self.parent.avar != 0 {
            Some((self.parent.font.data, self.parent.avar))
        } else {
            None
        };
        let fvar = self.parent.fvar;
        (0..fvar.axis_count())
            .map(move |i| fvar.get_axis(i).unwrap_or_default())
            .zip(self.inner.values)
            .map(move |(axis, value)| axis.normalized_coord(value, avar))
    }
}
