use core::convert::TryInto;

use super::error::Error;

pub(super) const fn map_binary_operator(word: spirv::Op) -> Result<crate::BinaryOperator, Error> {
    use crate::BinaryOperator;
    use spirv::Op;

    match word {
        // Arithmetic Instructions +, -, *, /, %
        Op::IAdd | Op::FAdd => Ok(BinaryOperator::Add),
        Op::ISub | Op::FSub => Ok(BinaryOperator::Subtract),
        Op::IMul | Op::FMul => Ok(BinaryOperator::Multiply),
        Op::UDiv | Op::SDiv | Op::FDiv => Ok(BinaryOperator::Divide),
        Op::SRem => Ok(BinaryOperator::Modulo),
        // Relational and Logical Instructions
        Op::IEqual | Op::FOrdEqual | Op::FUnordEqual | Op::LogicalEqual => {
            Ok(BinaryOperator::Equal)
        }
        Op::INotEqual | Op::FOrdNotEqual | Op::FUnordNotEqual | Op::LogicalNotEqual => {
            Ok(BinaryOperator::NotEqual)
        }
        Op::ULessThan | Op::SLessThan | Op::FOrdLessThan | Op::FUnordLessThan => {
            Ok(BinaryOperator::Less)
        }
        Op::ULessThanEqual
        | Op::SLessThanEqual
        | Op::FOrdLessThanEqual
        | Op::FUnordLessThanEqual => Ok(BinaryOperator::LessEqual),
        Op::UGreaterThan | Op::SGreaterThan | Op::FOrdGreaterThan | Op::FUnordGreaterThan => {
            Ok(BinaryOperator::Greater)
        }
        Op::UGreaterThanEqual
        | Op::SGreaterThanEqual
        | Op::FOrdGreaterThanEqual
        | Op::FUnordGreaterThanEqual => Ok(BinaryOperator::GreaterEqual),
        Op::BitwiseOr => Ok(BinaryOperator::InclusiveOr),
        Op::BitwiseXor => Ok(BinaryOperator::ExclusiveOr),
        Op::BitwiseAnd => Ok(BinaryOperator::And),
        _ => Err(Error::UnknownBinaryOperator(word)),
    }
}

pub(super) const fn map_relational_fun(
    word: spirv::Op,
) -> Result<crate::RelationalFunction, Error> {
    use crate::RelationalFunction as Rf;
    use spirv::Op;

    match word {
        Op::All => Ok(Rf::All),
        Op::Any => Ok(Rf::Any),
        Op::IsNan => Ok(Rf::IsNan),
        Op::IsInf => Ok(Rf::IsInf),
        _ => Err(Error::UnknownRelationalFunction(word)),
    }
}

pub(super) const fn map_vector_size(word: spirv::Word) -> Result<crate::VectorSize, Error> {
    match word {
        2 => Ok(crate::VectorSize::Bi),
        3 => Ok(crate::VectorSize::Tri),
        4 => Ok(crate::VectorSize::Quad),
        _ => Err(Error::InvalidVectorSize(word)),
    }
}

pub(super) fn map_image_dim(word: spirv::Word) -> Result<crate::ImageDimension, Error> {
    use spirv::Dim as D;
    match D::from_u32(word) {
        Some(D::Dim1D) => Ok(crate::ImageDimension::D1),
        Some(D::Dim2D) => Ok(crate::ImageDimension::D2),
        Some(D::Dim3D) => Ok(crate::ImageDimension::D3),
        Some(D::DimCube) => Ok(crate::ImageDimension::Cube),
        _ => Err(Error::UnsupportedImageDim(word)),
    }
}

pub(super) fn map_image_format(word: spirv::Word) -> Result<crate::StorageFormat, Error> {
    match spirv::ImageFormat::from_u32(word) {
        Some(spirv::ImageFormat::R8) => Ok(crate::StorageFormat::R8Unorm),
        Some(spirv::ImageFormat::R8Snorm) => Ok(crate::StorageFormat::R8Snorm),
        Some(spirv::ImageFormat::R8ui) => Ok(crate::StorageFormat::R8Uint),
        Some(spirv::ImageFormat::R8i) => Ok(crate::StorageFormat::R8Sint),
        Some(spirv::ImageFormat::R16) => Ok(crate::StorageFormat::R16Unorm),
        Some(spirv::ImageFormat::R16Snorm) => Ok(crate::StorageFormat::R16Snorm),
        Some(spirv::ImageFormat::R16ui) => Ok(crate::StorageFormat::R16Uint),
        Some(spirv::ImageFormat::R16i) => Ok(crate::StorageFormat::R16Sint),
        Some(spirv::ImageFormat::R16f) => Ok(crate::StorageFormat::R16Float),
        Some(spirv::ImageFormat::Rg8) => Ok(crate::StorageFormat::Rg8Unorm),
        Some(spirv::ImageFormat::Rg8Snorm) => Ok(crate::StorageFormat::Rg8Snorm),
        Some(spirv::ImageFormat::Rg8ui) => Ok(crate::StorageFormat::Rg8Uint),
        Some(spirv::ImageFormat::Rg8i) => Ok(crate::StorageFormat::Rg8Sint),
        Some(spirv::ImageFormat::R32ui) => Ok(crate::StorageFormat::R32Uint),
        Some(spirv::ImageFormat::R32i) => Ok(crate::StorageFormat::R32Sint),
        Some(spirv::ImageFormat::R32f) => Ok(crate::StorageFormat::R32Float),
        Some(spirv::ImageFormat::Rg16) => Ok(crate::StorageFormat::Rg16Unorm),
        Some(spirv::ImageFormat::Rg16Snorm) => Ok(crate::StorageFormat::Rg16Snorm),
        Some(spirv::ImageFormat::Rg16ui) => Ok(crate::StorageFormat::Rg16Uint),
        Some(spirv::ImageFormat::Rg16i) => Ok(crate::StorageFormat::Rg16Sint),
        Some(spirv::ImageFormat::Rg16f) => Ok(crate::StorageFormat::Rg16Float),
        Some(spirv::ImageFormat::Rgba8) => Ok(crate::StorageFormat::Rgba8Unorm),
        Some(spirv::ImageFormat::Rgba8Snorm) => Ok(crate::StorageFormat::Rgba8Snorm),
        Some(spirv::ImageFormat::Rgba8ui) => Ok(crate::StorageFormat::Rgba8Uint),
        Some(spirv::ImageFormat::Rgba8i) => Ok(crate::StorageFormat::Rgba8Sint),
        Some(spirv::ImageFormat::Rgb10a2ui) => Ok(crate::StorageFormat::Rgb10a2Uint),
        Some(spirv::ImageFormat::Rgb10A2) => Ok(crate::StorageFormat::Rgb10a2Unorm),
        Some(spirv::ImageFormat::R11fG11fB10f) => Ok(crate::StorageFormat::Rg11b10Ufloat),
        Some(spirv::ImageFormat::R64ui) => Ok(crate::StorageFormat::R64Uint),
        Some(spirv::ImageFormat::Rg32ui) => Ok(crate::StorageFormat::Rg32Uint),
        Some(spirv::ImageFormat::Rg32i) => Ok(crate::StorageFormat::Rg32Sint),
        Some(spirv::ImageFormat::Rg32f) => Ok(crate::StorageFormat::Rg32Float),
        Some(spirv::ImageFormat::Rgba16) => Ok(crate::StorageFormat::Rgba16Unorm),
        Some(spirv::ImageFormat::Rgba16Snorm) => Ok(crate::StorageFormat::Rgba16Snorm),
        Some(spirv::ImageFormat::Rgba16ui) => Ok(crate::StorageFormat::Rgba16Uint),
        Some(spirv::ImageFormat::Rgba16i) => Ok(crate::StorageFormat::Rgba16Sint),
        Some(spirv::ImageFormat::Rgba16f) => Ok(crate::StorageFormat::Rgba16Float),
        Some(spirv::ImageFormat::Rgba32ui) => Ok(crate::StorageFormat::Rgba32Uint),
        Some(spirv::ImageFormat::Rgba32i) => Ok(crate::StorageFormat::Rgba32Sint),
        Some(spirv::ImageFormat::Rgba32f) => Ok(crate::StorageFormat::Rgba32Float),
        _ => Err(Error::UnsupportedImageFormat(word)),
    }
}

pub(super) fn map_width(word: spirv::Word) -> Result<crate::Bytes, Error> {
    (word >> 3) // bits to bytes
        .try_into()
        .map_err(|_| Error::InvalidTypeWidth(word))
}

pub(super) fn map_builtin(word: spirv::Word, invariant: bool) -> Result<crate::BuiltIn, Error> {
    use spirv::BuiltIn as Bi;
    Ok(match spirv::BuiltIn::from_u32(word) {
        Some(Bi::Position | Bi::FragCoord) => crate::BuiltIn::Position { invariant },
        Some(Bi::ViewIndex) => crate::BuiltIn::ViewIndex,
        // vertex
        Some(Bi::BaseInstance) => crate::BuiltIn::BaseInstance,
        Some(Bi::BaseVertex) => crate::BuiltIn::BaseVertex,
        Some(Bi::ClipDistance) => crate::BuiltIn::ClipDistance,
        Some(Bi::CullDistance) => crate::BuiltIn::CullDistance,
        Some(Bi::InstanceIndex) => crate::BuiltIn::InstanceIndex,
        Some(Bi::PointSize) => crate::BuiltIn::PointSize,
        Some(Bi::VertexIndex) => crate::BuiltIn::VertexIndex,
        Some(Bi::DrawIndex) => crate::BuiltIn::DrawIndex,
        // fragment
        Some(Bi::FragDepth) => crate::BuiltIn::FragDepth,
        Some(Bi::PointCoord) => crate::BuiltIn::PointCoord,
        Some(Bi::FrontFacing) => crate::BuiltIn::FrontFacing,
        Some(Bi::PrimitiveId) => crate::BuiltIn::PrimitiveIndex,
        Some(Bi::BaryCoordKHR) => crate::BuiltIn::Barycentric { perspective: true },
        Some(Bi::BaryCoordNoPerspKHR) => crate::BuiltIn::Barycentric { perspective: false },
        Some(Bi::SampleId) => crate::BuiltIn::SampleIndex,
        Some(Bi::SampleMask) => crate::BuiltIn::SampleMask,
        // compute
        Some(Bi::GlobalInvocationId) => crate::BuiltIn::GlobalInvocationId,
        Some(Bi::LocalInvocationId) => crate::BuiltIn::LocalInvocationId,
        Some(Bi::LocalInvocationIndex) => crate::BuiltIn::LocalInvocationIndex,
        Some(Bi::WorkgroupId) => crate::BuiltIn::WorkGroupId,
        Some(Bi::WorkgroupSize) => crate::BuiltIn::WorkGroupSize,
        Some(Bi::NumWorkgroups) => crate::BuiltIn::NumWorkGroups,
        // subgroup
        Some(Bi::NumSubgroups) => crate::BuiltIn::NumSubgroups,
        Some(Bi::SubgroupId) => crate::BuiltIn::SubgroupId,
        Some(Bi::SubgroupSize) => crate::BuiltIn::SubgroupSize,
        Some(Bi::SubgroupLocalInvocationId) => crate::BuiltIn::SubgroupInvocationId,
        _ => return Err(Error::UnsupportedBuiltIn(word)),
    })
}

pub(super) fn map_storage_class(word: spirv::Word) -> Result<super::ExtendedClass, Error> {
    use super::ExtendedClass as Ec;
    use spirv::StorageClass as Sc;
    Ok(match Sc::from_u32(word) {
        Some(Sc::Function) => Ec::Global(crate::AddressSpace::Function),
        Some(Sc::Input) => Ec::Input,
        Some(Sc::Output) => Ec::Output,
        Some(Sc::Private) => Ec::Global(crate::AddressSpace::Private),
        Some(Sc::UniformConstant) => Ec::Global(crate::AddressSpace::Handle),
        Some(Sc::StorageBuffer) => Ec::Global(crate::AddressSpace::Storage {
            //Note: this is restricted by decorations later
            access: crate::StorageAccess::LOAD | crate::StorageAccess::STORE,
        }),
        // we expect the `Storage` case to be filtered out before calling this function.
        Some(Sc::Uniform) => Ec::Global(crate::AddressSpace::Uniform),
        Some(Sc::Workgroup) => Ec::Global(crate::AddressSpace::WorkGroup),
        Some(Sc::PushConstant) => Ec::Global(crate::AddressSpace::Immediate),
        _ => return Err(Error::UnsupportedStorageClass(word)),
    })
}
