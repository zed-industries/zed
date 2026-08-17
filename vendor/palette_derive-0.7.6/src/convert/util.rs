use std::collections::HashMap;

use proc_macro2::Span;
use syn::{parse_quote, Generics, Result, Type};

use crate::{
    color_types::{ColorInfo, MetaTypeSource},
    meta::TypeItemAttributes,
    util,
};

pub fn white_point_type(
    white_point: Option<&Type>,
    rgb_standard: Option<&Type>,
    luma_standard: Option<&Type>,
    internal: bool,
) -> Option<(Type, WhitePointSource)> {
    white_point
        .map(|white_point| (white_point.clone(), WhitePointSource::WhitePoint))
        .or_else(|| {
            rgb_standard.map(|rgb_standard| {
                let rgb_standard_path = util::path(["rgb", "RgbStandard"], internal);
                let rgb_space_path = util::path(["rgb", "RgbSpace"], internal);
                (
                    parse_quote!(<<#rgb_standard as #rgb_standard_path>::Space as #rgb_space_path>::WhitePoint),
                    WhitePointSource::RgbStandard,
                )
            })
        })
        .or_else(|| {
            luma_standard.map(|luma_standard| {
                let luma_standard_path = util::path(["luma", "LumaStandard"], internal);
                (
                    parse_quote!(<#luma_standard as #luma_standard_path>::WhitePoint),
                    WhitePointSource::LumaStandard,
                )
            })
        })
}

pub fn component_type(component: Option<Type>) -> Type {
    component.unwrap_or_else(|| parse_quote!(f32))
}

pub(crate) fn get_convert_color_type(
    color: &ColorInfo,
    white_point: &Type,
    component: &Type,
    meta: &TypeItemAttributes,
    generics: &mut Generics,
) -> syn::Result<(Type, UsedInput)> {
    let mut used_input = UsedInput::default();
    let color_type = color.get_type(
        MetaTypeSource::Generics(generics),
        component,
        white_point,
        &mut used_input,
        InputUser::Target,
        meta,
    )?;

    Ok((color_type, used_input))
}

pub(crate) fn find_nearest_color<'a>(
    color: &'a ColorInfo,
    meta: &TypeItemAttributes,
) -> Result<&'a ColorInfo> {
    let mut stack = vec![(color, 0)];
    let mut found = None;
    let mut visited = HashMap::new();

    // Make sure there is at least one valid color in the skip list
    assert!(!meta.skip_derives.is_empty());

    while let Some((color, distance)) = stack.pop() {
        if meta.skip_derives.contains(color.name) {
            if let Some((_, found_distance)) = found {
                if distance < found_distance {
                    found = Some((color, distance));
                    continue;
                }
            } else {
                found = Some((color, distance));
                continue;
            }
        }

        if let Some(&previous_distance) = visited.get(color.name) {
            if previous_distance <= distance {
                continue;
            }
        }

        visited.insert(color.name, distance);

        // Start by pushing the plan B routes...
        for group in &meta.color_groups {
            for candidate in group.colors {
                if color.name == candidate.preferred_source {
                    stack.push((&candidate.info, distance + 1));
                }
            }
        }

        // ...then push the preferred routes. They will be popped first.
        for group in &meta.color_groups {
            for candidate in group.colors {
                if color.name == candidate.info.name {
                    let preferred = group
                        .find_by_name(candidate.preferred_source)
                        .expect("preferred sources have to exist in the group");
                    stack.push((preferred, distance + 1));
                }
            }
        }
    }

    if let Some((color, _)) = found {
        Ok(color)
    } else {
        Err(::syn::parse::Error::new(
            Span::call_site(),
            format!(
                "none of the skipped colors can be used for converting from {}",
                color.name
            ),
        ))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WhitePointSource {
    WhitePoint,
    RgbStandard,
    LumaStandard,
    ConcreteType,
    GeneratedGeneric,
}

#[derive(Debug, Default)]
pub struct UsedInput {
    pub white_point: InputUsage,
}

#[derive(Debug, Default)]
pub struct InputUsage {
    used_by_target: bool,
    used_by_nearest: bool,
}

impl InputUsage {
    pub(crate) fn set_used(&mut self, user: InputUser) {
        match user {
            InputUser::Target => self.used_by_target = true,
            InputUser::Nearest => self.used_by_nearest = true,
        }
    }

    pub(crate) fn is_used(&self) -> bool {
        self.used_by_target || self.used_by_nearest
    }

    pub(crate) fn is_unconstrained(&self) -> bool {
        !self.used_by_target && self.used_by_nearest
    }
}

#[derive(Clone, Copy)]
pub enum InputUser {
    Target,
    Nearest,
}
