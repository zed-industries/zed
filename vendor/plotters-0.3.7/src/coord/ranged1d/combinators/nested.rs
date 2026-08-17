use crate::coord::ranged1d::{
    AsRangedCoord, DiscreteRanged, KeyPointHint, NoDefaultFormatting, Ranged, ValueFormatter,
};
use std::ops::Range;

/// Describe a value for a nested coordinate
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NestedValue<C, V> {
    /// Category value
    Category(C),
    /// One exact nested value
    Value(C, V),
}

impl<C, V> NestedValue<C, V> {
    /// Get the category of current nest value
    pub fn category(&self) -> &C {
        match self {
            NestedValue::Category(cat) => cat,
            NestedValue::Value(cat, _) => cat,
        }
    }
    /// Get the nested value from this value
    pub fn nested_value(&self) -> Option<&V> {
        match self {
            NestedValue::Category(_) => None,
            NestedValue::Value(_, val) => Some(val),
        }
    }
}

impl<C, V> From<(C, V)> for NestedValue<C, V> {
    fn from((cat, val): (C, V)) -> NestedValue<C, V> {
        NestedValue::Value(cat, val)
    }
}

impl<C, V> From<C> for NestedValue<C, V> {
    fn from(cat: C) -> NestedValue<C, V> {
        NestedValue::Category(cat)
    }
}

/// A nested coordinate spec which is a discrete coordinate on the top level and
/// for each value in discrete value, there is a secondary coordinate system.
/// And the value is defined as a tuple of primary coordinate value and secondary
/// coordinate value
pub struct NestedRange<Primary: DiscreteRanged, Secondary: Ranged> {
    primary: Primary,
    secondary: Vec<Secondary>,
}

impl<PT, ST, P, S> ValueFormatter<NestedValue<PT, ST>> for NestedRange<P, S>
where
    P: Ranged<ValueType = PT> + DiscreteRanged,
    S: Ranged<ValueType = ST>,
    P: ValueFormatter<PT>,
    S: ValueFormatter<ST>,
{
    fn format(value: &NestedValue<PT, ST>) -> String {
        match value {
            NestedValue::Category(cat) => P::format(cat),
            NestedValue::Value(_, val) => S::format(val),
        }
    }
}

impl<P: DiscreteRanged, S: Ranged> Ranged for NestedRange<P, S> {
    type FormatOption = NoDefaultFormatting;
    type ValueType = NestedValue<P::ValueType, S::ValueType>;

    fn range(&self) -> Range<Self::ValueType> {
        let primary_range = self.primary.range();

        let secondary_left = self.secondary[0].range().start;
        let secondary_right = self.secondary[self.primary.size() - 1].range().end;

        NestedValue::Value(primary_range.start, secondary_left)
            ..NestedValue::Value(primary_range.end, secondary_right)
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        let idx = self.primary.index_of(value.category()).unwrap_or(0);
        let total = self.primary.size();

        let bucket_size = (limit.1 - limit.0) / total as i32;
        let mut residual = (limit.1 - limit.0) % total as i32;

        if residual < 0 {
            residual += total as i32;
        }

        let s_left = limit.0 + bucket_size * idx as i32 + residual.min(idx as i32);
        let s_right = s_left + bucket_size + if (residual as usize) < idx { 1 } else { 0 };

        if let Some(secondary_value) = value.nested_value() {
            self.secondary[idx].map(secondary_value, (s_left, s_right))
        } else {
            (s_left + s_right) / 2
        }
    }

    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType> {
        if !hint.weight().allow_light_points() || hint.max_num_points() < self.primary.size() * 2 {
            self.primary
                .key_points(hint)
                .into_iter()
                .map(NestedValue::Category)
                .collect()
        } else {
            let secondary_size =
                (hint.max_num_points() - self.primary.size()) / self.primary.size();
            self.primary
                .values()
                .enumerate()
                .flat_map(|(idx, val)| {
                    std::iter::once(NestedValue::Category(val)).chain(
                        self.secondary[idx]
                            .key_points(secondary_size)
                            .into_iter()
                            .map(move |v| {
                                NestedValue::Value(self.primary.from_index(idx).unwrap(), v)
                            }),
                    )
                })
                .collect()
        }
    }
}

impl<P: DiscreteRanged, S: DiscreteRanged> DiscreteRanged for NestedRange<P, S> {
    fn size(&self) -> usize {
        self.secondary.iter().map(|x| x.size()).sum::<usize>()
    }

    fn index_of(&self, value: &Self::ValueType) -> Option<usize> {
        let p_idx = self.primary.index_of(value.category())?;
        let s_idx = self.secondary[p_idx].index_of(value.nested_value()?)?;
        Some(
            s_idx
                + self.secondary[..p_idx]
                    .iter()
                    .map(|x| x.size())
                    .sum::<usize>(),
        )
    }

    fn from_index(&self, mut index: usize) -> Option<Self::ValueType> {
        for (p_idx, snd) in self.secondary.iter().enumerate() {
            if snd.size() > index {
                return Some(NestedValue::Value(
                    self.primary.from_index(p_idx).unwrap(),
                    snd.from_index(index).unwrap(),
                ));
            }
            index -= snd.size();
        }
        None
    }
}

/// Used to build a nested coordinate system.
pub trait BuildNestedCoord: AsRangedCoord
where
    Self::CoordDescType: DiscreteRanged,
{
    /// Builds a nested coordinate system.
    fn nested_coord<S: AsRangedCoord>(
        self,
        builder: impl Fn(<Self::CoordDescType as Ranged>::ValueType) -> S,
    ) -> NestedRange<Self::CoordDescType, S::CoordDescType> {
        let primary: Self::CoordDescType = self.into();
        assert!(primary.size() > 0);

        let secondary = primary
            .values()
            .map(|value| builder(value).into())
            .collect();

        NestedRange { primary, secondary }
    }
}

impl<T: AsRangedCoord> BuildNestedCoord for T where T::CoordDescType: DiscreteRanged {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nested_coord() {
        let coord = (0..10).nested_coord(|x| 0..(x + 1));

        let range = coord.range();

        assert_eq!(NestedValue::Value(0, 0)..NestedValue::Value(10, 11), range);
        assert_eq!(coord.map(&NestedValue::Category(0), (0, 1100)), 50);
        assert_eq!(coord.map(&NestedValue::Value(0, 0), (0, 1100)), 0);
        assert_eq!(coord.map(&NestedValue::Value(5, 4), (0, 1100)), 567);

        assert_eq!(coord.size(), (2 + 12) * 11 / 2);
        assert_eq!(coord.index_of(&NestedValue::Value(5, 4)), Some(24));
        assert_eq!(coord.from_index(24), Some(NestedValue::Value(5, 4)));
    }
}
