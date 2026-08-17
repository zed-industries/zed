/* -*- Mode: C; tab-width: 8 -*-*/
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _CRMF_H_
#define _CRMF_H_

#include "seccomon.h"
#include "cert.h"
#include "crmft.h"
#include "secoid.h"
#include "secpkcs7.h"

SEC_BEGIN_PROTOS

/*
 * FUNCTION: CRMF_EncodeCertReqMsg
 * INPUTS:
 *    inCertReqMsg
 *        The Certificate Request Message to be encoded.
 *    fn
 *        A Callback function that the ASN1 encoder calls whenever
 *        the encoder wants to write out some DER encoded bytes.
 *    arg
 *        An opaque pointer that gets passed to the function fn
 * OUTPUT:
 *    The function fn will be called multiple times.  Look at the
 *    comments in crmft.h where the CRMFEncoderOutputCallback type is
 *    defined for information on proper behavior of the function fn.
 * RETURN:
 *    SECSuccess if encoding was successful.  Any other return value
 *    indicates an error occurred during encoding.
 */
extern SECStatus
CRMF_EncodeCertReqMsg(CRMFCertReqMsg *inCertReqMsg,
                      CRMFEncoderOutputCallback fn,
                      void *arg);

/*
 * FUNCTION: CRMF_EncoderCertRequest
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to be encoded.
 *    fn
 *        A Callback function that the ASN1 encoder calls whenever
 *        the encoder wants to write out some DER encoded bytes.
 *    arg
 *        An opaque pointer that gets passed to the function fn.
 * OUTPUT:
 *    The function fn will be called, probably multiple times whenever
 *    the ASN1 encoder wants to write out DER-encoded bytes.  Look at the
 *    comments in crmft.h where the CRMFEncoderOutputCallback type is
 *    defined for information on proper behavior of the function fn.
 * RETURN:
 *    SECSuccess if encoding was successful.  Any other return value
 *    indicates an error occurred during encoding.
 */
extern SECStatus CRMF_EncodeCertRequest(CRMFCertRequest *inCertReq,
                                        CRMFEncoderOutputCallback fn,
                                        void *arg);
/*
 * FUNCTION: CRMF_EncodeCertReqMessages
 * INPUTS:
 *    inCertReqMsgs
 *        An array of pointers to the Certificate Request Messages
 *        to encode.  The user must place a NULL pointer in the index
 *        after the last message to be encoded.  When the library runs
 *        into the NULL pointer, the library assumes there are no more
 *        messages to encode.
 *    fn
 *        A Callback function that the ASN1 encoder calls whenever
 *        the encoder wants to write out some DER encoded byts.
 *    arg
 *        An opaque pointer that gets passed to the function fn.
 *
 * NOTES:
 *    The parameter inCertReqMsgs needs to be an array with a NULL pointer
 *    to signal the end of messages.  An array in the form of
 *    {m1, m2, m3, NULL, m4, ...} will only encode the messages m1, m2, and
 *    m3.  All messages from m4 on will not be looked at by the library.
 *
 * OUTPUT:
 *    The function fn will be called, probably multiple times.  Look at the
 *    comments in crmft.h where the CRMFEncoderOutputCallback type is
 *    defined for information on proper behavior of the function fn.
 *
 * RETURN:
 * SECSuccess if encoding the Certificate Request Messages was successful.
 * Any other return value indicates an error occurred while encoding the
 * certificate request messages.
 */
extern SECStatus
CRMF_EncodeCertReqMessages(CRMFCertReqMsg **inCertReqMsgs,
                           CRMFEncoderOutputCallback fn,
                           void *arg);

/*
 * FUNCTION: CRMF_CreateCertReqMsg
 * INPUTS:
 *    NONE
 * OUTPUT:
 *    An empty CRMF Certificate Request Message.
 *    Before encoding this message, the user must set
 *    the ProofOfPossession field and the certificate
 *    request which are necessary for the full message.
 *    After the user no longer needs this CertReqMsg,
 *    the user must call CRMF_DestroyCertReqMsg to free
 *    all memory associated with the Certificate Request
 *    Message.
 * RETURN:
 *    A pointer to a Certificate Request Message.  The user
 *    must pass the return value of this function to
 *    CRMF_DestroyCertReqMsg after the Certificate Request
 *    Message is no longer necessary.
 */
extern CRMFCertReqMsg *CRMF_CreateCertReqMsg(void);

/*
 * FUNCTION: CRMF_DestroyCertReqMsg
 * INPUTS:
 *    inCertReqMsg
 *        The Certificate Request Message to destroy.
 *  NOTES:
 *    This function frees all the memory used for the Certificate
 *    Request Message and all the memory used in making copies of
 *    fields of elelments of the message, eg. the Proof Of Possession
 *    filed and the Cetificate Request.
 * RETURN:
 *    SECSuccess if destruction was successful.  Any other return value
 *    indicates an error while trying to free the memory associated
 *    with inCertReqMsg.
 *
 */
extern SECStatus CRMF_DestroyCertReqMsg(CRMFCertReqMsg *inCertReqMsg);

/*
 * FUNCTION: CRMF_CertReqMsgSetCertRequest
 * INPUTS:
 *    inCertReqMsg
 *        The Certificate Request Message that the function will set
 *        the certificate request for.
 *    inCertReq
 *        The Certificate Request that will be added to the Certificate
 *        Request Message.
 * NOTES:
 *    This function will make a copy of the Certificate Request passed in
 *    and store it as part of the Certificate Request Message.  Therefore,
 *    the user must not call this function until the Certificate Request
 *    has been fully built and is ready to be encoded.
 * RETURN:
 *    SECSuccess
 *        If copying the Certificate as a member of the Certificate
 *        request message was successful.
 *    Any other return value indicates a failure to copy the Certificate
 *    Request and make it a part of the Certificate Request Message.
 */
extern SECStatus CRMF_CertReqMsgSetCertRequest(CRMFCertReqMsg *inCertReqMsg,
                                               CRMFCertRequest *inCertReq);

/*
 * FUNCTION: CRMF_CreateCertRequest
 * INPUTS:
 *    inRequestID
 *        The ID that will be associated with this certificate request.
 * OUTPUTS:
 *    A certificate request which only has the requestID set.
 * NOTES:
 *    The user must call the function CRMF_DestroyCertRequest when
 *    the returned value is no longer needed.  This is usually the
 *    case after fully constructing the Certificate Request and then
 *    calling the function CRMF_CertReqMsgSetCertRequest.
 * RETURN:
 *    A pointer to the new Certificate Request.  A NULL return value
 *    indicates an error in creating the Certificate Request.
 */
extern CRMFCertRequest *CRMF_CreateCertRequest(PRUint32 inRequestID);

/*
 * FUNCTION: CRMF_DestroyCertRequest
 * INPUTS:
 *    inCertReq
 *        The Certificate Request that will be destroyed.
 * RETURN:
 *    SECSuccess
 *        If freeing the memory associated with the certificate request
 *        was successful.
 *    Any other return value indicates an error while trying to free the
 *    memory.
 */
extern SECStatus CRMF_DestroyCertRequest(CRMFCertRequest *inCertReq);

/*
 * FUNCTION: CRMF_CreateCertExtension
 * INPUTS:
 *    id
 *        The SECOidTag to associate with this CertExtension.  This must
 *        correspond to a valid Certificate Extension, if not the function
 *        will fail.
 *    isCritical
 *        A boolean value stating if the extension value is crtical.  PR_TRUE
 *        means the value is crtical.  PR_FALSE indicates the value is not
 *        critical.
 *    data
 *        This is the data associated with the extension.  The user of the
 *        library is responsible for making sure the value passed in is a
 *        valid interpretation of the certificate extension.
 * NOTES:
 * Use this function to create CRMFCertExtension Structures which will
 * then be passed to CRMF_AddFieldToCertTemplate as part of the
 * CRMFCertCreationInfo.extensions  The user must call
 * CRMF_DestroyCertExtension after the extension has been added to a certifcate
 * and the extension is no longer needed.
 *
 * RETURN:
 * A pointer to a newly created CertExtension.  A return value of NULL
 * indicates the id passed in was an invalid certificate extension.
 */
extern CRMFCertExtension *CRMF_CreateCertExtension(SECOidTag id,
                                                   PRBool isCritical,
                                                   SECItem *data);

/*
 * FUNCTION: CMRF_DestroyCertExtension
 * INPUTS:
 *    inExtension
 *        The Cert Extension to destroy
 * NOTES:
 * Destroy a structure allocated by CRMF_CreateCertExtension.
 *
 * RETURN:
 * SECSuccess if freeing the memory associated with the certificate extension
 * was successful.  Any other error indicates an error while freeing the
 * memory.
 */
extern SECStatus CRMF_DestroyCertExtension(CRMFCertExtension *inExtension);

/*
 * FUNCTION: CRMF_CertRequestSetTemplateField
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    inTemplateField
 *        An enumeration that indicates which field of the Certificate
 *        template to add.
 *    data
 *        A generic pointer that will be type cast according to the
 *        table under NOTES and used as the key for adding to the
 *        certificate template;
 * NOTES:
 *
 * Below is a table that tells what type to pass in as data
 * depending on the template field one wants to set.
 *
 * Look in crmft.h for the definition of CRMFCertTemplateField.
 *
 * In all cases, the library makes copies of the data passed in.
 *
 *   CRMFCertTemplateField    Type of data    What data means
 *   ---------------------    ------------    ---------------
 *   crmfVersion              long *          The version of
 *                                            the certificate
 *                                            to be created.
 *
 *   crmfSerialNumber         long *          The serial number
 *                                            for the cert to be
 *                                            created.
 *
 *   crmfSigningAlg           SECAlgorithm *  The ASN.1 object ID for
 *                                            the algorithm used in encoding
 *                                            the certificate.
 *
 *   crmfIssuer               CERTName *      Certificate Library
 *                                            representation of the ASN1 type
 *                                            Name from X.509
 *
 *   crmfValidity     CRMFValidityCreationInfo *  At least one of the two
 *                                                fields in the structure must
 *                                                be present.  A NULL pointer
 *                                                in the structure indicates
 *                                                that member should not be
 *                                                added.
 *
 *   crmfSubject              CERTName *      Certificate Library
 *                                            representation of the ASN1 type
 *                                            Name from X.509
 *
 *   crmfPublicKey    CERTSubjectPublicKeyInfo *  The public key info for the
 *                                                certificate being requested.
 *
 *   crmfIssuerUID            SECItem *           A bit string representation
 *                                                of the issuer UID. NOTE: The
 *                                                length is the number of bits
 *                                                and not the number of bytes.
 *
 *   crmfSubjectUID           SECItem*            A bit string representation
 *                                                of the subject UID. NOTE: The
 *                                                length is the number of bits
 *                                                and not the number of bytes.
 *
 *   crmfExtension   CRMFCertExtCreationInfo *     A pointer to the structure
 *                                                 populated with an array of
 *                                                 of certificate extensions
 *                                                 and an integer that tells
 *                                                 how many elements are in the
 *                                                 array. Look in crmft.h for
 *                                                 the definition of
 *                                                 CRMFCertExtCreationInfo
 * RETURN:
 *    SECSuccess if adding the desired field to the template was successful.
 *    Any other return value indicates failure when trying to add the field
 *    to the template.
 *
 */
extern SECStatus
CRMF_CertRequestSetTemplateField(CRMFCertRequest *inCertReq,
                                 CRMFCertTemplateField inTemplateField,
                                 void *data);

/*
 * FUNCTION: CRMF_CertRequestIsFieldPresent
 * INPUTS:
 *    inCertReq
 *        The certificate request to operate on.
 *    inTemplateField
 *        The enumeration for the template field the user wants to query
 *        about.
 * NOTES:
 * This function checks to see if the the field associated with inTemplateField
 * enumeration is already present in the certificate request passed in.
 *
 * RETURN:
 * The function returns PR_TRUE if the field associated with inTemplateField
 * is already present in the certificate request.  If the field is not present
 * the function returns PR_FALSE.
 */
extern PRBool
CRMF_CertRequestIsFieldPresent(CRMFCertRequest *inCertReq,
                               CRMFCertTemplateField inTemplateField);

/*
 * FUNCTION: CRMF_CertRequestIsControlPresent
 * INPUTS:
 *    inCertReq
 *        The certificate request to operate on.
 *    inControlType
 *        The type of control to look for.
 * NOTES:
 * This function looks at the control present in the certificate request
 * and returns PR_TRUE iff a control of type inControlType already exists.
 * The CRMF draft does not explicitly state that two controls of the same
 * type can not exist within the same request.  So the library will not
 * cause an error if you try to add a control and one of the same type
 * already exists.  It is up to the application to ensure that multiple
 * controls of the same type do not exist, if that is the desired behavior
 * by the application.
 *
 * RETURN:
 * The function returns PR_TRUE if a control of type inControlType already
 * exists in the certificate request.  If a control of type inControlType
 * does not exist, the function will return PR_FALSE.
 */
extern PRBool
CRMF_CertRequestIsControlPresent(CRMFCertRequest *inCertReq,
                                 CRMFControlType inControlType);

/*
 * FUNCTION: CRMF_CertRequestSetRegTokenControl
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    value
 *        The UTF8 value which will be the Registration Token Control
 *        for this Certificate Request.
 * NOTES:
 *    The library does no verification that the value passed in is
 *    a valid UTF8 value.  The caller must make sure of this in order
 *    to get an encoding that is valid.  The library will ultimately
 *    encode this value as it was passed in.
 * RETURN:
 *    SECSucces on successful addition of the Registration Token Control.
 *    Any other return value indicates an unsuccessful attempt to add the
 *    control.
 *
 */
extern SECStatus CRMF_CertRequestSetRegTokenControl(CRMFCertRequest *inCertReq,
                                                    SECItem *value);

/*
 * FUNCTION: CRMF_CertRequestSetAuthenticatorControl
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    value
 *        The UTF8 value that will become the Authenticator Control
 *        for the passed in Certificate Request.
 * NOTES:
 *    The library does no verification that the value passed in is
 *    a valid UTF8 value.  The caller must make sure of this in order
 *    to get an encoding that is valid.  The library will ultimately
 *    encode this value as it was passed in.
 * RETURN:
 *    SECSucces on successful addition of the Authenticator Control.
 *    Any other return value indicates an unsuccessful attempt to add the
 *    control.
 */
extern SECStatus
CRMF_CertRequestSetAuthenticatorControl(CRMFCertRequest *inCertReq,
                                        SECItem *value);

/*
 * FUNCTION: CRMF_CreateEncryptedKeyWithencryptedValue
 * INPUTS:
 *    inPrivKey
 *        This is the private key associated with a certificate that is
 *        being requested.  This structure will eventually wind up as
 *        a part of the PKIArchiveOptions Control.
 *    inCACert
 *        This is the certificate for the CA that will be receiving the
 *        certificate request for the private key passed in.
 * OUTPUT:
 *    A CRMFEncryptedKey that can ultimately be used as part of the
 *    PKIArchiveOptions Control.
 *
 * RETURN:
 *    A pointer to a CRMFEncyptedKey.  A NULL return value indicates an erro
 *    during the creation of the encrypted key.
 */
extern CRMFEncryptedKey *
CRMF_CreateEncryptedKeyWithEncryptedValue(SECKEYPrivateKey *inPrivKey,
                                          CERTCertificate *inCACert);

/*
 * FUNCTION: CRMF_DestroyEncryptedKey
 * INPUTS:
 *    inEncrKey
 *        The CRMFEncryptedKey to be destroyed.
 * NOTES:
 *    Frees all memory associated with the CRMFEncryptedKey passed in.
 * RETURN:
 *    SECSuccess if freeing the memory was successful.  Any other return
 *    value indicates an error while freeig the memroy.
 */
extern SECStatus CRMF_DestroyEncryptedKey(CRMFEncryptedKey *inEncrKey);

/*
 * FUNCTION: CRMF_CreatePKIArchiveOptions
 * INPUTS:
 *    inType
 *        An enumeration value indicating which option for
 *        PKIArchiveOptions to use.
 *    data
 *        A pointer that will be type-cast and de-referenced according
 *        to the table under NOTES.
 * NOTES:
 * A table listing what should be passed in as data
 * ------------------------------------------------
 *
 * inType                            data
 * ------                            ----
 * crmfEncryptedPrivateKey           CRMFEncryptedKey*
 * crmfKeyGenParameters              SECItem*(This needs to be an octet string)
 * crmfArchiveRemGenPrivKey          PRBool*
 *
 * RETURN:
 *    A pointer the a CRMFPKIArchiveOptions that can be added to a Certificate
 *    Request.  A NULL pointer indicates an error occurred while creating
 *    the CRMFPKIArchiveOptions Structure.
 */
extern CRMFPKIArchiveOptions *
CRMF_CreatePKIArchiveOptions(CRMFPKIArchiveOptionsType inType,
                             void *data);
/*
 * FUNCTION: CRMF_DestroyPKIArchiveOptions
 * INPUTS:
 *    inArchOpt
 *        A pointer to the CRMFPKIArchiveOptions structure to free.
 * NOTES:
 *    Will free all memory associated with 'inArchOpt'.
 * RETURN:
 *    SECSuccess if successful in freeing the memory used by 'inArchOpt'
 *    Any other return value indicates an error while freeing the memory.
 */
extern SECStatus
CRMF_DestroyPKIArchiveOptions(CRMFPKIArchiveOptions *inArchOpt);

/*
 * FUNCTION: CRMF_CertRequestSetPKIArchiveOptions
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to add the the options to.
 *    inOptions
 *        The Archive Options to add to the Certificate Request.
 * NOTES:
 *    Adds the PKIArchiveOption to the Certificate Request.  This is what
 *    enables Key Escrow to take place through CRMF.  The library makes
 *    its own copy of the information.
 * RETURN:
 *    SECSuccess if successful in adding the ArchiveOptions to the Certificate
 *    request.  Any other return value indicates an error when trying to add
 *    the Archive Options  to the Certificate Request.
 */
extern SECStatus
CRMF_CertRequestSetPKIArchiveOptions(CRMFCertRequest *inCertReq,
                                     CRMFPKIArchiveOptions *inOptions);

/*
 * FUNCTION: CRMF_CertReqMsgGetPOPType
 * INPUTS:
 *    inCertReqMsg
 *        The Certificate Request Message to operate on.
 * NOTES:
 *    Returns an enumeration value indicating the method of Proof
 *    of Possession that was used for the passed in Certificate Request
 *    Message.
 * RETURN:
 *    An enumeration indicating what method for Proof Of Possession is
 *    being used in this Certificate Request Message.  Look in the file
 *    crmft.h for the definition of CRMFPOPChoice for the possible return
 *    values.
 */
extern CRMFPOPChoice CRMF_CertReqMsgGetPOPType(CRMFCertReqMsg *inCertReqMsg);

/*
 * FUNCTION: CRMF_CertReqMsgSetRAVerifiedPOP
 * INPUT:
 *    InCertReqMsg
 *        The Certificate Request Message to operate on.
 * NOTES:
 *    This function will set the method of Proof Of Possession to
 *    crmfRAVerified which means the RA has already verified the
 *    requester does possess the private key.
 * RETURN:
 *    SECSuccess if adding RAVerified to the message is successful.
 *    Any other message indicates an error while trying to add RAVerified
 *    as the Proof of Possession.
 */
extern SECStatus CRMF_CertReqMsgSetRAVerifiedPOP(CRMFCertReqMsg *inCertReqMsg);

/*
 * FUNCTION: CRMF_CertReqMsgSetSignaturePOP
 * INPUT:
 *    inCertReqMsg
 *        The Certificate Request Message to add the SignaturePOP to.
 *    inPrivKey
 *        The Private Key which corresponds to the the Certificate Request
 *        Message.
 *    inPubKey
 *        The Public Key which corresponds to the Private Key passed in.
 *    inCertForInput
 *        A Certificate that in the future may be used to create
 *        POPOSigningKeyInput.
 *    fn
 *        A callback for retrieving a password which may be used in the
 *       future to generate POPOSigningKeyInput.
 *    arg
 *        An opaque pointer that would be passed to fn whenever it is
 *        called.
 * NOTES:
 * Adds Proof Of Possession to the CertRequest using the signature field
 * of the ProofOfPossession field.  NOTE: In order to use this option,
 * the certificate template must contain the publicKey at the very minimum.
 *
 * If you don't want the function to generate POPOSigningKeyInput, then
 * make sure the cert template already contains the subject and public key
 * values.  Currently creating POPOSigningKeyInput is not supported, so
 * a Message passed to this function must have the publicKey and the subject
 * as part of the template
 *
 * This will take care of creating the entire POPOSigningKey structure
 * that will become part of the message.
 *
 * inPrivKey is the key to be used in the signing operation when creating
 * POPOSigningKey structure.  This should be the key corresponding to
 * the certificate being requested.
 *
 * inCertForInput will be used if POPOSigningKeyInput needs to be generated.
 * It will be used in generating the authInfo.sender field.  If the parameter
 * is not passed in then authInfo.publicKeyMAC will be generated instead.
 * If passed in, this certificate needs to be a valid certificate.
 *
 * The last 3 arguments are for future compatibility in case we ever want to
 * support generating POPOSigningKeyInput.  Pass in NULL for all 3 if you
 * definitely don't want the function to even try to generate
 * POPOSigningKeyInput.  If you try to use POPOSigningKeyInput, the function
 * will fail.
 *
 * RETURN:
 *    SECSuccess if adding the Signature Proof Of Possession worked.
 *    Any other return value indicates an error in trying to add
 *    the Signature Proof Of Possession.
 */
extern SECStatus
CRMF_CertReqMsgSetSignaturePOP(CRMFCertReqMsg *inCertReqMsg,
                               SECKEYPrivateKey *inPrivKey,
                               SECKEYPublicKey *inPubKey,
                               CERTCertificate *inCertForInput,
                               CRMFMACPasswordCallback fn,
                               void *arg);

/*
 * FUNCTION: CRMF_CertReqMsgSetKeyEnciphermentPOP
 * INPUTS:
 *    inCertReqMsg
 *        The Certificate Request Message to operate on.
 *    inKeyChoice
 *        An enumeration indicating which POPOPrivKey Choice to use
 *        in constructing the KeyEnciphermentPOP.
 *    subseqMess
 *        This parameter must be provided iff inKeyChoice is
 *        crmfSubsequentMessage.  This details how the RA is to respond
 *        in order to perform Proof Of Possession.  Look in crmft.h under
 *        the definition of CRMFSubseqMessOptions for possible values.
 *    encPrivKey
 *        This parameter only needs to be provided if inKeyChoice is
 *        crmfThisMessage.  The item should contain the encrypted private
 *        key.
 *
 * NOTES:
 * Adds Proof Of Possession using the keyEncipherment field of
 * ProofOfPossession.
 *
 * The function looks at the the inKeyChoice parameter and interprets it in
 * in the following manner.
 *
 * If a parameter is not mentioned under interpretation, the function will not
 * look at its value when implementing that case.
 *
 * inKeyChoice          Interpretation
 * -----------          --------------
 * crmfThisMessage      This options requires that the encrypted private key
 *                      be included in the thisMessage field of POPOPrivKey.
 *                      We don't support this yet, so any clients who want
 *                      to use this feature have to implement a wrapping
 *                      function and agree with the server on how to properly
 *                      wrap the key.  That encrypted key must be passed in
 *                      as the encPrivKey parameter.
 *
 * crmfSubequentMessage Must pass in a value for subseqMess.  The value must
 *                      be either CRMFEncrCert or CRMFChallengeResp.  The
 *                      parameter encPrivKey will not be looked at in this
 *                      case.
 *
 * crmfDHMAC            This is not a valid option for this function.  Passing
 *                      in this value will result in the function returning
 *                      SECFailure.
 * RETURN:
 *    SECSuccess if adding KeyEnciphermentPOP was successful.  Any other return
 *    value indicates an error in adding KeyEnciphermentPOP.
 */
extern SECStatus
CRMF_CertReqMsgSetKeyEnciphermentPOP(CRMFCertReqMsg *inCertReqMsg,
                                     CRMFPOPOPrivKeyChoice inKeyChoice,
                                     CRMFSubseqMessOptions subseqMess,
                                     SECItem *encPrivKey);

/*
 * FUNCTION: CRMF_CertReqMsgSetKeyAgreementPOP
 * INPUTS:
 *    inCertReqMsg
 *        The Certificate Request Message to operate on.
 *    inKeyChoice
 *        An enumeration indicating which POPOPrivKey Choice to use
 *        in constructing the KeyAgreementPOP.
 *    subseqMess
 *        This parameter must be provided iff inKeyChoice is
 *        crmfSubsequentMessage.  This details how the RA is to respond
 *        in order to perform Proof Of Possession.  Look in crmft.h under
 *        the definition of CRMFSubseqMessOptions for possible values.
 *    encPrivKey
 *        This parameter only needs to be provided if inKeyChoice is
 *        crmfThisMessage.  The item should contain the encrypted private
 *        key.
 * Adds Proof Of Possession using the keyAgreement field of
 * ProofOfPossession.
 *
 * The function looks at the the inKeyChoice parameter and interprets it in
 * in the following manner.
 *
 * If a parameter is not mentioned under interpretation, the function will not
 * look at its value when implementing that case.
 *
 * inKeyChoice          Interpretation
 * -----------          --------------
 * crmfThisMessage      This options requires that the encrypted private key
 *                      be included in the thisMessage field of POPOPrivKey.
 *                      We don't support this yet, so any clients who want
 *                      to use this feature have to implement a wrapping
 *                      function and agree with the server on how to properly
 *                      wrap the key.  That encrypted key must be passed in
 *                      as the encPrivKey parameter.
 *
 * crmfSubequentMessage Must pass in a value for subseqMess.  The value must
 *                      be either crmfEncrCert or crmfChallengeResp.  The
 *                      parameter encPrivKey will not be looked at in this
 *                      case.
 *
 * crmfDHMAC            This option is not supported.
 */
extern SECStatus
CRMF_CertReqMsgSetKeyAgreementPOP(CRMFCertReqMsg *inCertReqMsg,
                                  CRMFPOPOPrivKeyChoice inKeyChoice,
                                  CRMFSubseqMessOptions subseqMess,
                                  SECItem *encPrivKey);

/*
 * FUNCTION: CRMF_CreateCertReqMsgFromDER
 * INPUTS:
 *    buf
 *        A buffer to the DER-encoded Certificate Request Message.
 *    len
 *        The length in bytes of the buffer 'buf'
 * NOTES:
 * This function passes the buffer to the ASN1 decoder and creates a
 * CRMFCertReqMsg structure.  Do not try adding any fields to a message
 * returned from this function.  Specifically adding more Controls or
 * Extensions may cause your program to crash.
 *
 * RETURN:
 *    A pointer to the Certificate Request Message structure.  A NULL return
 *    value indicates the library was unable to parse the DER.
 */
extern CRMFCertReqMsg *CRMF_CreateCertReqMsgFromDER(const char *buf, long len);

/*
 * FUNCTION: CRMF_CreateCertReqMessagesFromDER
 * INPUTS:
 *    buf
 *        A buffer to the DER-encoded Certificate Request Messages.
 *    len
 *        The length in bytes of buf
 * NOTES:
 * This function passes the buffer to the ASN1 decoder and creates a
 * CRMFCertReqMessages structure.  Do not try adding any fields to a message
 * derived from this function.  Specifically adding more Controls or
 * Extensions may cause your program to crash.
 * The user must call CRMF_DestroyCertReqMessages after the return value is
 * no longer needed, ie when all individual messages have been extracted.
 *
 * RETURN:
 *    A pointer to the Certificate Request Messages structure.  A NULL return
 *    value indicates the library was unable to parse the DER.
 */
extern CRMFCertReqMessages *
CRMF_CreateCertReqMessagesFromDER(const char *buf, long len);

/*
 * FUNCTION: CRMF_DestroyCertReqMessages
 * INPUTS
 *    inCertReqMsgs
 *        The Messages to destroy.
 * RETURN:
 *    SECSuccess if freeing the memory was done successfully.  Any other
 *    return value indicates an error in freeing up memory.
 */
extern SECStatus
CRMF_DestroyCertReqMessages(CRMFCertReqMessages *inCertReqMsgs);

/*
 * FUNCTION: CRMF_CertReqMessagesGetNumMessages
 * INPUTS:
 *    inCertReqMsgs
 *        The Request Messages to operate on.
 * RETURN:
 *    The number of messages contained in the in the Request Messages
 *    strucure.
 */
extern int
CRMF_CertReqMessagesGetNumMessages(CRMFCertReqMessages *inCertReqMsgs);

/*
 * FUNCTION: CRMF_CertReqMessagesGetCertReqMsgAtIndex
 * INPUTS:
 *    inReqMsgs
 *        The Certificate Request Messages to operate on.
 *    index
 *        The index of the single message the user wants a copy of.
 * NOTES:
 * This function returns a copy of the request messages stored at the
 * index corresponding to the parameter 'index'.  Indexing of the messages
 * is done in the same manner as a C array.  Meaning the valid index are
 * 0...numMessages-1.  User must call CRMF_DestroyCertReqMsg when done using
 * the return value of this function.
 *
 * RETURN:
 * SECSuccess if copying the message at the requested index was successful.
 * Any other return value indicates an invalid index or error while copying
 * the single request message.
 */
extern CRMFCertReqMsg *
CRMF_CertReqMessagesGetCertReqMsgAtIndex(CRMFCertReqMessages *inReqMsgs,
                                         int index);

/*
 * FUNCTION: CRMF_CertReqMsgGetID
 * INPUTS:
 *    inCertReqMsg
 *        The Certificate Request Message to get the ID from.
 *    destID
 *        A pointer to where the library can place the ID of the Message.
 * RETURN:
 *    SECSuccess if the function was able to retrieve the ID and place it
 *    at *destID.  Any other return value indicates an error meaning the value
 *    in *destId is un-reliable and should not be used by the caller of this
 *    function.
 *
 */
extern SECStatus CRMF_CertReqMsgGetID(CRMFCertReqMsg *inCertReqMsg,
                                      long *destID);

/*
 * FUNCTION: CRMF_DoesRequestHaveField
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    inField
 *        An enumeration indicating which filed of the certificate template
 *        to look for.
 * NOTES:
 * All the fields in a certificate template are optional.  This function
 * checks to see if the requested field is present.  Look in crmft.h at the
 * definition of CRMFCertTemplateField for possible values for possible
 * querying.
 *
 * RETURN:
 * PR_TRUE iff the field corresponding to 'inField' has been specified as part
 *         of 'inCertReq'
 * PR_FALSE iff the field corresponding to 'inField' has not been speicified
 *          as part of 'inCertReq'
 *
 */
extern PRBool CRMF_DoesRequestHaveField(CRMFCertRequest *inCertReq,
                                        CRMFCertTemplateField inField);

/*
 * FUNCTION: CRMF_CertReqMsgGetCertRequest
 * INPUTS:
 *    inCertReqMsg
 *        The Certificate Request Message to operate on.
 * NOTES:
 *    This function returns a copy of the Certificate Request to the user.
 *    The user can keep adding to this request and then making it a part
 *    of another message.  After the user no longer wants to use the
 *    returned request, the user must call CRMF_DestroyCertRequest and
 *    pass it the request returned by this function.
 * RETURN:
 *    A pointer to a copy of the certificate request contained by the message.
 *    A NULL return value indicates an error occurred while copying the
 *   certificate request.
 */
extern CRMFCertRequest *
CRMF_CertReqMsgGetCertRequest(CRMFCertReqMsg *inCertReqMsg);

/*
 * FUNCTION: CRMF_CertRequestGetCertTemplateVersion
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    version
 *        A pointer to where the library can store the version contatined
 *        in the certificate template within the certifcate request.
 * RETURN:
 *    SECSuccess if the Certificate template contains the version field.  In
 *    this case, *version will hold the value of the certificate template
 *    version.
 *    SECFailure indicates that version field was not present as part of
 *    of the certificate template.
 */
extern SECStatus
CRMF_CertRequestGetCertTemplateVersion(CRMFCertRequest *inCertReq,
                                       long *version);

/*
 * FUNCTION: CRMF_CertRequestGetCertTemplateSerialNumber
 * INPUTS:
 *    inCertReq
 *        The certificate request to operate on.
 *    serialNumber
 *        A pointer where the library can put the serial number contained
 *        in the certificate request's certificate template.
 * RETURN:
 * If a serial number exists in the CertTemplate of the request, the function
 * returns SECSuccess and the value at *serialNumber contains the serial
 * number.
 * If no serial number is present, then the function returns SECFailure and
 * the value at *serialNumber is un-changed.
 */
extern SECStatus
CRMF_CertRequestGetCertTemplateSerialNumber(CRMFCertRequest *inCertReq,
                                            long *serialNumber);

/*
 * FUNCTION: CRMF_CertRequestGetCertTemplateSigningAlg
 * INPUT:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    destAlg
 *        A Pointer to where the library can place a copy of the signing alg
 *        used in the cert request's cert template.
 * RETURN:
 * If the signingAlg is present in the CertRequest's CertTemplate, then
 * the function returns SECSuccess and places a copy of sigingAlg in
 * *destAlg.
 * If no signingAlg is present, then the function returns SECFailure and
 * the value at *destAlg is un-changed
 */
extern SECStatus
CRMF_CertRequestGetCertTemplateSigningAlg(CRMFCertRequest *inCertReq,
                                          SECAlgorithmID *destAlg);
/*
 * FUNCTION: CRMF_CertRequestGetCertTemplateIssuer
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    destIssuer
 *        A pointer to where the library can place a copy of the cert
 *        request's cert template issuer field.
 * RETURN:
 * If the issuer is present in the cert request cert template, the function
 * returns SECSuccess and places a  copy of the issuer in *destIssuer.
 * If there is no issuer present, the function returns SECFailure and the
 * value at *destIssuer is unchanged.
 */
extern SECStatus
CRMF_CertRequestGetCertTemplateIssuer(CRMFCertRequest *inCertReq,
                                      CERTName *destIssuer);

/*
 * FUNCTION: CRMF_CertRequestGetCertTemplateValidity
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    destValdity
 *        A pointer to where the library can place a copy of the validity
 *        info in the cert request cert template.
 * NOTES:
 * Pass the pointer to
 * RETURN:
 * If there is an OptionalValidity field, the function will return SECSuccess
 * and place the appropriate values in *destValidity->notBefore and
 * *destValidity->notAfter. (Each field is optional, but at least one will
 * be present if the function returns SECSuccess)
 *
 * If there is no OptionalValidity field, the function will return SECFailure
 * and the values at *destValidity will be un-changed.
 */
extern SECStatus
CRMF_CertRequestGetCertTemplateValidity(CRMFCertRequest *inCertReq,
                                        CRMFGetValidity *destValidity);
/*
 * FUNCTION: CRMF_DestroyGetValidity
 * INPUTS:
 *    inValidity
 *        A pointer to the memroy to be freed.
 * NOTES:
 * The function will free the memory allocated by the function
 * CRMF_CertRequestGetCertTemplateValidity.  That means only memory pointed
 * to within the CRMFGetValidity structure.  Since
 * CRMF_CertRequestGetCertTemplateValidity does not allocate memory for the
 * structure passed into it, it will not free it.  Meaning this function will
 * free the memory at inValidity->notBefore and inValidity->notAfter, but not
 * the memory directly at inValdity.
 *
 * RETURN:
 * SECSuccess if freeing the memory was successful.  Any other return value
 * indicates an error while freeing the memory.
 */
extern SECStatus
CRMF_DestroyGetValidity(CRMFGetValidity *inValidity);

/*
 * FUNCTION: CRMF_CertRequestGetCertTemplateSubject
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 *    destSubject
 *        A pointer to where the library can place a copy of the subject
 *        contained in the request's cert template.
 * RETURN:
 * If there is a subject in the CertTemplate, then the function returns
 * SECSuccess and a copy of the subject is placed in *destSubject.
 *
 * If there is no subject, the function returns SECFailure and the values at
 * *destSubject is unchanged.
 */
extern SECStatus
CRMF_CertRequestGetCertTemplateSubject(CRMFCertRequest *inCertReq,
                                       CERTName *destSubject);

/*
 * FUNCTION: CRMF_CertRequestGetCertTemplatePublicKey
 * INPUTS:
 *    inCertReq
 *        The Cert request to operate on.
 *    destPublicKey
 *        A pointer to where the library can place a copy of the request's
 *        cert template public key.
 * RETURN:
 * If there is a publicKey parameter in the CertRequest, the function returns
 * SECSuccess, and places a copy of the publicKey in *destPublicKey.
 *
 * If there is no publicKey, the function returns SECFailure and the value
 * at *destPublicKey is un-changed.
 */
extern SECStatus
CRMF_CertRequestGetCertTemplatePublicKey(CRMFCertRequest *inCertReq,
                                         CERTSubjectPublicKeyInfo *destPublicKey);

/*
 * FUNCTION: CRMF_CertRequestGetCertTemplateIssuerUID
 * INPUTS:
 *    inCertReq
 *        The Cert request to operate on.
 *    destIssuerUID
 *        A pointer to where the library can store a copy of the request's
 *        cert template destIssuerUID.
 *
 * NOTES:
 * destIssuerUID is a bit string and will be returned in a SECItem as
 * a bit string.  Meaning the len field contains the number of valid bits as
 * opposed to the number of bytes allocated.
 *
 * RETURN:
 * If the CertTemplate has an issuerUID, the function returns SECSuccess and
 * places a copy of the issuerUID in *destIssuerUID.
 *
 * If there is no issuerUID, the function returns SECFailure and the value
 * *destIssuerUID is unchanged.
 */
extern SECStatus
CRMF_CertRequestGetCertTemplateIssuerUID(CRMFCertRequest *inCertReq,
                                         SECItem *destIssuerUID);

/*
 * FUNCTION: CRMF_CertRequestGetCertTemplateSubjectUID
 *    inCertReq
 *        The Cert request to operate on.
 *    destSubjectUID
 *        A pointer to where the library can store a copy of the request's
 *        cert template destIssuerUID.
 *
 * NOTES:
 * destSubjectUID is a bit string and will be returned in a SECItem as
 * a bit string.  Meaning the len field contains the number of valid bits as
 * opposed to the number of bytes allocated.
 *
 * RETURN:
 * If the CertTemplate has an issuerUID, the function returns SECSuccess and
 * places a copy of the issuerUID in *destIssuerUID.
 *
 * If there is no issuerUID, the function returns SECSuccess and the value
 * *destIssuerUID is unchanged.
 */
extern SECStatus CRMF_GetCertTemplateSubjectUID(CRMFCertRequest *inCertReq,
                                                SECItem *destSubjectUID);

/*
 * FUNCTION: CRMF_CertRequestGetNumberOfExtensions
 * INPUTS:
 *    inCertReq
 *        The cert request to operate on.
 * RETURN:
 *    Returns the number of extensions contained by the Cert Request.
 */
extern int CRMF_CertRequestGetNumberOfExtensions(CRMFCertRequest *inCertReq);

/*
 * FUNCTION: CRMF_CertRequestGetExtensionAtIndex
 * INPUTS:
 *    inCertReq
 *        The Certificate request to operate on.
 *    index
 *        The index of the extension array whihc the user wants to access.
 * NOTES:
 * This function retrieves the extension at the index corresponding to the
 * parameter "index" indicates.  Indexing is done like a C array.
 * (0 ... numElements-1)
 *
 * Call CRMF_DestroyCertExtension when done using the return value.
 *
 * RETURN:
 *    A pointer to a copy of the extension at the desired index.  A NULL
 *    return value indicates an invalid index or an error while copying
 *    the extension.
 */
extern CRMFCertExtension *
CRMF_CertRequestGetExtensionAtIndex(CRMFCertRequest *inCertReq,
                                    int index);
/*
 * FUNCTION: CRMF_CertExtensionGetOidTag
 * INPUTS:
 *    inExtension

 *        The extension to operate on.
 * RETURN:
 *    Returns the SECOidTag associated with the cert extension passed in.
 */
extern SECOidTag CRMF_CertExtensionGetOidTag(CRMFCertExtension *inExtension);

/*
 * FUNCTION: CRMF_CertExtensionGetIsCritical
 * INPUT:
 *    inExt
 *        The cert extension to operate on.
 *
 * RETURN:
 * PR_TRUE if the extension is critical.
 * PR_FALSE if the extension is not critical.
 */
extern PRBool CRMF_CertExtensionGetIsCritical(CRMFCertExtension *inExt);

/*
 * FUNCTION: CRMF_CertExtensionGetValue
 * INPUT:
 *    inExtension
 *        The extension to operate on.
 * NOTES:
 * Caller is responsible for freeing the memory associated with the return
 * value.  Call SECITEM_FreeItem(retVal, PR_TRUE) when done using the return
 * value.
 *
 * RETURN:
 * A pointer to an item containig the value for the certificate extension.
 * A NULL return value indicates an error in copying the information.
 */
extern SECItem *CRMF_CertExtensionGetValue(CRMFCertExtension *inExtension);

/*
 * FUNCTION: CRMF_CertReqMsgGetPOPOSigningKey
 * INPUTS:
 *    inCertReqMsg
 *        The certificate request message to operate on.
 *    destKey
 *        A pointer to where the library can place a pointer to
 *        a copy of the Proof Of Possession Signing Key used
 *        by the message.
 *
 * RETURN:
 * Get the POPOSigningKey associated with this CRMFCertReqMsg.
 * If the CertReqMsg does not have a pop, the function returns
 * SECFailure and the value at *destKey is un-changed..
 *
 * If the CertReqMsg does have a pop, then the CertReqMsg's
 * POPOSigningKey will be placed at *destKey.
 */
extern SECStatus
CRMF_CertReqMsgGetPOPOSigningKey(CRMFCertReqMsg *inCertReqMsg,
                                 CRMFPOPOSigningKey **destKey);

/*
 * FUNCTION: CRMF_DestroyPOPOSigningKey
 * INPUTS:
 *    inKey
 *        The signing key to free.
 *
 * RETURN:
 * SECSuccess if freeing the memory was successful.  Any other return value
 * indicates an error while freeing memory.
 */
extern SECStatus CRMF_DestroyPOPOSigningKey(CRMFPOPOSigningKey *inKey);

/*
 * FUNCTION: CRMF_POPOSigningKeyGetAlgID
 * INPUTS:
 *    inSignKey
 *        The Signing Key to operate on.
 * RETURN:
 * Return the algorithmID used by the CRMFPOPOSigningKey.  User must
 * call SECOID_DestroyAlgorithmID(destID, PR_TRUE) when done using the
 * return value.
 */
extern SECAlgorithmID *
CRMF_POPOSigningKeyGetAlgID(CRMFPOPOSigningKey *inSignKey);

/*
 * FUNCTION: CRMF_POPOSigningKeyGetSignature
 * INPUTS:
 *    inSignKey
 *        The Signing Key to operate on.
 *
 * RETURN:
 * Get the actual signature stored away in the CRMFPOPOSigningKey.  SECItem
 * returned is a BIT STRING, so the len field is the number of bits as opposed
 * to the total number of bytes allocatd.  User must call
 * SECITEM_FreeItem(retVal,PR_TRUE) when done using the return value.
 */
extern SECItem *CRMF_POPOSigningKeyGetSignature(CRMFPOPOSigningKey *inSignKey);

/*
 * FUNCTION: CRMF_POPOSigningKeyGetInput
 * INPUTS:
 *    inSignKey
 *        The Signing Key to operate on.
 * NOTES:
 * This function will return the der encoded input that was read in while
 * decoding.  The API does not support this option when creating, so you
 * cannot add this field.
 *
 * RETURN:
 * Get the poposkInput that is part of the of the POPOSigningKey. If the
 * optional field is not part of the POPOSigningKey, the function returns
 * NULL.
 *
 * If the optional field is part of the POPOSingingKey, the function will
 * return a copy of the der encoded poposkInput.
 */
extern SECItem *CRMF_POPOSigningKeyGetInput(CRMFPOPOSigningKey *inSignKey);

/*
 * FUNCTION: CRMF_CertReqMsgGetPOPKeyEncipherment
 * INPUTS:
 *    inCertReqMsg
 *        The certificate request message to operate on.
 *    destKey
 *        A pointer to where the library can place a pointer to a
 *        copy of the POPOPrivKey representing Key Encipherment
 *        Proof of Possession.
 *NOTES:
 * This function gets the POPOPrivKey associated with this CRMFCertReqMsg
 * for Key Encipherment.
 *
 * RETURN:
 * If the CertReqMsg did not use Key Encipherment for Proof Of Possession, the
 * function returns SECFailure and the value at *destKey is un-changed.
 *
 * If the CertReqMsg did use Key Encipherment for ProofOfPossession, the
 * function returns SECSuccess and places the POPOPrivKey representing the
 * Key Encipherment Proof Of Possessin at *destKey.
 */
extern SECStatus
CRMF_CertReqMsgGetPOPKeyEncipherment(CRMFCertReqMsg *inCertReqMsg,
                                     CRMFPOPOPrivKey **destKey);

/*
 * FUNCTION: CRMF_CertReqMsgGetPOPKeyAgreement
 * INPUTS:
 *    inCertReqMsg
 *        The certificate request message to operate on.
 *    destKey
 *        A pointer to where the library can place a pointer to a
 *        copy of the POPOPrivKey representing Key Agreement
 *        Proof of Possession.
 * NOTES:
 * This function gets the POPOPrivKey associated with this CRMFCertReqMsg for
 * Key Agreement.
 *
 * RETURN:
 * If the CertReqMsg used Key Agreement for Proof Of Possession, the
 * function returns SECSuccess and the POPOPrivKey for Key Agreement
 * is placed at *destKey.
 *
 * If the CertReqMsg did not use Key Agreement for Proof Of Possession, the
 * function return SECFailure and the value at *destKey is unchanged.
 */
extern SECStatus
CRMF_CertReqMsgGetPOPKeyAgreement(CRMFCertReqMsg *inCertReqMsg,
                                  CRMFPOPOPrivKey **destKey);

/*
 * FUNCTION: CRMF_DestroyPOPOPrivKey
 * INPUTS:
 *    inPrivKey
 *        The POPOPrivKey to destroy.
 * NOTES:
 * Destroy a structure allocated by CRMF_GetPOPKeyEncipherment or
 * CRMF_GetPOPKeyAgreement.
 *
 * RETURN:
 * SECSuccess on successful destruction of the POPOPrivKey.
 * Any other return value indicates an error in freeing the
 * memory.
 */
extern SECStatus CRMF_DestroyPOPOPrivKey(CRMFPOPOPrivKey *inPrivKey);

/*
 * FUNCTION: CRMF_POPOPrivKeyGetChoice
 * INPUT:
 *    inKey
 *        The POPOPrivKey to operate on.
 * RETURN:
 * Returns which choice was used in constructing the POPPOPrivKey. Look at
 * the definition of CRMFPOPOPrivKeyChoice in crmft.h for the possible return
 * values.
 */
extern CRMFPOPOPrivKeyChoice CRMF_POPOPrivKeyGetChoice(CRMFPOPOPrivKey *inKey);

/*
 * FUNCTION: CRMF_POPOPrivKeyGetThisMessage
 * INPUTS:
 *    inKey
 *        The POPOPrivKey to operate on.
 *    destString
 *        A pointer to where the library can place a copy of the This Message
 *        field stored in the POPOPrivKey
 *
 * RETURN:
 * Returns the field thisMessage from the POPOPrivKey.
 * If the POPOPrivKey did not use the field thisMessage, the function
 * returns SECFailure and the value at *destString is unchanged.
 *
 * If the POPOPrivKey did use the field thisMessage, the function returns
 * SECSuccess and the BIT STRING representing thisMessage is placed
 * at *destString. BIT STRING representation means the len field is the
 * number of valid bits as opposed to the total number of bytes.
 */
extern SECStatus CRMF_POPOPrivKeyGetThisMessage(CRMFPOPOPrivKey *inKey,
                                                SECItem *destString);

/*
 * FUNCTION: CRMF_POPOPrivKeyGetSubseqMess
 * INPUTS:
 *    inKey
 *        The POPOPrivKey to operate on.
 *    destOpt
 *        A pointer to where the library can place the value of the
 *        Subsequent Message option used by POPOPrivKey.
 *
 * RETURN:
 * Retrieves the field subsequentMessage from the POPOPrivKey.
 * If the POPOPrivKey used the subsequentMessage option, the function
 * returns SECSuccess and places the appropriate enumerated value at
 * *destMessageOption.
 *
 * If the POPOPrivKey did not use the subsequenMessage option, the function
 * returns SECFailure and the value at *destOpt is un-changed.
 */
extern SECStatus CRMF_POPOPrivKeyGetSubseqMess(CRMFPOPOPrivKey *inKey,
                                               CRMFSubseqMessOptions *destOpt);

/*
 * FUNCTION: CRMF_POPOPrivKeyGetDHMAC
 * INPUTS:
 *    inKey
 *        The POPOPrivKey to operate on.
 *    destMAC
 *        A pointer to where the library can place a copy of the dhMAC
 *        field of the POPOPrivKey.
 *
 * NOTES:
 * Returns the field dhMAC from the POPOPrivKey.  The populated SECItem
 * is in BIT STRING format.
 *
 * RETURN:
 * If the POPOPrivKey used the dhMAC option, the function returns SECSuccess
 * and the BIT STRING for dhMAC will be placed at *destMAC.  The len field in
 * destMAC (ie destMAC->len) will be the valid number of bits as opposed to
 * the number of allocated bytes.
 *
 * If the POPOPrivKey did not use the dhMAC option, the function returns
 * SECFailure and the value at *destMAC is unchanged.
 *
 */
extern SECStatus CRMF_POPOPrivKeyGetDHMAC(CRMFPOPOPrivKey *inKey,
                                          SECItem *destMAC);

/*
 * FUNCTION: CRMF_CertRequestGetNumControls
 * INPUTS:
 *    inCertReq
 *        The Certificate Request to operate on.
 * RETURN:
 * Returns the number of Controls registered with this CertRequest.
 */
extern int CRMF_CertRequestGetNumControls(CRMFCertRequest *inCertReq);

/*
 * FUNCTION: CRMF_CertRequestGetControlAtIndex
 * INPUTS:
 *    inCertReq
 *        The certificate request to operate on.
 *    index
 *        The index of the control the user wants a copy of.
 * NOTES:
 * Function retrieves the Control at located at index.  The Controls
 * are numbered like a traditional C array (0 ... numElements-1)
 *
 * RETURN:
 * Returns a copy of the control at the index specified.  This is a copy
 * so the user must call CRMF_DestroyControl after the return value is no
 * longer needed.  A return value of NULL indicates an error while copying
 * the control or that the index was invalid.
 */
extern CRMFControl *
CRMF_CertRequestGetControlAtIndex(CRMFCertRequest *inCertReq,
                                  int index);

/*
 * FUNCTION: CRMF_DestroyControl
 * INPUTS:
 *    inControl
 *        The Control to destroy.
 * NOTES:
 * Destroy a CRMFControl allocated by CRMF_GetControlAtIndex.
 *
 * RETURN:
 * SECSuccess if freeing the memory was successful.  Any other return
 * value indicates an error while freeing the memory.
 */
extern SECStatus CRMF_DestroyControl(CRMFControl *inControl);

/*
 * FUNCTION: CRMF_ControlGetControlType
 * INPUTS:
 *    inControl
 *        The control to operate on.
 * NOTES:
 * The function returns an enumertion which indicates the type of control
 * 'inControl'.
 *
 * RETURN:
 * Look in crmft.h at the definition of the enumerated type CRMFControlType
 * for the possible return values.
 */
extern CRMFControlType CRMF_ControlGetControlType(CRMFControl *inControl);

/*
 * FUNCTION: CRMF_ControlGetRegTokenControlValue
 * INPUTS:
 *    inControl
 *        The Control to operate on.
 * NOTES:
 * The user must call SECITEM_FreeItem passing in the return value
 * after the returnvalue is no longer needed.

 * RETURN:
 * Return the value for a Registration Token Control.
 * The SECItem returned should be in UTF8 format.  A NULL
 * return value indicates there was no Registration Control associated
 * with the Control.
 * (This library will not verify format.  It assumes the client properly
 * formatted the strings when adding it or the message decoded was properly
 * formatted.  The library will just give back the bytes it was given.)
 */
extern SECItem *CRMF_ControlGetRegTokenControlValue(CRMFControl *inControl);

/*
 * FUNCTION: CRMF_ControlGetAuthenticatorControlValue
 * INPUTS:
 *    inControl
 *        The Control to operate on.
 * NOTES:
 * The user must call SECITEM_FreeItem passing in the return value
 * after the returnvalue is no longer needed.
 *
 * RETURN:
 * Return the value for the Authenticator Control.
 * The SECItem returned should be in UTF8 format.  A NULL
 * return value indicates there was no Authenticator Control associated
 * with the CRMFControl..
 * (This library will not verify format.  It assumes the client properly
 * formatted the strings when adding it or the message decoded was properly
 * formatted.  The library will just give back the bytes it was given.)
 */
extern SECItem *CRMF_ControlGetAuthicatorControlValue(CRMFControl *inControl);

/*
 * FUNCTION: CRMF_ControlGetPKIArchiveOptions
 * INPUTS:inControl
 *    The Control tooperate on.
 * NOTES:
 * This function returns a copy of the PKIArchiveOptions.  The user must call
 * the function CRMF_DestroyPKIArchiveOptions when the return value is no
 * longer needed.
 *
 * RETURN:
 * Get the PKIArchiveOptions associated with the Control.  A return
 * value of NULL indicates the Control was not a PKIArchiveOptions
 * Control.
 */
extern CRMFPKIArchiveOptions *
CRMF_ControlGetPKIArchiveOptions(CRMFControl *inControl);

/*
 * FUNCTION: CMRF_DestroyPKIArchiveOptions
 * INPUTS:
 *    inOptions
 *        The ArchiveOptions to destroy.
 * NOTE:
 * Destroy the CRMFPKIArchiveOptions structure.
 *
 * RETURN:
 * SECSuccess if successful in freeing all the memory associated with
 * the PKIArchiveOptions.  Any other return value indicates an error while
 * freeing the PKIArchiveOptions.
 */
extern SECStatus
CRMF_DestroyPKIArchiveOptions(CRMFPKIArchiveOptions *inOptions);

/*
 * FUNCTION: CRMF_PKIArchiveOptionsGetOptionType
 * INPUTS:
 *    inOptions
 *        The PKIArchiveOptions to operate on.
 * RETURN:
 * Returns the choice used for the PKIArchiveOptions.  Look at the definition
 * of CRMFPKIArchiveOptionsType in crmft.h for possible return values.
 */
extern CRMFPKIArchiveOptionsType
CRMF_PKIArchiveOptionsGetOptionType(CRMFPKIArchiveOptions *inOptions);

/*
 * FUNCTION: CRMF_PKIArchiveOptionsGetEncryptedPrivKey
 * INPUTS:
 *    inOpts
 *        The PKIArchiveOptions to operate on.
 *
 * NOTES:
 * The user must call CRMF_DestroyEncryptedKey when done using this return
 * value.
 *
 * RETURN:
 * Get the encryptedPrivKey field of the PKIArchiveOptions structure.
 * A return value of NULL indicates that encryptedPrivKey was not used as
 * the choice for this PKIArchiveOptions.
 */
extern CRMFEncryptedKey *
CRMF_PKIArchiveOptionsGetEncryptedPrivKey(CRMFPKIArchiveOptions *inOpts);

/*
 * FUNCTION: CRMF_EncryptedKeyGetChoice
 * INPUTS:
 *    inEncrKey
 *        The EncryptedKey to operate on.
 *
 * NOTES:
 * Get the choice used for representing the EncryptedKey.
 *
 * RETURN:
 * Returns the Choice used in representing the EncryptedKey.  Look in
 * crmft.h at the definition of CRMFEncryptedKeyChoice for possible return
 * values.
 */
extern CRMFEncryptedKeyChoice
CRMF_EncryptedKeyGetChoice(CRMFEncryptedKey *inEncrKey);

/*
 * FUNCTION: CRMF_EncryptedKeyGetEncryptedValue
 * INPUTS:
 *    inKey
 *        The EncryptedKey to operate on.
 *
 * NOTES:
 * The user must call CRMF_DestroyEncryptedValue passing in
 * CRMF_GetEncryptedValue's return value.
 *
 * RETURN:
 * A pointer to a copy of the EncryptedValue contained as a member of
 * the EncryptedKey.
 */
extern CRMFEncryptedValue *
CRMF_EncryptedKeyGetEncryptedValue(CRMFEncryptedKey *inKey);

/*
 * FUNCTION: CRMF_DestroyEncryptedValue
 * INPUTS:
 *    inEncrValue
 *        The EncryptedValue to destroy.
 *
 * NOTES:
 * Free up all memory associated with 'inEncrValue'.
 *
 * RETURN:
 * SECSuccess if freeing up the memory associated with the EncryptedValue
 * is successful. Any other return value indicates an error while freeing the
 * memory.
 */
extern SECStatus CRMF_DestroyEncryptedValue(CRMFEncryptedValue *inEncrValue);

/*
 * FUNCTION: CRMF_EncryptedValueGetEncValue
 * INPUTS:
 *    inEncValue
 *        The EncryptedValue to operate on.
 * NOTES:
 * Function retrieves the encValue from an EncryptedValue structure.
 *
 * RETURN:
 * A poiner to a SECItem containing the encValue of the EncryptedValue
 * structure.  The return value is in BIT STRING format, meaning the
 * len field of the return structure represents the number of valid bits
 * as opposed to the allocated number of bytes.
 * ANULL return value indicates an error in copying the encValue field.
 */
extern SECItem *CRMF_EncryptedValueGetEncValue(CRMFEncryptedValue *inEncValue);

/*
 * FUNCTION: CRMF_EncryptedValueGetIntendedAlg
 * INPUTS
 *    inEncValue
 *        The EncryptedValue to operate on.
 * NOTES:
 * Retrieve the IntendedAlg field from the EncryptedValue structure.
 * Call SECOID_DestroyAlgorithmID (destAlgID, PR_TRUE) after done using
 * the return value.  When present, this alogorithm is the alogrithm for
 * which the private key will be used.
 *
 * RETURN:
 * A Copy of the intendedAlg field.  A NULL return value indicates the
 * optional field was not present in the structure.
 */
extern SECAlgorithmID *
CRMF_EncryptedValueGetIntendedAlg(CRMFEncryptedValue *inEncValue);

/*
 * FUNCTION: CRMF_EncryptedValueGetSymmAlg
 * INPUTS
 *    inEncValue
 *        The EncryptedValue to operate on.
 * NOTES:
 * Retrieve the symmAlg field from the EncryptedValue structure.
 * Call SECOID_DestroyAlgorithmID (destAlgID, PR_TRUE) after done using
 * the return value.  When present, this is algorithm used to
 * encrypt the encValue of the EncryptedValue.
 *
 * RETURN:
 * A Copy of the symmAlg field.  A NULL return value indicates the
 * optional field was not present in the structure.
 */
extern SECAlgorithmID *
CRMF_EncryptedValueGetSymmAlg(CRMFEncryptedValue *inEncValue);

/*
 * FUNCTION: CRMF_EncryptedValueGetKeyAlg
 * INPUTS
 *    inEncValue
 *        The EncryptedValue to operate on.
 * NOTES:
 * Retrieve the keyAlg field from the EncryptedValue structure.
 * Call SECOID_DestroyAlgorithmID (destAlgID, PR_TRUE) after done using
 * the return value.  When present, this is the algorithm used to encrypt
 * the symmetric key in the encSymmKey field of the EncryptedValue structure.
 *
 * RETURN:
 * A Copy of the keyAlg field.  A NULL return value indicates the
 * optional field was not present in the structure.
 */
extern SECAlgorithmID *
CRMF_EncryptedValueGetKeyAlg(CRMFEncryptedValue *inEncValue);

/*
 * FUNCTION: CRMF_EncryptedValueGetValueHint
 * INPUTS:
 *    inEncValue
 *        The EncryptedValue to operate on.
 *
 * NOTES:
 * Return a copy of the der-encoded value hint.
 * User must call SECITEM_FreeItem(retVal, PR_TRUE) when done using the
 * return value.  When, present, this is a value that the client which
 * originally issued a certificate request can use to reproduce any data
 * it wants.  The RA does not know how to interpret this data.
 *
 * RETURN:
 * A copy of the valueHint field of the EncryptedValue.  A NULL return
 * value indicates the optional valueHint field is not present in the
 * EncryptedValue.
 */
extern SECItem *
CRMF_EncryptedValueGetValueHint(CRMFEncryptedValue *inEncValue);

/*
 * FUNCTION: CRMF_EncrypteValueGetEncSymmKey
 * INPUTS:
 *    inEncValue
 *        The EncryptedValue to operate on.
 *
 * NOTES:
 * Return a copy of the encSymmKey field. This field is the encrypted
 * symmetric key that the client uses in doing Public Key wrap of a private
 * key.  When present, this is the symmetric key that was used to wrap the
 * private key.  (The encrypted private key will be stored in encValue
 * of the same EncryptedValue structure.)  The user must call
 * SECITEM_FreeItem(retVal, PR_TRUE) when the return value is no longer
 * needed.
 *
 * RETURN:
 * A copy of the optional encSymmKey field of the EncryptedValue structure.
 * The return value will be in BIT STRING format, meaning the len field will
 * be the number of valid bits as opposed to the number of bytes. A return
 * value of NULL means the optional encSymmKey field was not present in
 * the EncryptedValue structure.
 */
extern SECItem *
CRMF_EncryptedValueGetEncSymmKey(CRMFEncryptedValue *inEncValue);

/*
 * FUNCTION: CRMF_PKIArchiveOptionsGetKeyGenParameters
 * INPUTS:
 *    inOptions
 *        The PKiArchiveOptions to operate on.
 *
 * NOTES:
 * User must call SECITEM_FreeItem(retVal, PR_TRUE) after the return
 * value is no longer needed.
 *
 * RETURN:
 * Get the keyGenParameters field of the PKIArchiveOptions.
 * A NULL return value indicates that keyGenParameters was not
 * used as the choice for this PKIArchiveOptions.
 *
 * The SECItem returned is in BIT STRING format (ie, the len field indicates
 * number of valid bits as opposed to allocated number of bytes.)
 */
extern SECItem *
CRMF_PKIArchiveOptionsGetKeyGenParameters(CRMFPKIArchiveOptions *inOptions);

/*
 * FUNCTION: CRMF_PKIArchiveOptionsGetArchiveRemGenPrivKey
 * INPUTS:
 *    inOpt
 *        The PKIArchiveOptions to operate on.
 *    destVal
 *        A pointer to where the library can place the value for
 *        arciveRemGenPrivKey
 * RETURN:
 * If the PKIArchiveOptions used the archiveRemGenPrivKey field, the
 * function returns SECSuccess and fills the value at *destValue with either
 * PR_TRUE or PR_FALSE, depending on what the PKIArchiveOptions has as a
 * value.
 *
 * If the PKIArchiveOptions does not use the archiveRemGenPrivKey field, the
 * function returns SECFailure and the value at *destValue is unchanged.
 */
extern SECStatus
CRMF_PKIArchiveOptionsGetArchiveRemGenPrivKey(CRMFPKIArchiveOptions *inOpt,
                                              PRBool *destVal);

/* Helper functions that can be used by other libraries. */
/*
 * A quick helper function to get the best wrap mechanism.
 */
extern CK_MECHANISM_TYPE CRMF_GetBestWrapPadMechanism(PK11SlotInfo *slot);

/*
 * A helper function to get a randomly generated IV from a mechanism
 * type.
 */
extern SECItem *CRMF_GetIVFromMechanism(CK_MECHANISM_TYPE mechType);

SEC_END_PROTOS
#endif /*_CRMF_H_*/
