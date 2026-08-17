//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::path`.

use std::path::*;

use crate::{
    arbitrary::{SMapped, StrategyFor},
    path::PathParams,
    prelude::{any, any_with, Arbitrary, Strategy},
    std_facade::{string::ToString, Arc, Box, Rc, String, Vec},
    strategy::{statics::static_map, MapInto},
};

arbitrary!(StripPrefixError; Path::new("").strip_prefix("a").unwrap_err());

/// A private type (not actually pub) representing the output of [`PathParams`] that can't be
/// referred to by API users.
///
/// The goal of this type is to encapsulate the output of `PathParams`. If this layer weren't
/// present, the type of `<PathBuf as Arbitrary>::Strategy` would be `SMapped<(bool, Vec<String>),
/// Self>`. This is a problem because it exposes the internal representation of `PathParams` as an
/// API. For example, if an additional parameter of randomness (e.g. another bool) were added, the
/// type of `Strategy` would change.
///
/// With `PathParamsOutput`, the type of `Strategy` is `SMapped<PathParamsOutput, Self>`, which is a
/// type that can't be named directly---only via `<PathBuf as Arbitrary>::Strategy`. The internal
/// representation of `PathParams` can be changed without affecting the API.
#[derive(Debug)]
pub struct PathParamsOutput {
    is_absolute: bool,
    components: Vec<String>,
}

impl Arbitrary for PathParamsOutput {
    type Parameters = PathParams;
    type Strategy = SMapped<(bool, Vec<String>), Self>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        static_map(
            (
                any::<bool>(),
                any_with::<Vec<String>>((
                    args.components(),
                    args.component_regex(),
                )),
            ),
            |(is_absolute, components)| Self {
                is_absolute,
                components,
            },
        )
    }
}

/// This implementation accepts as its argument a [`PathParams`] struct. It generates either a
/// relative or an absolute path with equal probability.
///
/// Currently, this implementation does not generate:
///
/// * Paths that are not valid UTF-8 (this is unlikely to change)
/// * Paths with a [`PrefixComponent`](std::path::PrefixComponent) on Windows, e.g. `C:\` (this may
///   change in the future)
impl Arbitrary for PathBuf {
    type Parameters = PathParams;
    type Strategy = SMapped<PathParamsOutput, Self>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        static_map(
            any_with::<PathParamsOutput>(args),
            |PathParamsOutput {
                 is_absolute,
                 components,
             }| {
                let mut out = PathBuf::new();
                if is_absolute {
                    out.push(&MAIN_SEPARATOR.to_string());
                }

                for component in components {
                    // If a component has an embedded / (or \ on Windows), remove it from the
                    // string.
                    let component = component
                        .chars()
                        .filter(|&c| !std::path::is_separator(c))
                        .collect::<String>();
                    out.push(&component);
                }

                out
            },
        )
    }
}

macro_rules! dst_wrapped {
    ($($w: ident),*) => {
        $(
            /// This implementation is identical to [the `Arbitrary` implementation for
            /// `PathBuf`](trait.Arbitrary.html#impl-Arbitrary-for-PathBuf).
            impl Arbitrary for $w<Path> {
                type Parameters = PathParams;
                type Strategy = MapInto<StrategyFor<PathBuf>, Self>;

                fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                    any_with::<PathBuf>(args).prop_map_into()
                }
            }
        )*
    }
}

dst_wrapped!(Box, Rc, Arc);

#[cfg(test)]
mod test {
    no_panic_test!(
        strip_prefix_error => StripPrefixError,
        path_buf => PathBuf,
        box_path => Box<Path>,
        rc_path => Rc<Path>,
        arc_path => Arc<Path>
    );
}
