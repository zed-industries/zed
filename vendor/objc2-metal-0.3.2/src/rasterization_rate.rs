use crate::{MTLRasterizationRateLayerDescriptor, MTLRasterizationRateMapDescriptor};
use objc2::extern_methods;

impl MTLRasterizationRateLayerDescriptor {
    extern_methods!(
        /// The number of quality samples that this descriptor uses to
        /// describe its current function, for the horizontal and vertical
        /// axis. The depth component of the returned `MTLSize` is always 0.
        #[cfg(feature = "MTLTypes")]
        #[unsafe(method(sampleCount))]
        pub fn sampleCount(&self) -> crate::MTLSize;
    );
}

impl MTLRasterizationRateMapDescriptor {
    extern_methods!(
        #[cfg(feature = "MTLTypes")]
        /// Convenience descriptor creation function for an arbitrary amount of layers stored in a C-array.
        ///
        /// Parameter `screenSize`: The dimensions, in screen space pixels, of the region where variable rasterization is applied. The depth component of MTLSize is ignored.
        ///
        /// Parameter `layerCount`: The number of layers in the descriptor.
        ///
        /// Parameter `layers`: An array of pointers to layer descriptors. The array must contain layerCount non-null pointers to MTLRasterizationRateLayerDescriptor instances.
        ///
        /// Returns: A descriptor containing all the specified layers. Add or remove layers using setObject:atIndexedSubscript:.
        ///
        /// The function copies the array of pointers internally, the caller need not keep the array alive after creating the descriptor.
        #[unsafe(method(rasterizationRateMapDescriptorWithScreenSize:layerCount:layers:))]
        pub unsafe fn rasterizationRateMapDescriptorWithScreenSize_layerCount_layers(
            screen_size: crate::MTLSize,
            layer_count: objc2_foundation::NSUInteger,
            layers: core::ptr::NonNull<core::ptr::NonNull<MTLRasterizationRateLayerDescriptor>>,
        ) -> objc2::rc::Retained<MTLRasterizationRateMapDescriptor>;
    );
}
