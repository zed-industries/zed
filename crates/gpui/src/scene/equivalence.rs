use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SceneEquivalence {
    primitives: Vec<PrimitiveRecord>,
    operations: Vec<String>,
    valid_batches: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PrimitiveRecord {
    identity: String,
    bounds: [u32; 4],
    order: (DrawOrder, u8, usize),
}

impl SceneEquivalence {
    pub(crate) fn compare(&self, fresh: &Self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.valid_batches && fresh.valid_batches,
            "invalid scene batch ordering"
        );
        anyhow::ensure!(
            self.operations == fresh.operations,
            "painter operations differ"
        );
        anyhow::ensure!(
            self.primitives.len() == fresh.primitives.len(),
            "primitive counts differ"
        );
        for (left, right) in self.primitives.iter().zip(&fresh.primitives) {
            anyhow::ensure!(
                left.identity == right.identity,
                "primitive fields differ: {} != {}",
                left.identity,
                right.identity
            );
        }
        let mut spatial_order: Vec<_> = (0..self.primitives.len()).collect();
        spatial_order.sort_by(|left, right| {
            f32::from_bits(self.primitives[*left].bounds[0])
                .total_cmp(&f32::from_bits(self.primitives[*right].bounds[0]))
        });
        for (index, &left) in spatial_order.iter().enumerate() {
            let left_bounds = self.primitives[left].bounds.map(f32::from_bits);
            for &right in &spatial_order[index + 1..] {
                let right_bounds = self.primitives[right].bounds.map(f32::from_bits);
                if right_bounds[0] >= left_bounds[2] {
                    break;
                }
                if left_bounds[1] < right_bounds[3] && right_bounds[1] < left_bounds[3] {
                    anyhow::ensure!(
                        self.primitives[left]
                            .order
                            .cmp(&self.primitives[right].order)
                            == fresh.primitives[left]
                                .order
                                .cmp(&fresh.primitives[right].order),
                        "overlapping primitive order differs: {} / {}",
                        self.primitives[left].identity,
                        self.primitives[right].identity
                    );
                }
            }
        }
        Ok(())
    }
}

impl Scene {
    pub(crate) fn equivalence_snapshot(&self) -> SceneEquivalence {
        let mut primitives = Vec::new();
        let mut valid_batches = true;
        macro_rules! append {
            ($field:ident, $kind:ident) => {
                valid_batches &= self.$field.is_sorted_by_key(|value| value.order);
                valid_batches &= self
                    .$field
                    .last()
                    .is_none_or(|value| value.order < u32::MAX);
                primitives.extend(self.$field.iter().enumerate().map(|(index, primitive)| {
                    let bounds = primitive.bounds.intersect(&primitive.content_mask.bounds);
                    let mut normalized = primitive.clone();
                    normalized.order = 0;
                    PrimitiveRecord {
                        identity: format!("{}:{normalized:?}", stringify!($field)),
                        bounds: [
                            bounds.origin.x.0.to_bits(),
                            bounds.origin.y.0.to_bits(),
                            bounds.right().0.to_bits(),
                            bounds.bottom().0.to_bits(),
                        ],
                        order: (primitive.order, PrimitiveKind::$kind as u8, index),
                    }
                }));
            };
        }
        append!(shadows, Shadow);
        append!(quads, Quad);
        append!(paths, Path);
        append!(underlines, Underline);
        append!(monochrome_sprites, MonochromeSprite);
        append!(subpixel_sprites, SubpixelSprite);
        append!(polychrome_sprites, PolychromeSprite);
        append!(surfaces, Surface);
        macro_rules! check_textures {
            ($($field:ident),*) => {
                $(valid_batches &= self.$field.is_sorted_by_key(|value| (
                    value.order, value.tile.texture_id.index
                ));)*
            };
        }
        check_textures!(monochrome_sprites, subpixel_sprites, polychrome_sprites);
        primitives.sort_by(|left, right| {
            left.identity
                .cmp(&right.identity)
                .then(left.order.cmp(&right.order))
        });
        let operations = if let Some(runs) = &self.runs {
            runs.canonical_operations()
        } else {
            self.canonical_operations(0)
        };
        SceneEquivalence {
            primitives,
            operations,
            valid_batches,
        }
    }

    pub(super) fn canonical_operations(&self, path_offset: usize) -> Vec<String> {
        self.paint_operations
            .iter()
            .map(|operation| match *operation {
                PaintOperation::StartLayer(bounds) => format!("layer:{bounds:?}"),
                PaintOperation::EndLayer => "end-layer".into(),
                PaintOperation::Primitive(primitive) => {
                    macro_rules! normalized {
                        ($value:expr, $name:literal) => {{
                            let mut value = $value;
                            value.order = 0;
                            format!(concat!($name, ":{:?}"), value)
                        }};
                    }
                    match primitive {
                        RecordedPrimitive::Shadow(value) => normalized!(value, "shadow"),
                        RecordedPrimitive::Quad(value) => normalized!(value, "quad"),
                        RecordedPrimitive::Underline(value) => normalized!(value, "underline"),
                        RecordedPrimitive::MonochromeSprite(value) => {
                            normalized!(value, "monochrome")
                        }
                        RecordedPrimitive::SubpixelSprite(value) => normalized!(value, "subpixel"),
                        RecordedPrimitive::PolychromeSprite(value) => {
                            normalized!(value, "polychrome")
                        }
                        RecordedPrimitive::Path { id, .. } => {
                            let mut path = self.path(id).clone();
                            path.id.0 += path_offset;
                            normalized!(path, "path")
                        }
                        RecordedPrimitive::Surface { index, .. } => {
                            normalized!(self.surface(index).clone(), "surface")
                        }
                    }
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_swapped_overlapping_primitives() {
        let record = |identity: &str, order| PrimitiveRecord {
            identity: identity.into(),
            bounds: [
                0.0_f32.to_bits(),
                0.0_f32.to_bits(),
                10.0_f32.to_bits(),
                10.0_f32.to_bits(),
            ],
            order: (order, PrimitiveKind::Quad as u8, 0),
        };
        let fresh = SceneEquivalence {
            primitives: vec![record("left", 1), record("right", 2)],
            operations: vec!["left".into(), "right".into()],
            valid_batches: true,
        };
        let mut swapped = fresh.clone();
        swapped.primitives[0].order.0 = 2;
        swapped.primitives[1].order.0 = 1;
        assert!(swapped.compare(&fresh).is_err());
        let mut renumbered = fresh.clone();
        renumbered.primitives[0].order.0 = 100;
        renumbered.primitives[1].order.0 = 200;
        assert!(renumbered.compare(&fresh).is_ok());
    }
}
