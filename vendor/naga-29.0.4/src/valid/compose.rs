use crate::arena::Handle;
use crate::proc::TypeResolution;

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ComposeError {
    #[error("Composing of type {0:?} can't be done")]
    Type(Handle<crate::Type>),
    #[error("Composing expects {expected} components but {given} were given")]
    ComponentCount { given: u32, expected: u32 },
    #[error("Composing {index}'s component type is not expected")]
    ComponentType { index: u32 },
}

pub fn validate_compose(
    self_ty_handle: Handle<crate::Type>,
    gctx: crate::proc::GlobalCtx,
    component_resolutions: impl ExactSizeIterator<Item = TypeResolution>,
) -> Result<(), ComposeError> {
    use crate::TypeInner as Ti;

    match gctx.types[self_ty_handle].inner {
        // vectors are composed from scalars or other vectors
        Ti::Vector { size, scalar } => {
            let mut total = 0;
            for (index, comp_res) in component_resolutions.enumerate() {
                total += match *comp_res.inner_with(gctx.types) {
                    Ti::Scalar(comp_scalar) if comp_scalar == scalar => 1,
                    Ti::Vector {
                        size: comp_size,
                        scalar: comp_scalar,
                    } if comp_scalar == scalar => comp_size as u32,
                    ref other => {
                        log::error!(
                            "Vector component[{index}] type {other:?}, building {scalar:?}"
                        );
                        return Err(ComposeError::ComponentType {
                            index: index as u32,
                        });
                    }
                };
            }
            if size as u32 != total {
                return Err(ComposeError::ComponentCount {
                    expected: size as u32,
                    given: total,
                });
            }
        }
        // matrix are composed from column vectors
        Ti::Matrix {
            columns,
            rows,
            scalar,
        } => {
            let inner = Ti::Vector { size: rows, scalar };
            if columns as usize != component_resolutions.len() {
                return Err(ComposeError::ComponentCount {
                    expected: columns as u32,
                    given: component_resolutions.len() as u32,
                });
            }
            for (index, comp_res) in component_resolutions.enumerate() {
                if comp_res.inner_with(gctx.types) != &inner {
                    log::error!("Matrix component[{index}] type {comp_res:?}");
                    return Err(ComposeError::ComponentType {
                        index: index as u32,
                    });
                }
            }
        }
        Ti::Array {
            base,
            size: crate::ArraySize::Constant(count),
            stride: _,
        } => {
            if count.get() as usize != component_resolutions.len() {
                return Err(ComposeError::ComponentCount {
                    expected: count.get(),
                    given: component_resolutions.len() as u32,
                });
            }
            for (index, comp_res) in component_resolutions.enumerate() {
                if !gctx.compare_types(&TypeResolution::Handle(base), &comp_res) {
                    log::error!("Array component[{index}] type {comp_res:?}");
                    return Err(ComposeError::ComponentType {
                        index: index as u32,
                    });
                }
            }
        }
        Ti::Struct { ref members, .. } => {
            if members.len() != component_resolutions.len() {
                return Err(ComposeError::ComponentCount {
                    given: component_resolutions.len() as u32,
                    expected: members.len() as u32,
                });
            }
            for (index, (member, comp_res)) in members.iter().zip(component_resolutions).enumerate()
            {
                if !gctx.compare_types(&TypeResolution::Handle(member.ty), &comp_res) {
                    log::error!("Struct component[{index}] type {comp_res:?}");
                    return Err(ComposeError::ComponentType {
                        index: index as u32,
                    });
                }
            }
        }
        ref other => {
            log::error!("Composing of {other:?}");
            return Err(ComposeError::Type(self_ty_handle));
        }
    }

    Ok(())
}
