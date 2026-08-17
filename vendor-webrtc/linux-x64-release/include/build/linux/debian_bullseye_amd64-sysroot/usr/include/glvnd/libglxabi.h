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

#if !defined(__LIB_GLX_ABI_H)
#define __LIB_GLX_ABI_H

#include <stdint.h>
#include <GL/glx.h>

#include "glvnd/GLdispatchABI.h"

#if defined(__cplusplus)
extern "C" {
#endif

/*!
 * \defgroup glxvendorabi GLX Vendor ABI
 *
 * Definition of ABI exported by libGLX.so to libGLX_VENDOR.so libraries.
 *
 * Each vendor is associated with three distinct dispatch table types:
 *
 * - static GLX dispatch table: this is the fixed list of GLX 1.4 entrypoints
 *   provided by the vendor at load time during the initial handshake.
 * - dynamic GLX dispatch table: this is a structure allocated by the API
 *   library at runtime used to manage GLX extension functions which are not
 *   present in the static table.
 * - core GL dispatch table: this is a structure maintained by the API library
 *   which contains both GL core (static) and GL extension (dynamic) functions.
 *
 * Note that while the implementations of most GLX functions in a vendor
 * library is mostly unchanged from a traditional, single-vendor driver, libGLX
 * has additional requirements for GLXContext and GLXFBConfig handle values.
 *
 * First, all GLXContext and GLXFBConfig handles have to be unique between
 * vendor libraries. That is, every GLXContext or GLXFBConfig handle must map
 * to exactly one vendor library, so that libGLX knows which library to dispatch
 * to.
 *
 * To do that, all GLXContext and GLXFBConfig handles *must* be a pointer to an
 * address that the vendor library somehow controls. The address doesn't need
 * to be readable or writable, but it must be an address that no other vendor
 * library would use.
 *
 * The address could be a pointer to a structure, or an address in a statically
 * or dynamically allocated array. It could even be a file mapping, or even an
 * offset into wherever the vendor library itself is mapped.
 *
 * A vendor library may not, however, use anything like an index or an XID for
 * a GLXContext or GLXFBConfig handle.
 *
 * GLXContext handles must also be globally unique across all display
 * connections in the entire process. That is, a vendor library may not return
 * the same GLXContext handle for two different contexts, even if they're on
 * different displays or different servers.
 *
 * GLXFBConfigs may be duplicated between multiple displays, as long as they
 * are still unique between vendors. Some applications even depend on this:
 * They will look up a GLXFBConfig handle with one connection, and then try to
 * use that config on another connection.
 *
 * @{
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
#define GLX_VENDOR_ABI_MAJOR_VERSION ((uint32_t) 1)
#define GLX_VENDOR_ABI_MINOR_VERSION ((uint32_t) 0)
#define GLX_VENDOR_ABI_VERSION ((GLX_VENDOR_ABI_MAJOR_VERSION << 16) | GLX_VENDOR_ABI_MINOR_VERSION)
static inline uint32_t GLX_VENDOR_ABI_GET_MAJOR_VERSION(uint32_t version)
{
    return version >> 16;
}
static inline uint32_t GLX_VENDOR_ABI_GET_MINOR_VERSION(uint32_t version)
{
    return version & 0xFFFF;
}


/*!
 * This opaque structure stores function pointers for GLX extension functions.
 * It is allocated at runtime by the API library. Vendor-provided dispatch
 * functions retrieve and operate on this structure using the API below.
 */
typedef struct __GLXvendorInfoRec __GLXvendorInfo;

/****************************************************************************
 * API library exports                                                      *
 ****************************************************************************/

/*!
 * Functions exported by libGLX.so.
 *
 * These functions are exported by libGLX, and should be used by the
 * vendor-implemented dispatch functions to lookup and call into the right
 * vendor.
 *
 * These functions should only be called from the GLX dispatch functions, never
 * from the actual implementation of any function. libGLX.so may be holding a
 * non-recursive lock when it calls into the vendor library, so trying to call
 * back into libGLX could deadlock.
 */
typedef struct __GLXapiExportsRec {
    /*!
     * This fetches the appropriate dynamic GLX dispatch table given the display
     * and screen number.
     */
    __GLXvendorInfo *(*getDynDispatch)(Display *dpy,
                                                 const int screen);

    /*!
     * This function retrieves the appropriate current dynamic dispatch table,
     * if a GL context is current. Otherwise, this returns NULL.
     */
    __GLXvendorInfo *(*getCurrentDynDispatch)(void);

    /*!
     * This function retrieves an entry point from the dynamic dispatch table
     * given an index into the table.
     */
    __GLXextFuncPtr           (*fetchDispatchEntry)
        (__GLXvendorInfo *dynDispatch, int index);

    /************************************************************************
     * This routine is used by the vendor to lookup its context structure.
     * The contents of this structure are opaque to the API library and
     * vendor-dependent.
     ************************************************************************/

    /*!
     * This retrieves the current context for this thread.
     */
    GLXContext                (*getCurrentContext)(void);

    /************************************************************************
     * These routines are used by vendor dispatch functions to look up
     * and add mappings between various objects and vendors.
     ************************************************************************/

    /*!
     * Records the vendor for a context. The vendor must be the one returned
     * for the XVisualInfo or GLXFBConfig that the context is created from.
     *
     * \param dpy The display pointer.
     * \param context The context handle.
     * \param vendor The vendor that created the context.
     * \return Zero on success, non-zero on error.
     */
    int (*addVendorContextMapping)(Display *dpy, GLXContext context, __GLXvendorInfo *vendor);

    /*!
     * Removes a mapping from context to vendor. The context must have been
     * added with \p addVendorContextMapping.
     */
    void (*removeVendorContextMapping)(Display *dpy, GLXContext context);

    /*!
     * Looks up the vendor for a context.
     *
     * If no mapping is found, then this function will return \c NULL. No
     * errors are raised, so the dispatch function must raise any appropriate X
     * errors.
     *
     * Note that this function does not take a display connection, since
     * there are cases (e.g., glXGetContextIDEXT) that take a GLXContext but
     * not a display.
     *
     * \param context The context to look up.
     * \return The vendor for the context, or NULL if no matching context was
     * found.
     */
    __GLXvendorInfo * (*vendorFromContext)(GLXContext context);

    int (*addVendorFBConfigMapping)(Display *dpy, GLXFBConfig config, __GLXvendorInfo *vendor);
    void (*removeVendorFBConfigMapping)(Display *dpy, GLXFBConfig config);
    __GLXvendorInfo * (*vendorFromFBConfig)(Display *dpy, GLXFBConfig config);

    int (*addVendorDrawableMapping)(Display *dpy, GLXDrawable drawable, __GLXvendorInfo *vendor);
    void (*removeVendorDrawableMapping)(Display *dpy, GLXDrawable drawable);

    /*!
     * Looks up the vendor for a drawable.
     *
     * If the drawable was created from another GLX function, then this will
     * return the same vendor library that was used to create it.
     *
     * If the drawable was not created from GLX (a regular X window, for
     * example), then libGLX.so will use the x11glvnd server extension to
     * figure out a vendor library.
     *
     * All of this should be opaque to a dispatch function, since the only
     * thing that matters is finding out which vendor to dispatch to.
     */
    __GLXvendorInfo * (*vendorFromDrawable)(Display *dpy, GLXDrawable drawable);

} __GLXapiExports;

/*****************************************************************************
 * API library imports                                                       *
 *****************************************************************************/

/*!
 * This structure stores required and optional vendor library callbacks.
 */
typedef struct __GLXapiImportsRec {
    /*!
     * Checks if the vendor library can support a given X screen. If this
     * returns false, then libGLX will fall back to the indirect rendering
     * library (if one exists).
     *
     * \param dpy The display connection.
     * \param screen The screen number.
     * \return True if the vendor library can support this screen.
     */
    Bool (* isScreenSupported) (Display *dpy, int screen);

    /*!
     * This retrieves the pointer to the real GLX or core GL function.
     *
     * \param procName The name of the function.
     * \return A pointer to a function, or \c NULL if the vendor does not
     * support the function.
     */
    void        *(*getProcAddress)        (const GLubyte *procName);

    /*!
     * This retrieves vendor-neutral functions which use the
     * __GLXdispatchTableDynamic API above to dispatch to the correct vendor.
     *
     * A vendor library must provide a dispatch function for all GLX functions
     * that it supports. If \c getDispatchAddress returns NULL, but
     * \c getProcAddress returns non-NULL, then libGLX will assume that the
     * function is a GL function, not GLX.
     *
     * That allows libGLX to dispatch GL and GLX functions correctly, even in
     * the case of a GL function that starts with "glX".
     *
     * \param procName The name of the function.
     * \return A pointer to a function, or \c NULL if the vendor does not
     * support the function or \p procName is not a GLX function.
     */
    void        *(*getDispatchAddress)    (const GLubyte *procName);

    /*!
     * This notifies the vendor library which dispatch table index is
     * assigned to a particular GLX extension function.
     */
    void        (*setDispatchIndex)      (const GLubyte *procName, int index);

    /*!
     * (OPTIONAL) This notifies the vendor library when an X error was
     * generated due to a detected error in the GLX API stream.
     *
     * This may be \c NULL, in which case the vendor library is not notified of
     * any errors.
     *
     * \note this is a notification only -- libGLX takes care of actually
     * reporting the error.
     *
     * \param dpy The display connection.
     * \param error The error code.
     * \param resid The XID associated with the error, if any.
     * \param opcode The minor opcode of the function that generated the error.
     * \param coreX11error True if the error code is a core X11 error, or False
     * if it's a GLX error code.
     *
     * \return True if libGLX should report the error to the application.
     */
    Bool        (*notifyError)  (Display *dpy, unsigned char error,
                                 XID resid, unsigned char opcode,
                                 Bool coreX11error);

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

} __GLXapiImports;

/*****************************************************************************/

#define __GLX_MAIN_PROTO_NAME "__glx_Main"
#define __GLX_MAIN_PROTO(version, exports, vendor, imports) \
    Bool __glx_Main(uint32_t version, \
                    const __GLXapiExports *exports, \
                    __GLXvendorInfo *vendor, \
                    __GLXapiImports *imports)

typedef Bool (*__PFNGLXMAINPROC)
    (uint32_t version, const __GLXapiExports *exports, __GLXvendorInfo *vendor, __GLXapiImports *imports);

/*!
 * Vendor libraries must export a function called __glx_Main() with the
 * following prototype.
 *
 * This function also performs a handshake based on the ABI version number.
 * Vendor libraries can optionally use the version number to support older
 * versions of the ABI.
 *
 * \param[in] version The ABI version. The upper 16 bits contains the major version
 * number, and the lower 16 bits contains the minor version number.
 *
 * \param[in] exports The table of functions provided by libGLX. This pointer will
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
Bool __glx_Main(uint32_t version,
                                  const __GLXapiExports *exports,
                                  __GLXvendorInfo *vendor,
                                  __GLXapiImports *imports);

/*!
 * @}
 */

#if defined(__cplusplus)
}
#endif

#endif /* __LIB_GLX_ABI_H */
