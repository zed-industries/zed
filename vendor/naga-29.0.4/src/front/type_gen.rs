/*!
Type generators.
*/

use alloc::{string::ToString, vec};

use crate::{arena::Handle, span::Span};

impl crate::Module {
    /// Populate this module's [`SpecialTypes::ray_desc`] type.
    ///
    /// [`SpecialTypes::ray_desc`] is the type of the [`descriptor`] operand of
    /// an [`Initialize`] [`RayQuery`] statement. In WGSL, it is a struct type
    /// referred to as `RayDesc`.
    ///
    /// Backends consume values of this type to drive platform APIs, so if you
    /// change any its fields, you must update the backends to match. Look for
    /// backend code dealing with [`RayQueryFunction::Initialize`].
    ///
    /// [`SpecialTypes::ray_desc`]: crate::SpecialTypes::ray_desc
    /// [`descriptor`]: crate::RayQueryFunction::Initialize::descriptor
    /// [`Initialize`]: crate::RayQueryFunction::Initialize
    /// [`RayQuery`]: crate::Statement::RayQuery
    /// [`RayQueryFunction::Initialize`]: crate::RayQueryFunction::Initialize
    pub fn generate_ray_desc_type(&mut self) -> Handle<crate::Type> {
        if let Some(handle) = self.special_types.ray_desc {
            return handle;
        }

        let ty_flag = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar(crate::Scalar::U32),
            },
            Span::UNDEFINED,
        );
        let ty_scalar = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar(crate::Scalar::F32),
            },
            Span::UNDEFINED,
        );
        let ty_vector = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Vector {
                    size: crate::VectorSize::Tri,
                    scalar: crate::Scalar::F32,
                },
            },
            Span::UNDEFINED,
        );

        let handle = self.types.insert(
            crate::Type {
                name: Some("RayDesc".to_string()),
                inner: crate::TypeInner::Struct {
                    members: vec![
                        crate::StructMember {
                            name: Some("flags".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 0,
                        },
                        crate::StructMember {
                            name: Some("cull_mask".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 4,
                        },
                        crate::StructMember {
                            name: Some("tmin".to_string()),
                            ty: ty_scalar,
                            binding: None,
                            offset: 8,
                        },
                        crate::StructMember {
                            name: Some("tmax".to_string()),
                            ty: ty_scalar,
                            binding: None,
                            offset: 12,
                        },
                        crate::StructMember {
                            name: Some("origin".to_string()),
                            ty: ty_vector,
                            binding: None,
                            offset: 16,
                        },
                        crate::StructMember {
                            name: Some("dir".to_string()),
                            ty: ty_vector,
                            binding: None,
                            offset: 32,
                        },
                    ],
                    span: 48,
                },
            },
            Span::UNDEFINED,
        );

        self.special_types.ray_desc = Some(handle);
        handle
    }

    /// Make sure the types for the vertex return are in the module's type
    pub fn generate_vertex_return_type(&mut self) -> Handle<crate::Type> {
        if let Some(handle) = self.special_types.ray_vertex_return {
            return handle;
        }
        let ty_vec3f = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Vector {
                    size: crate::VectorSize::Tri,
                    scalar: crate::Scalar::F32,
                },
            },
            Span::UNDEFINED,
        );
        let array = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Array {
                    base: ty_vec3f,
                    size: crate::ArraySize::Constant(core::num::NonZeroU32::new(3).unwrap()),
                    stride: 16,
                },
            },
            Span::UNDEFINED,
        );
        self.special_types.ray_vertex_return = Some(array);
        array
    }

    /// Populate this module's [`SpecialTypes::ray_intersection`] type.
    ///
    /// [`SpecialTypes::ray_intersection`] is the type of a
    /// `RayQueryGetIntersection` expression. In WGSL, it is a struct type
    /// referred to as `RayIntersection`.
    ///
    /// Backends construct values of this type based on platform APIs, so if you
    /// change any its fields, you must update the backends to match. Look for
    /// the backend's handling for [`Expression::RayQueryGetIntersection`].
    ///
    /// [`SpecialTypes::ray_intersection`]: crate::SpecialTypes::ray_intersection
    /// [`Expression::RayQueryGetIntersection`]: crate::Expression::RayQueryGetIntersection
    pub fn generate_ray_intersection_type(&mut self) -> Handle<crate::Type> {
        if let Some(handle) = self.special_types.ray_intersection {
            return handle;
        }

        let ty_flag = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar(crate::Scalar::U32),
            },
            Span::UNDEFINED,
        );
        let ty_scalar = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar(crate::Scalar::F32),
            },
            Span::UNDEFINED,
        );
        let ty_barycentrics = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Vector {
                    size: crate::VectorSize::Bi,
                    scalar: crate::Scalar::F32,
                },
            },
            Span::UNDEFINED,
        );
        let ty_bool = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar(crate::Scalar::BOOL),
            },
            Span::UNDEFINED,
        );
        let ty_transform = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Matrix {
                    columns: crate::VectorSize::Quad,
                    rows: crate::VectorSize::Tri,
                    scalar: crate::Scalar::F32,
                },
            },
            Span::UNDEFINED,
        );

        let handle = self.types.insert(
            crate::Type {
                name: Some("RayIntersection".to_string()),
                inner: crate::TypeInner::Struct {
                    members: vec![
                        crate::StructMember {
                            name: Some("kind".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 0,
                        },
                        crate::StructMember {
                            name: Some("t".to_string()),
                            ty: ty_scalar,
                            binding: None,
                            offset: 4,
                        },
                        crate::StructMember {
                            name: Some("instance_custom_data".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 8,
                        },
                        crate::StructMember {
                            name: Some("instance_index".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 12,
                        },
                        crate::StructMember {
                            name: Some("sbt_record_offset".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 16,
                        },
                        crate::StructMember {
                            name: Some("geometry_index".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 20,
                        },
                        crate::StructMember {
                            name: Some("primitive_index".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 24,
                        },
                        crate::StructMember {
                            name: Some("barycentrics".to_string()),
                            ty: ty_barycentrics,
                            binding: None,
                            offset: 28,
                        },
                        crate::StructMember {
                            name: Some("front_face".to_string()),
                            ty: ty_bool,
                            binding: None,
                            offset: 36,
                        },
                        crate::StructMember {
                            name: Some("object_to_world".to_string()),
                            ty: ty_transform,
                            binding: None,
                            offset: 48,
                        },
                        crate::StructMember {
                            name: Some("world_to_object".to_string()),
                            ty: ty_transform,
                            binding: None,
                            offset: 112,
                        },
                    ],
                    span: 176,
                },
            },
            Span::UNDEFINED,
        );

        self.special_types.ray_intersection = Some(handle);
        handle
    }

    /// Generate [`SpecialTypes::external_texture_params`] and
    /// [`SpecialTypes::external_texture_transfer_function`].
    ///
    /// Other than the WGSL backend, every backend that supports external
    /// textures does so by lowering them to a set of ordinary textures and
    /// some parameters saying how to sample from them. These types are used
    /// for said parameters. Note that they are not used by the IR, but
    /// generated purely as a convenience for the backends.
    ///
    /// [`SpecialTypes::external_texture_params`]: crate::ir::SpecialTypes::external_texture_params
    /// [`SpecialTypes::external_texture_transfer_function`]: crate::ir::SpecialTypes::external_texture_transfer_function
    pub fn generate_external_texture_types(&mut self) {
        if self.special_types.external_texture_params.is_some() {
            return;
        }

        let ty_f32 = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar(crate::Scalar::F32),
            },
            Span::UNDEFINED,
        );
        let ty_u32 = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar(crate::Scalar::U32),
            },
            Span::UNDEFINED,
        );
        let ty_vec2u = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Vector {
                    size: crate::VectorSize::Bi,
                    scalar: crate::Scalar::U32,
                },
            },
            Span::UNDEFINED,
        );
        let ty_mat3x2f = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Matrix {
                    columns: crate::VectorSize::Tri,
                    rows: crate::VectorSize::Bi,
                    scalar: crate::Scalar::F32,
                },
            },
            Span::UNDEFINED,
        );
        let ty_mat3x3f = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Matrix {
                    columns: crate::VectorSize::Tri,
                    rows: crate::VectorSize::Tri,
                    scalar: crate::Scalar::F32,
                },
            },
            Span::UNDEFINED,
        );
        let ty_mat4x4f = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Matrix {
                    columns: crate::VectorSize::Quad,
                    rows: crate::VectorSize::Quad,
                    scalar: crate::Scalar::F32,
                },
            },
            Span::UNDEFINED,
        );

        let transfer_fn_handle = self.types.insert(
            crate::Type {
                name: Some("NagaExternalTextureTransferFn".to_string()),
                inner: crate::TypeInner::Struct {
                    members: vec![
                        crate::StructMember {
                            name: Some("a".to_string()),
                            ty: ty_f32,
                            binding: None,
                            offset: 0,
                        },
                        crate::StructMember {
                            name: Some("b".to_string()),
                            ty: ty_f32,
                            binding: None,
                            offset: 4,
                        },
                        crate::StructMember {
                            name: Some("g".to_string()),
                            ty: ty_f32,
                            binding: None,
                            offset: 8,
                        },
                        crate::StructMember {
                            name: Some("k".to_string()),
                            ty: ty_f32,
                            binding: None,
                            offset: 12,
                        },
                    ],
                    span: 16,
                },
            },
            Span::UNDEFINED,
        );
        self.special_types.external_texture_transfer_function = Some(transfer_fn_handle);

        let params_handle = self.types.insert(
            crate::Type {
                name: Some("NagaExternalTextureParams".to_string()),
                inner: crate::TypeInner::Struct {
                    members: vec![
                        crate::StructMember {
                            name: Some("yuv_conversion_matrix".to_string()),
                            ty: ty_mat4x4f,
                            binding: None,
                            offset: 0,
                        },
                        crate::StructMember {
                            name: Some("gamut_conversion_matrix".to_string()),
                            ty: ty_mat3x3f,
                            binding: None,
                            offset: 64,
                        },
                        crate::StructMember {
                            name: Some("src_tf".to_string()),
                            ty: transfer_fn_handle,
                            binding: None,
                            offset: 112,
                        },
                        crate::StructMember {
                            name: Some("dst_tf".to_string()),
                            ty: transfer_fn_handle,
                            binding: None,
                            offset: 128,
                        },
                        crate::StructMember {
                            name: Some("sample_transform".to_string()),
                            ty: ty_mat3x2f,
                            binding: None,
                            offset: 144,
                        },
                        crate::StructMember {
                            name: Some("load_transform".to_string()),
                            ty: ty_mat3x2f,
                            binding: None,
                            offset: 168,
                        },
                        crate::StructMember {
                            name: Some("size".to_string()),
                            ty: ty_vec2u,
                            binding: None,
                            offset: 192,
                        },
                        crate::StructMember {
                            name: Some("num_planes".to_string()),
                            ty: ty_u32,
                            binding: None,
                            offset: 200,
                        },
                    ],
                    span: 208,
                },
            },
            Span::UNDEFINED,
        );
        self.special_types.external_texture_params = Some(params_handle);
    }

    /// Populate this module's [`SpecialTypes::predeclared_types`] type and return the handle.
    ///
    /// [`SpecialTypes::predeclared_types`]: crate::SpecialTypes::predeclared_types
    pub fn generate_predeclared_type(
        &mut self,
        special_type: crate::PredeclaredType,
    ) -> Handle<crate::Type> {
        if let Some(value) = self.special_types.predeclared_types.get(&special_type) {
            return *value;
        }

        let name = special_type.struct_name();
        let ty = match special_type {
            crate::PredeclaredType::AtomicCompareExchangeWeakResult(scalar) => {
                let bool_ty = self.types.insert(
                    crate::Type {
                        name: None,
                        inner: crate::TypeInner::Scalar(crate::Scalar::BOOL),
                    },
                    Span::UNDEFINED,
                );
                let scalar_ty = self.types.insert(
                    crate::Type {
                        name: None,
                        inner: crate::TypeInner::Scalar(scalar),
                    },
                    Span::UNDEFINED,
                );

                crate::Type {
                    name: Some(name),
                    inner: crate::TypeInner::Struct {
                        members: vec![
                            crate::StructMember {
                                name: Some("old_value".to_string()),
                                ty: scalar_ty,
                                binding: None,
                                offset: 0,
                            },
                            crate::StructMember {
                                name: Some("exchanged".to_string()),
                                ty: bool_ty,
                                binding: None,
                                offset: scalar.width as u32,
                            },
                        ],
                        span: scalar.width as u32 * 2,
                    },
                }
            }
            crate::PredeclaredType::ModfResult { size, scalar } => {
                let float_ty = self.types.insert(
                    crate::Type {
                        name: None,
                        inner: crate::TypeInner::Scalar(scalar),
                    },
                    Span::UNDEFINED,
                );

                let (member_ty, second_offset) = if let Some(size) = size {
                    let vec_ty = self.types.insert(
                        crate::Type {
                            name: None,
                            inner: crate::TypeInner::Vector { size, scalar },
                        },
                        Span::UNDEFINED,
                    );
                    (vec_ty, size as u32 * scalar.width as u32)
                } else {
                    (float_ty, scalar.width as u32)
                };

                crate::Type {
                    name: Some(name),
                    inner: crate::TypeInner::Struct {
                        members: vec![
                            crate::StructMember {
                                name: Some("fract".to_string()),
                                ty: member_ty,
                                binding: None,
                                offset: 0,
                            },
                            crate::StructMember {
                                name: Some("whole".to_string()),
                                ty: member_ty,
                                binding: None,
                                offset: second_offset,
                            },
                        ],
                        span: second_offset * 2,
                    },
                }
            }
            crate::PredeclaredType::FrexpResult { size, scalar } => {
                let float_ty = self.types.insert(
                    crate::Type {
                        name: None,
                        inner: crate::TypeInner::Scalar(scalar),
                    },
                    Span::UNDEFINED,
                );

                let int_ty = self.types.insert(
                    crate::Type {
                        name: None,
                        inner: crate::TypeInner::Scalar(crate::Scalar {
                            kind: crate::ScalarKind::Sint,
                            width: scalar.width,
                        }),
                    },
                    Span::UNDEFINED,
                );

                let (fract_member_ty, exp_member_ty, second_offset) = if let Some(size) = size {
                    let vec_float_ty = self.types.insert(
                        crate::Type {
                            name: None,
                            inner: crate::TypeInner::Vector { size, scalar },
                        },
                        Span::UNDEFINED,
                    );
                    let vec_int_ty = self.types.insert(
                        crate::Type {
                            name: None,
                            inner: crate::TypeInner::Vector {
                                size,
                                scalar: crate::Scalar {
                                    kind: crate::ScalarKind::Sint,
                                    width: scalar.width,
                                },
                            },
                        },
                        Span::UNDEFINED,
                    );
                    (vec_float_ty, vec_int_ty, size as u32 * scalar.width as u32)
                } else {
                    (float_ty, int_ty, scalar.width as u32)
                };

                crate::Type {
                    name: Some(name),
                    inner: crate::TypeInner::Struct {
                        members: vec![
                            crate::StructMember {
                                name: Some("fract".to_string()),
                                ty: fract_member_ty,
                                binding: None,
                                offset: 0,
                            },
                            crate::StructMember {
                                name: Some("exp".to_string()),
                                ty: exp_member_ty,
                                binding: None,
                                offset: second_offset,
                            },
                        ],
                        span: second_offset * 2,
                    },
                }
            }
        };

        let handle = self.types.insert(ty, Span::UNDEFINED);
        self.special_types
            .predeclared_types
            .insert(special_type, handle);
        handle
    }
}
