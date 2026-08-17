use alloc::vec;
use ttf_parser::colr::{ClipBox, CompositeMode, Paint};
use ttf_parser::{GlyphId, RectF, Transform};

/*
 * This file implements bounds-extraction as well as boundedness
 * computation of COLRv1 fonts as described in:
 *
 * https://learn.microsoft.com/en-us/typography/opentype/spec/colr#glyph-metrics-and-boundedness
 */

#[derive(Copy, Clone)]
pub(crate) struct hb_extents_t {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}

impl hb_extents_t {
    pub fn is_empty(&self) -> bool {
        self.x_min >= self.x_max || self.y_min >= self.y_max
    }
    pub fn is_void(&self) -> bool {
        self.x_min > self.x_max
    }

    pub fn union_(&mut self, o: &hb_extents_t) {
        self.x_min = o.x_min.min(o.x_min);
        self.y_min = o.y_min.min(o.y_min);
        self.x_max = o.x_max.max(o.x_max);
        self.y_max = o.y_max.max(o.y_max);
    }

    pub fn intersect(&mut self, o: &hb_extents_t) {
        self.x_min = o.x_min.max(o.x_min);
        self.y_min = o.y_min.max(o.y_min);
        self.x_max = o.x_max.min(o.x_max);
        self.y_max = o.y_max.min(o.y_max);
    }
}

impl From<RectF> for hb_extents_t {
    fn from(val: RectF) -> Self {
        Self {
            x_min: val.x_min,
            y_min: val.y_min,
            x_max: val.x_max,
            y_max: val.y_max,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum status_t {
    EMPTY,
    BOUNDED,
    UNBOUNDED,
}

#[derive(Clone, Copy)]
pub(crate) struct hb_bounds_t {
    status: status_t,
    extents: hb_extents_t,
}

impl hb_bounds_t {
    fn from_extents(extents: &hb_extents_t) -> Self {
        let status = if extents.is_empty() {
            status_t::EMPTY
        } else {
            status_t::BOUNDED
        };

        hb_bounds_t {
            extents: *extents,
            status,
        }
    }

    fn from_status(status: status_t) -> Self {
        hb_bounds_t {
            status,
            ..hb_bounds_t::default()
        }
    }

    fn union(&mut self, o: &hb_bounds_t) {
        if o.status == status_t::UNBOUNDED {
            self.status = status_t::UNBOUNDED;
        } else if o.status == status_t::BOUNDED {
            if self.status == status_t::EMPTY {
                *self = *o;
            } else if self.status == status_t::BOUNDED {
                self.extents.union_(&o.extents);
            }
        }
    }

    fn intersect(&mut self, o: &hb_bounds_t) {
        if o.status == status_t::EMPTY {
            self.status = status_t::EMPTY;
        } else if o.status == status_t::BOUNDED {
            if self.status == status_t::UNBOUNDED {
                *self = *o;
            } else if self.status == status_t::BOUNDED {
                self.extents.intersect(&o.extents);

                if self.extents.is_empty() {
                    self.status = status_t::EMPTY;
                }
            }
        }
    }
}

impl Default for hb_bounds_t {
    fn default() -> Self {
        Self::from_extents(&hb_extents_t {
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
        })
    }
}

pub(crate) struct hb_paint_extents_context_t<'a> {
    clips: vec::Vec<hb_bounds_t>,
    groups: vec::Vec<hb_bounds_t>,
    transforms: vec::Vec<Transform>,
    // Doesn't exist in harfbuzz. The reason we need it is that in harfbuzz, composite modes
    // are passed as part of `pop`, while ttf-parser passes it as part of `push`, so we need to
    // store it in the meanwhile.
    composite_modes: vec::Vec<CompositeMode>,
    face: &'a ttf_parser::Face<'a>,
    current_glyph: GlyphId,
}

impl<'a> hb_paint_extents_context_t<'a> {
    pub(crate) fn new(face: &'a ttf_parser::Face<'a>) -> Self {
        Self {
            clips: vec![hb_bounds_t::from_status(status_t::UNBOUNDED)],
            groups: vec![hb_bounds_t::from_status(status_t::EMPTY)],
            transforms: vec![Transform::default()],
            composite_modes: vec![CompositeMode::SourceOver],
            face,
            current_glyph: Default::default(),
        }
    }

    pub(crate) fn get_extents(&self) -> hb_extents_t {
        // harfbuzz doesn't have the unwrap_or_default part, but in a valid font
        // this should always be valid anyway.
        self.groups.last().copied().unwrap_or_default().extents
    }

    fn push_transform(&mut self, trans: &Transform) {
        let t = self
            .transforms
            .last()
            .copied()
            .unwrap_or(Transform::default());
        let new = Transform::combine(t, *trans);
        self.transforms.push(new);
    }

    fn pop_transform(&mut self) {
        self.transforms.pop();
    }

    fn push_clip(&mut self, mut extents: hb_extents_t) {
        if let Some(r) = self.transforms.last_mut() {
            r.transform_extents(&mut extents);
        }

        let b = hb_bounds_t::from_extents(&extents);
        self.clips.push(b);
    }

    fn pop_clip(&mut self) {
        self.clips.pop();
    }

    fn push_group(&mut self) {
        self.groups.push(hb_bounds_t::default());
    }

    fn pop_group(&mut self) {
        if let Some(mode) = self.composite_modes.pop() {
            if let Some(src_bounds) = self.groups.pop() {
                if let Some(backdrop_bounds) = self.groups.last_mut() {
                    match mode {
                        CompositeMode::Clear => backdrop_bounds.status = status_t::EMPTY,
                        CompositeMode::Source | CompositeMode::SourceOut => {
                            *backdrop_bounds = src_bounds
                        }
                        CompositeMode::Destination | CompositeMode::DestinationOut => {}
                        CompositeMode::SourceIn | CompositeMode::DestinationIn => {
                            backdrop_bounds.intersect(&src_bounds)
                        }
                        _ => backdrop_bounds.union(&src_bounds),
                    }
                }
            }
        }
    }

    fn paint(&mut self) {
        if let (Some(clip), Some(group)) = (self.clips.last(), self.groups.last_mut()) {
            group.union(clip);
        }
    }
}

impl ttf_parser::colr::Painter<'_> for hb_paint_extents_context_t<'_> {
    fn outline_glyph(&mut self, glyph_id: GlyphId) {
        self.current_glyph = glyph_id;
    }

    fn paint(&mut self, _: Paint<'_>) {
        self.paint();
    }

    fn push_clip(&mut self) {
        if let Some(glyph_bbox) = self.face.glyph_bounding_box(self.current_glyph) {
            self.push_clip(hb_extents_t {
                x_min: glyph_bbox.x_min as f32,
                y_min: glyph_bbox.y_min as f32,
                x_max: glyph_bbox.x_max as f32,
                y_max: glyph_bbox.y_max as f32,
            });
        }
    }

    fn push_clip_box(&mut self, clipbox: ClipBox) {
        self.push_clip(clipbox.into());
    }

    fn pop_clip(&mut self) {
        self.pop_clip();
    }

    fn push_layer(&mut self, mode: CompositeMode) {
        self.composite_modes.push(mode);
        self.push_group();
    }

    fn pop_layer(&mut self) {
        self.pop_group();
    }

    fn push_transform(&mut self, transform: Transform) {
        self.push_transform(&transform);
    }

    fn pop_transform(&mut self) {
        self.pop_transform();
    }
}

trait TransformExt {
    fn transform_distance(&self, dx: &mut f32, dy: &mut f32);
    fn transform_point(&self, x: &mut f32, y: &mut f32);
    fn transform_extents(&self, extents: &mut hb_extents_t);
}

impl TransformExt for Transform {
    fn transform_distance(&self, dx: &mut f32, dy: &mut f32) {
        let new_x = self.a * (*dx) + self.c * (*dy);
        let new_y = self.b * (*dx) + self.d * (*dy);
        *dx = new_x;
        *dy = new_y;
    }

    fn transform_point(&self, x: &mut f32, y: &mut f32) {
        self.transform_distance(x, y);
        *x += self.e;
        *y += self.f;
    }

    fn transform_extents(&self, extents: &mut hb_extents_t) {
        let mut quad_x = [0.0f32; 4];
        let mut quad_y = [0.0f32; 4];

        quad_x[0] = extents.x_min;
        quad_y[0] = extents.y_min;
        quad_x[1] = extents.x_min;
        quad_y[1] = extents.y_max;
        quad_x[2] = extents.x_max;
        quad_y[2] = extents.y_min;
        quad_x[3] = extents.x_max;
        quad_y[3] = extents.y_max;

        for i in 0..4 {
            self.transform_point(&mut quad_x[i], &mut quad_y[i])
        }

        extents.x_max = quad_x[0];
        extents.x_min = extents.x_max;
        extents.y_max = quad_y[0];
        extents.y_min = extents.y_max;

        for i in 1..4 {
            extents.x_min = extents.x_min.min(quad_x[i]);
            extents.y_min = extents.y_min.min(quad_y[i]);
            extents.x_max = extents.x_max.max(quad_x[i]);
            extents.y_max = extents.y_max.max(quad_y[i]);
        }
    }
}
