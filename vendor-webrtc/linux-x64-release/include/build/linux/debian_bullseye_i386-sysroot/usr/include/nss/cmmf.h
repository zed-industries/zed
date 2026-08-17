/* -*- Mode: C; tab-width: 8 -*-*/
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _CMMF_H_
#define _CMMF_H_
/*
 * These are the functions exported by the security library for
 * implementing Certificate Management Message Formats (CMMF).
 *
 * This API is designed against July 1998 CMMF draft.  Please read this
 * draft before trying to use this API in an application that use CMMF.
 */
#include "seccomon.h"
#include "cmmft.h"
#include "crmf.h"

SEC_BEGIN_PROTOS

/******************* Creation Functions *************************/

/*
 * FUNCTION: CMMF_CreateCertRepContent
 * INPUTS:
 *    NONE
 * NOTES:
 *    This function will create an empty CMMFCertRepContent Structure.
 *    The client of the library must set the CMMFCertResponses.
 *    Call CMMF_CertRepContentSetCertResponse to accomplish this task.
 *    If the client of the library also wants to include the chain of
 *    CA certs required to make the certificates in CMMFCertResponse valid,
 *    then the user must also set the caPubs field of CMMFCertRepContent.
 *    Call CMMF_CertRepContentSetCAPubs to accomplish this.  After setting
 *    the desired fields, the user can then call CMMF_EncodeCertRepContent
 *    to DER-encode the CertRepContent.
 * RETURN:
 *    A pointer to the CMMFCertRepContent.  A NULL return value indicates
 *    an error in allocating memory or failure to initialize the structure.
 */
extern CMMFCertRepContent *CMMF_CreateCertRepContent(void);

/*
 * FUNCTION: CMMF_CreateCertRepContentFromDER
 * INPUTS
 *    db
 *        The certificate database where the certificates will be placed.
 *        The certificates will be placed in the temporary database associated
 *        with the handle.
 *    buf
 *        A buffer to the DER-encoded CMMFCertRepContent
 *    len
 *        The length in bytes of the buffer 'buf'
 * NOTES:
 *    This function passes the buffer to the ASN1 decoder and creates a
 *    CMMFCertRepContent structure.  The user must call
 *    CMMF_DestroyCertRepContent after the return value is no longer needed.
 *
 * RETURN:
 *    A pointer to the CMMFCertRepContent structure.  A NULL return
 *    value indicates the library was unable to parse the DER.
 */
extern CMMFCertRepContent *
CMMF_CreateCertRepContentFromDER(CERTCertDBHandle *db,
                                 const char *buf,
                                 long len);

/*
 * FUNCTION: CMMF_CreateCertResponse
 * INPUTS:
 *    inCertReqId
 *        The Certificate Request Id this response is for.
 * NOTES:
 *    This creates a CMMFCertResponse.  This response should correspond
 *    to a request that was received via CRMF.  From the CRMF message you
 *    can get the Request Id to pass in as inCertReqId, in essence binding
 *    a CMRFCertRequest message to the CMMFCertResponse created by this
 *    function.  If no requuest id is associated with the response to create
 *    then the user should pass in -1 for 'inCertReqId'.
 *
 * RETURN:
 *    A pointer to the new CMMFCertResponse corresponding to the request id
 *    passed in.  A NULL return value indicates an error while trying to
 *    create the CMMFCertResponse.
 */
extern CMMFCertResponse *CMMF_CreateCertResponse(long inCertReqId);

/*
 * FUNCTION: CMMF_CreateKeyRecRepContent
 * INPUTS:
 *    NONE
 * NOTES:
 *    This function creates a new empty CMMFKeyRecRepContent structure.
 *    At the very minimum, the user  must call
 *    CMMF_KeyRecRepContentSetPKIStatusInfoStatus field to have an
 *    encodable structure.  Depending on what the response is, the user may
 *    have to set other fields as well to properly build up the structure so
 *    that it can be encoded.  Refer to the CMMF draft for how to properly
 *    set up a CMMFKeyRecRepContent. This is the structure that an RA returns
 *    to an end entity when doing key recovery.

 *    The user must call CMMF_DestroyKeyRecRepContent when the return value
 *    is no longer needed.
 * RETURN:
 *    A pointer to the empty CMMFKeyRecRepContent.  A return value of NULL
 *    indicates an error in allocating memory or initializing the structure.
 */
extern CMMFKeyRecRepContent *CMMF_CreateKeyRecRepContent(void);

/*
 * FUNCTION: CMMF_CreateKeyRecRepContentFromDER
 * INPUTS:
 *    db
 *        The handle for the certificate database where the decoded
 *        certificates will be placed.  The decoded certificates will
 *        be placed in the temporary database associated with the
 *        handle.
 *    buf
 *        A buffer contatining the DER-encoded CMMFKeyRecRepContent
 *    len
 *        The length in bytes of the buffer 'buf'
 * NOTES
 *    This function passes the buffer to the ASN1 decoder and creates a
 *    CMMFKeyRecRepContent structure.
 *
 * RETURN:
 *    A pointer to the CMMFKeyRecRepContent structure.  A NULL return
 *    value indicates the library was unable to parse the DER.
 */
extern CMMFKeyRecRepContent *
CMMF_CreateKeyRecRepContentFromDER(CERTCertDBHandle *db,
                                   const char *buf,
                                   long len);

/*
 * FUNCTION: CMMF_CreatePOPODecKeyChallContent
 * INPUTS:
 *    NONE
 * NOTES:
 *    This function creates an empty CMMFPOPODecKeyChallContent.  The user
 *    must add the challenges individually specifying the random number to
 *    be used and the public key to be used when creating each individual
 *    challenge.  User can accomplish this by calling the function
 *    CMMF_POPODecKeyChallContentSetNextChallenge.
 * RETURN:
 *    A pointer to a CMMFPOPODecKeyChallContent structure.  Ther user can
 *    then call CMMF_EncodePOPODecKeyChallContent passing in the return
 *    value from this function after setting all of the challenges.  A
 *    return value of NULL indicates an error while creating the
 *    CMMFPOPODecKeyChallContent structure.
 */
extern CMMFPOPODecKeyChallContent *
CMMF_CreatePOPODecKeyChallContent(void);

/*
 * FUNCTION: CMMF_CreatePOPODecKeyChallContentFromDER
 * INPUTS
 *    buf
 *        A buffer containing the DER-encoded CMMFPOPODecKeyChallContent
 *    len
 *        The length in bytes of the buffer 'buf'
 * NOTES:
 *    This function passes the buffer to the ASN1 decoder and creates a
 *    CMMFPOPODecKeyChallContent structure.
 *
 * RETURN:
 *    A pointer to the CMMFPOPODecKeyChallContent structure.  A NULL return
 *    value indicates the library was unable to parse the DER.
 */
extern CMMFPOPODecKeyChallContent *
CMMF_CreatePOPODecKeyChallContentFromDER(const char *buf, long len);

/*
 * FUNCTION: CMMF_CreatePOPODecKeyRespContentFromDER
 * INPUTS:
 *    buf
 *        A buffer contatining the DER-encoded CMMFPOPODecKeyRespContent
 *    len
 *        The length in bytes of the buffer 'buf'
 * NOTES
 *    This function passes the buffer to the ASN1 decoder and creates a
 *    CMMFPOPODecKeyRespContent structure.
 *
 * RETURN:
 *    A pointer to the CMMFPOPODecKeyRespContent structure.  A NULL return
 *    value indicates the library was unable to parse the DER.
 */
extern CMMFPOPODecKeyRespContent *
CMMF_CreatePOPODecKeyRespContentFromDER(const char *buf, long len);

/************************** Set Functions *************************/

/*
 * FUNCTION: CMMF_CertRepContentSetCertResponses
 * INPUTS:
 *    inCertRepContent
 *        The CMMFCertRepContent to operate on.
 *    inCertResponses
 *        An array of pointers to CMMFCertResponse structures to
 *        add to the CMMFCertRepContent structure.
 *    inNumResponses
 *        The length of the array 'inCertResponses'
 * NOTES:
 *    This function will add the CMMFCertResponse structure to the
 *    CMMFCertRepContent passed in.  The CMMFCertResponse field of
 *    CMMFCertRepContent is required, so the client must call this function
 *    before calling CMMF_EncodeCertRepContent.  If the user calls
 *    CMMF_EncodeCertRepContent before calling this function,
 *    CMMF_EncodeCertRepContent will fail.
 *
 * RETURN:
 *    SECSuccess if adding the CMMFCertResponses to the CMMFCertRepContent
 *    structure was successful.  Any other return value indicates an error
 *    while trying to add the CMMFCertResponses.
 */
extern SECStatus
CMMF_CertRepContentSetCertResponses(CMMFCertRepContent *inCertRepContent,
                                    CMMFCertResponse **inCertResponses,
                                    int inNumResponses);

/*
 * FUNCTION: CMMF_CertRepContentSetCAPubs
 * INPUTS:
 *    inCertRepContent
 *        The CMMFCertRepContent to operate on.
 *    inCAPubs
 *        The certificate list which makes up the chain of CA certificates
 *        required to make the issued cert valid.
 * NOTES:
 *    This function will set the the certificates in the CA chain as part
 *    of the CMMFCertRepContent.  This field is an optional member of the
 *    CMMFCertRepContent structure, so the client is not required to call
 *    this function before calling CMMF_EncodeCertRepContent.
 *
 * RETURN:
 *    SECSuccess if adding the 'inCAPubs' to the CERTRepContent was successful.
 *    Any other return value indicates an error while adding 'inCAPubs' to the
 *    CMMFCertRepContent structure.
 *
 */
extern SECStatus
CMMF_CertRepContentSetCAPubs(CMMFCertRepContent *inCertRepContent,
                             CERTCertList *inCAPubs);

/*
 * FUNCTION: CMMF_CertResponseSetPKIStatusInfoStatus
 * INPUTS:
 *    inCertResp
 *        The CMMFCertResponse to operate on.
 *     inPKIStatus
 *        The value to set for the PKIStatusInfo.status field.
 * NOTES:
 *    This function will set the CertResponse.status.status field of
 *    the CMMFCertResponse structure.  (View the definition of CertResponse
 *    in the CMMF draft to see exactly which value this talks about.)  This
 *    field is a required member of the structure, so the user must call this
 *    function in order to have a CMMFCertResponse that can be encoded.
 *
 * RETURN:
 *    SECSuccess if setting the field with the passed in value was successful.
 *    Any other return value indicates an error while trying to set the field.
 */
extern SECStatus
CMMF_CertResponseSetPKIStatusInfoStatus(CMMFCertResponse *inCertResp,
                                        CMMFPKIStatus inPKIStatus);

/*
 * FUNCTION: CMMF_CertResponseSetCertificate
 * INPUTS:
 *    inCertResp
 *        The CMMFCertResponse to operate on.
 *    inCertificate
 *        The certificate to add to the
 *        CertResponse.CertifiedKeyPair.certOrEncCert.certificate field.
 * NOTES:
 *    This function will take the certificate and make it a member of the
 *    CMMFCertResponse.  The certificate should be the actual certificate
 *    being issued via the response.
 *
 * RETURN:
 *    SECSuccess if adding the certificate to the response was successful.
 *    Any other return value indicates an error in adding the certificate to
 *    the CertResponse.
 */
extern SECStatus
CMMF_CertResponseSetCertificate(CMMFCertResponse *inCertResp,
                                CERTCertificate *inCertificate);

/*
 * FUNCTION: CMMF_KeyRecRepContentSetPKIStatusInfoStatus
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent to operate on.
 *    inPKIStatus
 *        The value to set the PKIStatusInfo.status field to.
 * NOTES:
 *    This function sets the only required field for the KeyRecRepContent.
 *    In most cases, the user will set this field and other fields of the
 *    structure to properly create the CMMFKeyRecRepContent structure.
 *    Refer to the CMMF draft to see which fields need to be set in order
 *    to create the desired CMMFKeyRecRepContent.
 *
 * RETURN:
 *    SECSuccess if setting the PKIStatusInfo.status field was successful.
 *    Any other return value indicates an error in setting the field.
 */
extern SECStatus
CMMF_KeyRecRepContentSetPKIStatusInfoStatus(CMMFKeyRecRepContent *inKeyRecRep,
                                            CMMFPKIStatus inPKIStatus);

/*
 * FUNCTION: CMMF_KeyRecRepContentSetNewSignCert
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent to operate on.
 *    inNewSignCert
 *        The new signing cert to add to the CMMFKeyRecRepContent structure.
 * NOTES:
 *    This function sets the new signeing cert in the CMMFKeyRecRepContent
 *    structure.
 *
 * RETURN:
 *    SECSuccess if setting the new signing cert was successful.  Any other
 *    return value indicates an error occurred while trying to add the
 *    new signing certificate.
 */
extern SECStatus
CMMF_KeyRecRepContentSetNewSignCert(CMMFKeyRecRepContent *inKeyRecRep,
                                    CERTCertificate *inNewSignCert);

/*
 * FUNCTION: CMMF_KeyRecRepContentSetCACerts
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent to operate on.
 *    inCACerts
 *        The list of CA certificates required to construct a valid
 *        certificate chain with the certificates that will be returned
 *        to the end user via this KeyRecRepContent.
 * NOTES:
 *    This function sets the caCerts that are required to form a chain with the
 *    end entity certificates that are being re-issued in this
 *    CMMFKeyRecRepContent structure.
 *
 * RETURN:
 *    SECSuccess if adding the caCerts was successful.  Any other return value
 *    indicates an error while tring to add the caCerts.
 */
extern SECStatus
CMMF_KeyRecRepContentSetCACerts(CMMFKeyRecRepContent *inKeyRecRep,
                                CERTCertList *inCACerts);

/*
 * FUNCTION: CMMF_KeyRecRepContentSetCertifiedKeyPair
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent to operate on.
 *    inCert
 *        The certificate to add to the CMMFKeyRecRepContent structure.
 *    inPrivKey
 *        The private key associated with the certificate above passed in.
 *    inPubKey
 *        The public key to use for wrapping the private key.
 * NOTES:
 *    This function adds another certificate-key pair to the
 *    CMMFKeyRecRepcontent structure.  There may be more than one
 *    certificate-key pair in the structure, so the user must call this
 *    function multiple times to add more than one cert-key pair.
 *
 * RETURN:
 *    SECSuccess if adding the certified key pair was successful.  Any other
 *    return value indicates an error in adding certified key pair to
 *    CMMFKeyRecRepContent structure.
 */
extern SECStatus
CMMF_KeyRecRepContentSetCertifiedKeyPair(CMMFKeyRecRepContent *inKeyRecRep,
                                         CERTCertificate *inCert,
                                         SECKEYPrivateKey *inPrivKey,
                                         SECKEYPublicKey *inPubKey);

/*
 * FUNCTION: CMMF_POPODecKeyChallContentSetNextChallenge
 * INPUTS:
 *    inDecKeyChall
 *        The CMMFPOPODecKeyChallContent to operate on.
 *    inRandom
 *        The random number to use when generating the challenge,
 *    inSender
 *        The GeneralName representation of the sender of the challenge.
 *    inPubKey
 *        The public key to use when encrypting the challenge.
 *    passwdArg
 *        This value will be passed to the function used for getting a
 *        password.  The password for getting a password should be registered
 *        by calling PK11_SetPasswordFunc before this function is called.
 *        If no password callback is registered and the library needs to
 *        authenticate to the slot for any reason, this function will fail.
 * NOTES:
 *    This function adds a challenge to the end of the list of challenges
 *    contained by 'inDecKeyChall'.  Refer to the CMMF draft on how the
 *    the random number passed in and the sender's GeneralName are used
 *    to generate the challenge and witness fields of the challenge.  This
 *    library will use SHA1 as the one-way function for generating the
 *    witess field of the challenge.
 *
 * RETURN:
 *    SECSuccess if generating the challenge and adding to the end of list
 *    of challenges was successful.  Any other return value indicates an error
 *    while trying to generate the challenge.
 */
extern SECStatus
CMMF_POPODecKeyChallContentSetNextChallenge(CMMFPOPODecKeyChallContent *inDecKeyChall,
                                            long inRandom,
                                            CERTGeneralName *inSender,
                                            SECKEYPublicKey *inPubKey,
                                            void *passwdArg);

/************************** Encoding Functions *************************/

/*
 * FUNCTION: CMMF_EncodeCertRepContent
 * INPUTS:
 *    inCertRepContent
 *        The CMMFCertRepContent to DER-encode.
 *    inCallback
 *        A callback function that the ASN1 encoder will call whenever it
 *        wants to write out DER-encoded bytes.  Look at the defintion of
 *        CRMFEncoderOutputCallback in crmft.h for a description of the
 *        parameters to the function.
 *    inArg
 *        An opaque pointer to a user-supplied argument that will be passed
 *        to the callback funtion whenever the function is called.
 * NOTES:
 *    The CMMF library will use the same DER-encoding scheme as the CRMF
 *    library.  In other words, when reading CRMF comments that pertain to
 *    encoding, those comments apply to the CMMF libray as well.
 *    The callback function will be called multiple times, each time supplying
 *    the next chunk of DER-encoded bytes.  The user must concatenate the
 *    output of each successive call to the callback in order to get the
 *    entire DER-encoded CMMFCertRepContent structure.
 *
 * RETURN:
 *    SECSuccess if encoding the CMMFCertRepContent was successful.  Any
 *    other return value indicates an error while decoding the structure.
 */
extern SECStatus
CMMF_EncodeCertRepContent(CMMFCertRepContent *inCertRepContent,
                          CRMFEncoderOutputCallback inCallback,
                          void *inArg);

/*
 * FUNCTION: CMMF_EncodeKeyRecRepContent
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRepContent to DER-encode.
 *    inCallback
 *        A callback function that the ASN1 encoder will call whenever it
 *        wants to write out DER-encoded bytes.  Look at the defintion of
 *        CRMFEncoderOutputCallback in crmft.h for a description of the
 *        parameters to the function.
 *    inArg
 *        An opaque pointer to a user-supplied argument that will be passed
 *        to the callback funtion whenever the function is called.
 * NOTES:
 *    The CMMF library will use the same DER-encoding scheme as the CRMF
 *    library.  In other words, when reading CRMF comments that pertain to
 *    encoding, those comments apply to the CMMF libray as well.
 *    The callback function will be called multiple times, each time supplying
 *    the next chunk of DER-encoded bytes.  The user must concatenate the
 *    output of each successive call to the callback in order to get the
 *    entire DER-encoded CMMFCertRepContent structure.
 *
 * RETURN:
 *    SECSuccess if encoding the CMMFKeyRecRepContent was successful.  Any
 *    other return value indicates an error while decoding the structure.
 */
extern SECStatus
CMMF_EncodeKeyRecRepContent(CMMFKeyRecRepContent *inKeyRecRep,
                            CRMFEncoderOutputCallback inCallback,
                            void *inArg);

/*
 * FUNCTION: CMMF_EncodePOPODecKeyChallContent
 * INPUTS:
 *    inDecKeyChall
 *        The CMMFDecKeyChallContent to operate on.
 *    inCallback
 *        A callback function that the ASN1 encoder will call whenever it
 *        wants to write out DER-encoded bytes.  Look at the defintion of
 *        CRMFEncoderOutputCallback in crmft.h for a description of the
 *        parameters to the function.
 *    inArg
 *        An opaque pointer to a user-supplied argument that will be passed
 *        to the callback function whenever the function is called.
 * NOTES:
 *    The CMMF library will use the same DER-encoding scheme as the CRMF
 *    library.  In other words, when reading CRMF comments that pertain to
 *    encoding, those comments apply to the CMMF libray as well.
 *    The callback function will be called multiple times, each time supplying
 *    the next chunk of DER-encoded bytes.  The user must concatenate the
 *    output of each successive call to the callback in order to get the
 *    entire DER-encoded CMMFCertRepContent structure.
 *    The DER will be an encoding of the type POPODecKeyChallContents, which
 *    is just a sequence of challenges.
 *
 * RETURN:
 *    SECSuccess if encoding was successful.  Any other return value indicates
 *    an error in trying to encode the Challenges.
 */
extern SECStatus
CMMF_EncodePOPODecKeyChallContent(CMMFPOPODecKeyChallContent *inDecKeyChall,
                                  CRMFEncoderOutputCallback inCallback,
                                  void *inArg);

/*
 * FUNCTION: CMMF_EncodePOPODecKeyRespContent
 * INPUTS:
 *    inDecodedRand
 *        An array of integers to encode as the responses to
 *        CMMFPOPODecKeyChallContent.  The integers must be in the same order
 *        as the challenges extracted from CMMFPOPODecKeyChallContent.
 *    inNumRand
 *        The number of random integers contained in the array 'inDecodedRand'
 *    inCallback
 *        A callback function that the ASN1 encoder will call whenever it
 *        wants to write out DER-encoded bytes.  Look at the defintion of
 *        CRMFEncoderOutputCallback in crmft.h for a description of the
 *        parameters to the function.
 *    inArg
 *        An opaque pointer to a user-supplied argument that will be passed
 *        to the callback funtion whenever the function is called.
 * NOTES:
 *    The CMMF library will use the same DER-encoding scheme as the CRMF
 *    library.  In other words, when reading CRMF comments that pertain to
 *    encoding, those comments apply to the CMMF libray as well.
 *    The callback function will be called multiple times, each time supplying
 *    the next chunk of DER-encoded bytes.  The user must concatenate the
 *    output of each successive call to the callback in order to get the
 *    entire DER-encoded  POPODecKeyRespContent.
 *
 * RETURN:
 *    SECSuccess if encoding was successful.  Any other return value indicates
 *    an error in trying to encode the Challenges.
 */
extern SECStatus
CMMF_EncodePOPODecKeyRespContent(long *inDecodedRand,
                                 int inNumRand,
                                 CRMFEncoderOutputCallback inCallback,
                                 void *inArg);

/***************  Accessor function  ***********************************/

/*
 * FUNCTION: CMMF_CertRepContentGetCAPubs
 * INPUTS:
 *    inCertRepContent
 *        The CMMFCertRepContent to extract the caPubs from.
 * NOTES:
 *    This function will return a copy of the list of certificates that
 *    make up the chain of CA's required to make the cert issued valid.
 *    The user must call CERT_DestroyCertList on the return value when
 *    done using the return value.
 *
 *    Only call this function on a CertRepContent that has been decoded.
 *    The client must call CERT_DestroyCertList when the certificate list
 *    is no longer needed.
 *
 *    The certs in the list will not be in the temporary database.  In order
 *    to make these certificates a part of the permanent CA internal database,
 *    the user must collect the der for all of these certs and call
 *    CERT_ImportCAChain.  Afterwards the certs will be part of the permanent
 *    database.
 *
 * RETURN:
 *    A pointer to the CERTCertList representing the CA chain associated
 *    with the issued cert.  A NULL return value indicates  that no CA Pubs
 *    were available in the CMMFCertRepContent structure.
 */
extern CERTCertList *
CMMF_CertRepContentGetCAPubs(CMMFCertRepContent *inCertRepContent);

/*
 * FUNCTION: CMMF_CertRepContentGetNumResponses
 * INPUTS:
 *    inCertRepContent
 *        The CMMFCertRepContent to operate on.
 * NOTES:
 *    This function will return the number of CertResponses that are contained
 *    by the CMMFCertRepContent passed in.
 *
 * RETURN:
 *    The number of CMMFCertResponses contained in the structure passed in.
 */
extern int
CMMF_CertRepContentGetNumResponses(CMMFCertRepContent *inCertRepContent);

/*
 * FUNCTION: CMMF_CertRepContentGetResponseAtIndex
 * INPUTS:
 *    inCertRepContent
 *        The CMMFCertRepContent to operate on.
 *    inIndex
 *        The index of the CMMFCertResponse the user wants a copy of.
 * NOTES:
 *    This function creates a copy of the CMMFCertResponse at the index
 *    corresponding to the parameter 'inIndex'.  Indexing is done like a
 *    traditional C array, ie the valid indexes are (0...numResponses-1).
 *    The user must call CMMF_DestroyCertResponse after the return value is
 *    no longer needed.
 *
 * RETURN:
 *    A pointer to the CMMFCertResponse at the index corresponding to
 *    'inIndex'.  A return value of NULL indicates an error in copying
 *    the CMMFCertResponse.
 */
extern CMMFCertResponse *
CMMF_CertRepContentGetResponseAtIndex(CMMFCertRepContent *inCertRepContent,
                                      int inIndex);

/*
 * FUNCTION: CMMF_CertResponseGetCertReqId
 * INPUTS:
 *    inCertResp
 *        The CMMFCertResponse to operate on.
 * NOTES:
 *    This function returns the CertResponse.certReqId from the
 *    CMMFCertResponse structure passed in.  If the return value is -1, that
 *    means there is no associated certificate request with the CertResponse.
 * RETURN:
 *    A long representing the id of the certificate request this
 *    CMMFCertResponse corresponds to.  A return value of -1 indicates an
 *    error in extracting the value of the integer.
 */
extern long CMMF_CertResponseGetCertReqId(CMMFCertResponse *inCertResp);

/*
 * FUNCTION: CMMF_CertResponseGetPKIStatusInfoStatus
 * INPUTS:
 *    inCertResp
 *        The CMMFCertResponse to operate on.
 * NOTES:
 *    This function returns the CertResponse.status.status field of the
 *    CMMFCertResponse structure.
 *
 * RETURN:
 *    The enumerated value corresponding to the PKIStatus defined in the CMMF
 *    draft.  See the CMMF draft for the definition of PKIStatus.  See crmft.h
 *    for the definition of CMMFPKIStatus.
 */
extern CMMFPKIStatus
CMMF_CertResponseGetPKIStatusInfoStatus(CMMFCertResponse *inCertResp);

/*
 * FUNCTION: CMMF_CertResponseGetCertificate
 * INPUTS:
 *    inCertResp
 *        The Certificate Response to operate on.
 *    inCertdb
 *        This is the certificate database where the function will place the
 *        newly issued certificate.
 * NOTES:
 *    This function retrieves the CertResponse.certifiedKeyPair.certificate
 *    from the CMMFCertResponse.  The user will get a copy of that certificate
 *    so  the user must call CERT_DestroyCertificate when the return value is
 *    no longer needed.  The certificate returned will be in the temporary
 *    certificate database.
 *
 * RETURN:
 *    A pointer to a copy of the certificate contained within the
 *    CMMFCertResponse.  A return value of NULL indicates an error while trying
 *    to make a copy of the certificate.
 */
extern CERTCertificate *
CMMF_CertResponseGetCertificate(CMMFCertResponse *inCertResp,
                                CERTCertDBHandle *inCertdb);

/*
 * FUNCTION: CMMF_KeyRecRepContentGetPKIStatusInfoStatus
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent structure to operate on.
 * NOTES:
 *    This function retrieves the KeyRecRepContent.status.status field of
 *    the CMMFKeyRecRepContent structure.
 * RETURN:
 *    The CMMFPKIStatus corresponding to the value held in the
 *    CMMFKeyRecRepContent structure.
 */
extern CMMFPKIStatus
CMMF_KeyRecRepContentGetPKIStatusInfoStatus(CMMFKeyRecRepContent *inKeyRecRep);

/*
 * FUNCTION: CMMF_KeyRecRepContentGetNewSignCert
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent to operate on.
 * NOTES:
 *    This function retrieves the KeyRecRepContent.newSignCert field of the
 *    CMMFKeyRecRepContent structure.  The user must call
 *    CERT_DestroyCertificate when the return value is no longer needed. The
 *    returned certificate will be in the temporary database.  The user
 *    must then place the certificate permanently in whatever token the
 *    user determines is the proper destination.  A return value of NULL
 *    indicates the newSigCert field was not present.
 */
extern CERTCertificate *
CMMF_KeyRecRepContentGetNewSignCert(CMMFKeyRecRepContent *inKeyRecRep);

/*
 * FUNCTION: CMMF_KeyRecRepContentGetCACerts
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent to operate on.
 * NOTES:
 *    This function returns a CERTCertList which contains all of the
 *    certficates that are in the sequence KeyRecRepContent.caCerts
 *    User must call CERT_DestroyCertList when the return value is no longer
 *    needed.  All of these certificates will be placed in the tempoaray
 *    database.
 *
 * RETURN:
 *    A pointer to the list of caCerts contained in the CMMFKeyRecRepContent
 *    structure.  A return value of NULL indicates the library was not able to
 *    make a copy of the certifcates.  This may be because there are no caCerts
 *    included in the CMMFKeyRecRepContent strucure or an internal error.  Call
 *    CMMF_KeyRecRepContentHasCACerts to find out if there are any caCerts
 *    included in 'inKeyRecRep'.
 */
extern CERTCertList *
CMMF_KeyRecRepContentGetCACerts(CMMFKeyRecRepContent *inKeyRecRep);

/*
 * FUNCTION: CMMF_KeyRecRepContentGetNumKeyPairs
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent to operate on.
 * RETURN:
 *    This function returns the number of CMMFCertifiedKeyPair structures that
 *    that are stored in the KeyRecRepContent structure.
 */
extern int
CMMF_KeyRecRepContentGetNumKeyPairs(CMMFKeyRecRepContent *inKeyRecRep);

/*
 * FUNCTION: CMMF_KeyRecRepContentGetCertKeyAtIndex
 * INPUTS:
 *    inKeyRecRepContent
 *        The CMMFKeyRecRepContent to operate on.
 *    inIndex
 *        The index of the desired CMMFCertifiedKeyPair
 * NOTES:
 *    This function retrieves the CMMFCertifiedKeyPair structure at the index
 *    'inIndex'.  Valid indexes are 0...(numKeyPairs-1)  The user must call
 *    CMMF_DestroyCertifiedKeyPair when the return value is no longer needed.
 *
 * RETURN:
 *    A pointer to the Certified Key Pair at the desired index.  A return value
 *    of NULL indicates an error in extracting the Certified Key Pair at the
 *    desired index.
 */
extern CMMFCertifiedKeyPair *
CMMF_KeyRecRepContentGetCertKeyAtIndex(CMMFKeyRecRepContent *inKeyRecRep,
                                       int inIndex);

/*
 * FUNCTION: CMMF_CertifiedKeyPairGetCertificate
 * INPUTS:
 *    inCertKeyPair
 *        The CMMFCertifiedKeyPair to operate on.
 *    inCertdb
 *        The database handle for the database you want this certificate
 *        to wind up in.
 * NOTES:
 *    This function retrieves the certificate at
 *    CertifiedKeyPair.certOrEncCert.certificate
 *    The user must call CERT_DestroyCertificate when the return value is no
 *    longer needed.  The user must import this certificate as a token object
 *    onto PKCS#11 slot in order to make it a permanent object.  The returned
 *    certificate will be in the temporary database.
 *
 * RETURN:
 *    A pointer to the certificate contained within the certified key pair.
 *    A return value of NULL indicates an error in creating the copy of the
 *    certificate.
 */
extern CERTCertificate *
CMMF_CertifiedKeyPairGetCertificate(CMMFCertifiedKeyPair *inCertKeyPair,
                                    CERTCertDBHandle *inCertdb);

/*
 * FUNCTION: CMMF_POPODecKeyChallContentGetNumChallenges
 * INPUTS:
 *    inKeyChallCont
 *        The CMMFPOPODecKeyChallContent to operate on.
 * RETURN:
 *    This function returns the number of CMMFChallenges are contained in
 *    the CMMFPOPODecKeyChallContent structure.
 */
extern int CMMF_POPODecKeyChallContentGetNumChallenges(CMMFPOPODecKeyChallContent *inKeyChallCont);

/*
 * FUNCTION: CMMF_POPODecKeyChallContentGetPublicValue
 * ---------------------------------------------------
 * INPUTS:
 *    inKeyChallCont
 *        The CMMFPOPODecKeyChallContent to operate on.
 *    inIndex
 *        The index of the Challenge within inKeyChallCont to operate on.
 *        Indexes start from 0, ie the Nth Challenge corresponds to index
 *        N-1.
 * NOTES:
 * This function retrieves the public value stored away in the Challenge at
 * index inIndex of inKeyChallCont.
 * RETURN:
 * A pointer to a SECItem containing the public value.  User must call
 * SECITEM_FreeItem on the return value when the value is no longer necessary.
 * A return value of NULL indicates an error while retrieving the public value.
 */
extern SECItem *CMMF_POPODecKeyChallContentGetPublicValue(CMMFPOPODecKeyChallContent *inKeyChallCont,
                                                          int inIndex);

/*
 * FUNCTION: CMMF_POPODecKeyChallContentGetRandomNumber
 * INPUTS:
 *    inChallContent
 *        The CMMFPOPODecKeyChallContent to operate on.
 *    inIndex
 *        The index of the challenge to look at.  Valid indexes are 0 through
 *        (CMMF_POPODecKeyChallContentGetNumChallenges(inChallContent) - 1).
 *    inDest
 *        A pointer to a user supplied buffer where the library
 *        can place a copy of the random integer contatained in the
 *        challenge.
 * NOTES:
 *    This function returns the value held in the decrypted Rand structure
 *    corresponding to the random integer.  The user must call
 *    CMMF_POPODecKeyChallContentDecryptChallenge before calling this function.  Call
 *    CMMF_ChallengeIsDecrypted to find out if the challenge has been
 *    decrypted.
 *
 * RETURN:
 *    SECSuccess indicates the witness field has been previously decrypted
 *    and the value for the random integer was successfully placed at *inDest.
 *    Any other return value indicates an error and that the value at *inDest
 *    is not a valid value.
 */
extern SECStatus CMMF_POPODecKeyChallContentGetRandomNumber(CMMFPOPODecKeyChallContent *inKeyChallCont,
                                                            int inIndex,
                                                            long *inDest);

/*
 * FUNCTION: CMMF_POPODecKeyRespContentGetNumResponses
 * INPUTS:
 *    inRespCont
 *        The POPODecKeyRespContent to operate on.
 * RETURN:
 * This function returns the number of responses contained in inRespContent.
 */
extern int
CMMF_POPODecKeyRespContentGetNumResponses(CMMFPOPODecKeyRespContent *inRespCont);

/*
 * FUNCTION: CMMF_POPODecKeyRespContentGetResponse
 * INPUTS:
 *    inRespCont
 *        The POPODecKeyRespContent to operate on.
 *    inIndex
 *        The index of the response to retrieve.
 *        The Nth response is at index N-1, ie the 1st response is at index 0,
 *        the 2nd response is at index 1, and so on.
 *    inDest
 *        A pointer to a pre-allocated buffer where the library can put the
 *        value of the response located at inIndex.
 * NOTES:
 * The function returns the response contained at index inIndex.
 * CMMFPOPODecKeyRespContent is a structure that the server will generally
 * get in response to a CMMFPOPODecKeyChallContent.  The server will expect
 * to see the responses in the same order as it constructed them in
 * the CMMFPOPODecKeyChallContent structure.
 * RETURN:
 * SECSuccess if getting the response at the desired index was successful.  Any
 * other return value indicates an errror.
 */
extern SECStatus
CMMF_POPODecKeyRespContentGetResponse(CMMFPOPODecKeyRespContent *inRespCont,
                                      int inIndex,
                                      long *inDest);

/************************* Destructor Functions ******************************/

/*
 * FUNCTION: CMMF_DestroyCertResponse
 * INPUTS:
 *    inCertResp
 *        The CMMFCertResponse to destroy.
 * NOTES:
 *    This function frees all the memory associated with the CMMFCertResponse
 *    passed in.
 * RETURN:
 *    SECSuccess if freeing the memory was successful.  Any other return value
 *    indicates an error while freeing the memory.
 */
extern SECStatus CMMF_DestroyCertResponse(CMMFCertResponse *inCertResp);

/*
 * FUNCTION: CMMF_DestroyCertRepContent
 * INPUTS:
 *    inCertRepContent
 *        The CMMFCertRepContent to destroy
 * NOTES:
 *    This function frees the memory associated with the CMMFCertRepContent
 *    passed in.
 * RETURN:
 *    SECSuccess if freeing all the memory associated with the
 *    CMMFCertRepContent passed in is successful.  Any other return value
 *    indicates an error while freeing the memory.
 */
extern SECStatus
CMMF_DestroyCertRepContent(CMMFCertRepContent *inCertRepContent);

/*
 * FUNCTION: CMMF_DestroyKeyRecRepContent
 * INPUTS:
 *    inKeyRecRep
 *        The CMMFKeyRecRepContent to destroy.
 * NOTES:
 *    This function destroys all the memory associated with the
 *    CMMFKeyRecRepContent passed in.
 *
 * RETURN:
 *    SECSuccess if freeing all the memory is successful.  Any other return
 *    value indicates an error in freeing the memory.
 */
extern SECStatus
CMMF_DestroyKeyRecRepContent(CMMFKeyRecRepContent *inKeyRecRep);

/*
 * FUNCTION: CMMF_DestroyCertifiedKeyPair
 * INPUTS:
 *    inCertKeyPair
 *        The CMMFCertifiedKeyPair to operate on.
 * NOTES:
 *    This function frees up all the memory associated with 'inCertKeyPair'
 *
 * RETURN:
 *    SECSuccess if freeing all the memory associated with 'inCertKeyPair'
 *    is successful.  Any other return value indicates an error while trying
 *    to free the memory.
 */
extern SECStatus
CMMF_DestroyCertifiedKeyPair(CMMFCertifiedKeyPair *inCertKeyPair);

/*
 * FUNCTION: CMMF_DestroyPOPODecKeyRespContent
 * INPUTS:
 *    inDecKeyResp
 *        The CMMFPOPODecKeyRespContent structure to free.
 * NOTES:
 *    This function frees up all the memory associate with the
 *    CMMFPOPODecKeyRespContent.
 *
 * RETURN:
 *    SECSuccess if freeing up all the memory associated with the
 *    CMMFPOPODecKeyRespContent structure is successful.  Any other
 *    return value indicates an error while freeing the memory.
 */
extern SECStatus
CMMF_DestroyPOPODecKeyRespContent(CMMFPOPODecKeyRespContent *inDecKeyResp);

/************************** Miscellaneous Functions *************************/

/*
 * FUNCTION: CMMF_CertifiedKeyPairUnwrapPrivKey
 * INPUTS:
 *    inCertKeyPair
 *        The CMMFCertifiedKeyPair to operate on.
 *    inPrivKey
 *        The private key to use to un-wrap the private key
 *    inNickName
 *        This is the nickname that will be associated with the private key
 *        to be unwrapped.
 *    inSlot
 *        The PKCS11 slot where the unwrapped private key should end up.
 *    inCertdb
 *        The Certificate database with which the new key will be associated.
 *    destPrivKey
 *        A pointer to memory where the library can place a pointer to the
 *        private key after importing the key onto the specified slot.
 *    wincx
 *        An opaque pointer that the library will use in a callback function
 *        to get the password if necessary.
 *
 * NOTES:
 *    This function uses the private key passed in to unwrap the private key
 *    contained within the CMMFCertifiedKeyPair structure. After this
 *    function successfully returns, the private key has been unwrapped and
 *    placed in the specified slot.
 *
 * RETURN:
 *    SECSuccess if unwrapping the private key was successful.  Any other
 *    return value indicates an error while trying to un-wrap the private key.
 */
extern SECStatus
CMMF_CertifiedKeyPairUnwrapPrivKey(CMMFCertifiedKeyPair *inKeyPair,
                                   SECKEYPrivateKey *inPrivKey,
                                   SECItem *inNickName,
                                   PK11SlotInfo *inSlot,
                                   CERTCertDBHandle *inCertdb,
                                   SECKEYPrivateKey **destPrivKey,
                                   void *wincx);

/*
 * FUNCTION: CMMF_KeyRecRepContentHasCACerts
 * INPUTS:
 *    inKeyRecRecp
 *        The CMMFKeyRecRepContent to operate on.
 * RETURN:
 *    This function returns PR_TRUE if there are one or more certificates in
 *    the sequence KeyRecRepContent.caCerts within the CMMFKeyRecRepContent
 *    structure.  The function will return PR_FALSE if there are 0 certificate
 *    in the above mentioned sequence.
 */
extern PRBool
CMMF_KeyRecRepContentHasCACerts(CMMFKeyRecRepContent *inKeyRecRep);

/*
 * FUNCTION: CMMF_POPODecKeyChallContDecryptChallenge
 * INPUTS:
 *    inChalCont
 *        The CMMFPOPODecKeyChallContent to operate on.
 *    inIndex
 *        The index of the Challenge to operate on.  The 1st Challenge is
 *        at index 0, the second at index 1 and so forth.
 *    inPrivKey
 *        The private key to use to decrypt the witness field.
 * NOTES:
 *    This function uses the private key to decrypt the challenge field
 *    contained in the appropriate challenge.  Make sure the private key matches
 *    the public key that was used to encrypt the witness.  Use
 *    CMMF_POPODecKeyChallContentGetPublicValue to get the public value of
 *    the key used to encrypt the witness and then use that to determine the
 *    appropriate private key.  This can be done by calling PK11_MakeIDFromPubKey
 *    and then passing that return value to PK11_FindKeyByKeyID.  The creator of
 *    the challenge will most likely be an RA that has the public key
 *    from a Cert request.  So the private key should be the private key
 *    associated with public key in that request.  This function will also
 *    verify the witness field of the challenge.  This function also verifies
 *    that the sender and witness hashes match within the challenge.
 *
 * RETURN:
 *    SECSuccess if decrypting the witness field was successful.  This does
 *    not indicate that the decrypted data is valid, since the private key
 *    passed in may not be the actual key needed to properly decrypt the
 *    witness field.  Meaning that there is a decrypted structure now, but
 *    may be garbage because the private key was incorrect.
 *    Any other return value indicates the function could not complete the
 *    decryption process.
 */
extern SECStatus
CMMF_POPODecKeyChallContDecryptChallenge(CMMFPOPODecKeyChallContent *inChalCont,
                                         int inIndex,
                                         SECKEYPrivateKey *inPrivKey);

/*
 * FUNCTION: CMMF_DestroyPOPODecKeyChallContent
 * INPUTS:
 *    inDecKeyCont
 *        The CMMFPOPODecKeyChallContent to free
 * NOTES:
 *    This function frees up all the memory associated with the
 *    CMMFPOPODecKeyChallContent
 * RETURN:
 *    SECSuccess if freeing up all the memory associatd with the
 *    CMMFPOPODecKeyChallContent is successful.  Any other return value
 *    indicates an error while freeing the memory.
 *
 */
extern SECStatus
CMMF_DestroyPOPODecKeyChallContent(CMMFPOPODecKeyChallContent *inDecKeyCont);

SEC_END_PROTOS
#endif /* _CMMF_H_ */
