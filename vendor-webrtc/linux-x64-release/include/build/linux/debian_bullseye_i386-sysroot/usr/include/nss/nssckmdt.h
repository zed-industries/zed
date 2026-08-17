/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef NSSCKMDT_H
#define NSSCKMDT_H

/*
 * nssckmdt.h
 *
 * This file specifies the basic types that must be implemented by
 * any Module using the NSS Cryptoki Framework.
 */

#ifndef NSSBASET_H
#include "nssbaset.h"
#endif /* NSSBASET_H */

#ifndef NSSCKT_H
#include "nssckt.h"
#endif /* NSSCKT_H */

#ifndef NSSCKFWT_H
#include "nssckfwt.h"
#endif /* NSSCKFWT_H */

typedef struct NSSCKMDInstanceStr NSSCKMDInstance;
typedef struct NSSCKMDSlotStr NSSCKMDSlot;
typedef struct NSSCKMDTokenStr NSSCKMDToken;
typedef struct NSSCKMDSessionStr NSSCKMDSession;
typedef struct NSSCKMDCryptoOperationStr NSSCKMDCryptoOperation;
typedef struct NSSCKMDFindObjectsStr NSSCKMDFindObjects;
typedef struct NSSCKMDMechanismStr NSSCKMDMechanism;
typedef struct NSSCKMDObjectStr NSSCKMDObject;

/*
 * NSSCKFWItem
 *
 * This is a structure used by modules to return object attributes.
 * The needsFreeing bit indicates whether the object needs to be freed.
 * If so, the framework will call the FreeAttribute function on the item
 * after it is done using it.
 *
 */

typedef struct {
    PRBool needsFreeing;
    NSSItem *item;
} NSSCKFWItem;

/*
 * NSSCKMDInstance
 *
 * This is the basic handle for an instance of a PKCS#11 Module.
 * It is returned by the Module's CreateInstance routine, and
 * may be obtained from the corresponding NSSCKFWInstance object.
 * It contains a pointer for use by the Module, to store any
 * instance-related data, and it contains the EPV for a set of
 * routines which the Module may implement for use by the Framework.
 * Some of these routines are optional; others are mandatory.
 */

struct NSSCKMDInstanceStr {
    /*
     * The Module may use this pointer for its own purposes.
     */
    void *etc;

    /*
     * This routine is called by the Framework to initialize
     * the Module.  This routine is optional; if unimplemented,
     * it won't be called.  If this routine returns an error,
     * then the initialization will fail.
     */
    CK_RV(PR_CALLBACK *Initialize)
    (
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSUTF8 *configurationData);

    /*
     * This routine is called when the Framework is finalizing
     * the PKCS#11 Module.  It is the last thing called before
     * the NSSCKFWInstance's NSSArena is destroyed.  This routine
     * is optional; if unimplemented, it merely won't be called.
     */
    void(PR_CALLBACK *Finalize)(
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
   * This routine gets the number of slots.  This value must
   * never change, once the instance is initialized.  This
   * routine must be implemented.  It may return zero on error.
   */
    CK_ULONG(PR_CALLBACK *GetNSlots)
    (
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns the version of the Cryptoki standard
     * to which this Module conforms.  This routine is optional;
     * if unimplemented, the Framework uses the version to which
     * ~it~ was implemented.
     */
    CK_VERSION(PR_CALLBACK *GetCryptokiVersion)
    (
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns a pointer to a UTF8-encoded string
     * containing the manufacturer ID for this Module.  Only
     * the characters completely encoded in the first thirty-
     * two bytes are significant.  This routine is optional.
     * The string returned is never freed; if dynamically generated,
     * the space for it should be allocated from the NSSArena
     * that may be obtained from the NSSCKFWInstance.  This
     * routine may return NULL upon error; however if *pError
     * is CKR_OK, the NULL will be considered the valid response.
     */
    NSSUTF8 *(PR_CALLBACK *GetManufacturerID)(
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns a pointer to a UTF8-encoded string
     * containing a description of this Module library.  Only
     * the characters completely encoded in the first thirty-
     * two bytes are significant.  This routine is optional.
     * The string returned is never freed; if dynamically generated,
     * the space for it should be allocated from the NSSArena
     * that may be obtained from the NSSCKFWInstance.  This
     * routine may return NULL upon error; however if *pError
     * is CKR_OK, the NULL will be considered the valid response.
     */
    NSSUTF8 *(PR_CALLBACK *GetLibraryDescription)(
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns the version of this Module library.
     * This routine is optional; if unimplemented, the Framework
     * will assume a Module library version of 0.1.
     */
    CK_VERSION(PR_CALLBACK *GetLibraryVersion)
    (
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if the Module wishes to
     * handle session objects.  This routine is optional.
     * If this routine is NULL, or if it exists but returns
     * CK_FALSE, the Framework will assume responsibility
     * for managing session objects.
     */
    CK_BBOOL(PR_CALLBACK *ModuleHandlesSessionObjects)
    (
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine stuffs pointers to NSSCKMDSlot objects into
     * the specified array; one for each slot supported by this
     * instance.  The Framework will determine the size needed
     * for the array by calling GetNSlots.  This routine is
     * required.
     */
    CK_RV(PR_CALLBACK *GetSlots)
    (
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDSlot *slots[]);

    /*
     * This call returns a pointer to the slot in which an event
     * has occurred.  If the block argument is CK_TRUE, the call
     * should block until a slot event occurs; if CK_FALSE, it
     * should check to see if an event has occurred, occurred,
     * but return NULL (and set *pError to CK_NO_EVENT) if one
     * hasn't.  This routine is optional; if unimplemented, the
     * Framework will assume that no event has happened.  This
     * routine may return NULL upon error.
     */
    NSSCKMDSlot *(PR_CALLBACK *WaitForSlotEvent)(
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_BBOOL block,
        CK_RV *pError);

    /*
     * This object may be extended in future versions of the
     * NSS Cryptoki Framework.  To allow for some flexibility
     * in the area of binary compatibility, this field should
     * be NULL.
     */
    void *null;
};

/*
 * NSSCKMDSlot
 *
 * This is the basic handle for a PKCS#11 Module Slot.  It is
 * created by the NSSCKMDInstance->GetSlots call, and may be
 * obtained from the Framework's corresponding NSSCKFWSlot
 * object.  It contains a pointer for use by the Module, to
 * store any slot-related data, and it contains the EPV for
 * a set of routines which the Module may implement for use
 * by the Framework.  Some of these routines are optional.
 */

struct NSSCKMDSlotStr {
    /*
     * The Module may use this pointer for its own purposes.
     */
    void *etc;

    /*
     * This routine is called during the Framework initialization
     * step, after the Framework Instance has obtained the list
     * of slots (by calling NSSCKMDInstance->GetSlots).  Any slot-
     * specific initialization can be done here.  This routine is
     * optional; if unimplemented, it won't be called.  Note that
     * if this routine returns an error, the entire Framework
     * initialization for this Module will fail.
     */
    CK_RV(PR_CALLBACK *Initialize)
    (
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine is called when the Framework is finalizing
     * the PKCS#11 Module.  This call (for each of the slots)
     * is the last thing called before NSSCKMDInstance->Finalize.
     * This routine is optional; if unimplemented, it merely
     * won't be called.  Note: In the rare circumstance that
     * the Framework initialization cannot complete (due to,
     * for example, memory limitations), this can be called with
     * a NULL value for fwSlot.
     */
    void(PR_CALLBACK *Destroy)(
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns a pointer to a UTF8-encoded string
     * containing a description of this slot.  Only the characters
     * completely encoded in the first sixty-four bytes are
     * significant.  This routine is optional.  The string
     * returned is never freed; if dynamically generated,
     * the space for it should be allocated from the NSSArena
     * that may be obtained from the NSSCKFWInstance.  This
     * routine may return NULL upon error; however if *pError
     * is CKR_OK, the NULL will be considered the valid response.
     */
    NSSUTF8 *(PR_CALLBACK *GetSlotDescription)(
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns a pointer to a UTF8-encoded string
     * containing a description of the manufacturer of this slot.
     * Only the characters completely encoded in the first thirty-
     * two bytes are significant.  This routine is optional.
     * The string  returned is never freed; if dynamically generated,
     * the space for it should be allocated from the NSSArena
     * that may be obtained from the NSSCKFWInstance.  This
     * routine may return NULL upon error; however if *pError
     * is CKR_OK, the NULL will be considered the valid response.
     */
    NSSUTF8 *(PR_CALLBACK *GetManufacturerID)(
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns CK_TRUE if a token is present in this
     * slot.  This routine is optional; if unimplemented, CK_TRUE
     * is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetTokenPresent)
    (
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if the slot supports removable
     * tokens.  This routine is optional; if unimplemented, CK_FALSE
     * is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetRemovableDevice)
    (
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if this slot is a hardware
     * device, or CK_FALSE if this slot is a software device.  This
     * routine is optional; if unimplemented, CK_FALSE is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetHardwareSlot)
    (
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the version of this slot's hardware.
     * This routine is optional; if unimplemented, the Framework
     * will assume a hardware version of 0.1.
     */
    CK_VERSION(PR_CALLBACK *GetHardwareVersion)
    (
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the version of this slot's firmware.
     * This routine is optional; if unimplemented, the Framework
     * will assume a hardware version of 0.1.
     */
    CK_VERSION(PR_CALLBACK *GetFirmwareVersion)
    (
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine should return a pointer to an NSSCKMDToken
     * object corresponding to the token in the specified slot.
     * The NSSCKFWToken object passed in has an NSSArena
     * available which is dedicated for this token.  This routine
     * must be implemented.  This routine may return NULL upon
     * error.
     */
    NSSCKMDToken *(PR_CALLBACK *GetToken)(
        NSSCKMDSlot *mdSlot,
        NSSCKFWSlot *fwSlot,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This object may be extended in future versions of the
     * NSS Cryptoki Framework.  To allow for some flexibility
     * in the area of binary compatibility, this field should
     * be NULL.
     */
    void *null;
};

/*
 * NSSCKMDToken
 *
 * This is the basic handle for a PKCS#11 Token.  It is created by
 * the NSSCKMDSlot->GetToken call, and may be obtained from the
 * Framework's corresponding NSSCKFWToken object.  It contains a
 * pointer for use by the Module, to store any token-related
 * data, and it contains the EPV for a set of routines which the
 * Module may implement for use by the Framework.  Some of these
 * routines are optional.
 */

struct NSSCKMDTokenStr {
    /*
     * The Module may use this pointer for its own purposes.
     */
    void *etc;

    /*
     * This routine is used to prepare a Module token object for
     * use.  It is called after the NSSCKMDToken object is obtained
     * from NSSCKMDSlot->GetToken.  It is named "Setup" here because
     * Cryptoki already defines "InitToken" to do the process of
     * wiping out any existing state on a token and preparing it for
     * a new use.  This routine is optional; if unimplemented, it
     * merely won't be called.
     */
    CK_RV(PR_CALLBACK *Setup)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine is called by the Framework whenever it notices
     * that the token object is invalid.  (Typically this is when a
     * routine indicates an error such as CKR_DEVICE_REMOVED).  This
     * call is the last thing called before the NSSArena in the
     * corresponding NSSCKFWToken is destroyed.  This routine is
     * optional; if unimplemented, it merely won't be called.
     */
    void(PR_CALLBACK *Invalidate)(
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine initialises the token in the specified slot.
     * This routine is optional; if unimplemented, the Framework
     * will fail this operation with an error of CKR_DEVICE_ERROR.
     */

    CK_RV(PR_CALLBACK *InitToken)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSItem *pin,
        NSSUTF8 *label);

    /*
     * This routine returns a pointer to a UTF8-encoded string
     * containing this token's label.  Only the characters
     * completely encoded in the first thirty-two bytes are
     * significant.  This routine is optional.  The string
     * returned is never freed; if dynamically generated,
     * the space for it should be allocated from the NSSArena
     * that may be obtained from the NSSCKFWInstance.  This
     * routine may return NULL upon error; however if *pError
     * is CKR_OK, the NULL will be considered the valid response.
     */
    NSSUTF8 *(PR_CALLBACK *GetLabel)(
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns a pointer to a UTF8-encoded string
     * containing this token's manufacturer ID.  Only the characters
     * completely encoded in the first thirty-two bytes are
     * significant.  This routine is optional.  The string
     * returned is never freed; if dynamically generated,
     * the space for it should be allocated from the NSSArena
     * that may be obtained from the NSSCKFWInstance.  This
     * routine may return NULL upon error; however if *pError
     * is CKR_OK, the NULL will be considered the valid response.
     */
    NSSUTF8 *(PR_CALLBACK *GetManufacturerID)(
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns a pointer to a UTF8-encoded string
     * containing this token's model name.  Only the characters
     * completely encoded in the first thirty-two bytes are
     * significant.  This routine is optional.  The string
     * returned is never freed; if dynamically generated,
     * the space for it should be allocated from the NSSArena
     * that may be obtained from the NSSCKFWInstance.  This
     * routine may return NULL upon error; however if *pError
     * is CKR_OK, the NULL will be considered the valid response.
     */
    NSSUTF8 *(PR_CALLBACK *GetModel)(
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns a pointer to a UTF8-encoded string
     * containing this token's serial number.  Only the characters
     * completely encoded in the first thirty-two bytes are
     * significant.  This routine is optional.  The string
     * returned is never freed; if dynamically generated,
     * the space for it should be allocated from the NSSArena
     * that may be obtained from the NSSCKFWInstance.  This
     * routine may return NULL upon error; however if *pError
     * is CKR_OK, the NULL will be considered the valid response.
     */
    NSSUTF8 *(PR_CALLBACK *GetSerialNumber)(
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns CK_TRUE if the token has its own
     * random number generator.  This routine is optional; if
     * unimplemented, CK_FALSE is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetHasRNG)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if this token is write-protected.
     * This routine is optional; if unimplemented, CK_FALSE is
     * assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetIsWriteProtected)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if this token requires a login.
     * This routine is optional; if unimplemented, CK_FALSE is
     * assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetLoginRequired)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if the normal user's PIN on this
     * token has been initialised.  This routine is optional; if
     * unimplemented, CK_FALSE is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetUserPinInitialized)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if a successful save of a
     * session's cryptographic operations state ~always~ contains
     * all keys needed to restore the state of the session.  This
     * routine is optional; if unimplemented, CK_FALSE is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetRestoreKeyNotNeeded)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if the token has its own
     * hardware clock.  This routine is optional; if unimplemented,
     * CK_FALSE is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetHasClockOnToken)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if the token has a protected
     * authentication path.  This routine is optional; if
     * unimplemented, CK_FALSE is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetHasProtectedAuthenticationPath)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns CK_TRUE if the token supports dual
     * cryptographic operations within a single session.  This
     * routine is optional; if unimplemented, CK_FALSE is assumed.
     */
    CK_BBOOL(PR_CALLBACK *GetSupportsDualCryptoOperations)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * XXX fgmr-- should we have a call to return all the flags
     * at once, for folks who already know about Cryptoki?
     */

    /*
     * This routine returns the maximum number of sessions that
     * may be opened on this token.  This routine is optional;
     * if unimplemented, the special value CK_UNAVAILABLE_INFORMATION
     * is assumed.  XXX fgmr-- or CK_EFFECTIVELY_INFINITE?
     */
    CK_ULONG(PR_CALLBACK *GetMaxSessionCount)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the maximum number of read/write
     * sesisons that may be opened on this token.  This routine
     * is optional; if unimplemented, the special value
     * CK_UNAVAILABLE_INFORMATION is assumed.  XXX fgmr-- or
     * CK_EFFECTIVELY_INFINITE?
     */
    CK_ULONG(PR_CALLBACK *GetMaxRwSessionCount)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the maximum PIN code length that is
     * supported on this token.  This routine is optional;
     * if unimplemented, the special value CK_UNAVAILABLE_INFORMATION
     * is assumed.
     */
    CK_ULONG(PR_CALLBACK *GetMaxPinLen)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the minimum PIN code length that is
     * supported on this token.  This routine is optional; if
     * unimplemented, the special value CK_UNAVAILABLE_INFORMATION
     *  is assumed.  XXX fgmr-- or 0?
     */
    CK_ULONG(PR_CALLBACK *GetMinPinLen)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the total amount of memory on the token
     * in which public objects may be stored.  This routine is
     * optional; if unimplemented, the special value
     * CK_UNAVAILABLE_INFORMATION is assumed.
     */
    CK_ULONG(PR_CALLBACK *GetTotalPublicMemory)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the amount of unused memory on the
     * token in which public objects may be stored.  This routine
     * is optional; if unimplemented, the special value
     * CK_UNAVAILABLE_INFORMATION is assumed.
     */
    CK_ULONG(PR_CALLBACK *GetFreePublicMemory)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the total amount of memory on the token
     * in which private objects may be stored.  This routine is
     * optional; if unimplemented, the special value
     * CK_UNAVAILABLE_INFORMATION is assumed.
     */
    CK_ULONG(PR_CALLBACK *GetTotalPrivateMemory)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the amount of unused memory on the
     * token in which private objects may be stored.  This routine
     * is optional; if unimplemented, the special value
     * CK_UNAVAILABLE_INFORMATION is assumed.
     */
    CK_ULONG(PR_CALLBACK *GetFreePrivateMemory)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the version number of this token's
     * hardware.  This routine is optional; if unimplemented,
     * the value 0.1 is assumed.
     */
    CK_VERSION(PR_CALLBACK *GetHardwareVersion)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the version number of this token's
     * firmware.  This routine is optional; if unimplemented,
     * the value 0.1 is assumed.
     */
    CK_VERSION(PR_CALLBACK *GetFirmwareVersion)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine stuffs the current UTC time, as obtained from
     * the token, into the sixteen-byte buffer in the form
     * YYYYMMDDhhmmss00.  This routine need only be implemented
     * by token which indicate that they have a real-time clock.
     * XXX fgmr-- think about time formats.
     */
    CK_RV(PR_CALLBACK *GetUTCTime)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_CHAR utcTime[16]);

    /*
     * This routine creates a session on the token, and returns
     * the corresponding NSSCKMDSession object.  The value of
     * rw will be CK_TRUE if the session is to be a read/write
     * session, or CK_FALSE otherwise.  An NSSArena dedicated to
     * the new session is available from the specified NSSCKFWSession.
     * This routine may return NULL upon error.
     */
    NSSCKMDSession *(PR_CALLBACK *OpenSession)(
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKFWSession *fwSession,
        CK_BBOOL rw,
        CK_RV *pError);

    /*
     * This routine returns the number of PKCS#11 Mechanisms
     * supported by this token.  This routine is optional; if
     * unimplemented, zero is assumed.
     */
    CK_ULONG(PR_CALLBACK *GetMechanismCount)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine stuffs into the specified array the types
     * of the mechanisms supported by this token.  The Framework
     * determines the size of the array by calling GetMechanismCount.
     */
    CK_RV(PR_CALLBACK *GetMechanismTypes)
    (
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_MECHANISM_TYPE types[]);

    /*
     * This routine returns a pointer to a Module mechanism
     * object corresponding to a specified type.  This routine
     * need only exist for tokens implementing at least one
     * mechanism.
     */
    NSSCKMDMechanism *(PR_CALLBACK *GetMechanism)(
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_MECHANISM_TYPE which,
        CK_RV *pError);

    /*
     * This object may be extended in future versions of the
     * NSS Cryptoki Framework.  To allow for some flexibility
     * in the area of binary compatibility, this field should
     * be NULL.
     */
    void *null;
};

/*
 * NSSCKMDSession
 *
 * This is the basic handle for a session on a PKCS#11 Token.  It
 * is created by NSSCKMDToken->OpenSession, and may be obtained
 * from the Framework's corresponding NSSCKFWSession object.  It
 * contains a pointer for use by the Module, to store any session-
 * realted data, and it contains the EPV for a set of routines
 * which the Module may implement for use by the Framework.  Some
 * of these routines are optional.
 */

struct NSSCKMDSessionStr {
    /*
     * The Module may use this pointer for its own purposes.
     */
    void *etc;

    /*
     * This routine is called by the Framework when a session is
     * closed.  This call is the last thing called before the
     * NSSArena in the correspoinding NSSCKFWSession is destroyed.
     * This routine is optional; if unimplemented, it merely won't
     * be called.
     */
    void(PR_CALLBACK *Close)(
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine is used to get any device-specific error.
     * This routine is optional.
     */
    CK_ULONG(PR_CALLBACK *GetDeviceError)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine is used to log in a user to the token.  This
     * routine is optional, since the Framework's NSSCKFWSession
     * object keeps track of the login state.
     */
    CK_RV(PR_CALLBACK *Login)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_USER_TYPE userType,
        NSSItem *pin,
        CK_STATE oldState,
        CK_STATE newState);

    /*
     * This routine is used to log out a user from the token.  This
     * routine is optional, since the Framework's NSSCKFWSession
     * object keeps track of the login state.
     */
    CK_RV(PR_CALLBACK *Logout)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_STATE oldState,
        CK_STATE newState);

    /*
     * This routine is used to initialize the normal user's PIN or
     * password.  This will only be called in the "read/write
     * security officer functions" state.  If this token has a
     * protected authentication path, then the pin argument will
     * be NULL.  This routine is optional; if unimplemented, the
     * Framework will return the error CKR_TOKEN_WRITE_PROTECTED.
     */
    CK_RV(PR_CALLBACK *InitPIN)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSItem *pin);

    /*
     * This routine is used to modify a user's PIN or password.  This
     * routine will only be called in the "read/write security officer
     * functions" or "read/write user functions" state.  If this token
     * has a protected authentication path, then the pin arguments
     * will be NULL.  This routine is optional; if unimplemented, the
     * Framework will return the error CKR_TOKEN_WRITE_PROTECTED.
     */
    CK_RV(PR_CALLBACK *SetPIN)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSItem *oldPin,
        NSSItem *newPin);

    /*
     * This routine is used to find out how much space would be required
     * to save the current operational state.  This routine is optional;
     * if unimplemented, the Framework will reject any attempts to save
     * the operational state with the error CKR_STATE_UNSAVEABLE.  This
     * routine may return zero on error.
     */
    CK_ULONG(PR_CALLBACK *GetOperationStateLen)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine is used to store the current operational state.  This
     * routine is only required if GetOperationStateLen is implemented
     * and can return a nonzero value.  The buffer in the specified item
     * will be pre-allocated, and the length will specify the amount of
     * space available (which may be more than GetOperationStateLen
     * asked for, but which will not be smaller).
     */
    CK_RV(PR_CALLBACK *GetOperationState)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSItem *buffer);

    /*
     * This routine is used to restore an operational state previously
     * obtained with GetOperationState.  The Framework will take pains
     * to be sure that the state is (or was at one point) valid; if the
     * Module notices that the state is invalid, it should return an
     * error, but it is not required to be paranoid about the issue.
     * [XXX fgmr-- should (can?) the framework verify the keys match up?]
     * This routine is required only if GetOperationState is implemented.
     */
    CK_RV(PR_CALLBACK *SetOperationState)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSItem *state,
        NSSCKMDObject *mdEncryptionKey,
        NSSCKFWObject *fwEncryptionKey,
        NSSCKMDObject *mdAuthenticationKey,
        NSSCKFWObject *fwAuthenticationKey);

    /*
     * This routine is used to create an object.  The specified template
     * will only specify a session object if the Module has indicated
     * that it wishes to handle its own session objects.  This routine
     * is optional; if unimplemented, the Framework will reject the
     * operation with the error CKR_TOKEN_WRITE_PROTECTED.  Space for
     * token objects should come from the NSSArena available from the
     * NSSCKFWToken object; space for session objects (if supported)
     * should come from the NSSArena available from the NSSCKFWSession
     * object.  The appropriate NSSArena pointer will, as a convenience,
     * be passed as the handyArenaPointer argument.  This routine may
     * return NULL upon error.
     */
    NSSCKMDObject *(PR_CALLBACK *CreateObject)(
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSArena *handyArenaPointer,
        CK_ATTRIBUTE_PTR pTemplate,
        CK_ULONG ulAttributeCount,
        CK_RV *pError);

    /*
     * This routine is used to make a copy of an object.  It is entirely
     * optional; if unimplemented, the Framework will try to use
     * CreateObject instead.  If the Module has indicated that it does
     * not wish to handle session objects, then this routine will only
     * be called to copy a token object to another token object.
     * Otherwise, either the original object or the new may be of
     * either the token or session variety.  As with CreateObject, the
     * handyArenaPointer will point to the appropriate arena for the
     * new object.  This routine may return NULL upon error.
     */
    NSSCKMDObject *(PR_CALLBACK *CopyObject)(
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdOldObject,
        NSSCKFWObject *fwOldObject,
        NSSArena *handyArenaPointer,
        CK_ATTRIBUTE_PTR pTemplate,
        CK_ULONG ulAttributeCount,
        CK_RV *pError);

    /*
     * This routine is used to begin an object search.  This routine may
     * be unimplemented only if the Module does not handle session
     * objects, and if none of its tokens have token objects.  The
     * NSSCKFWFindObjects pointer has an NSSArena that may be used for
     * storage for the life of this "find" operation.  This routine may
     * return NULL upon error.  If the Module can determine immediately
     * that the search will not find any matching objects, it may return
     * NULL, and specify CKR_OK as the error.
     */
    NSSCKMDFindObjects *(PR_CALLBACK *FindObjectsInit)(
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_ATTRIBUTE_PTR pTemplate,
        CK_ULONG ulAttributeCount,
        CK_RV *pError);

    /*
     * This routine seeds the random-number generator.  It is
     * optional, even if GetRandom is implemented.  If unimplemented,
     * the Framework will issue the error CKR_RANDOM_SEED_NOT_SUPPORTED.
     */
    CK_RV(PR_CALLBACK *SeedRandom)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSItem *seed);

    /*
     * This routine gets random data.  It is optional.  If unimplemented,
     * the Framework will issue the error CKR_RANDOM_NO_RNG.
     */
    CK_RV(PR_CALLBACK *GetRandom)
    (
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSItem *buffer);

    /*
     * This object may be extended in future versions of the
     * NSS Cryptoki Framework.  To allow for some flexibility
     * in the area of binary compatibility, this field should
     * be NULL.
     */
    void *null;
};

/*
 * NSSCKMDFindObjects
 *
 * This is the basic handle for an object search.  It is
 * created by NSSCKMDSession->FindObjectsInit, and may be
 * obtained from the Framework's corresponding object.
 * It contains a pointer for use by the Module, to store
 * any search-related data, and it contains the EPV for a
 * set of routines which the Module may implement for use
 * by the Framework.  Some of these routines are optional.
 */

struct NSSCKMDFindObjectsStr {
    /*
     * The Module may use this pointer for its own purposes.
     */
    void *etc;

    /*
     * This routine is called by the Framework to finish a
     * search operation.  Note that the Framework may finish
     * a search before it has completed.  This routine is
     * optional; if unimplemented, it merely won't be called.
     */
    void(PR_CALLBACK *Final)(
        NSSCKMDFindObjects *mdFindObjects,
        NSSCKFWFindObjects *fwFindObjects,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine is used to obtain another pointer to an
     * object matching the search criteria.  This routine is
     * required.  If no (more) objects match the search, it
     * should return NULL and set the error to CKR_OK.
     */
    NSSCKMDObject *(PR_CALLBACK *Next)(
        NSSCKMDFindObjects *mdFindObjects,
        NSSCKFWFindObjects *fwFindObjects,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSArena *arena,
        CK_RV *pError);

    /*
     * This object may be extended in future versions of the
     * NSS Cryptoki Framework.  To allow for some flexibility
     * in the area of binary compatibility, this field should
     * be NULL.
     */
    void *null;
};

/*
 * NSSCKMDCryptoOperaion
 *
 * This is the basic handle for an encryption, decryption,
 * sign, verify, or hash opertion.
 * created by NSSCKMDMechanism->XXXXInit, and may be
 * obtained from the Framework's corresponding object.
 * It contains a pointer for use by the Module, to store
 * any intermediate data, and it contains the EPV for a
 * set of routines which the Module may implement for use
 * by the Framework.  Some of these routines are optional.
 */

struct NSSCKMDCryptoOperationStr {
    /*
     * The Module may use this pointer for its own purposes.
     */
    void *etc;

    /*
     * This routine is called by the Framework clean up the mdCryptoOperation
     * structure.
     * This routine is optional; if unimplemented, it will be ignored.
     */
    void(PR_CALLBACK *Destroy)(
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * how many bytes do we need to finish this buffer?
     * must be implemented if Final is implemented.
     */
    CK_ULONG(PR_CALLBACK *GetFinalLength)
    (
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * how many bytes do we need to complete the next operation.
     * used in both Update and UpdateFinal.
     */
    CK_ULONG(PR_CALLBACK *GetOperationLength)
    (
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        const NSSItem *inputBuffer,
        CK_RV *pError);

    /*
     * This routine is called by the Framework to finish a
     * search operation.  Note that the Framework may finish
     * a search before it has completed.  This routine is
     * optional; if unimplemented, it merely won't be called.
     * The respective final call with fail with CKR_FUNCTION_FAILED
     * Final should not free the mdCryptoOperation.
     */
    CK_RV(PR_CALLBACK *Final)
    (
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSItem *outputBuffer);

    /*
     * This routine is called by the Framework to complete the
     * next step in an encryption/decryption operation.
     * This routine is optional; if unimplemented, the respective
     * update call with fail with CKR_FUNCTION_FAILED.
     * Update should not be implemented for signing/verification/digest
     * mechanisms.
     */
    CK_RV(PR_CALLBACK *Update)
    (
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        const NSSItem *inputBuffer,
        NSSItem *outputBuffer);

    /*
     * This routine is called by the Framework to complete the
     * next step in a signing/verification/digest operation.
     * This routine is optional; if unimplemented, the respective
     * update call with fail with CKR_FUNCTION_FAILED
     * Update should not be implemented for encryption/decryption
     * mechanisms.
     */
    CK_RV(PR_CALLBACK *DigestUpdate)
    (
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        const NSSItem *inputBuffer);

    /*
     * This routine is called by the Framework to complete a
     * single step operation. This routine is optional; if unimplemented,
     * the framework will use the Update and Final functions to complete
     * the operation.
     */
    CK_RV(PR_CALLBACK *UpdateFinal)
    (
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        const NSSItem *inputBuffer,
        NSSItem *outputBuffer);

    /*
     * This routine is called by the Framework to complete next
     * step in a combined operation. The Decrypt/Encrypt mechanism
     * should define and drive the combo step.
     * This routine is optional; if unimplemented,
     * the framework will use the appropriate Update functions to complete
     * the operation.
     */
    CK_RV(PR_CALLBACK *UpdateCombo)
    (
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDCryptoOperation *mdPeerCryptoOperation,
        NSSCKFWCryptoOperation *fwPeerCryptoOperation,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        const NSSItem *inputBuffer,
        NSSItem *outputBuffer);

    /*
     * Hash a key directly into the digest
     */
    CK_RV(PR_CALLBACK *DigestKey)
    (
        NSSCKMDCryptoOperation *mdCryptoOperation,
        NSSCKFWCryptoOperation *fwCryptoOperation,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdKey,
        NSSCKFWObject *fwKey);

    /*
     * This object may be extended in future versions of the
     * NSS Cryptoki Framework.  To allow for some flexibility
     * in the area of binary compatibility, this field should
     * be NULL.
     */
    void *null;
};

/*
 * NSSCKMDMechanism
 *
 */

struct NSSCKMDMechanismStr {
    /*
     * The Module may use this pointer for its own purposes.
     */
    void *etc;

    /*
     * This also frees the fwMechanism if appropriate.
     * If it is not supplied, the Framework will assume that the Token
     * Manages a static list of mechanisms and the function will not be called.
     */
    void(PR_CALLBACK *Destroy)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the minimum key size allowed for
     * this mechanism.  This routine is optional; if unimplemented,
     * zero will be assumed.  This routine may return zero on
     * error; if the error is CKR_OK, zero will be accepted as
     * a valid response.
     */
    CK_ULONG(PR_CALLBACK *GetMinKeySize)
    (
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine returns the maximum key size allowed for
     * this mechanism.  This routine is optional; if unimplemented,
     * zero will be assumed.  This routine may return zero on
     * error; if the error is CKR_OK, zero will be accepted as
     * a valid response.
     */
    CK_ULONG(PR_CALLBACK *GetMaxKeySize)
    (
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine is called to determine if the mechanism is
     * implemented in hardware or software.  It returns CK_TRUE
     * if it is done in hardware.
     */
    CK_BBOOL(PR_CALLBACK *GetInHardware)
    (
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * The crypto routines themselves.  Most crypto operations may
     * be performed in two ways, streaming and single-part.  The
     * streaming operations involve the use of (typically) three
     * calls-- an Init method to set up the operation, an Update
     * method to feed data to the operation, and a Final method to
     * obtain the final result.  Single-part operations involve
     * one method, to perform the crypto operation all at once.
     *
     * The NSS Cryptoki Framework can implement the single-part
     * operations in terms of the streaming operations on behalf
     * of the Module.  There are a few variances.
     *
     * Only the Init Functions are defined by the mechanism. Each
     * init function will return a NSSCKFWCryptoOperation which
     * can supply update, final, the single part updateFinal, and
     * the combo updateCombo functions.
     *
     * For simplicity, the routines are listed in summary here:
     *
     *  EncryptInit,
     *  DecryptInit,
     *  DigestInit,
     *  SignInit,
     *  SignRecoverInit;
     *  VerifyInit,
     *  VerifyRecoverInit;
     *
     * The key-management routines are
     *
     *  GenerateKey
     *  GenerateKeyPair
     *  WrapKey
     *  UnwrapKey
     *  DeriveKey
     *
     * All of these routines based on the Cryptoki API;
     * see PKCS#11 for further information.
     */

    /*
     */
    NSSCKMDCryptoOperation *(PR_CALLBACK *EncryptInit)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdKey,
        NSSCKFWObject *fwKey,
        CK_RV *pError);

    /*
     */
    NSSCKMDCryptoOperation *(PR_CALLBACK *DecryptInit)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdKey,
        NSSCKFWObject *fwKey,
        CK_RV *pError);

    /*
     */
    NSSCKMDCryptoOperation *(PR_CALLBACK *DigestInit)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     */
    NSSCKMDCryptoOperation *(PR_CALLBACK *SignInit)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdKey,
        NSSCKFWObject *fwKey,
        CK_RV *pError);

    /*
     */
    NSSCKMDCryptoOperation *(PR_CALLBACK *VerifyInit)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdKey,
        NSSCKFWObject *fwKey,
        CK_RV *pError);

    /*
     */
    NSSCKMDCryptoOperation *(PR_CALLBACK *SignRecoverInit)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdKey,
        NSSCKFWObject *fwKey,
        CK_RV *pError);

    /*
     */
    NSSCKMDCryptoOperation *(PR_CALLBACK *VerifyRecoverInit)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdKey,
        NSSCKFWObject *fwKey,
        CK_RV *pError);

    /*
     * Key management operations.
     */

    /*
     * This routine generates a key.  This routine may return NULL
     * upon error.
     */
    NSSCKMDObject *(PR_CALLBACK *GenerateKey)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_ATTRIBUTE_PTR pTemplate,
        CK_ULONG ulAttributeCount,
        CK_RV *pError);

    /*
     * This routine generates a key pair.
     */
    CK_RV(PR_CALLBACK *GenerateKeyPair)
    (
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_ATTRIBUTE_PTR pPublicKeyTemplate,
        CK_ULONG ulPublicKeyAttributeCount,
        CK_ATTRIBUTE_PTR pPrivateKeyTemplate,
        CK_ULONG ulPrivateKeyAttributeCount,
        NSSCKMDObject **pPublicKey,
        NSSCKMDObject **pPrivateKey);

    /*
     * This routine wraps a key.
     */
    CK_ULONG(PR_CALLBACK *GetWrapKeyLength)
    (
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdWrappingKey,
        NSSCKFWObject *fwWrappingKey,
        NSSCKMDObject *mdWrappedKey,
        NSSCKFWObject *fwWrappedKey,
        CK_RV *pError);

    /*
     * This routine wraps a key.
     */
    CK_RV(PR_CALLBACK *WrapKey)
    (
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdWrappingKey,
        NSSCKFWObject *fwWrappingKey,
        NSSCKMDObject *mdKeyObject,
        NSSCKFWObject *fwKeyObject,
        NSSItem *wrappedKey);

    /*
     * This routine unwraps a key.  This routine may return NULL
     * upon error.
     */
    NSSCKMDObject *(PR_CALLBACK *UnwrapKey)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdWrappingKey,
        NSSCKFWObject *fwWrappingKey,
        NSSItem *wrappedKey,
        CK_ATTRIBUTE_PTR pTemplate,
        CK_ULONG ulAttributeCount,
        CK_RV *pError);

    /*
     * This routine derives a key.  This routine may return NULL
     * upon error.
     */
    NSSCKMDObject *(PR_CALLBACK *DeriveKey)(
        NSSCKMDMechanism *mdMechanism,
        NSSCKFWMechanism *fwMechanism,
        CK_MECHANISM_PTR pMechanism,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        NSSCKMDObject *mdBaseKey,
        NSSCKFWObject *fwBaseKey,
        CK_ATTRIBUTE_PTR pTemplate,
        CK_ULONG ulAttributeCount,
        CK_RV *pError);

    /*
     * This object may be extended in future versions of the
     * NSS Cryptoki Framework.  To allow for some flexibility
     * in the area of binary compatibility, this field should
     * be NULL.
     */
    void *null;
};

/*
 * NSSCKMDObject
 *
 * This is the basic handle for any object used by a PKCS#11 Module.
 * Modules must implement it if they support their own objects, and
 * the Framework supports it for Modules that do not handle session
 * objects.  This type contains a pointer for use by the implementor,
 * to store any object-specific data, and it contains an EPV for a
 * set of routines used to access the object.
 */

struct NSSCKMDObjectStr {
    /*
     * The implementation my use this pointer for its own purposes.
     */
    void *etc;

    /*
     * This routine is called by the Framework when it is letting
     * go of an object handle.  It can be used by the Module to
     * free any resources tied up by an object "in use."  It is
     * optional.
     */
    void(PR_CALLBACK *Finalize)(
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine is used to completely destroy an object.
     * It is optional.  The parameter fwObject might be NULL
     * if the framework runs out of memory at the wrong moment.
     */
    CK_RV(PR_CALLBACK *Destroy)
    (
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This helper routine is used by the Framework, and is especially
     * useful when it is managing session objects on behalf of the
     * Module.  This routine is optional; if unimplemented, the
     * Framework will actually look up the CKA_TOKEN attribute.  In the
     * event of an error, just make something up-- the Framework will
     * find out soon enough anyway.
     */
    CK_BBOOL(PR_CALLBACK *IsTokenObject)
    (
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance);

    /*
     * This routine returns the number of attributes of which this
     * object consists.  It is mandatory.  It can return zero on
     * error.
     */
    CK_ULONG(PR_CALLBACK *GetAttributeCount)
    (
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This routine stuffs the attribute types into the provided array.
     * The array size (as obtained from GetAttributeCount) is passed in
     * as a check; return CKR_BUFFER_TOO_SMALL if the count is wrong
     * (either too big or too small).
     */
    CK_RV(PR_CALLBACK *GetAttributeTypes)
    (
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_ATTRIBUTE_TYPE_PTR typeArray,
        CK_ULONG ulCount);

    /*
     * This routine returns the size (in bytes) of the specified
     * attribute.  It can return zero on error.
     */
    CK_ULONG(PR_CALLBACK *GetAttributeSize)
    (
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_ATTRIBUTE_TYPE attribute,
        CK_RV *pError);

    /*
     * This routine returns an NSSCKFWItem structure.
     * The item pointer points to an NSSItem containing the attribute value.
     * The needsFreeing bit tells the framework whether to call the
     * FreeAttribute function . Upon error, an NSSCKFWItem structure
     * with a NULL NSSItem item pointer will be returned
     */
    NSSCKFWItem(PR_CALLBACK *GetAttribute)(
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_ATTRIBUTE_TYPE attribute,
        CK_RV *pError);

    /*
     * This routine returns CKR_OK if the attribute could be freed.
     */
    CK_RV(PR_CALLBACK *FreeAttribute)
    (
        NSSCKFWItem *item);

    /*
     * This routine changes the specified attribute.  If unimplemented,
     * the object will be considered read-only.
     */
    CK_RV(PR_CALLBACK *SetAttribute)
    (
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_ATTRIBUTE_TYPE attribute,
        NSSItem *value);

    /*
     * This routine returns the storage requirements of this object,
     * in bytes.  Cryptoki doesn't strictly define the definition,
     * but it should relate to the values returned by the "Get Memory"
     * routines of the NSSCKMDToken.  This routine is optional; if
     * unimplemented, the Framework will consider this information
     * sensitive.  This routine may return zero on error.  If the
     * specified error is CKR_OK, zero will be accepted as a valid
     * response.
     */
    CK_ULONG(PR_CALLBACK *GetObjectSize)
    (
        NSSCKMDObject *mdObject,
        NSSCKFWObject *fwObject,
        NSSCKMDSession *mdSession,
        NSSCKFWSession *fwSession,
        NSSCKMDToken *mdToken,
        NSSCKFWToken *fwToken,
        NSSCKMDInstance *mdInstance,
        NSSCKFWInstance *fwInstance,
        CK_RV *pError);

    /*
     * This object may be extended in future versions of the
     * NSS Cryptoki Framework.  To allow for some flexibility
     * in the area of binary compatibility, this field should
     * be NULL.
     */
    void *null;
};

#endif /* NSSCKMDT_H */
