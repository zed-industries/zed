use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, GenericParam, Generics, Ident, Type};

use crate::{
    convert::util::{InputUser, UsedInput, WhitePointSource},
    meta::TypeItemAttributes,
    util,
};

pub(crate) struct ColorGroup {
    pub(crate) root_type: ColorInfo,
    pub(crate) colors: &'static [ColorType],
}

impl ColorGroup {
    pub(crate) fn check_availability(&self, name: &str) -> Result<(), ColorError> {
        if name == self.root_type.name {
            return Ok(());
        }

        for color in self.colors {
            if name != color.info.name {
                continue;
            }

            return Ok(());
        }

        Err(ColorError::UnknownColor)
    }

    pub(crate) fn color_names(&'static self) -> ColorNames {
        ColorNames {
            root_type: Some(&self.root_type),
            colors: self.colors.iter(),
        }
    }

    pub(crate) fn find_type_by_name(&self, name: &str) -> Option<&ColorType> {
        self.colors.iter().find(|color| color.info.name == name)
    }

    pub(crate) fn find_by_name(&self, name: &str) -> Option<&ColorInfo> {
        if self.root_type.name == name {
            Some(&self.root_type)
        } else {
            self.find_type_by_name(name).map(|ty| &ty.info)
        }
    }
}

pub(crate) struct ColorType {
    pub(crate) info: ColorInfo,
    pub(crate) infer_group: bool,
    pub(crate) preferred_source: &'static str,
}

type MetaTypeGeneratorFn = fn(
    self_color: &ColorInfo,
    meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    meta: &TypeItemAttributes,
) -> syn::Result<Type>;

pub(crate) struct ColorInfo {
    pub(crate) name: &'static str,
    pub(crate) module: Option<&'static str>,
    pub(crate) default_white_point: InternalExternal<Option<&'static [&'static str]>>,
    pub(crate) get_meta_type: Option<MetaTypeGeneratorFn>,
}

impl ColorInfo {
    pub(crate) fn get_path(&self, internal: bool) -> TokenStream {
        if let Some(module) = self.module {
            util::path([module, self.name], internal)
        } else {
            util::path([self.name], internal)
        }
    }

    pub(crate) fn get_type(
        &self,
        meta_type_source: MetaTypeSource,
        component: &Type,
        white_point: &Type,
        used_input: &mut UsedInput,
        user: InputUser,
        meta: &TypeItemAttributes,
    ) -> syn::Result<Type> {
        let meta_type: Option<Type> = self
            .get_meta_type
            .map(|get| get(self, meta_type_source, white_point, used_input, user, meta))
            .transpose()?;

        let color_path = self.get_path(meta.internal);

        if let Some(meta_type) = meta_type {
            Ok(parse_quote!(#color_path::<#meta_type, #component>))
        } else {
            Ok(parse_quote!(#color_path::<#component>))
        }
    }

    pub(crate) fn get_default_white_point(&self, internal: bool) -> (Type, WhitePointSource) {
        let path = if internal {
            self.default_white_point.internal
        } else {
            self.default_white_point.external
        };

        path.map(|path| {
            (
                util::path_type(path, internal),
                WhitePointSource::ConcreteType,
            )
        })
        .unwrap_or_else(|| (parse_quote!(_Wp), WhitePointSource::GeneratedGeneric))
    }
}

pub(crate) struct InternalExternal<T> {
    pub(crate) internal: T,
    pub(crate) external: T,
}

pub(crate) struct ColorNames {
    root_type: Option<&'static ColorInfo>,
    colors: std::slice::Iter<'static, ColorType>,
}

impl Iterator for ColorNames {
    type Item = &'static ColorInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(root_type) = self.root_type.take() {
            return Some(root_type);
        }

        self.colors.next().map(|color| &color.info)
    }
}

/// These are the disjoint networks of possible conversions. It's possible to
/// convert directly to and from each color within each group, while converting
/// between the groups requires additional runtime data.
pub(crate) static COLOR_GROUPS: &[&ColorGroup] = &[
    &XYZ_COLORS,
    &CAM16_JCH_COLORS,
    &CAM16_JMH_COLORS,
    &CAM16_JSH_COLORS,
    &CAM16_QCH_COLORS,
    &CAM16_QMH_COLORS,
    &CAM16_QSH_COLORS,
];

// The XYZ color group is where most colors should be. All of these have some
// connection to `Xyz`.

pub(crate) static XYZ_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo {
        name: "Xyz",
        module: None,
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_white_point),
    },
    colors: &[
        ColorType {
            info: ColorInfo {
                name: "Rgb",
                module: Some("rgb"),
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_rgb_standard),
            },
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: ColorInfo {
                name: "Luma",
                module: Some("luma"),
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_luma_standard),
            },
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: ColorInfo {
                name: "Hsl",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_rgb_standard),
            },
            infer_group: true,
            preferred_source: "Rgb",
        },
        ColorType {
            info: ColorInfo {
                name: "Hsluv",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_white_point),
            },
            infer_group: true,
            preferred_source: "Lchuv",
        },
        ColorType {
            info: ColorInfo {
                name: "Hsv",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_rgb_standard),
            },
            infer_group: true,
            preferred_source: "Rgb",
        },
        ColorType {
            info: ColorInfo {
                name: "Hwb",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_rgb_standard),
            },
            infer_group: true,
            preferred_source: "Hsv",
        },
        ColorType {
            info: ColorInfo {
                name: "Lab",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_white_point),
            },
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: ColorInfo {
                name: "Lch",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_white_point),
            },
            infer_group: true,
            preferred_source: "Lab",
        },
        ColorType {
            info: ColorInfo {
                name: "Lchuv",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_white_point),
            },
            infer_group: true,
            preferred_source: "Luv",
        },
        ColorType {
            info: ColorInfo {
                name: "Luv",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_white_point),
            },
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: ColorInfo {
                name: "Oklab",
                module: None,
                default_white_point: InternalExternal {
                    internal: Some(&["white_point", "D65"]),
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: None,
            },
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: ColorInfo {
                name: "Oklch",
                module: None,
                default_white_point: InternalExternal {
                    internal: Some(&["white_point", "D65"]),
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: None,
            },
            infer_group: true,
            preferred_source: "Oklab",
        },
        ColorType {
            info: ColorInfo {
                name: "Okhsl",
                module: None,
                default_white_point: InternalExternal {
                    internal: Some(&["white_point", "D65"]),
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: None,
            },
            infer_group: true,
            preferred_source: "Oklab",
        },
        ColorType {
            info: ColorInfo {
                name: "Okhsv",
                module: None,
                default_white_point: InternalExternal {
                    internal: Some(&["white_point", "D65"]),
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: None,
            },
            infer_group: true,
            preferred_source: "Oklab",
        },
        ColorType {
            info: ColorInfo {
                name: "Okhwb",
                module: None,
                default_white_point: InternalExternal {
                    internal: Some(&["white_point", "D65"]),
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: None,
            },
            infer_group: true,
            preferred_source: "Okhsv",
        },
        ColorType {
            info: ColorInfo {
                name: "Yxy",
                module: None,
                default_white_point: InternalExternal {
                    internal: None,
                    external: Some(&["white_point", "D65"]),
                },
                get_meta_type: Some(get_white_point),
            },
            infer_group: true,
            preferred_source: "Xyz",
        },
    ],
};

// The CAM16 groups are a bit special, since they require information about the
// viewing conditions to convert between each other.

static CAM16_JCH_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo {
        name: "Cam16Jch",
        module: Some("cam16"),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    },
    colors: &[ColorType {
        info: ColorInfo {
            name: "Cam16",
            module: Some("cam16"),
            default_white_point: InternalExternal {
                internal: None,
                external: None,
            },
            get_meta_type: None,
        },
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Jch",
    }],
};

static CAM16_JMH_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo {
        name: "Cam16Jmh",
        module: Some("cam16"),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    },
    colors: &[
        ColorType {
            info: ColorInfo {
                name: "Cam16",
                module: Some("cam16"),
                default_white_point: InternalExternal {
                    internal: None,
                    external: None,
                },
                get_meta_type: None,
            },
            infer_group: false, // For generating connections only from `Cam16`, but not to it
            preferred_source: "Cam16Jmh",
        },
        // CAM16 UCS
        ColorType {
            info: ColorInfo {
                name: "Cam16UcsJmh",
                module: Some("cam16"),
                default_white_point: InternalExternal {
                    internal: None,
                    external: None,
                },
                get_meta_type: None,
            },
            infer_group: true,
            preferred_source: "Cam16Jmh",
        },
        ColorType {
            info: ColorInfo {
                name: "Cam16UcsJab",
                module: Some("cam16"),
                default_white_point: InternalExternal {
                    internal: None,
                    external: None,
                },
                get_meta_type: None,
            },
            infer_group: true,
            preferred_source: "Cam16UcsJmh",
        },
    ],
};

static CAM16_JSH_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo {
        name: "Cam16Jsh",
        module: Some("cam16"),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    },
    colors: &[ColorType {
        info: ColorInfo {
            name: "Cam16",
            module: Some("cam16"),
            default_white_point: InternalExternal {
                internal: None,
                external: None,
            },
            get_meta_type: None,
        },
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Jsh",
    }],
};

static CAM16_QCH_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo {
        name: "Cam16Qch",
        module: Some("cam16"),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    },
    colors: &[ColorType {
        info: ColorInfo {
            name: "Cam16",
            module: Some("cam16"),
            default_white_point: InternalExternal {
                internal: None,
                external: None,
            },
            get_meta_type: None,
        },
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Qch",
    }],
};

static CAM16_QMH_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo {
        name: "Cam16Qmh",
        module: Some("cam16"),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    },
    colors: &[ColorType {
        info: ColorInfo {
            name: "Cam16",
            module: Some("cam16"),
            default_white_point: InternalExternal {
                internal: None,
                external: None,
            },
            get_meta_type: None,
        },
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Qmh",
    }],
};

static CAM16_QSH_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo {
        name: "Cam16Qsh",
        module: Some("cam16"),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    },
    colors: &[ColorType {
        info: ColorInfo {
            name: "Cam16",
            module: Some("cam16"),
            default_white_point: InternalExternal {
                internal: None,
                external: None,
            },
            get_meta_type: None,
        },
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Qsh",
    }],
};

pub(crate) enum ColorError {
    UnknownColor,
}

fn get_rgb_standard(
    self_color: &ColorInfo,
    meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    meta: &TypeItemAttributes,
) -> syn::Result<Type> {
    if let Some(rgb_standard) = &meta.rgb_standard {
        Ok(rgb_standard.clone())
    } else {
        match meta_type_source {
            MetaTypeSource::Generics(generics) => {
                used_input.white_point.set_used(user);

                let rgb_standard_path = util::path(["rgb", "RgbStandard"], meta.internal);
                let rgb_space_path = util::path(["rgb", "RgbSpace"], meta.internal);

                generics.params.push(GenericParam::Type(
                    Ident::new("_S", Span::call_site()).into(),
                ));
                let where_clause = generics.make_where_clause();

                where_clause
                    .predicates
                    .push(parse_quote!(_S: #rgb_standard_path));
                where_clause
                    .predicates
                    .push(parse_quote!(_S::Space: #rgb_space_path<WhitePoint = #white_point>));

                Ok(parse_quote!(_S))
            }
            MetaTypeSource::OtherColor(other_color) => {
                match other_color.name {
                    "Rgb" | "Hsl" | "Hsv" | "Hwb" => Ok(parse_quote!(_S)),
                    _ => Err(syn::parse::Error::new(
                        Span::call_site(),
                        format!(
                            "could not determine which RGB standard to use when converting to and from `{}` via `{}`",
                            other_color.name,
                            self_color.name
                        ),
                    )),
                }
            }
        }
    }
}

fn get_luma_standard(
    _self_color: &ColorInfo,
    meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    meta: &TypeItemAttributes,
) -> syn::Result<Type> {
    if let Some(luma_standard) = meta.luma_standard.as_ref() {
        return Ok(luma_standard.clone());
    }

    used_input.white_point.set_used(user);

    match meta_type_source {
        MetaTypeSource::Generics(generics) => {
            let luma_standard_path = util::path(["luma", "LumaStandard"], meta.internal);

            generics.params.push(GenericParam::Type(
                Ident::new("_S", Span::call_site()).into(),
            ));

            generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(_S: #luma_standard_path<WhitePoint = #white_point>));

            Ok(parse_quote!(_S))
        }
        MetaTypeSource::OtherColor(_) => {
            let linear_path = util::path(["encoding", "Linear"], meta.internal);

            Ok(parse_quote!(#linear_path<#white_point>))
        }
    }
}

fn get_white_point(
    _self_color: &ColorInfo,
    _meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    _meta: &TypeItemAttributes,
) -> syn::Result<Type> {
    used_input.white_point.set_used(user);
    Ok(white_point.clone())
}

pub(crate) enum MetaTypeSource<'a> {
    OtherColor(&'a ColorInfo),
    Generics(&'a mut Generics),
}
