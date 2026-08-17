/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
 * Copyright (C) 1994-1999 RSA Security Inc. Licence to copy this document
 * is granted provided that it is identified as "RSA Security In.c Public-Key
 * Cryptography Standards (PKCS)" in all material mentioning or referencing
 * this document.
 *
 * The latest version of this header can be found at:
 *    http://www.rsalabs.com/pkcs/pkcs-11/index.html
 */
#ifndef _PKCS11_H_
#define _PKCS11_H_ 1

#ifdef __cplusplus
extern "C" {
#endif

/* Before including this file (pkcs11.h) (or pkcs11t.h by
 * itself), 6 platform-specific macros must be defined.  These
 * macros are described below, and typical definitions for them
 * are also given.  Be advised that these definitions can depend
 * on both the platform and the compiler used (and possibly also
 * on whether a PKCS #11 library is linked statically or
 * dynamically).
 *
 * In addition to defining these 6 macros, the packing convention
 * for PKCS #11 structures should be set.  The PKCS #11
 * convention on packing is that structures should be 1-byte
 * aligned.
 *
 * In a Win32 environment, this might be done by using the
 * following preprocessor directive before including pkcs11.h
 * or pkcs11t.h:
 *
 * #pragma pack(push, cryptoki, 1)
 *
 * and using the following preprocessor directive after including
 * pkcs11.h or pkcs11t.h:
 *
 * #pragma pack(pop, cryptoki)
 *
 * In a UNIX environment, you're on your own here.  You might
 * not need to do anything.
 *
 *
 * Now for the macros:
 *
 *
 * 1. CK_PTR: The indirection string for making a pointer to an
 * object.  It can be used like this:
 *
 * typedef CK_BYTE CK_PTR CK_BYTE_PTR;
 *
 * In a Win32 environment, it might be defined by
 *
 * #define CK_PTR *
 *
 * In a UNIX environment, it might be defined by
 *
 * #define CK_PTR *
 *
 *
 * 2. CK_DEFINE_FUNCTION(returnType, name): A macro which makes
 * an exportable PKCS #11 library function definition out of a
 * return type and a function name.  It should be used in the
 * following fashion to define the exposed PKCS #11 functions in
 * a PKCS #11 library:
 *
 * CK_DEFINE_FUNCTION(CK_RV, C_Initialize)(
 *   CK_VOID_PTR pReserved
 * )
 * {
 *   ...
 * }
 *
 * For defining a function in a Win32 PKCS #11 .dll, it might be
 * defined by
 *
 * #define CK_DEFINE_FUNCTION(returnType, name) \
 *   returnType __declspec(dllexport) name
 *
 * In a UNIX environment, it might be defined by
 *
 * #define CK_DEFINE_FUNCTION(returnType, name) \
 *   returnType name
 *
 *
 * 3. CK_DECLARE_FUNCTION(returnType, name): A macro which makes
 * an importable PKCS #11 library function declaration out of a
 * return type and a function name.  It should be used in the
 * following fashion:
 *
 * extern CK_DECLARE_FUNCTION(CK_RV, C_Initialize)(
 *   CK_VOID_PTR pReserved
 * );
 *
 * For declaring a function in a Win32 PKCS #11 .dll, it might
 * be defined by
 *
 * #define CK_DECLARE_FUNCTION(returnType, name) \
 *   returnType __declspec(dllimport) name
 *
 * In a UNIX environment, it might be defined by
 *
 * #define CK_DECLARE_FUNCTION(returnType, name) \
 *   returnType name
 *
 *
 * 4. CK_DECLARE_FUNCTION_POINTER(returnType, name): A macro
 * which makes a PKCS #11 API function pointer declaration or
 * function pointer type declaration out of a return type and a
 * function name.  It should be used in the following fashion:
 *
 * // Define funcPtr to be a pointer to a PKCS #11 API function
 * // taking arguments args and returning CK_RV.
 * CK_DECLARE_FUNCTION_POINTER(CK_RV, funcPtr)(args);
 *
 * or
 *
 * // Define funcPtrType to be the type of a pointer to a
 * // PKCS #11 API function taking arguments args and returning
 * // CK_RV, and then define funcPtr to be a variable of type
 * // funcPtrType.
 * typedef CK_DECLARE_FUNCTION_POINTER(CK_RV, funcPtrType)(args);
 * funcPtrType funcPtr;
 *
 * For accessing functions in a Win32 PKCS #11 .dll, in might be
 * defined by
 *
 * #define CK_DECLARE_FUNCTION_POINTER(returnType, name) \
 *   returnType __declspec(dllimport) (* name)
 *
 * In a UNIX environment, it might be defined by
 *
 * #define CK_DECLARE_FUNCTION_POINTER(returnType, name) \
 *   returnType (* name)
 *
 *
 * 5. CK_CALLBACK_FUNCTION(returnType, name): A macro which makes
 * a function pointer type for an application callback out of
 * a return type for the callback and a name for the callback.
 * It should be used in the following fashion:
 *
 * CK_CALLBACK_FUNCTION(CK_RV, myCallback)(args);
 *
 * to declare a function pointer, myCallback, to a callback
 * which takes arguments args and returns a CK_RV.  It can also
 * be used like this:
 *
 * typedef CK_CALLBACK_FUNCTION(CK_RV, myCallbackType)(args);
 * myCallbackType myCallback;
 *
 * In a Win32 environment, it might be defined by
 *
 * #define CK_CALLBACK_FUNCTION(returnType, name) \
 *   returnType (* name)
 *
 * In a UNIX environment, it might be defined by
 *
 * #define CK_CALLBACK_FUNCTION(returnType, name) \
 *   returnType (* name)
 *
 *
 * 6. NULL_PTR: This macro is the value of a NULL pointer.
 *
 * In any ANSI/ISO C environment (and in many others as well),
 * this should be defined by
 *
 * #ifndef NULL_PTR
 * #define NULL_PTR 0
 * #endif
 */

/* All the various PKCS #11 types and #define'd values are in the
 * file pkcs11t.h. */
#include "pkcs11t.h"

#define __PASTE(x, y) x##y

#ifndef CK_PKCS11_3_0
/* remember that we set it so we can unset it at the end */
#define __NSS_CK_PKCS11_3_IMPLICIT 1
#define CK_PKCS11_3_0 1
#endif

/* ==============================================================
 * Define the "extern" form of all the entry points.
 * ==============================================================
 */

#define CK_NEED_ARG_LIST 1
#define CK_PKCS11_FUNCTION_INFO(name) \
    CK_DECLARE_FUNCTION(CK_RV, name)

/* pkcs11f.h has all the information about the PKCS #11
 * function prototypes. */
#include "pkcs11f.h"

#undef CK_NEED_ARG_LIST
#undef CK_PKCS11_FUNCTION_INFO

/* ==============================================================
 * Define the typedef form of all the entry points.  That is, for
 * each PKCS #11 function C_XXX, define a type CK_C_XXX which is
 * a pointer to that kind of function.
 * ==============================================================
 */

#define CK_NEED_ARG_LIST 1
#define CK_PKCS11_FUNCTION_INFO(name) \
    typedef CK_DECLARE_FUNCTION_POINTER(CK_RV, __PASTE(CK_, name))

/* pkcs11f.h has all the information about the PKCS #11
 * function prototypes. */
#include "pkcs11f.h"

#undef CK_NEED_ARG_LIST
#undef CK_PKCS11_FUNCTION_INFO

/* ==============================================================
 * Define structed vector of entry points.  A CK_FUNCTION_3_0_LIST
 * contains a CK_VERSION indicating a library's PKCS #11 version
 * and then a whole slew of function pointers to the routines in
 * the library.  This type was declared, but not defined, in
 * pkcs11t.h.
 * ==============================================================
 */

#define CK_PKCS11_FUNCTION_INFO(name) \
    __PASTE(CK_, name)                \
    name;

#include "pkcs11p.h"
struct CK_FUNCTION_LIST_3_0 {

    CK_VERSION version; /* PKCS #11 version */

/* Pile all the function pointers into the CK_FUNCTION_LIST_3_0. */
/* pkcs11f.h has all the information about the PKCS #11
 * function prototypes. */
#include "pkcs11f.h"
};

#define CK_PKCS11_2_0_ONLY 1

/* now define the 2.0 function list */
struct CK_FUNCTION_LIST {

    CK_VERSION version; /* PKCS #11 version */

/* Pile all the function pointers into the CK_FUNCTION_LIST. */
/* pkcs11f.h has all the information about the PKCS #11
 * function prototypes. */
#include "pkcs11f.h"
};
#include "pkcs11u.h"

#undef CK_PKCS11_FUNCTION_INFO
#undef CK_PKCS11_2_0_ONLY

#ifdef __NSS_CK_PKCS11_3_IMPLICIT
#undef CK_PKCS11_3_0
#undef __NSS_CK_PKCS11_3_IMPLICIT
#endif

#undef __PASTE

#ifdef __cplusplus
}
#endif

#endif
