# TE binaries

Binaries located in this directory are precompiled PE32/PE32+ binaries using a
terse executable (TE) header as defined in the Platform Initialization (PI)
specification: [TE](https://uefi.org/specs/PI/1.8/V1_TE_Image.html#te-header).
These binaries were compiled using the
[EDK2](https://github.com/tianocore/edk2) build system.

## test_image.te

This binary is a simple Terse executable binary

## test_image_loaded.bin

This binary is the same as `test_image.te`, but it has been loaded by a loader,
meaning the sections have been placed in the expected address. Please note that
this particular binary has not been relocated, so no relocations have been
applied

## test_image_relocated.bin

This binary is the same as `test_image.te`, but it has been loaded by a loader,
meaning the sections have been placed in the expected address, and any any
relocations have been applied.
