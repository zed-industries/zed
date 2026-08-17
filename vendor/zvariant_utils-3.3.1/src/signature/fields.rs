use super::Signature;

/// Signatures of the fields of a [`Signature::Structure`].
#[derive(Debug, Clone)]
pub enum Fields {
    Static {
        fields: &'static [&'static Signature],
    },
    Dynamic {
        fields: Box<[Signature]>,
    },
}

impl Fields {
    /// A iterator over the fields' signatures.
    pub fn iter(&self) -> impl Iterator<Item = &Signature> {
        use std::slice::Iter;

        enum Fields<'a> {
            Static(Iter<'static, &'static Signature>),
            Dynamic(Iter<'a, Signature>),
        }

        impl<'a> Iterator for Fields<'a> {
            type Item = &'a Signature;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Fields::Static(iter) => iter.next().copied(),
                    Fields::Dynamic(iter) => iter.next(),
                }
            }
        }

        match self {
            Self::Static { fields } => Fields::Static(fields.iter()),
            Self::Dynamic { fields } => Fields::Dynamic(fields.iter()),
        }
    }

    /// The number of fields.
    pub const fn len(&self) -> usize {
        match self {
            Self::Static { fields } => fields.len(),
            Self::Dynamic { fields } => fields.len(),
        }
    }

    /// Whether there are no fields.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<Box<[Signature]>> for Fields {
    fn from(fields: Box<[Signature]>) -> Self {
        Fields::Dynamic { fields }
    }
}

impl From<Vec<Signature>> for Fields {
    fn from(fields: Vec<Signature>) -> Self {
        Fields::Dynamic {
            fields: fields.into(),
        }
    }
}

impl<const N: usize> From<[Signature; N]> for Fields {
    fn from(fields: [Signature; N]) -> Self {
        Fields::Dynamic {
            fields: fields.into(),
        }
    }
}

impl From<&'static [&'static Signature]> for Fields {
    fn from(fields: &'static [&'static Signature]) -> Self {
        Fields::Static { fields }
    }
}
