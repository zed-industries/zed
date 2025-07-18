The files in this folder are required for the Windows platform support in the gpui library.


#### amd_ags_x64.lib & amd_ags_x86.lib

These libraries are used for AMD GPU support, currently only used on Windows, mainly for retrieving GPU information and capabilities. They are linked against the AMD AGS (AMD GPU Services) library.

The official AMD AGS documentation can be found [here](https://gpuopen.com/amd-gpu-services-ags-library). And these two files are grabbed from the [official AMD AGS repository](https://github.com/GPUOpen-LibrariesAndSDKs/AGS_SDK), currently at version 6.3.0.

If you want to update these files, don't forget to update the value of `AGS_CURRENT_VERSION` in `gpui/src/platform/windows/directx_renderer.rs` as well.
