use super::*;

use std::ops::{Index, IndexMut};

/**
This is a map from fields to some value.

If you put this in a type,and use Default to initialize it,
you must remember to replace the `FieldMap` using either `FieldMap::defaulted` or `FieldMap::with`

*/
#[derive(Debug)]
pub struct FieldMap<T> {
    // The outer vec is the enum variant (if it's a struct/union it's a single element Vec),
    // the inner one is the field within a variant/struct/union.
    fields: Vec<Vec<T>>,
}

impl<T> FieldMap<T> {
    /// Constructs an FieldMap which maps each field in the DataStructure to a value
    /// (obtained by mapping each individual field to `T` using a closure).
    pub fn with<'a, F>(ds: &'a DataStructure<'a>, mut f: F) -> Self
    where
        F: FnMut(&'a Field<'a>) -> T,
    {
        Self {
            fields: ds
                .variants
                .iter()
                .map(|vari| vari.fields.iter().map(&mut f).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (FieldIndex, &'_ T)> + Clone + '_ {
        self.fields.iter().enumerate().flat_map(|(v_i, v)| {
            v.iter().enumerate().map(move |(f_i, f)| {
                let index = FieldIndex {
                    variant: v_i as _,
                    pos: f_i as _,
                };
                (index, f)
            })
        })
    }

    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (FieldIndex, &'_ mut T)> + '_ {
        self.fields.iter_mut().enumerate().flat_map(|(v_i, v)| {
            v.iter_mut().enumerate().map(move |(f_i, f)| {
                let index = FieldIndex {
                    variant: v_i as _,
                    pos: f_i as _,
                };
                (index, f)
            })
        })
    }
}

impl<'a, T> Index<FieldIndex> for FieldMap<T> {
    type Output = T;

    fn index(&self, index: FieldIndex) -> &T {
        &self.fields[index.variant][index.pos]
    }
}

impl<'a, T> IndexMut<FieldIndex> for FieldMap<T> {
    fn index_mut(&mut self, index: FieldIndex) -> &mut T {
        &mut self.fields[index.variant][index.pos]
    }
}

impl<'a, T> Index<&'a Field<'a>> for FieldMap<T> {
    type Output = T;

    fn index(&self, field: &'a Field<'a>) -> &T {
        let index = field.index;
        &self.fields[index.variant][index.pos]
    }
}

impl<'a, T> IndexMut<&'a Field<'a>> for FieldMap<T> {
    fn index_mut(&mut self, field: &'a Field<'a>) -> &mut T {
        let index = field.index;
        &mut self.fields[index.variant][index.pos]
    }
}
