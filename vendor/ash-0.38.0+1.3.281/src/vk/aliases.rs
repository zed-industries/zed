use crate::vk::bitflags::*;
use crate::vk::definitions::*;
use crate::vk::enums::*;
pub type GeometryFlagsNV = GeometryFlagsKHR;
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;
pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
pub type AccessFlags2KHR = AccessFlags2;
pub type PipelineStageFlags2KHR = PipelineStageFlags2;
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;
pub type RenderingFlagsKHR = RenderingFlags;
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
pub type FenceImportFlagsKHR = FenceImportFlags;
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
pub type ResolveModeFlagsKHR = ResolveModeFlags;
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
pub type SubmitFlagsKHR = SubmitFlags;
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
pub type PrivateDataSlotEXT = PrivateDataSlot;
pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;
pub type PointClippingBehaviorKHR = PointClippingBehavior;
pub type QueueGlobalPriorityEXT = QueueGlobalPriorityKHR;
pub type TimeDomainEXT = TimeDomainKHR;
pub type SemaphoreTypeKHR = SemaphoreType;
pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;
pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;
pub type GeometryTypeNV = GeometryTypeKHR;
pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;
pub type LineRasterizationModeEXT = LineRasterizationModeKHR;
pub type ScopeNV = ScopeKHR;
pub type ComponentTypeNV = ComponentTypeKHR;
pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
pub type ChromaLocationKHR = ChromaLocation;
pub type SamplerReductionModeEXT = SamplerReductionMode;
pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;
pub type DriverIdKHR = DriverId;
pub type DevicePrivateDataCreateInfoEXT<'a> = DevicePrivateDataCreateInfo<'a>;
pub type PrivateDataSlotCreateInfoEXT<'a> = PrivateDataSlotCreateInfo<'a>;
pub type PhysicalDevicePrivateDataFeaturesEXT<'a> = PhysicalDevicePrivateDataFeatures<'a>;
pub type PhysicalDeviceFeatures2KHR<'a> = PhysicalDeviceFeatures2<'a>;
pub type PhysicalDeviceProperties2KHR<'a> = PhysicalDeviceProperties2<'a>;
pub type FormatProperties2KHR<'a> = FormatProperties2<'a>;
pub type ImageFormatProperties2KHR<'a> = ImageFormatProperties2<'a>;
pub type PhysicalDeviceImageFormatInfo2KHR<'a> = PhysicalDeviceImageFormatInfo2<'a>;
pub type QueueFamilyProperties2KHR<'a> = QueueFamilyProperties2<'a>;
pub type PhysicalDeviceMemoryProperties2KHR<'a> = PhysicalDeviceMemoryProperties2<'a>;
pub type SparseImageFormatProperties2KHR<'a> = SparseImageFormatProperties2<'a>;
pub type PhysicalDeviceSparseImageFormatInfo2KHR<'a> = PhysicalDeviceSparseImageFormatInfo2<'a>;
pub type ConformanceVersionKHR = ConformanceVersion;
pub type PhysicalDeviceDriverPropertiesKHR<'a> = PhysicalDeviceDriverProperties<'a>;
pub type PhysicalDeviceVariablePointersFeaturesKHR<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
pub type PhysicalDeviceVariablePointerFeaturesKHR<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
pub type PhysicalDeviceVariablePointerFeatures<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
pub type PhysicalDeviceExternalImageFormatInfoKHR<'a> = PhysicalDeviceExternalImageFormatInfo<'a>;
pub type ExternalImageFormatPropertiesKHR<'a> = ExternalImageFormatProperties<'a>;
pub type PhysicalDeviceExternalBufferInfoKHR<'a> = PhysicalDeviceExternalBufferInfo<'a>;
pub type ExternalBufferPropertiesKHR<'a> = ExternalBufferProperties<'a>;
pub type PhysicalDeviceIDPropertiesKHR<'a> = PhysicalDeviceIDProperties<'a>;
pub type ExternalMemoryImageCreateInfoKHR<'a> = ExternalMemoryImageCreateInfo<'a>;
pub type ExternalMemoryBufferCreateInfoKHR<'a> = ExternalMemoryBufferCreateInfo<'a>;
pub type ExportMemoryAllocateInfoKHR<'a> = ExportMemoryAllocateInfo<'a>;
pub type PhysicalDeviceExternalSemaphoreInfoKHR<'a> = PhysicalDeviceExternalSemaphoreInfo<'a>;
pub type ExternalSemaphorePropertiesKHR<'a> = ExternalSemaphoreProperties<'a>;
pub type ExportSemaphoreCreateInfoKHR<'a> = ExportSemaphoreCreateInfo<'a>;
pub type PhysicalDeviceExternalFenceInfoKHR<'a> = PhysicalDeviceExternalFenceInfo<'a>;
pub type ExternalFencePropertiesKHR<'a> = ExternalFenceProperties<'a>;
pub type ExportFenceCreateInfoKHR<'a> = ExportFenceCreateInfo<'a>;
pub type PhysicalDeviceMultiviewFeaturesKHR<'a> = PhysicalDeviceMultiviewFeatures<'a>;
pub type PhysicalDeviceMultiviewPropertiesKHR<'a> = PhysicalDeviceMultiviewProperties<'a>;
pub type RenderPassMultiviewCreateInfoKHR<'a> = RenderPassMultiviewCreateInfo<'a>;
pub type PhysicalDeviceGroupPropertiesKHR<'a> = PhysicalDeviceGroupProperties<'a>;
pub type MemoryAllocateFlagsInfoKHR<'a> = MemoryAllocateFlagsInfo<'a>;
pub type BindBufferMemoryInfoKHR<'a> = BindBufferMemoryInfo<'a>;
pub type BindBufferMemoryDeviceGroupInfoKHR<'a> = BindBufferMemoryDeviceGroupInfo<'a>;
pub type BindImageMemoryInfoKHR<'a> = BindImageMemoryInfo<'a>;
pub type BindImageMemoryDeviceGroupInfoKHR<'a> = BindImageMemoryDeviceGroupInfo<'a>;
pub type DeviceGroupRenderPassBeginInfoKHR<'a> = DeviceGroupRenderPassBeginInfo<'a>;
pub type DeviceGroupCommandBufferBeginInfoKHR<'a> = DeviceGroupCommandBufferBeginInfo<'a>;
pub type DeviceGroupSubmitInfoKHR<'a> = DeviceGroupSubmitInfo<'a>;
pub type DeviceGroupBindSparseInfoKHR<'a> = DeviceGroupBindSparseInfo<'a>;
pub type DeviceGroupDeviceCreateInfoKHR<'a> = DeviceGroupDeviceCreateInfo<'a>;
pub type DescriptorUpdateTemplateEntryKHR = DescriptorUpdateTemplateEntry;
pub type DescriptorUpdateTemplateCreateInfoKHR<'a> = DescriptorUpdateTemplateCreateInfo<'a>;
pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;
pub type RenderPassInputAttachmentAspectCreateInfoKHR<'a> =
    RenderPassInputAttachmentAspectCreateInfo<'a>;
pub type PhysicalDevice16BitStorageFeaturesKHR<'a> = PhysicalDevice16BitStorageFeatures<'a>;
pub type PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR<'a> =
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a>;
pub type BufferMemoryRequirementsInfo2KHR<'a> = BufferMemoryRequirementsInfo2<'a>;
pub type DeviceBufferMemoryRequirementsKHR<'a> = DeviceBufferMemoryRequirements<'a>;
pub type ImageMemoryRequirementsInfo2KHR<'a> = ImageMemoryRequirementsInfo2<'a>;
pub type ImageSparseMemoryRequirementsInfo2KHR<'a> = ImageSparseMemoryRequirementsInfo2<'a>;
pub type DeviceImageMemoryRequirementsKHR<'a> = DeviceImageMemoryRequirements<'a>;
pub type MemoryRequirements2KHR<'a> = MemoryRequirements2<'a>;
pub type SparseImageMemoryRequirements2KHR<'a> = SparseImageMemoryRequirements2<'a>;
pub type PhysicalDevicePointClippingPropertiesKHR<'a> = PhysicalDevicePointClippingProperties<'a>;
pub type MemoryDedicatedRequirementsKHR<'a> = MemoryDedicatedRequirements<'a>;
pub type MemoryDedicatedAllocateInfoKHR<'a> = MemoryDedicatedAllocateInfo<'a>;
pub type ImageViewUsageCreateInfoKHR<'a> = ImageViewUsageCreateInfo<'a>;
pub type PipelineTessellationDomainOriginStateCreateInfoKHR<'a> =
    PipelineTessellationDomainOriginStateCreateInfo<'a>;
pub type SamplerYcbcrConversionInfoKHR<'a> = SamplerYcbcrConversionInfo<'a>;
pub type SamplerYcbcrConversionCreateInfoKHR<'a> = SamplerYcbcrConversionCreateInfo<'a>;
pub type BindImagePlaneMemoryInfoKHR<'a> = BindImagePlaneMemoryInfo<'a>;
pub type ImagePlaneMemoryRequirementsInfoKHR<'a> = ImagePlaneMemoryRequirementsInfo<'a>;
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR<'a> =
    PhysicalDeviceSamplerYcbcrConversionFeatures<'a>;
pub type SamplerYcbcrConversionImageFormatPropertiesKHR<'a> =
    SamplerYcbcrConversionImageFormatProperties<'a>;
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT<'a> =
    PhysicalDeviceSamplerFilterMinmaxProperties<'a>;
pub type SamplerReductionModeCreateInfoEXT<'a> = SamplerReductionModeCreateInfo<'a>;
pub type PhysicalDeviceInlineUniformBlockFeaturesEXT<'a> =
    PhysicalDeviceInlineUniformBlockFeatures<'a>;
pub type PhysicalDeviceInlineUniformBlockPropertiesEXT<'a> =
    PhysicalDeviceInlineUniformBlockProperties<'a>;
pub type WriteDescriptorSetInlineUniformBlockEXT<'a> = WriteDescriptorSetInlineUniformBlock<'a>;
pub type DescriptorPoolInlineUniformBlockCreateInfoEXT<'a> =
    DescriptorPoolInlineUniformBlockCreateInfo<'a>;
pub type ImageFormatListCreateInfoKHR<'a> = ImageFormatListCreateInfo<'a>;
pub type PhysicalDeviceMaintenance3PropertiesKHR<'a> = PhysicalDeviceMaintenance3Properties<'a>;
pub type PhysicalDeviceMaintenance4FeaturesKHR<'a> = PhysicalDeviceMaintenance4Features<'a>;
pub type PhysicalDeviceMaintenance4PropertiesKHR<'a> = PhysicalDeviceMaintenance4Properties<'a>;
pub type DescriptorSetLayoutSupportKHR<'a> = DescriptorSetLayoutSupport<'a>;
pub type PhysicalDeviceShaderDrawParameterFeatures<'a> =
    PhysicalDeviceShaderDrawParametersFeatures<'a>;
pub type PhysicalDeviceShaderFloat16Int8FeaturesKHR<'a> =
    PhysicalDeviceShaderFloat16Int8Features<'a>;
pub type PhysicalDeviceFloat16Int8FeaturesKHR<'a> = PhysicalDeviceShaderFloat16Int8Features<'a>;
pub type PhysicalDeviceFloatControlsPropertiesKHR<'a> = PhysicalDeviceFloatControlsProperties<'a>;
pub type PhysicalDeviceHostQueryResetFeaturesEXT<'a> = PhysicalDeviceHostQueryResetFeatures<'a>;
pub type DeviceQueueGlobalPriorityCreateInfoEXT<'a> = DeviceQueueGlobalPriorityCreateInfoKHR<'a>;
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT<'a> =
    PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'a>;
pub type QueueFamilyGlobalPriorityPropertiesEXT<'a> = QueueFamilyGlobalPriorityPropertiesKHR<'a>;
pub type CalibratedTimestampInfoEXT<'a> = CalibratedTimestampInfoKHR<'a>;
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT<'a> =
    PhysicalDeviceDescriptorIndexingFeatures<'a>;
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT<'a> =
    PhysicalDeviceDescriptorIndexingProperties<'a>;
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT<'a> =
    DescriptorSetLayoutBindingFlagsCreateInfo<'a>;
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT<'a> =
    DescriptorSetVariableDescriptorCountAllocateInfo<'a>;
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT<'a> =
    DescriptorSetVariableDescriptorCountLayoutSupport<'a>;
pub type AttachmentDescription2KHR<'a> = AttachmentDescription2<'a>;
pub type AttachmentReference2KHR<'a> = AttachmentReference2<'a>;
pub type SubpassDescription2KHR<'a> = SubpassDescription2<'a>;
pub type SubpassDependency2KHR<'a> = SubpassDependency2<'a>;
pub type RenderPassCreateInfo2KHR<'a> = RenderPassCreateInfo2<'a>;
pub type SubpassBeginInfoKHR<'a> = SubpassBeginInfo<'a>;
pub type SubpassEndInfoKHR<'a> = SubpassEndInfo<'a>;
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR<'a> =
    PhysicalDeviceTimelineSemaphoreFeatures<'a>;
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR<'a> =
    PhysicalDeviceTimelineSemaphoreProperties<'a>;
pub type SemaphoreTypeCreateInfoKHR<'a> = SemaphoreTypeCreateInfo<'a>;
pub type TimelineSemaphoreSubmitInfoKHR<'a> = TimelineSemaphoreSubmitInfo<'a>;
pub type SemaphoreWaitInfoKHR<'a> = SemaphoreWaitInfo<'a>;
pub type SemaphoreSignalInfoKHR<'a> = SemaphoreSignalInfo<'a>;
pub type VertexInputBindingDivisorDescriptionEXT = VertexInputBindingDivisorDescriptionKHR;
pub type PipelineVertexInputDivisorStateCreateInfoEXT<'a> =
    PipelineVertexInputDivisorStateCreateInfoKHR<'a>;
pub type PhysicalDevice8BitStorageFeaturesKHR<'a> = PhysicalDevice8BitStorageFeatures<'a>;
pub type PhysicalDeviceVulkanMemoryModelFeaturesKHR<'a> =
    PhysicalDeviceVulkanMemoryModelFeatures<'a>;
pub type PhysicalDeviceShaderAtomicInt64FeaturesKHR<'a> =
    PhysicalDeviceShaderAtomicInt64Features<'a>;
pub type PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'a> =
    PhysicalDeviceVertexAttributeDivisorFeaturesKHR<'a>;
pub type PhysicalDeviceDepthStencilResolvePropertiesKHR<'a> =
    PhysicalDeviceDepthStencilResolveProperties<'a>;
pub type SubpassDescriptionDepthStencilResolveKHR<'a> = SubpassDescriptionDepthStencilResolve<'a>;
pub type PhysicalDeviceFragmentShaderBarycentricFeaturesNV<'a> =
    PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'a>;
pub type ImageStencilUsageCreateInfoEXT<'a> = ImageStencilUsageCreateInfo<'a>;
pub type PhysicalDeviceScalarBlockLayoutFeaturesEXT<'a> =
    PhysicalDeviceScalarBlockLayoutFeatures<'a>;
pub type PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR<'a> =
    PhysicalDeviceUniformBufferStandardLayoutFeatures<'a>;
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR<'a> =
    PhysicalDeviceBufferDeviceAddressFeatures<'a>;
pub type PhysicalDeviceBufferAddressFeaturesEXT<'a> =
    PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a>;
pub type BufferDeviceAddressInfoKHR<'a> = BufferDeviceAddressInfo<'a>;
pub type BufferDeviceAddressInfoEXT<'a> = BufferDeviceAddressInfo<'a>;
pub type BufferOpaqueCaptureAddressCreateInfoKHR<'a> = BufferOpaqueCaptureAddressCreateInfo<'a>;
pub type PhysicalDeviceImagelessFramebufferFeaturesKHR<'a> =
    PhysicalDeviceImagelessFramebufferFeatures<'a>;
pub type FramebufferAttachmentsCreateInfoKHR<'a> = FramebufferAttachmentsCreateInfo<'a>;
pub type FramebufferAttachmentImageInfoKHR<'a> = FramebufferAttachmentImageInfo<'a>;
pub type RenderPassAttachmentBeginInfoKHR<'a> = RenderPassAttachmentBeginInfo<'a>;
pub type PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT<'a> =
    PhysicalDeviceTextureCompressionASTCHDRFeatures<'a>;
pub type PipelineCreationFeedbackEXT = PipelineCreationFeedback;
pub type PipelineCreationFeedbackCreateInfoEXT<'a> = PipelineCreationFeedbackCreateInfo<'a>;
pub type QueryPoolCreateInfoINTEL<'a> = QueryPoolPerformanceQueryCreateInfoINTEL<'a>;
pub type PhysicalDeviceIndexTypeUint8FeaturesEXT<'a> = PhysicalDeviceIndexTypeUint8FeaturesKHR<'a>;
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR<'a> =
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a>;
pub type AttachmentReferenceStencilLayoutKHR<'a> = AttachmentReferenceStencilLayout<'a>;
pub type AttachmentDescriptionStencilLayoutKHR<'a> = AttachmentDescriptionStencilLayout<'a>;
pub type PipelineInfoEXT<'a> = PipelineInfoKHR<'a>;
pub type PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT<'a> =
    PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a>;
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT<'a> =
    PhysicalDeviceTexelBufferAlignmentProperties<'a>;
pub type PhysicalDeviceSubgroupSizeControlFeaturesEXT<'a> =
    PhysicalDeviceSubgroupSizeControlFeatures<'a>;
pub type PhysicalDeviceSubgroupSizeControlPropertiesEXT<'a> =
    PhysicalDeviceSubgroupSizeControlProperties<'a>;
pub type PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT<'a> =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;
pub type ShaderRequiredSubgroupSizeCreateInfoEXT<'a> =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR<'a> = MemoryOpaqueCaptureAddressAllocateInfo<'a>;
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR<'a> = DeviceMemoryOpaqueCaptureAddressInfo<'a>;
pub type PhysicalDeviceLineRasterizationFeaturesEXT<'a> =
    PhysicalDeviceLineRasterizationFeaturesKHR<'a>;
pub type PhysicalDeviceLineRasterizationPropertiesEXT<'a> =
    PhysicalDeviceLineRasterizationPropertiesKHR<'a>;
pub type PipelineRasterizationLineStateCreateInfoEXT<'a> =
    PipelineRasterizationLineStateCreateInfoKHR<'a>;
pub type PhysicalDevicePipelineCreationCacheControlFeaturesEXT<'a> =
    PhysicalDevicePipelineCreationCacheControlFeatures<'a>;
pub type PhysicalDeviceToolPropertiesEXT<'a> = PhysicalDeviceToolProperties<'a>;
pub type AabbPositionsNV = AabbPositionsKHR;
pub type TransformMatrixNV = TransformMatrixKHR;
pub type AccelerationStructureInstanceNV = AccelerationStructureInstanceKHR;
pub type PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR<'a> =
    PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a>;
pub type PhysicalDeviceImageRobustnessFeaturesEXT<'a> = PhysicalDeviceImageRobustnessFeatures<'a>;
pub type BufferCopy2KHR<'a> = BufferCopy2<'a>;
pub type ImageCopy2KHR<'a> = ImageCopy2<'a>;
pub type ImageBlit2KHR<'a> = ImageBlit2<'a>;
pub type BufferImageCopy2KHR<'a> = BufferImageCopy2<'a>;
pub type ImageResolve2KHR<'a> = ImageResolve2<'a>;
pub type CopyBufferInfo2KHR<'a> = CopyBufferInfo2<'a>;
pub type CopyImageInfo2KHR<'a> = CopyImageInfo2<'a>;
pub type BlitImageInfo2KHR<'a> = BlitImageInfo2<'a>;
pub type CopyBufferToImageInfo2KHR<'a> = CopyBufferToImageInfo2<'a>;
pub type CopyImageToBufferInfo2KHR<'a> = CopyImageToBufferInfo2<'a>;
pub type ResolveImageInfo2KHR<'a> = ResolveImageInfo2<'a>;
pub type PhysicalDeviceShaderTerminateInvocationFeaturesKHR<'a> =
    PhysicalDeviceShaderTerminateInvocationFeatures<'a>;
pub type PhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'a> =
    PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a>;
pub type MutableDescriptorTypeListVALVE<'a> = MutableDescriptorTypeListEXT<'a>;
pub type MutableDescriptorTypeCreateInfoVALVE<'a> = MutableDescriptorTypeCreateInfoEXT<'a>;
pub type MemoryBarrier2KHR<'a> = MemoryBarrier2<'a>;
pub type ImageMemoryBarrier2KHR<'a> = ImageMemoryBarrier2<'a>;
pub type BufferMemoryBarrier2KHR<'a> = BufferMemoryBarrier2<'a>;
pub type DependencyInfoKHR<'a> = DependencyInfo<'a>;
pub type SemaphoreSubmitInfoKHR<'a> = SemaphoreSubmitInfo<'a>;
pub type CommandBufferSubmitInfoKHR<'a> = CommandBufferSubmitInfo<'a>;
pub type SubmitInfo2KHR<'a> = SubmitInfo2<'a>;
pub type PhysicalDeviceSynchronization2FeaturesKHR<'a> = PhysicalDeviceSynchronization2Features<'a>;
pub type PhysicalDeviceShaderIntegerDotProductFeaturesKHR<'a> =
    PhysicalDeviceShaderIntegerDotProductFeatures<'a>;
pub type PhysicalDeviceShaderIntegerDotProductPropertiesKHR<'a> =
    PhysicalDeviceShaderIntegerDotProductProperties<'a>;
pub type FormatProperties3KHR<'a> = FormatProperties3<'a>;
pub type PipelineRenderingCreateInfoKHR<'a> = PipelineRenderingCreateInfo<'a>;
pub type RenderingInfoKHR<'a> = RenderingInfo<'a>;
pub type RenderingAttachmentInfoKHR<'a> = RenderingAttachmentInfo<'a>;
pub type PhysicalDeviceDynamicRenderingFeaturesKHR<'a> = PhysicalDeviceDynamicRenderingFeatures<'a>;
pub type CommandBufferInheritanceRenderingInfoKHR<'a> = CommandBufferInheritanceRenderingInfo<'a>;
pub type AttachmentSampleCountInfoNV<'a> = AttachmentSampleCountInfoAMD<'a>;
pub type PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'a> =
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a>;
pub type ImageSubresource2EXT<'a> = ImageSubresource2KHR<'a>;
pub type SubresourceLayout2EXT<'a> = SubresourceLayout2KHR<'a>;
