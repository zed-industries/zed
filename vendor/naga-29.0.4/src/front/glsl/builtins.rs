use alloc::{vec, vec::Vec};

use super::{
    ast::{
        BuiltinVariations, FunctionDeclaration, FunctionKind, Overload, ParameterInfo,
        ParameterQualifier,
    },
    context::Context,
    Error, ErrorKind, Frontend, Result,
};
use crate::{
    BinaryOperator, DerivativeAxis as Axis, DerivativeControl as Ctrl, Expression, Handle,
    ImageClass, ImageDimension as Dim, ImageQuery, MathFunction, Module, RelationalFunction,
    SampleLevel, Scalar, ScalarKind as Sk, Span, Type, TypeInner, UnaryOperator, VectorSize,
};

impl crate::ScalarKind {
    const fn dummy_storage_format(&self) -> crate::StorageFormat {
        match *self {
            Sk::Sint => crate::StorageFormat::R16Sint,
            Sk::Uint => crate::StorageFormat::R16Uint,
            _ => crate::StorageFormat::R16Float,
        }
    }
}

impl Module {
    /// Helper function, to create a function prototype for a builtin
    fn add_builtin(&mut self, args: Vec<TypeInner>, builtin: MacroCall) -> Overload {
        let mut parameters = Vec::with_capacity(args.len());
        let mut parameters_info = Vec::with_capacity(args.len());

        for arg in args {
            parameters.push(self.types.insert(
                Type {
                    name: None,
                    inner: arg,
                },
                Span::default(),
            ));
            parameters_info.push(ParameterInfo {
                qualifier: ParameterQualifier::In,
                depth: false,
            });
        }

        Overload {
            parameters,
            parameters_info,
            kind: FunctionKind::Macro(builtin),
            defined: false,
            internal: true,
            void: false,
        }
    }
}

const fn make_coords_arg(number_of_components: usize, kind: Sk) -> TypeInner {
    let scalar = Scalar { kind, width: 4 };

    match number_of_components {
        1 => TypeInner::Scalar(scalar),
        _ => TypeInner::Vector {
            size: match number_of_components {
                2 => VectorSize::Bi,
                3 => VectorSize::Tri,
                _ => VectorSize::Quad,
            },
            scalar,
        },
    }
}

/// Inject builtins into the declaration
///
/// This is done to not add a large startup cost and not increase memory
/// usage if it isn't needed.
pub fn inject_builtin(
    declaration: &mut FunctionDeclaration,
    module: &mut Module,
    name: &str,
    mut variations: BuiltinVariations,
) {
    log::trace!(
        "{} variations: {:?} {:?}",
        name,
        variations,
        declaration.variations
    );
    // Don't regeneate variations
    variations.remove(declaration.variations);
    declaration.variations |= variations;

    if variations.contains(BuiltinVariations::STANDARD) {
        inject_standard_builtins(declaration, module, name)
    }

    if variations.contains(BuiltinVariations::DOUBLE) {
        inject_double_builtin(declaration, module, name)
    }

    match name {
        "texture"
        | "textureGrad"
        | "textureGradOffset"
        | "textureLod"
        | "textureLodOffset"
        | "textureOffset"
        | "textureProj"
        | "textureProjGrad"
        | "textureProjGradOffset"
        | "textureProjLod"
        | "textureProjLodOffset"
        | "textureProjOffset" => {
            let f = |kind, dim, arrayed, multi, shadow| {
                for bits in 0..=0b11 {
                    let variant = bits & 0b1 != 0;
                    let bias = bits & 0b10 != 0;

                    let (proj, offset, level_type) = match name {
                        // texture(gsampler, gvec P, [float bias]);
                        "texture" => (false, false, TextureLevelType::None),
                        // textureGrad(gsampler, gvec P, gvec dPdx, gvec dPdy);
                        "textureGrad" => (false, false, TextureLevelType::Grad),
                        // textureGradOffset(gsampler, gvec P, gvec dPdx, gvec dPdy, ivec offset);
                        "textureGradOffset" => (false, true, TextureLevelType::Grad),
                        // textureLod(gsampler, gvec P, float lod);
                        "textureLod" => (false, false, TextureLevelType::Lod),
                        // textureLodOffset(gsampler, gvec P, float lod, ivec offset);
                        "textureLodOffset" => (false, true, TextureLevelType::Lod),
                        // textureOffset(gsampler, gvec+1 P, ivec offset, [float bias]);
                        "textureOffset" => (false, true, TextureLevelType::None),
                        // textureProj(gsampler, gvec+1 P, [float bias]);
                        "textureProj" => (true, false, TextureLevelType::None),
                        // textureProjGrad(gsampler, gvec+1 P, gvec dPdx, gvec dPdy);
                        "textureProjGrad" => (true, false, TextureLevelType::Grad),
                        // textureProjGradOffset(gsampler, gvec+1 P, gvec dPdx, gvec dPdy, ivec offset);
                        "textureProjGradOffset" => (true, true, TextureLevelType::Grad),
                        // textureProjLod(gsampler, gvec+1 P, float lod);
                        "textureProjLod" => (true, false, TextureLevelType::Lod),
                        // textureProjLodOffset(gsampler, gvec+1 P, gvec dPdx, gvec dPdy, ivec offset);
                        "textureProjLodOffset" => (true, true, TextureLevelType::Lod),
                        // textureProjOffset(gsampler, gvec+1 P, ivec offset, [float bias]);
                        "textureProjOffset" => (true, true, TextureLevelType::None),
                        _ => unreachable!(),
                    };

                    let builtin = MacroCall::Texture {
                        proj,
                        offset,
                        shadow,
                        level_type,
                    };

                    // Parse out the variant settings.
                    let grad = level_type == TextureLevelType::Grad;
                    let lod = level_type == TextureLevelType::Lod;

                    let supports_variant = proj && !shadow;
                    if variant && !supports_variant {
                        continue;
                    }

                    if bias && !matches!(level_type, TextureLevelType::None) {
                        continue;
                    }

                    // Proj doesn't work with arrayed or Cube
                    if proj && (arrayed || dim == Dim::Cube) {
                        continue;
                    }

                    // texture operations with offset are not supported for cube maps
                    if dim == Dim::Cube && offset {
                        continue;
                    }

                    // sampler2DArrayShadow can't be used in textureLod or in texture with bias
                    if (lod || bias) && arrayed && shadow && dim == Dim::D2 {
                        continue;
                    }

                    // TODO: glsl supports using bias with depth samplers but naga doesn't
                    if bias && shadow {
                        continue;
                    }

                    let class = match shadow {
                        true => ImageClass::Depth { multi },
                        false => ImageClass::Sampled { kind, multi },
                    };

                    let image = TypeInner::Image {
                        dim,
                        arrayed,
                        class,
                    };

                    let num_coords_from_dim = image_dims_to_coords_size(dim).min(3);
                    let mut num_coords = num_coords_from_dim;

                    if shadow && proj {
                        num_coords = 4;
                    } else if dim == Dim::D1 && shadow {
                        num_coords = 3;
                    } else if shadow {
                        num_coords += 1;
                    } else if proj {
                        if variant && num_coords == 4 {
                            // Normal form already has 4 components, no need to have a variant form.
                            continue;
                        } else if variant {
                            num_coords = 4;
                        } else {
                            num_coords += 1;
                        }
                    }

                    if !(dim == Dim::D1 && shadow) {
                        num_coords += arrayed as usize;
                    }

                    // Special case: texture(gsamplerCubeArrayShadow) kicks the shadow compare ref to a separate argument,
                    // since it would otherwise take five arguments. It also can't take a bias, nor can it be proj/grad/lod/offset
                    // (presumably because nobody asked for it, and implementation complexity?)
                    if num_coords >= 5 {
                        if lod || grad || offset || proj || bias {
                            continue;
                        }
                        debug_assert!(dim == Dim::Cube && shadow && arrayed);
                    }
                    debug_assert!(num_coords <= 5);

                    let vector = make_coords_arg(num_coords, Sk::Float);
                    let mut args = vec![image, vector];

                    if num_coords == 5 {
                        args.push(TypeInner::Scalar(Scalar::F32));
                    }

                    match level_type {
                        TextureLevelType::Lod => {
                            args.push(TypeInner::Scalar(Scalar::F32));
                        }
                        TextureLevelType::Grad => {
                            args.push(make_coords_arg(num_coords_from_dim, Sk::Float));
                            args.push(make_coords_arg(num_coords_from_dim, Sk::Float));
                        }
                        TextureLevelType::None => {}
                    };

                    if offset {
                        args.push(make_coords_arg(num_coords_from_dim, Sk::Sint));
                    }

                    if bias {
                        args.push(TypeInner::Scalar(Scalar::F32));
                    }

                    declaration
                        .overloads
                        .push(module.add_builtin(args, builtin));
                }
            };

            texture_args_generator(TextureArgsOptions::SHADOW | variations.into(), f)
        }
        "textureSize" => {
            let f = |kind, dim, arrayed, multi, shadow| {
                let class = match shadow {
                    true => ImageClass::Depth { multi },
                    false => ImageClass::Sampled { kind, multi },
                };

                let image = TypeInner::Image {
                    dim,
                    arrayed,
                    class,
                };

                let mut args = vec![image];

                if !multi {
                    args.push(TypeInner::Scalar(Scalar::I32))
                }

                declaration
                    .overloads
                    .push(module.add_builtin(args, MacroCall::TextureSize { arrayed }))
            };

            texture_args_generator(
                TextureArgsOptions::SHADOW | TextureArgsOptions::MULTI | variations.into(),
                f,
            )
        }
        "textureQueryLevels" => {
            let f = |kind, dim, arrayed, multi, shadow| {
                let class = match shadow {
                    true => ImageClass::Depth { multi },
                    false => ImageClass::Sampled { kind, multi },
                };

                let image = TypeInner::Image {
                    dim,
                    arrayed,
                    class,
                };

                declaration
                    .overloads
                    .push(module.add_builtin(vec![image], MacroCall::TextureQueryLevels))
            };

            texture_args_generator(TextureArgsOptions::SHADOW | variations.into(), f)
        }
        "texelFetch" | "texelFetchOffset" => {
            let offset = "texelFetchOffset" == name;
            let f = |kind, dim, arrayed, multi, _shadow| {
                // Cube images aren't supported
                if let Dim::Cube = dim {
                    return;
                }

                let image = TypeInner::Image {
                    dim,
                    arrayed,
                    class: ImageClass::Sampled { kind, multi },
                };

                let dim_value = image_dims_to_coords_size(dim);
                let coordinates = make_coords_arg(dim_value + arrayed as usize, Sk::Sint);

                let mut args = vec![image, coordinates, TypeInner::Scalar(Scalar::I32)];

                if offset {
                    args.push(make_coords_arg(dim_value, Sk::Sint));
                }

                declaration
                    .overloads
                    .push(module.add_builtin(args, MacroCall::ImageLoad { multi }))
            };

            // Don't generate shadow images since they aren't supported
            texture_args_generator(TextureArgsOptions::MULTI | variations.into(), f)
        }
        "imageSize" => {
            let f = |kind: Sk, dim, arrayed, _, _| {
                // Naga doesn't support cube images and it's usefulness
                // is questionable, so they won't be supported for now
                if dim == Dim::Cube {
                    return;
                }

                let image = TypeInner::Image {
                    dim,
                    arrayed,
                    class: ImageClass::Storage {
                        format: kind.dummy_storage_format(),
                        access: crate::StorageAccess::empty(),
                    },
                };

                declaration
                    .overloads
                    .push(module.add_builtin(vec![image], MacroCall::TextureSize { arrayed }))
            };

            texture_args_generator(variations.into(), f)
        }
        "imageLoad" => {
            let f = |kind: Sk, dim, arrayed, _, _| {
                // Naga doesn't support cube images and it's usefulness
                // is questionable, so they won't be supported for now
                if dim == Dim::Cube {
                    return;
                }

                let image = TypeInner::Image {
                    dim,
                    arrayed,
                    class: ImageClass::Storage {
                        format: kind.dummy_storage_format(),
                        access: crate::StorageAccess::LOAD,
                    },
                };

                let dim_value = image_dims_to_coords_size(dim);
                let mut coord_size = dim_value + arrayed as usize;
                // > Every OpenGL API call that operates on cubemap array
                // > textures takes layer-faces, not array layers
                //
                // So this means that imageCubeArray only takes a three component
                // vector coordinate and the third component is a layer index.
                if Dim::Cube == dim && arrayed {
                    coord_size = 3
                }
                let coordinates = make_coords_arg(coord_size, Sk::Sint);

                let args = vec![image, coordinates];

                declaration
                    .overloads
                    .push(module.add_builtin(args, MacroCall::ImageLoad { multi: false }))
            };

            // Don't generate shadow nor multisampled images since they aren't supported
            texture_args_generator(variations.into(), f)
        }
        "imageStore" => {
            let f = |kind: Sk, dim, arrayed, _, _| {
                // Naga doesn't support cube images and it's usefulness
                // is questionable, so they won't be supported for now
                if dim == Dim::Cube {
                    return;
                }

                let image = TypeInner::Image {
                    dim,
                    arrayed,
                    class: ImageClass::Storage {
                        format: kind.dummy_storage_format(),
                        access: crate::StorageAccess::STORE,
                    },
                };

                let dim_value = image_dims_to_coords_size(dim);
                let mut coord_size = dim_value + arrayed as usize;
                // > Every OpenGL API call that operates on cubemap array
                // > textures takes layer-faces, not array layers
                //
                // So this means that imageCubeArray only takes a three component
                // vector coordinate and the third component is a layer index.
                if Dim::Cube == dim && arrayed {
                    coord_size = 3
                }
                let coordinates = make_coords_arg(coord_size, Sk::Sint);

                let args = vec![
                    image,
                    coordinates,
                    TypeInner::Vector {
                        size: VectorSize::Quad,
                        scalar: Scalar { kind, width: 4 },
                    },
                ];

                let mut overload = module.add_builtin(args, MacroCall::ImageStore);
                overload.void = true;
                declaration.overloads.push(overload)
            };

            // Don't generate shadow nor multisampled images since they aren't supported
            texture_args_generator(variations.into(), f)
        }
        _ => {}
    }
}

/// Injects the builtins into declaration that don't need any special variations
fn inject_standard_builtins(
    declaration: &mut FunctionDeclaration,
    module: &mut Module,
    name: &str,
) {
    // Some samplers (sampler1D, etc...) can be float, int, or uint
    let anykind_sampler = if name.starts_with("sampler") {
        Some((name, Sk::Float))
    } else if name.starts_with("usampler") {
        Some((&name[1..], Sk::Uint))
    } else if name.starts_with("isampler") {
        Some((&name[1..], Sk::Sint))
    } else {
        None
    };
    if let Some((sampler, kind)) = anykind_sampler {
        match sampler {
            "sampler1D" | "sampler1DArray" | "sampler2D" | "sampler2DArray" | "sampler2DMS"
            | "sampler2DMSArray" | "sampler3D" | "samplerCube" | "samplerCubeArray" => {
                declaration.overloads.push(module.add_builtin(
                    vec![
                        TypeInner::Image {
                            dim: match sampler {
                                "sampler1D" | "sampler1DArray" => Dim::D1,
                                "sampler2D" | "sampler2DArray" | "sampler2DMS"
                                | "sampler2DMSArray" => Dim::D2,
                                "sampler3D" => Dim::D3,
                                _ => Dim::Cube,
                            },
                            arrayed: matches!(
                                sampler,
                                "sampler1DArray"
                                    | "sampler2DArray"
                                    | "sampler2DMSArray"
                                    | "samplerCubeArray"
                            ),
                            class: ImageClass::Sampled {
                                kind,
                                multi: matches!(sampler, "sampler2DMS" | "sampler2DMSArray"),
                            },
                        },
                        TypeInner::Sampler { comparison: false },
                    ],
                    MacroCall::Sampler,
                ));
                return;
            }
            _ => (),
        }
    }

    match name {
        // Shadow sampler can only be of kind `Sk::Float`
        "sampler1DShadow"
        | "sampler1DArrayShadow"
        | "sampler2DShadow"
        | "sampler2DArrayShadow"
        | "samplerCubeShadow"
        | "samplerCubeArrayShadow" => {
            let dim = match name {
                "sampler1DShadow" | "sampler1DArrayShadow" => Dim::D1,
                "sampler2DShadow" | "sampler2DArrayShadow" => Dim::D2,
                _ => Dim::Cube,
            };
            let arrayed = matches!(
                name,
                "sampler1DArrayShadow" | "sampler2DArrayShadow" | "samplerCubeArrayShadow"
            );

            for i in 0..2 {
                let ty = TypeInner::Image {
                    dim,
                    arrayed,
                    class: match i {
                        0 => ImageClass::Sampled {
                            kind: Sk::Float,
                            multi: false,
                        },
                        _ => ImageClass::Depth { multi: false },
                    },
                };

                declaration.overloads.push(module.add_builtin(
                    vec![ty, TypeInner::Sampler { comparison: true }],
                    MacroCall::SamplerShadow,
                ))
            }
        }
        "sin" | "exp" | "exp2" | "sinh" | "cos" | "cosh" | "tan" | "tanh" | "acos" | "asin"
        | "log" | "log2" | "radians" | "degrees" | "asinh" | "acosh" | "atanh"
        | "floatBitsToInt" | "floatBitsToUint" | "dFdx" | "dFdxFine" | "dFdxCoarse" | "dFdy"
        | "dFdyFine" | "dFdyCoarse" | "fwidth" | "fwidthFine" | "fwidthCoarse" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b100 {
                let size = match bits {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };
                let scalar = Scalar::F32;

                declaration.overloads.push(module.add_builtin(
                    vec![match size {
                        Some(size) => TypeInner::Vector { size, scalar },
                        None => TypeInner::Scalar(scalar),
                    }],
                    match name {
                        "sin" => MacroCall::MathFunction(MathFunction::Sin),
                        "exp" => MacroCall::MathFunction(MathFunction::Exp),
                        "exp2" => MacroCall::MathFunction(MathFunction::Exp2),
                        "sinh" => MacroCall::MathFunction(MathFunction::Sinh),
                        "cos" => MacroCall::MathFunction(MathFunction::Cos),
                        "cosh" => MacroCall::MathFunction(MathFunction::Cosh),
                        "tan" => MacroCall::MathFunction(MathFunction::Tan),
                        "tanh" => MacroCall::MathFunction(MathFunction::Tanh),
                        "acos" => MacroCall::MathFunction(MathFunction::Acos),
                        "asin" => MacroCall::MathFunction(MathFunction::Asin),
                        "log" => MacroCall::MathFunction(MathFunction::Log),
                        "log2" => MacroCall::MathFunction(MathFunction::Log2),
                        "asinh" => MacroCall::MathFunction(MathFunction::Asinh),
                        "acosh" => MacroCall::MathFunction(MathFunction::Acosh),
                        "atanh" => MacroCall::MathFunction(MathFunction::Atanh),
                        "radians" => MacroCall::MathFunction(MathFunction::Radians),
                        "degrees" => MacroCall::MathFunction(MathFunction::Degrees),
                        "floatBitsToInt" => MacroCall::BitCast(Sk::Sint),
                        "floatBitsToUint" => MacroCall::BitCast(Sk::Uint),
                        "dFdxCoarse" => MacroCall::Derivate(Axis::X, Ctrl::Coarse),
                        "dFdyCoarse" => MacroCall::Derivate(Axis::Y, Ctrl::Coarse),
                        "fwidthCoarse" => MacroCall::Derivate(Axis::Width, Ctrl::Coarse),
                        "dFdxFine" => MacroCall::Derivate(Axis::X, Ctrl::Fine),
                        "dFdyFine" => MacroCall::Derivate(Axis::Y, Ctrl::Fine),
                        "fwidthFine" => MacroCall::Derivate(Axis::Width, Ctrl::Fine),
                        "dFdx" => MacroCall::Derivate(Axis::X, Ctrl::None),
                        "dFdy" => MacroCall::Derivate(Axis::Y, Ctrl::None),
                        "fwidth" => MacroCall::Derivate(Axis::Width, Ctrl::None),
                        _ => unreachable!(),
                    },
                ))
            }
        }
        "intBitsToFloat" | "uintBitsToFloat" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b100 {
                let size = match bits {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };
                let scalar = match name {
                    "intBitsToFloat" => Scalar::I32,
                    _ => Scalar::U32,
                };

                declaration.overloads.push(module.add_builtin(
                    vec![match size {
                        Some(size) => TypeInner::Vector { size, scalar },
                        None => TypeInner::Scalar(scalar),
                    }],
                    MacroCall::BitCast(Sk::Float),
                ))
            }
        }
        "pow" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b100 {
                let size = match bits {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };
                let scalar = Scalar::F32;
                let ty = || match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };

                declaration.overloads.push(
                    module
                        .add_builtin(vec![ty(), ty()], MacroCall::MathFunction(MathFunction::Pow)),
                )
            }
        }
        "abs" | "sign" => {
            // bits layout
            // bit 0 through 1 - dims
            // bit 2 - float/sint
            for bits in 0..0b1000 {
                let size = match bits & 0b11 {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };
                let scalar = match bits >> 2 {
                    0b0 => Scalar::F32,
                    _ => Scalar::I32,
                };

                let args = vec![match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                }];

                declaration.overloads.push(module.add_builtin(
                    args,
                    MacroCall::MathFunction(match name {
                        "abs" => MathFunction::Abs,
                        "sign" => MathFunction::Sign,
                        _ => unreachable!(),
                    }),
                ))
            }
        }
        "bitCount" | "bitfieldReverse" | "bitfieldExtract" | "bitfieldInsert" | "findLSB"
        | "findMSB" => {
            let fun = match name {
                "bitCount" => MathFunction::CountOneBits,
                "bitfieldReverse" => MathFunction::ReverseBits,
                "bitfieldExtract" => MathFunction::ExtractBits,
                "bitfieldInsert" => MathFunction::InsertBits,
                "findLSB" => MathFunction::FirstTrailingBit,
                "findMSB" => MathFunction::FirstLeadingBit,
                _ => unreachable!(),
            };

            let mc = match fun {
                MathFunction::ExtractBits => MacroCall::BitfieldExtract,
                MathFunction::InsertBits => MacroCall::BitfieldInsert,
                _ => MacroCall::MathFunction(fun),
            };

            // bits layout
            // bit 0 - int/uint
            // bit 1 through 2 - dims
            for bits in 0..0b1000 {
                let scalar = match bits & 0b1 {
                    0b0 => Scalar::I32,
                    _ => Scalar::U32,
                };
                let size = match bits >> 1 {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };

                let ty = || match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };

                let mut args = vec![ty()];

                match fun {
                    MathFunction::ExtractBits => {
                        args.push(TypeInner::Scalar(Scalar::I32));
                        args.push(TypeInner::Scalar(Scalar::I32));
                    }
                    MathFunction::InsertBits => {
                        args.push(ty());
                        args.push(TypeInner::Scalar(Scalar::I32));
                        args.push(TypeInner::Scalar(Scalar::I32));
                    }
                    _ => {}
                }

                // we need to cast the return type of findLsb / findMsb
                let mc = if scalar.kind == Sk::Uint {
                    match mc {
                        MacroCall::MathFunction(MathFunction::FirstTrailingBit) => {
                            MacroCall::FindLsbUint
                        }
                        MacroCall::MathFunction(MathFunction::FirstLeadingBit) => {
                            MacroCall::FindMsbUint
                        }
                        mc => mc,
                    }
                } else {
                    mc
                };

                declaration.overloads.push(module.add_builtin(args, mc))
            }
        }
        "packSnorm4x8" | "packUnorm4x8" | "packSnorm2x16" | "packUnorm2x16" | "packHalf2x16" => {
            let fun = match name {
                "packSnorm4x8" => MathFunction::Pack4x8snorm,
                "packUnorm4x8" => MathFunction::Pack4x8unorm,
                "packSnorm2x16" => MathFunction::Pack2x16unorm,
                "packUnorm2x16" => MathFunction::Pack2x16snorm,
                "packHalf2x16" => MathFunction::Pack2x16float,
                _ => unreachable!(),
            };

            let ty = match fun {
                MathFunction::Pack4x8snorm | MathFunction::Pack4x8unorm => TypeInner::Vector {
                    size: VectorSize::Quad,
                    scalar: Scalar::F32,
                },
                MathFunction::Pack2x16unorm
                | MathFunction::Pack2x16snorm
                | MathFunction::Pack2x16float => TypeInner::Vector {
                    size: VectorSize::Bi,
                    scalar: Scalar::F32,
                },
                _ => unreachable!(),
            };

            let args = vec![ty];

            declaration
                .overloads
                .push(module.add_builtin(args, MacroCall::MathFunction(fun)));
        }
        "unpackSnorm4x8" | "unpackUnorm4x8" | "unpackSnorm2x16" | "unpackUnorm2x16"
        | "unpackHalf2x16" => {
            let fun = match name {
                "unpackSnorm4x8" => MathFunction::Unpack4x8snorm,
                "unpackUnorm4x8" => MathFunction::Unpack4x8unorm,
                "unpackSnorm2x16" => MathFunction::Unpack2x16snorm,
                "unpackUnorm2x16" => MathFunction::Unpack2x16unorm,
                "unpackHalf2x16" => MathFunction::Unpack2x16float,
                _ => unreachable!(),
            };

            let args = vec![TypeInner::Scalar(Scalar::U32)];

            declaration
                .overloads
                .push(module.add_builtin(args, MacroCall::MathFunction(fun)));
        }
        "atan" => {
            // bits layout
            // bit 0 - atan/atan2
            // bit 1 through 2 - dims
            for bits in 0..0b1000 {
                let fun = match bits & 0b1 {
                    0b0 => MathFunction::Atan,
                    _ => MathFunction::Atan2,
                };
                let size = match bits >> 1 {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };
                let scalar = Scalar::F32;
                let ty = || match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };

                let mut args = vec![ty()];

                if fun == MathFunction::Atan2 {
                    args.push(ty())
                }

                declaration
                    .overloads
                    .push(module.add_builtin(args, MacroCall::MathFunction(fun)))
            }
        }
        "all" | "any" | "not" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b11 {
                let size = match bits {
                    0b00 => VectorSize::Bi,
                    0b01 => VectorSize::Tri,
                    _ => VectorSize::Quad,
                };

                let args = vec![TypeInner::Vector {
                    size,
                    scalar: Scalar::BOOL,
                }];

                let fun = match name {
                    "all" => MacroCall::Relational(RelationalFunction::All),
                    "any" => MacroCall::Relational(RelationalFunction::Any),
                    "not" => MacroCall::Unary(UnaryOperator::LogicalNot),
                    _ => unreachable!(),
                };

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        "lessThan" | "greaterThan" | "lessThanEqual" | "greaterThanEqual" => {
            for bits in 0..0b1001 {
                let (size, scalar) = match bits {
                    0b0000 => (VectorSize::Bi, Scalar::F32),
                    0b0001 => (VectorSize::Tri, Scalar::F32),
                    0b0010 => (VectorSize::Quad, Scalar::F32),
                    0b0011 => (VectorSize::Bi, Scalar::I32),
                    0b0100 => (VectorSize::Tri, Scalar::I32),
                    0b0101 => (VectorSize::Quad, Scalar::I32),
                    0b0110 => (VectorSize::Bi, Scalar::U32),
                    0b0111 => (VectorSize::Tri, Scalar::U32),
                    _ => (VectorSize::Quad, Scalar::U32),
                };

                let ty = || TypeInner::Vector { size, scalar };
                let args = vec![ty(), ty()];

                let fun = MacroCall::Binary(match name {
                    "lessThan" => BinaryOperator::Less,
                    "greaterThan" => BinaryOperator::Greater,
                    "lessThanEqual" => BinaryOperator::LessEqual,
                    "greaterThanEqual" => BinaryOperator::GreaterEqual,
                    _ => unreachable!(),
                });

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        "equal" | "notEqual" => {
            for bits in 0..0b1100 {
                let (size, scalar) = match bits {
                    0b0000 => (VectorSize::Bi, Scalar::F32),
                    0b0001 => (VectorSize::Tri, Scalar::F32),
                    0b0010 => (VectorSize::Quad, Scalar::F32),
                    0b0011 => (VectorSize::Bi, Scalar::I32),
                    0b0100 => (VectorSize::Tri, Scalar::I32),
                    0b0101 => (VectorSize::Quad, Scalar::I32),
                    0b0110 => (VectorSize::Bi, Scalar::U32),
                    0b0111 => (VectorSize::Tri, Scalar::U32),
                    0b1000 => (VectorSize::Quad, Scalar::U32),
                    0b1001 => (VectorSize::Bi, Scalar::BOOL),
                    0b1010 => (VectorSize::Tri, Scalar::BOOL),
                    _ => (VectorSize::Quad, Scalar::BOOL),
                };

                let ty = || TypeInner::Vector { size, scalar };
                let args = vec![ty(), ty()];

                let fun = MacroCall::Binary(match name {
                    "equal" => BinaryOperator::Equal,
                    "notEqual" => BinaryOperator::NotEqual,
                    _ => unreachable!(),
                });

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        "min" | "max" => {
            // bits layout
            // bit 0 through 1 - scalar kind
            // bit 2 through 4 - dims
            for bits in 0..0b11100 {
                let scalar = match bits & 0b11 {
                    0b00 => Scalar::F32,
                    0b01 => Scalar::I32,
                    0b10 => Scalar::U32,
                    _ => continue,
                };
                let (size, second_size) = match bits >> 2 {
                    0b000 => (None, None),
                    0b001 => (Some(VectorSize::Bi), None),
                    0b010 => (Some(VectorSize::Tri), None),
                    0b011 => (Some(VectorSize::Quad), None),
                    0b100 => (Some(VectorSize::Bi), Some(VectorSize::Bi)),
                    0b101 => (Some(VectorSize::Tri), Some(VectorSize::Tri)),
                    _ => (Some(VectorSize::Quad), Some(VectorSize::Quad)),
                };

                let args = vec![
                    match size {
                        Some(size) => TypeInner::Vector { size, scalar },
                        None => TypeInner::Scalar(scalar),
                    },
                    match second_size {
                        Some(size) => TypeInner::Vector { size, scalar },
                        None => TypeInner::Scalar(scalar),
                    },
                ];

                let fun = match name {
                    "max" => MacroCall::Splatted(MathFunction::Max, size, 1),
                    "min" => MacroCall::Splatted(MathFunction::Min, size, 1),
                    _ => unreachable!(),
                };

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        "mix" => {
            // bits layout
            // bit 0 through 1 - dims
            // bit 2 through 4 - types
            //
            // 0b10011 is the last element since splatted single elements
            // were already added
            for bits in 0..0b10011 {
                let size = match bits & 0b11 {
                    0b00 => Some(VectorSize::Bi),
                    0b01 => Some(VectorSize::Tri),
                    0b10 => Some(VectorSize::Quad),
                    _ => None,
                };
                let (scalar, splatted, boolean) = match bits >> 2 {
                    0b000 => (Scalar::I32, false, true),
                    0b001 => (Scalar::U32, false, true),
                    0b010 => (Scalar::F32, false, true),
                    0b011 => (Scalar::F32, false, false),
                    _ => (Scalar::F32, true, false),
                };

                let ty = |scalar| match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };
                let args = vec![
                    ty(scalar),
                    ty(scalar),
                    match (boolean, splatted) {
                        (true, _) => ty(Scalar::BOOL),
                        (_, false) => TypeInner::Scalar(scalar),
                        _ => ty(scalar),
                    },
                ];

                declaration.overloads.push(module.add_builtin(
                    args,
                    match boolean {
                        true => MacroCall::MixBoolean,
                        false => MacroCall::Splatted(MathFunction::Mix, size, 2),
                    },
                ))
            }
        }
        "clamp" => {
            // bits layout
            // bit 0 through 1 - float/int/uint
            // bit 2 through 3 - dims
            // bit 4 - splatted
            //
            // 0b11010 is the last element since splatted single elements
            // were already added
            for bits in 0..0b11011 {
                let scalar = match bits & 0b11 {
                    0b00 => Scalar::F32,
                    0b01 => Scalar::I32,
                    0b10 => Scalar::U32,
                    _ => continue,
                };
                let size = match (bits >> 2) & 0b11 {
                    0b00 => Some(VectorSize::Bi),
                    0b01 => Some(VectorSize::Tri),
                    0b10 => Some(VectorSize::Quad),
                    _ => None,
                };
                let splatted = bits & 0b10000 == 0b10000;

                let base_ty = || match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };
                let limit_ty = || match splatted {
                    true => TypeInner::Scalar(scalar),
                    false => base_ty(),
                };

                let args = vec![base_ty(), limit_ty(), limit_ty()];

                declaration
                    .overloads
                    .push(module.add_builtin(args, MacroCall::Clamp(size)))
            }
        }
        "barrier" => declaration
            .overloads
            .push(module.add_builtin(Vec::new(), MacroCall::Barrier)),
        // Add common builtins with floats
        _ => inject_common_builtin(declaration, module, name, 4),
    }
}

/// Injects the builtins into declaration that need doubles
fn inject_double_builtin(declaration: &mut FunctionDeclaration, module: &mut Module, name: &str) {
    match name {
        "abs" | "sign" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b100 {
                let size = match bits {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };
                let scalar = Scalar::F64;

                let args = vec![match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                }];

                declaration.overloads.push(module.add_builtin(
                    args,
                    MacroCall::MathFunction(match name {
                        "abs" => MathFunction::Abs,
                        "sign" => MathFunction::Sign,
                        _ => unreachable!(),
                    }),
                ))
            }
        }
        "min" | "max" => {
            // bits layout
            // bit 0 through 2 - dims
            for bits in 0..0b111 {
                let (size, second_size) = match bits {
                    0b000 => (None, None),
                    0b001 => (Some(VectorSize::Bi), None),
                    0b010 => (Some(VectorSize::Tri), None),
                    0b011 => (Some(VectorSize::Quad), None),
                    0b100 => (Some(VectorSize::Bi), Some(VectorSize::Bi)),
                    0b101 => (Some(VectorSize::Tri), Some(VectorSize::Tri)),
                    _ => (Some(VectorSize::Quad), Some(VectorSize::Quad)),
                };
                let scalar = Scalar::F64;

                let args = vec![
                    match size {
                        Some(size) => TypeInner::Vector { size, scalar },
                        None => TypeInner::Scalar(scalar),
                    },
                    match second_size {
                        Some(size) => TypeInner::Vector { size, scalar },
                        None => TypeInner::Scalar(scalar),
                    },
                ];

                let fun = match name {
                    "max" => MacroCall::Splatted(MathFunction::Max, size, 1),
                    "min" => MacroCall::Splatted(MathFunction::Min, size, 1),
                    _ => unreachable!(),
                };

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        "mix" => {
            // bits layout
            // bit 0 through 1 - dims
            // bit 2 through 3 - splatted/boolean
            //
            // 0b1010 is the last element since splatted with single elements
            // is equal to normal single elements
            for bits in 0..0b1011 {
                let size = match bits & 0b11 {
                    0b00 => Some(VectorSize::Quad),
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => None,
                };
                let scalar = Scalar::F64;
                let (splatted, boolean) = match bits >> 2 {
                    0b00 => (false, false),
                    0b01 => (false, true),
                    _ => (true, false),
                };

                let ty = |scalar| match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };
                let args = vec![
                    ty(scalar),
                    ty(scalar),
                    match (boolean, splatted) {
                        (true, _) => ty(Scalar::BOOL),
                        (_, false) => TypeInner::Scalar(scalar),
                        _ => ty(scalar),
                    },
                ];

                declaration.overloads.push(module.add_builtin(
                    args,
                    match boolean {
                        true => MacroCall::MixBoolean,
                        false => MacroCall::Splatted(MathFunction::Mix, size, 2),
                    },
                ))
            }
        }
        "clamp" => {
            // bits layout
            // bit 0 through 1 - dims
            // bit 2 - splatted
            //
            // 0b110 is the last element since splatted with single elements
            // is equal to normal single elements
            for bits in 0..0b111 {
                let scalar = Scalar::F64;
                let size = match bits & 0b11 {
                    0b00 => Some(VectorSize::Bi),
                    0b01 => Some(VectorSize::Tri),
                    0b10 => Some(VectorSize::Quad),
                    _ => None,
                };
                let splatted = bits & 0b100 == 0b100;

                let base_ty = || match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };
                let limit_ty = || match splatted {
                    true => TypeInner::Scalar(scalar),
                    false => base_ty(),
                };

                let args = vec![base_ty(), limit_ty(), limit_ty()];

                declaration
                    .overloads
                    .push(module.add_builtin(args, MacroCall::Clamp(size)))
            }
        }
        "lessThan" | "greaterThan" | "lessThanEqual" | "greaterThanEqual" | "equal"
        | "notEqual" => {
            let scalar = Scalar::F64;
            for bits in 0..0b11 {
                let size = match bits {
                    0b00 => VectorSize::Bi,
                    0b01 => VectorSize::Tri,
                    _ => VectorSize::Quad,
                };

                let ty = || TypeInner::Vector { size, scalar };
                let args = vec![ty(), ty()];

                let fun = MacroCall::Binary(match name {
                    "lessThan" => BinaryOperator::Less,
                    "greaterThan" => BinaryOperator::Greater,
                    "lessThanEqual" => BinaryOperator::LessEqual,
                    "greaterThanEqual" => BinaryOperator::GreaterEqual,
                    "equal" => BinaryOperator::Equal,
                    "notEqual" => BinaryOperator::NotEqual,
                    _ => unreachable!(),
                });

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        // Add common builtins with doubles
        _ => inject_common_builtin(declaration, module, name, 8),
    }
}

/// Injects the builtins into declaration that can used either float or doubles
fn inject_common_builtin(
    declaration: &mut FunctionDeclaration,
    module: &mut Module,
    name: &str,
    float_width: crate::Bytes,
) {
    let float_scalar = Scalar {
        kind: Sk::Float,
        width: float_width,
    };
    match name {
        "ceil" | "round" | "roundEven" | "floor" | "fract" | "trunc" | "sqrt" | "inversesqrt"
        | "normalize" | "length" | "isinf" | "isnan" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b100 {
                let size = match bits {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };

                let args = vec![match size {
                    Some(size) => TypeInner::Vector {
                        size,
                        scalar: float_scalar,
                    },
                    None => TypeInner::Scalar(float_scalar),
                }];

                let fun = match name {
                    "ceil" => MacroCall::MathFunction(MathFunction::Ceil),
                    "round" | "roundEven" => MacroCall::MathFunction(MathFunction::Round),
                    "floor" => MacroCall::MathFunction(MathFunction::Floor),
                    "fract" => MacroCall::MathFunction(MathFunction::Fract),
                    "trunc" => MacroCall::MathFunction(MathFunction::Trunc),
                    "sqrt" => MacroCall::MathFunction(MathFunction::Sqrt),
                    "inversesqrt" => MacroCall::MathFunction(MathFunction::InverseSqrt),
                    "normalize" => MacroCall::MathFunction(MathFunction::Normalize),
                    "length" => MacroCall::MathFunction(MathFunction::Length),
                    "isinf" => MacroCall::Relational(RelationalFunction::IsInf),
                    "isnan" => MacroCall::Relational(RelationalFunction::IsNan),
                    _ => unreachable!(),
                };

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        "dot" | "reflect" | "distance" | "ldexp" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b100 {
                let size = match bits {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };
                let ty = |scalar| match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };

                let fun = match name {
                    "dot" => MacroCall::MathFunction(MathFunction::Dot),
                    "reflect" => MacroCall::MathFunction(MathFunction::Reflect),
                    "distance" => MacroCall::MathFunction(MathFunction::Distance),
                    "ldexp" => MacroCall::MathFunction(MathFunction::Ldexp),
                    _ => unreachable!(),
                };

                let second_scalar = match fun {
                    MacroCall::MathFunction(MathFunction::Ldexp) => Scalar::I32,
                    _ => float_scalar,
                };

                declaration
                    .overloads
                    .push(module.add_builtin(vec![ty(float_scalar), ty(second_scalar)], fun))
            }
        }
        "transpose" => {
            // bits layout
            // bit 0 through 3 - dims
            for bits in 0..0b1001 {
                let (rows, columns) = match bits {
                    0b0000 => (VectorSize::Bi, VectorSize::Bi),
                    0b0001 => (VectorSize::Bi, VectorSize::Tri),
                    0b0010 => (VectorSize::Bi, VectorSize::Quad),
                    0b0011 => (VectorSize::Tri, VectorSize::Bi),
                    0b0100 => (VectorSize::Tri, VectorSize::Tri),
                    0b0101 => (VectorSize::Tri, VectorSize::Quad),
                    0b0110 => (VectorSize::Quad, VectorSize::Bi),
                    0b0111 => (VectorSize::Quad, VectorSize::Tri),
                    _ => (VectorSize::Quad, VectorSize::Quad),
                };

                declaration.overloads.push(module.add_builtin(
                    vec![TypeInner::Matrix {
                        columns,
                        rows,
                        scalar: float_scalar,
                    }],
                    MacroCall::MathFunction(MathFunction::Transpose),
                ))
            }
        }
        "inverse" | "determinant" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b11 {
                let (rows, columns) = match bits {
                    0b00 => (VectorSize::Bi, VectorSize::Bi),
                    0b01 => (VectorSize::Tri, VectorSize::Tri),
                    _ => (VectorSize::Quad, VectorSize::Quad),
                };

                let args = vec![TypeInner::Matrix {
                    columns,
                    rows,
                    scalar: float_scalar,
                }];

                declaration.overloads.push(module.add_builtin(
                    args,
                    MacroCall::MathFunction(match name {
                        "inverse" => MathFunction::Inverse,
                        "determinant" => MathFunction::Determinant,
                        _ => unreachable!(),
                    }),
                ))
            }
        }
        "mod" | "step" => {
            // bits layout
            // bit 0 through 2 - dims
            for bits in 0..0b111 {
                let (size, second_size) = match bits {
                    0b000 => (None, None),
                    0b001 => (Some(VectorSize::Bi), None),
                    0b010 => (Some(VectorSize::Tri), None),
                    0b011 => (Some(VectorSize::Quad), None),
                    0b100 => (Some(VectorSize::Bi), Some(VectorSize::Bi)),
                    0b101 => (Some(VectorSize::Tri), Some(VectorSize::Tri)),
                    _ => (Some(VectorSize::Quad), Some(VectorSize::Quad)),
                };

                let mut args = Vec::with_capacity(2);
                let step = name == "step";

                for i in 0..2 {
                    let maybe_size = match i == step as u32 {
                        true => size,
                        false => second_size,
                    };

                    args.push(match maybe_size {
                        Some(size) => TypeInner::Vector {
                            size,
                            scalar: float_scalar,
                        },
                        None => TypeInner::Scalar(float_scalar),
                    })
                }

                let fun = match name {
                    "mod" => MacroCall::Mod(size),
                    "step" => MacroCall::Splatted(MathFunction::Step, size, 0),
                    _ => unreachable!(),
                };

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        // TODO: https://github.com/gfx-rs/naga/issues/2526
        // "modf" | "frexp" => { ... }
        "cross" => {
            let args = vec![
                TypeInner::Vector {
                    size: VectorSize::Tri,
                    scalar: float_scalar,
                },
                TypeInner::Vector {
                    size: VectorSize::Tri,
                    scalar: float_scalar,
                },
            ];

            declaration
                .overloads
                .push(module.add_builtin(args, MacroCall::MathFunction(MathFunction::Cross)))
        }
        "outerProduct" => {
            // bits layout
            // bit 0 through 3 - dims
            for bits in 0..0b1001 {
                let (size1, size2) = match bits {
                    0b0000 => (VectorSize::Bi, VectorSize::Bi),
                    0b0001 => (VectorSize::Bi, VectorSize::Tri),
                    0b0010 => (VectorSize::Bi, VectorSize::Quad),
                    0b0011 => (VectorSize::Tri, VectorSize::Bi),
                    0b0100 => (VectorSize::Tri, VectorSize::Tri),
                    0b0101 => (VectorSize::Tri, VectorSize::Quad),
                    0b0110 => (VectorSize::Quad, VectorSize::Bi),
                    0b0111 => (VectorSize::Quad, VectorSize::Tri),
                    _ => (VectorSize::Quad, VectorSize::Quad),
                };

                let args = vec![
                    TypeInner::Vector {
                        size: size1,
                        scalar: float_scalar,
                    },
                    TypeInner::Vector {
                        size: size2,
                        scalar: float_scalar,
                    },
                ];

                declaration
                    .overloads
                    .push(module.add_builtin(args, MacroCall::MathFunction(MathFunction::Outer)))
            }
        }
        "faceforward" | "fma" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b100 {
                let size = match bits {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };

                let ty = || match size {
                    Some(size) => TypeInner::Vector {
                        size,
                        scalar: float_scalar,
                    },
                    None => TypeInner::Scalar(float_scalar),
                };
                let args = vec![ty(), ty(), ty()];

                let fun = match name {
                    "faceforward" => MacroCall::MathFunction(MathFunction::FaceForward),
                    "fma" => MacroCall::MathFunction(MathFunction::Fma),
                    _ => unreachable!(),
                };

                declaration.overloads.push(module.add_builtin(args, fun))
            }
        }
        "refract" => {
            // bits layout
            // bit 0 through 1 - dims
            for bits in 0..0b100 {
                let size = match bits {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };

                let ty = || match size {
                    Some(size) => TypeInner::Vector {
                        size,
                        scalar: float_scalar,
                    },
                    None => TypeInner::Scalar(float_scalar),
                };
                let args = vec![ty(), ty(), TypeInner::Scalar(Scalar::F32)];
                declaration
                    .overloads
                    .push(module.add_builtin(args, MacroCall::MathFunction(MathFunction::Refract)))
            }
        }
        "smoothstep" => {
            // bit 0 - splatted
            // bit 1 through 2 - dims
            for bits in 0..0b1000 {
                let splatted = bits & 0b1 == 0b1;
                let size = match bits >> 1 {
                    0b00 => None,
                    0b01 => Some(VectorSize::Bi),
                    0b10 => Some(VectorSize::Tri),
                    _ => Some(VectorSize::Quad),
                };

                if splatted && size.is_none() {
                    continue;
                }

                let base_ty = || match size {
                    Some(size) => TypeInner::Vector {
                        size,
                        scalar: float_scalar,
                    },
                    None => TypeInner::Scalar(float_scalar),
                };
                let ty = || match splatted {
                    true => TypeInner::Scalar(float_scalar),
                    false => base_ty(),
                };
                declaration.overloads.push(module.add_builtin(
                    vec![ty(), ty(), base_ty()],
                    MacroCall::SmoothStep { splatted: size },
                ))
            }
        }
        // The function isn't a builtin or we don't yet support it
        _ => {}
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextureLevelType {
    None,
    Lod,
    Grad,
}

/// A compiler defined builtin function
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MacroCall {
    Sampler,
    SamplerShadow,
    Texture {
        proj: bool,
        offset: bool,
        shadow: bool,
        level_type: TextureLevelType,
    },
    TextureSize {
        arrayed: bool,
    },
    TextureQueryLevels,
    ImageLoad {
        multi: bool,
    },
    ImageStore,
    MathFunction(MathFunction),
    FindLsbUint,
    FindMsbUint,
    BitfieldExtract,
    BitfieldInsert,
    Relational(RelationalFunction),
    Unary(UnaryOperator),
    Binary(BinaryOperator),
    Mod(Option<VectorSize>),
    Splatted(MathFunction, Option<VectorSize>, usize),
    MixBoolean,
    Clamp(Option<VectorSize>),
    BitCast(Sk),
    Derivate(Axis, Ctrl),
    Barrier,
    /// SmoothStep needs a separate variant because it might need it's inputs
    /// to be splatted depending on the overload
    SmoothStep {
        /// The size of the splat operation if some
        splatted: Option<VectorSize>,
    },
}

impl MacroCall {
    /// Adds the necessary expressions and statements to the passed body and
    /// finally returns the final expression with the correct result
    pub fn call(
        &self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        args: &mut [Handle<Expression>],
        meta: Span,
    ) -> Result<Option<Handle<Expression>>> {
        Ok(Some(match *self {
            MacroCall::Sampler => {
                ctx.samplers.insert(args[0], args[1]);
                args[0]
            }
            MacroCall::SamplerShadow => {
                sampled_to_depth(ctx, args[0], meta, &mut frontend.errors);
                ctx.invalidate_expression(args[0], meta)?;
                ctx.samplers.insert(args[0], args[1]);
                args[0]
            }
            MacroCall::Texture {
                proj,
                offset,
                shadow,
                level_type,
            } => {
                let mut coords = args[1];

                if proj {
                    let size = match *ctx.resolve_type(coords, meta)? {
                        TypeInner::Vector { size, .. } => size,
                        _ => unreachable!(),
                    };
                    let mut right = ctx.add_expression(
                        Expression::AccessIndex {
                            base: coords,
                            index: size as u32 - 1,
                        },
                        Span::default(),
                    )?;
                    let left = if let VectorSize::Bi = size {
                        ctx.add_expression(
                            Expression::AccessIndex {
                                base: coords,
                                index: 0,
                            },
                            Span::default(),
                        )?
                    } else {
                        let size = match size {
                            VectorSize::Tri => VectorSize::Bi,
                            _ => VectorSize::Tri,
                        };
                        right = ctx.add_expression(
                            Expression::Splat { size, value: right },
                            Span::default(),
                        )?;
                        ctx.vector_resize(size, coords, Span::default())?
                    };
                    coords = ctx.add_expression(
                        Expression::Binary {
                            op: BinaryOperator::Divide,
                            left,
                            right,
                        },
                        Span::default(),
                    )?;
                }

                let extra = args.get(2).copied();
                let comps = frontend.coordinate_components(ctx, args[0], coords, extra, meta)?;

                let mut num_args = 2;

                if comps.used_extra {
                    num_args += 1;
                };

                // Parse out explicit texture level.
                let mut level = match level_type {
                    TextureLevelType::None => SampleLevel::Auto,

                    TextureLevelType::Lod => {
                        num_args += 1;

                        if shadow {
                            log::debug!("Assuming LOD {:?} is zero", args[2],);

                            SampleLevel::Zero
                        } else {
                            SampleLevel::Exact(args[2])
                        }
                    }

                    TextureLevelType::Grad => {
                        num_args += 2;

                        if shadow {
                            log::debug!(
                                "Assuming gradients {:?} and {:?} are not greater than 1",
                                args[2],
                                args[3],
                            );
                            SampleLevel::Zero
                        } else {
                            SampleLevel::Gradient {
                                x: args[2],
                                y: args[3],
                            }
                        }
                    }
                };

                let texture_offset = match offset {
                    true => {
                        let offset_arg = args[num_args];
                        num_args += 1;
                        Some(offset_arg)
                    }
                    false => None,
                };

                // Now go back and look for optional bias arg (if available)
                if let TextureLevelType::None = level_type {
                    level = args
                        .get(num_args)
                        .copied()
                        .map_or(SampleLevel::Auto, SampleLevel::Bias);
                }

                texture_call(ctx, args[0], level, comps, texture_offset, meta)?
            }

            MacroCall::TextureSize { arrayed } => {
                let mut expr = ctx.add_expression(
                    Expression::ImageQuery {
                        image: args[0],
                        query: ImageQuery::Size {
                            level: args.get(1).copied(),
                        },
                    },
                    Span::default(),
                )?;

                if arrayed {
                    let mut components = Vec::with_capacity(4);

                    let size = match *ctx.resolve_type(expr, meta)? {
                        TypeInner::Vector { size: ori_size, .. } => {
                            for index in 0..(ori_size as u32) {
                                components.push(ctx.add_expression(
                                    Expression::AccessIndex { base: expr, index },
                                    Span::default(),
                                )?)
                            }

                            match ori_size {
                                VectorSize::Bi => VectorSize::Tri,
                                _ => VectorSize::Quad,
                            }
                        }
                        _ => {
                            components.push(expr);
                            VectorSize::Bi
                        }
                    };

                    components.push(ctx.add_expression(
                        Expression::ImageQuery {
                            image: args[0],
                            query: ImageQuery::NumLayers,
                        },
                        Span::default(),
                    )?);

                    let ty = ctx.module.types.insert(
                        Type {
                            name: None,
                            inner: TypeInner::Vector {
                                size,
                                scalar: Scalar::U32,
                            },
                        },
                        Span::default(),
                    );

                    expr = ctx.add_expression(Expression::Compose { components, ty }, meta)?
                }

                ctx.add_expression(
                    Expression::As {
                        expr,
                        kind: Sk::Sint,
                        convert: Some(4),
                    },
                    Span::default(),
                )?
            }
            MacroCall::TextureQueryLevels => {
                let expr = ctx.add_expression(
                    Expression::ImageQuery {
                        image: args[0],
                        query: ImageQuery::NumLevels,
                    },
                    Span::default(),
                )?;

                ctx.add_expression(
                    Expression::As {
                        expr,
                        kind: Sk::Sint,
                        convert: Some(4),
                    },
                    Span::default(),
                )?
            }
            MacroCall::ImageLoad { multi } => {
                let comps = frontend.coordinate_components(ctx, args[0], args[1], None, meta)?;
                let (sample, level) = match (multi, args.get(2)) {
                    (_, None) => (None, None),
                    (true, Some(&arg)) => (Some(arg), None),
                    (false, Some(&arg)) => (None, Some(arg)),
                };
                ctx.add_expression(
                    Expression::ImageLoad {
                        image: args[0],
                        coordinate: comps.coordinate,
                        array_index: comps.array_index,
                        sample,
                        level,
                    },
                    Span::default(),
                )?
            }
            MacroCall::ImageStore => {
                let comps = frontend.coordinate_components(ctx, args[0], args[1], None, meta)?;
                ctx.emit_restart();
                ctx.body.push(
                    crate::Statement::ImageStore {
                        image: args[0],
                        coordinate: comps.coordinate,
                        array_index: comps.array_index,
                        value: args[2],
                    },
                    meta,
                );
                return Ok(None);
            }
            MacroCall::MathFunction(fun) => ctx.add_expression(
                Expression::Math {
                    fun,
                    arg: args[0],
                    arg1: args.get(1).copied(),
                    arg2: args.get(2).copied(),
                    arg3: args.get(3).copied(),
                },
                Span::default(),
            )?,
            mc @ (MacroCall::FindLsbUint | MacroCall::FindMsbUint) => {
                let fun = match mc {
                    MacroCall::FindLsbUint => MathFunction::FirstTrailingBit,
                    MacroCall::FindMsbUint => MathFunction::FirstLeadingBit,
                    _ => unreachable!(),
                };
                let res = ctx.add_expression(
                    Expression::Math {
                        fun,
                        arg: args[0],
                        arg1: None,
                        arg2: None,
                        arg3: None,
                    },
                    Span::default(),
                )?;
                ctx.add_expression(
                    Expression::As {
                        expr: res,
                        kind: Sk::Sint,
                        convert: Some(4),
                    },
                    Span::default(),
                )?
            }
            MacroCall::BitfieldInsert => {
                let conv_arg_2 = ctx.add_expression(
                    Expression::As {
                        expr: args[2],
                        kind: Sk::Uint,
                        convert: Some(4),
                    },
                    Span::default(),
                )?;
                let conv_arg_3 = ctx.add_expression(
                    Expression::As {
                        expr: args[3],
                        kind: Sk::Uint,
                        convert: Some(4),
                    },
                    Span::default(),
                )?;
                ctx.add_expression(
                    Expression::Math {
                        fun: MathFunction::InsertBits,
                        arg: args[0],
                        arg1: Some(args[1]),
                        arg2: Some(conv_arg_2),
                        arg3: Some(conv_arg_3),
                    },
                    Span::default(),
                )?
            }
            MacroCall::BitfieldExtract => {
                let conv_arg_1 = ctx.add_expression(
                    Expression::As {
                        expr: args[1],
                        kind: Sk::Uint,
                        convert: Some(4),
                    },
                    Span::default(),
                )?;
                let conv_arg_2 = ctx.add_expression(
                    Expression::As {
                        expr: args[2],
                        kind: Sk::Uint,
                        convert: Some(4),
                    },
                    Span::default(),
                )?;
                ctx.add_expression(
                    Expression::Math {
                        fun: MathFunction::ExtractBits,
                        arg: args[0],
                        arg1: Some(conv_arg_1),
                        arg2: Some(conv_arg_2),
                        arg3: None,
                    },
                    Span::default(),
                )?
            }
            MacroCall::Relational(fun) => ctx.add_expression(
                Expression::Relational {
                    fun,
                    argument: args[0],
                },
                Span::default(),
            )?,
            MacroCall::Unary(op) => {
                ctx.add_expression(Expression::Unary { op, expr: args[0] }, Span::default())?
            }
            MacroCall::Binary(op) => ctx.add_expression(
                Expression::Binary {
                    op,
                    left: args[0],
                    right: args[1],
                },
                Span::default(),
            )?,
            MacroCall::Mod(size) => {
                ctx.implicit_splat(&mut args[1], meta, size)?;

                // x - y * floor(x / y)

                let div = ctx.add_expression(
                    Expression::Binary {
                        op: BinaryOperator::Divide,
                        left: args[0],
                        right: args[1],
                    },
                    Span::default(),
                )?;
                let floor = ctx.add_expression(
                    Expression::Math {
                        fun: MathFunction::Floor,
                        arg: div,
                        arg1: None,
                        arg2: None,
                        arg3: None,
                    },
                    Span::default(),
                )?;
                let mult = ctx.add_expression(
                    Expression::Binary {
                        op: BinaryOperator::Multiply,
                        left: floor,
                        right: args[1],
                    },
                    Span::default(),
                )?;
                ctx.add_expression(
                    Expression::Binary {
                        op: BinaryOperator::Subtract,
                        left: args[0],
                        right: mult,
                    },
                    Span::default(),
                )?
            }
            MacroCall::Splatted(fun, size, i) => {
                ctx.implicit_splat(&mut args[i], meta, size)?;

                ctx.add_expression(
                    Expression::Math {
                        fun,
                        arg: args[0],
                        arg1: args.get(1).copied(),
                        arg2: args.get(2).copied(),
                        arg3: args.get(3).copied(),
                    },
                    Span::default(),
                )?
            }
            MacroCall::MixBoolean => ctx.add_expression(
                Expression::Select {
                    condition: args[2],
                    accept: args[1],
                    reject: args[0],
                },
                Span::default(),
            )?,
            MacroCall::Clamp(size) => {
                ctx.implicit_splat(&mut args[1], meta, size)?;
                ctx.implicit_splat(&mut args[2], meta, size)?;

                ctx.add_expression(
                    Expression::Math {
                        fun: MathFunction::Clamp,
                        arg: args[0],
                        arg1: args.get(1).copied(),
                        arg2: args.get(2).copied(),
                        arg3: args.get(3).copied(),
                    },
                    Span::default(),
                )?
            }
            MacroCall::BitCast(kind) => ctx.add_expression(
                Expression::As {
                    expr: args[0],
                    kind,
                    convert: None,
                },
                Span::default(),
            )?,
            MacroCall::Derivate(axis, ctrl) => ctx.add_expression(
                Expression::Derivative {
                    axis,
                    ctrl,
                    expr: args[0],
                },
                Span::default(),
            )?,
            MacroCall::Barrier => {
                ctx.emit_restart();
                ctx.body.push(
                    crate::Statement::ControlBarrier(crate::Barrier::all()),
                    meta,
                );
                return Ok(None);
            }
            MacroCall::SmoothStep { splatted } => {
                ctx.implicit_splat(&mut args[0], meta, splatted)?;
                ctx.implicit_splat(&mut args[1], meta, splatted)?;

                ctx.add_expression(
                    Expression::Math {
                        fun: MathFunction::SmoothStep,
                        arg: args[0],
                        arg1: args.get(1).copied(),
                        arg2: args.get(2).copied(),
                        arg3: None,
                    },
                    Span::default(),
                )?
            }
        }))
    }
}

fn texture_call(
    ctx: &mut Context,
    image: Handle<Expression>,
    level: SampleLevel,
    comps: CoordComponents,
    offset: Option<Handle<Expression>>,
    meta: Span,
) -> Result<Handle<Expression>> {
    if let Some(sampler) = ctx.samplers.get(&image).copied() {
        let mut array_index = comps.array_index;

        if let Some(ref mut array_index_expr) = array_index {
            ctx.conversion(array_index_expr, meta, Scalar::I32)?;
        }

        Ok(ctx.add_expression(
            Expression::ImageSample {
                image,
                sampler,
                gather: None, //TODO
                coordinate: comps.coordinate,
                array_index,
                offset,
                level,
                depth_ref: comps.depth_ref,
                clamp_to_edge: false,
            },
            meta,
        )?)
    } else {
        Err(Error {
            kind: ErrorKind::SemanticError("Bad call".into()),
            meta,
        })
    }
}

/// Helper struct for texture calls with the separate components from the vector argument
///
/// Obtained by calling [`coordinate_components`](Frontend::coordinate_components)
#[derive(Debug)]
struct CoordComponents {
    coordinate: Handle<Expression>,
    depth_ref: Option<Handle<Expression>>,
    array_index: Option<Handle<Expression>>,
    used_extra: bool,
}

impl Frontend {
    /// Helper function for texture calls, splits the vector argument into it's components
    fn coordinate_components(
        &mut self,
        ctx: &mut Context,
        image: Handle<Expression>,
        coord: Handle<Expression>,
        extra: Option<Handle<Expression>>,
        meta: Span,
    ) -> Result<CoordComponents> {
        if let TypeInner::Image {
            dim,
            arrayed,
            class,
        } = *ctx.resolve_type(image, meta)?
        {
            let image_size = match dim {
                Dim::D1 => None,
                Dim::D2 => Some(VectorSize::Bi),
                Dim::D3 => Some(VectorSize::Tri),
                Dim::Cube => Some(VectorSize::Tri),
            };
            let coord_size = match *ctx.resolve_type(coord, meta)? {
                TypeInner::Vector { size, .. } => Some(size),
                _ => None,
            };
            let (shadow, storage) = match class {
                ImageClass::Depth { .. } => (true, false),
                ImageClass::Storage { .. } => (false, true),
                ImageClass::Sampled { .. } => (false, false),
                ImageClass::External => unreachable!(),
            };

            let coordinate = match (image_size, coord_size) {
                (Some(size), Some(coord_s)) if size != coord_s => {
                    ctx.vector_resize(size, coord, Span::default())?
                }
                (None, Some(_)) => ctx.add_expression(
                    Expression::AccessIndex {
                        base: coord,
                        index: 0,
                    },
                    Span::default(),
                )?,
                _ => coord,
            };

            let mut coord_index = image_size.map_or(1, |s| s as u32);

            let array_index = if arrayed && !(storage && dim == Dim::Cube) {
                let index = coord_index;
                coord_index += 1;

                Some(ctx.add_expression(
                    Expression::AccessIndex { base: coord, index },
                    Span::default(),
                )?)
            } else {
                None
            };
            let mut used_extra = false;
            let depth_ref = match shadow {
                true => {
                    let index = coord_index;

                    if index == 4 {
                        used_extra = true;
                        extra
                    } else {
                        Some(ctx.add_expression(
                            Expression::AccessIndex { base: coord, index },
                            Span::default(),
                        )?)
                    }
                }
                false => None,
            };

            Ok(CoordComponents {
                coordinate,
                depth_ref,
                array_index,
                used_extra,
            })
        } else {
            self.errors.push(Error {
                kind: ErrorKind::SemanticError("Type is not an image".into()),
                meta,
            });

            Ok(CoordComponents {
                coordinate: coord,
                depth_ref: None,
                array_index: None,
                used_extra: false,
            })
        }
    }
}

/// Helper function to cast a expression holding a sampled image to a
/// depth image.
pub fn sampled_to_depth(
    ctx: &mut Context,
    image: Handle<Expression>,
    meta: Span,
    errors: &mut Vec<Error>,
) {
    // Get the a mutable type handle of the underlying image storage
    let ty = match ctx[image] {
        Expression::GlobalVariable(handle) => &mut ctx.module.global_variables.get_mut(handle).ty,
        Expression::FunctionArgument(i) => {
            // Mark the function argument as carrying a depth texture
            ctx.parameters_info[i as usize].depth = true;
            // NOTE: We need to later also change the parameter type
            &mut ctx.arguments[i as usize].ty
        }
        _ => {
            // Only globals and function arguments are allowed to carry an image
            return errors.push(Error {
                kind: ErrorKind::SemanticError("Not a valid texture expression".into()),
                meta,
            });
        }
    };

    match ctx.module.types[*ty].inner {
        // Update the image class to depth in case it already isn't
        TypeInner::Image {
            class,
            dim,
            arrayed,
        } => match class {
            ImageClass::Sampled { multi, .. } => {
                *ty = ctx.module.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Image {
                            dim,
                            arrayed,
                            class: ImageClass::Depth { multi },
                        },
                    },
                    Span::default(),
                )
            }
            ImageClass::Depth { .. } => {}
            // Other image classes aren't allowed to be transformed to depth
            ImageClass::Storage { .. } => errors.push(Error {
                kind: ErrorKind::SemanticError("Not a texture".into()),
                meta,
            }),
            ImageClass::External => unreachable!(),
        },
        _ => errors.push(Error {
            kind: ErrorKind::SemanticError("Not a texture".into()),
            meta,
        }),
    };

    // Copy the handle to allow borrowing the `ctx` again
    let ty = *ty;

    // If the image was passed through a function argument we also need to change
    // the corresponding parameter
    if let Expression::FunctionArgument(i) = ctx[image] {
        ctx.parameters[i as usize] = ty;
    }
}

bitflags::bitflags! {
    /// Influences the operation [`texture_args_generator`]
    struct TextureArgsOptions: u32 {
        /// Generates multisampled variants of images
        const MULTI = 1 << 0;
        /// Generates shadow variants of images
        const SHADOW = 1 << 1;
        /// Generates standard images
        const STANDARD = 1 << 2;
        /// Generates cube arrayed images
        const CUBE_ARRAY = 1 << 3;
        /// Generates cube arrayed images
        const D2_MULTI_ARRAY = 1 << 4;
    }
}

impl From<BuiltinVariations> for TextureArgsOptions {
    fn from(variations: BuiltinVariations) -> Self {
        let mut options = TextureArgsOptions::empty();
        if variations.contains(BuiltinVariations::STANDARD) {
            options |= TextureArgsOptions::STANDARD
        }
        if variations.contains(BuiltinVariations::CUBE_TEXTURES_ARRAY) {
            options |= TextureArgsOptions::CUBE_ARRAY
        }
        if variations.contains(BuiltinVariations::D2_MULTI_TEXTURES_ARRAY) {
            options |= TextureArgsOptions::D2_MULTI_ARRAY
        }
        options
    }
}

/// Helper function to generate the image components for texture/image builtins
///
/// Calls the passed function `f` with:
/// ```text
/// f(ScalarKind, ImageDimension, arrayed, multi, shadow)
/// ```
///
/// `options` controls extra image variants generation like multisampling and depth,
/// see the struct documentation
fn texture_args_generator(
    options: TextureArgsOptions,
    mut f: impl FnMut(crate::ScalarKind, Dim, bool, bool, bool),
) {
    for kind in [Sk::Float, Sk::Uint, Sk::Sint].iter().copied() {
        for dim in [Dim::D1, Dim::D2, Dim::D3, Dim::Cube].iter().copied() {
            for arrayed in [false, true].iter().copied() {
                if dim == Dim::Cube && arrayed {
                    if !options.contains(TextureArgsOptions::CUBE_ARRAY) {
                        continue;
                    }
                } else if Dim::D2 == dim
                    && options.contains(TextureArgsOptions::MULTI)
                    && arrayed
                    && options.contains(TextureArgsOptions::D2_MULTI_ARRAY)
                {
                    // multisampling for sampler2DMSArray
                    f(kind, dim, arrayed, true, false);
                } else if !options.contains(TextureArgsOptions::STANDARD) {
                    continue;
                }

                f(kind, dim, arrayed, false, false);

                // 3D images can't be neither arrayed nor shadow
                // so we break out early, this way arrayed will always
                // be false and we won't hit the shadow branch
                if let Dim::D3 = dim {
                    break;
                }

                if Dim::D2 == dim && options.contains(TextureArgsOptions::MULTI) && !arrayed {
                    // multisampling
                    f(kind, dim, arrayed, true, false);
                }

                if Sk::Float == kind && options.contains(TextureArgsOptions::SHADOW) {
                    // shadow
                    f(kind, dim, arrayed, false, true);
                }
            }
        }
    }
}

/// Helper functions used to convert from a image dimension into a integer representing the
/// number of components needed for the coordinates vector (1 means scalar instead of vector)
const fn image_dims_to_coords_size(dim: Dim) -> usize {
    match dim {
        Dim::D1 => 1,
        Dim::D2 => 2,
        _ => 3,
    }
}
