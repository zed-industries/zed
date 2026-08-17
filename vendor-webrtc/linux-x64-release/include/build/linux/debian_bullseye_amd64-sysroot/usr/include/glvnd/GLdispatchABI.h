/*
 * Copyright (c) 2013, NVIDIA CORPORATION.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and/or associated documentation files (the
 * "Materials"), to deal in the Materials without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Materials, and to
 * permit persons to whom the Materials are furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * unaltered in all copies or substantial portions of the Materials.
 * Any additions, deletions, or changes to the original source files
 * must be clearly indicated in accompanying documentation.
 *
 * If only executable code is distributed, then the accompanying
 * documentation must state that "this software is based in part on the
 * work of the Khronos Group."
 *
 * THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * MATERIALS OR THE USE OR OTHER DEALINGS IN THE MATERIALS.
 */

#include <GL/gl.h>

#if !defined(__GL_DISPATCH_ABI_H)
#define __GL_DISPATCH_ABI_H

#if defined(__cplusplus)
extern "C" {
#endif

/*!
 * \defgroup gldispatchabi GL dispatching ABI
 *
 * This is not a complete ABI, but rather a fragment common to the libEGL and
 * libGLX ABIs.  Changes to this file should be accompanied by a version bump to
 * these client ABIs.
 */

/*!
 * Thread-local implementation used by libglvnd. This is passed into the patch
 * function callback via the type parameter.
 *
 * For most architectures, the vendor library can ignore this parameter, since
 * it will always be the same value. It's used for systems like ARM, where the
 * stubs might be use the ARM or Thumb instruction sets.
 *
 * The stub type does not make any distinction between TLS and TSD stubs. The
 * entire purpose of entrypoint rewriting is to skip the dispatch table in
 * libGLdispatch.so, so it doesn't matter how that dispatch table is stored.
 */
enum {
    /*!
     * Indicates that the stubs aren't defined in assembly. For example, if the
     * dispatch stubs are written in C. Vendor libraries generally won't see
     * this value.
     */
    __GLDISPATCH_STUB_UNKNOWN,

    /*!
     * Used for stubs on x86 systems.
     */
    __GLDISPATCH_STUB_X86,

    /*!
     * Used for stubs on x86-64 systems.
     */
    __GLDISPATCH_STUB_X86_64,

    /*!
     * Used for stubs on ARMv7, using the Thumb instruction set.
     */
    __GLDISPATCH_STUB_ARMV7_THUMB,

    /*!
     * Used for stubs on ARMv7, using the normal ARM instruction set.
     */
    __GLDISPATCH_STUB_ARMV7_ARM,

    /*!
     * Used for stubs on ARMv8/aarch64.
     */
    __GLDISPATCH_STUB_AARCH64,

    /*!
     * Used for stubs on x32 builds (x86-64 with 32-bit pointers).
     */
    __GLDISPATCH_STUB_X32,

    /*!
     * Used for stubs on PPC64 systems.
     */
    __GLDISPATCH_STUB_PPC64,

    /*!
     * Used for stubs on PPC64LE systems. Same as PPC64, for compatibility.
     */
    __GLDISPATCH_STUB_PPC64LE = __GLDISPATCH_STUB_PPC64,
};

/*!
 * A callback function called by the vendor library to fetch the address of an
 * entrypoint.
 *
 * The function returns two pointers, one writable and one executable. The two
 * pointers may or may not be the same virtual address, but they will both be
 * mappings of the same physical memory.
 *
 * The vendor library should write its entrypoint to the address returned by
 * \p writePtr, but should use the address from \p execPtr for things like
 * calculating PC-relative offsets.
 *
 * Note that if this function fails, then the vendor library can still try to
 * patch other entrypoints.
 *
 * Note that on ARM, the low-order bit of both \c execPtr and \p writePtr will
 * be zero, even if the stub uses the thumb instruction set. The vendor library
 * should use the \c type parameter of \c initiatePatch to determine which
 * instruction set to use.
 *
 * \param funcName The function name.
 * \param[out] writePtr The pointer that the vendor library can write to.
 * \param[out] execPtr The pointer to the executable code.
 * \return GL_TRUE if the entrypoint exists, or GL_FALSE if it doesn't.
 */
typedef GLboolean (*DispatchPatchLookupStubOffset)(const char *funcName,
        void **writePtr, const void **execPtr);

#if defined(__cplusplus)
}
#endif

#endif // __GL_DISPATCH_ABI_H
