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

#if !defined(__LIB_EGL_ABI_H)
#define __LIB_EGL_ABI_H

#include <stdint.h>
#include <EGL/egl.h>
#include <EGL/eglext.h>

#include "glvnd/GLdispatchABI.h"

#if defined(__cplusplus)
extern "C" {
#endif

/*!
 * \defgroup eglvendorabi EGL Vendor ABI
 *
 * Definition of ABI exported by libEGL.so to libEGL_VENDOR.so libraries.
 *
 * Each vendor is associated with three distinct dispatch table types:
 *
 * - static EGL dispatch table: this is the fixed list of EGL 1.5 entrypoints
 *   provided by the vendor at load time during the initial handshake.
 * - dynamic EGL dispatch table: this is a structure allocated by the API
 *   library at runtime used to manage EGL extension functions which are not
 *   present in the static table.
 * - core GL dispatch table: this is a structure maintained by the API library
 *   which contains both GL core (static) and GL extension (dynamic) functions.
 *
 * @{
 */

/*
 * Rendering API handling:
 *
 * libEGL only supports OpenGL and OpenGL ES, not OpenVG. If OpenVG or any
 * other API is added, then the major version number will be incremented.
 *
 * When the application calls eglBindAPI, libEGL will forward the call to every
 * vendor library. In addition, a vendor library can query the current API from
 * libEGL using the getCurrentApi callback.
 *
 * Vendor libraries are not required to support both GL and GLES, but they must
 * be able to deal with either one as the current rendering API. If a vendor
 * doesn't support the current API, then it should return an error from
 * eglCreateContext.
 */

/*!
 * Current version of the ABI.
 *
 * This version number contains a major number in the high-order 16 bits, and
 * a minor version number in the low-order 16 bits.
 *
 * The major version number is incremented when an interface change will break
 * backwards compatibility with existing vendor libraries. The minor version
 * number is incremented when there's a change but existing vendor libraries
 * will still work.
 */
#define EGL_VENDOR_ABI_MAJOR_VERSION ((uint32_t) 0)
#define EGL_VENDOR_ABI_MINOR_VERSION ((uint32_t) 1)
#define EGL_VENDOR_ABI_VERSION ((EGL_VENDOR_ABI_MAJOR_VERSION << 16) | EGL_VENDOR_ABI_MINOR_VERSION)
static inline uint32_t EGL_VENDOR_ABI_GET_MAJOR_VERSION(uint32_t version)
{
    return version >> 16;
}
static inline uint32_t EGL_VENDOR_ABI_GET_MINOR_VERSION(uint32_t version)
{
    return version & 0xFFFF;
}


/*!
 * This opaque structure stores function pointers for EGL extension functions.
 * It is allocated at runtime by the API library. Vendor-provided dispatch
 * functions retrieve and operate on this structure using the API below.
 */
typedef struct __EGLvendorInfoRec __EGLvendorInfo;

/****************************************************************************
 * API library exports                                                      *
 ****************************************************************************/

typedef struct __EGLapiExportsRec {
    /************************************************************************
     * The following routines are used by vendor-implemented EGL dispatch
     * functions to lookup and call into the right vendor.
     ************************************************************************/

    /*!
     * This function must be called at the start of every EGL dispatch stub. It
     * performs any necessary per-call bookkeeping.
     */
    void (* threadInit) (void);

    EGLenum (* getCurrentApi)(void);

    /*!
     * This function retrieves the appropriate current dynamic dispatch table,
     * if a GL context is current. Otherwise, this returns NULL.
     */
    __EGLvendorInfo *(*getCurrentVendor)(void);

    /*!
     * This retrieves the current context for this thread.
     */
    EGLContext (*getCurrentContext)(void);

    /*!
     * Returns the current display for this thread.
     */
    EGLDisplay (*getCurrentDisplay)(void);

    /*!
     * Returns the current drawable for this thread.
     */
    EGLSurface (*getCurrentSurface)(EGLint readDraw);

    /*!
     * This function retrieves an entry point from the dynamic dispatch table
     * given an index into the table.
     */
    __eglMustCastToProperFunctionPointerType (*fetchDispatchEntry) (__EGLvendorInfo *dynDispatch, int index);

    /*!
     * Sets the last error for the current thread. The error will be returned
     * the next time the app calls eglGetError().
     *
     * This function will override a call to \c setLastVendor, and vice-versa.
     */
    void (* setEGLError) (EGLint errorCode);

    /*!
     * Notifies libEGL about the vendor library that an EGL function is
     * dispatched to. This is used to look up the last EGL error code from a
     * vendor.
     *
     * A vendor library may generate an EGL error asynchronously, if it
     * offloads some EGL functions onto a worker thread. In that case, calling
     * \c setEGLError to set the last error would not work.
     *
     * When an EGL dispatch stubs finds the vendor library, it must call this
     * function before it dispatches the function call.
     *
     * This function will override a call to \c setEGLError, and vice-versa.
     *
     * \param vendor The vendor library of the current EGL function call.
     * \return EGL_TRUE on success, or EGL_FALSE on failure.
     */
    EGLBoolean (* setLastVendor) (__EGLvendorInfo *vendor);

    /************************************************************************
     * These routines are used by vendor dispatch functions to look up
     * and add mappings between various objects and vendors.
     ************************************************************************/

    // As far as I can tell, all EGL functions that take a context or drawable
    // also take an EGLDisplay. So, we don't need any extra functions for
    // mapping a context/surface/whatever to a EGLDisplay or vendor.

    /*!
     * Returns the EGL vendor for an EGLDisplay handle.
     */
    __EGLvendorInfo *(*getVendorFromDisplay)(EGLDisplay dpy);

    /*!
     * Returns the EGL vendor for an EGLDeviceEXT handle.
     */
    __EGLvendorInfo *(*getVendorFromDevice)(EGLDeviceEXT dev);
} __EGLapiExports;

/*****************************************************************************
 * API library imports                                                       *
 *****************************************************************************/

/*!
 * The enum values accepted by \c __EGLapiImports::getVendorString.
 */
enum
{
    /*!
     * The set of platform extensions that the vendor supports.
     *
     * Platform extensions typically just add enums to the existing
     * eglGetPlatformDisplay function, so we don't need any additional logic
     * or special handling from libEGL itself.
     *
     * As a result, vendor libraries can add these extensions without having
     * to modify libEGL to match.
     */
    __EGL_VENDOR_STRING_PLATFORM_EXTENSIONS,
};

/*!
 * This structure stores required and optional vendor library callbacks.
 */
typedef struct __EGLapiImportsRec {
    /**
     * Creates an EGLDisplay. This function is used to handle both
     * \c eglGetDisplay and \c eglGetPlatformDisplay.
     *
     * For some platform types, libEGL may be able to identify a specific
     * vendor to use.
     *
     * If libEGL can't figure out which vendor to use on its own, then it will
     * go through the list of available vendor libraries and call this function
     * until one succeeds.
     *
     * If the application calls eglGetPlatformDisplay, then the parameters are
     * passed through from the application.
     *
     * If the application calls eglGetDisplay, and the native display handle is
     * not \c EGL_DEFAULT_DISPLAY, then libEGL will try to guess a platform
     * type, and will pass that platform to the vendor library.
     *
     * Lastly, if eglGetDisplay is called with \c EGL_DEFAULT_DISPLAY, then
     * libEGL will call into each vendor library with \p platform set to
     * \c EGL_NONE. The vendor library can then select a default display to
     * return.
     *
     * In all cases, if the vendor library can't find a matching EGLDisplay,
     * then it should return \c EGL_NO_DISPLAY. Any errors should be reported
     * through the vendor's \c eglGetError function.
     */
    EGLDisplay (* getPlatformDisplay) (EGLenum platform, void *nativeDisplay,
            const EGLAttrib *attrib_list);

    /*!
     * Checks if the vendor library supports a given client API (that is, the
     * API value passed to \c eglBindAPI).
     */
    EGLBoolean (* getSupportsAPI) (EGLenum api);

    /*!
     * (OPTIONAL) Returns a string from the vendor library. This is used for
     * anything that isn't available from eglQueryString.
     */
    const char * (* getVendorString) (int name);

    /*
     * NOTE: libEGL will load all of the vendor libraries up front, so it
     * handles extension functions slightly differently than EGL.
     */

    /*!
     * This retrieves the pointer to the real EGL or core GL function.
     *
     * \param procName The name of the function.
     * \return A pointer to a function, or \c NULL if the vendor does not
     * support the function.
     */
    void * (* getProcAddress) (const char *procName);

    /*!
     * This retrieves vendor-neutral functions which use the
     * __EGLdispatchTableDynamic API above to dispatch to the correct vendor.
     *
     * A vendor library must provide a dispatch function for all EGL display
     * extension functions that it supports.
     *
     * Client extension functions cannot be dispatched based on an EGDisplay,
     * so they must be handled in libEGL itself.
     *
     * \param procName The name of the function.
     * \return A pointer to a function, or \c NULL if the vendor does not
     * support the function or \p procName is not a EGL display function.
     */
    void * (*getDispatchAddress) (const char *procName);

    /*!
     * This notifies the vendor library which dispatch table index is
     * assigned to a particular EGL extension function.
     */
    void (*setDispatchIndex) (const char *procName, int index);

    /*
     * The vendor library may use the isPatchSupported, initiatePatch,
     * releasePatch, and patchThreadAttach callbacks to re-write libglvnd's
     * entrypoints at make current time, provided no other contexts are current
     * and the TLS model supports this functionality. This is a performance
     * optimization that may not be available at runtime; the vendor library
     * must not depend on this functionality for correctness.
     *
     * To use this optimization, the vendor library must provide at least the
     * isPatchSupported and initiatePatch entrypoints.
     *
     * Note that these functions are identical to the ones used in GLX.
     */

    /*!
     * (OPTIONAL) Checks to see if the vendor library supports patching the
     * given stub type and size.
     *
     * \param type The type of entrypoints. This will be a one of the
     * __GLDISPATCH_STUB_* values.
     * \param stubSize The maximum size of the stub that the vendor library can
     * write, in bytes.
     * \param lookupStubOffset A callback into libglvnd to look up the address
     * of each entrypoint.
     */
    GLboolean (* isPatchSupported)(int type, int stubSize);

    /*!
     * (OPTIONAL) Called by libglvnd to request that a vendor library patch its
     * top-level entrypoints.
     *
     * The vendor library should use the \p lookupStubOffset callback to find
     * the addresses of each entrypoint.
     *
     * This function may be called more than once to patch multiple sets of
     * entrypoints. For example, depending on how they're built, libOpenGL.so
     * or libGL.so may have their own entrypoints that are separate functions
     * from the ones in libGLdispatch.
     *
     * Note that during this call is the only time that the entrypoints can be
     * modified. After the call to \c initiatePatch returns, the vendor library
     * should treat the entrypoints as read-only.
     *
     * \param type The type of entrypoints. This will be a one of the
     * __GLDISPATCH_STUB_* values.
     * \param stubSize The maximum size of the stub that the vendor library can
     * write, in bytes.
     * \param lookupStubOffset A callback into libglvnd to look up the address
     * of each entrypoint.
     *
     * \return GL_TRUE if the vendor library supports patching with this type
     * and size.
     */
    GLboolean (*initiatePatch)(int type,
                               int stubSize,
                               DispatchPatchLookupStubOffset lookupStubOffset);

    /*!
     * (OPTIONAL) Called by libglvnd to notify the current vendor that it no
     * longer owns the top-level entrypoints.
     *
     * Libglvnd will take care of the restoring the entrypoints back to their
     * original state. The vendor library must not try to modify them.
     */
    void (*releasePatch)(void);

    /*!
     * (OPTIONAL) Called at the start of window-system functions (GLX and EGL).
     * This callback allows vendor libraries to perform any per-thread
     * initialization.
     *
     * This is basically a workaround for broken applications. A lot of apps
     * will make one or more invalid GLX/EGL calls on a thread (often including
     * a MakeCurrent with invalid parameters), and then will try to call an
     * OpenGL function.
     *
     * A non-libglvnd-based driver would be able to initialize any thread state
     * even on a bogus GLX call, but with libglvnd, those calls wouldn't get
     * past libGLX.
     *
     * This function is optional. If it's \c NULL, then libGLdispatch will
     * simply ignore it.
     *
     * \note This function may be called concurrently from multiple threads.
     */
    void (*patchThreadAttach)(void);

    /*!
     * (OPTIONAL) Tries to determine the platform type for a native display.
     *
     * If the vendor library provides this function, then libglvnd will call it
     * to determine which platform to use for a native display handle in
     * eglGetDisplay.
     *
     * If no vendor library identifies the platform, then libglvnd will fall
     * back to its own platform detection logic.
     *
     * Libglvnd can call this function for any native display handle except
     * \c EGL_DEFAULT_DISPLAY.
     *
     * No matter what the value of \p native_display, the vendor library must
     * not crash, and must not return a false match. If the vendor library
     * can't identify the display, then it must return \c EGL_NONE.
     *
     * In particular, that means that a vendor library must not return any sort
     * of default or fallback platform.
     *
     * \param native_display The native display handle passed to eglGetDisplay.
     * \return Either a platform type enum or EGL_NONE.
     */
    EGLenum (* findNativeDisplayPlatform) (void *native_display);
} __EGLapiImports;

/*****************************************************************************/

#define __EGL_MAIN_PROTO_NAME "__egl_Main"

typedef EGLBoolean (* __PFNEGLMAINPROC) (uint32_t version, const __EGLapiExports *exports,
        __EGLvendorInfo *vendor, __EGLapiImports *imports);

/*!
 * Vendor libraries must export a function called __egl_Main() with the
 * following prototype.
 *
 * This function also performs a handshake based on the ABI version number.
 * Vendor libraries can optionally use the version number to support older
 * versions of the ABI.
 *
 * \param[in] version The ABI version. The upper 16 bits contains the major version
 * number, and the lower 16 bits contains the minor version number.
 *
 * \param[in] exports The table of functions provided by libEGL. This pointer will
 * remain valid for as long as the vendor is loaded.
 *
 * \param[in] vendor The opaque pointer used to identify this vendor library. This
 * may be used in future versions to provide additional per-vendor information.
 *
 * \param[out] imports The function table that the vendor library should fill
 * in. The vendor library must assign every non-optional function in the
 * struct.
 *
 * \return True on success. If the vendor library does not support the
 * requested ABI version or if some other error occurs, then it should return
 * False.
 */
EGLBoolean __egl_Main(uint32_t version, const __EGLapiExports *exports,
        __EGLvendorInfo *vendor, __EGLapiImports *imports);

/*!
 * @}
 */

#if defined(__cplusplus)
}
#endif

#endif /* __LIB_EGL_ABI_H */
