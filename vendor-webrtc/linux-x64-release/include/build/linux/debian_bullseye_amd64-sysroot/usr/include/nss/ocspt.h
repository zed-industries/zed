/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Public header for exported OCSP types.
 */

#ifndef _OCSPT_H_
#define _OCSPT_H_

/*
 * The following are all opaque types.  If someone needs to get at
 * a field within, then we need to fix the API.  Try very hard not
 * make the type available to them.
 */
typedef struct CERTOCSPRequestStr CERTOCSPRequest;
typedef struct CERTOCSPResponseStr CERTOCSPResponse;

/*
 * XXX I think only those first two above should need to be exported,
 * but until I know for certain I am leaving the rest of these here, too.
 */
typedef struct CERTOCSPCertIDStr CERTOCSPCertID;
typedef struct CERTOCSPSingleResponseStr CERTOCSPSingleResponse;

/*
 * This interface is described in terms of an HttpClient which
 * supports at least a specified set of functions. (An implementer may
 * provide HttpClients with additional functionality accessible only to
 * users with a particular implementation in mind.) The basic behavior
 * is provided by defining a set of functions, listed in an
 * SEC_HttpServerFcnStruct. If the implementor of a SpecificHttpClient
 * registers his SpecificHttpClient as the default HttpClient, then his
 * functions will be called by the user of an HttpClient, such as an
 * OCSPChecker.
 *
 * The implementer of a specific HttpClient (e.g., the NSS-provided
 * DefaultHttpClient), populates an SEC_HttpClientFcnStruct, uses it to
 * register his client, and waits for his functions to be called.
 *
 * For future expandability, the SEC_HttpClientFcnStruct is defined as a
 * union, with the version field acting as a selector. The proposed
 * initial version of the structure is given following the definition
 * of the union. The HttpClientState structure is implementation-
 * dependent, and should be opaque to the user.
 */

typedef void *SEC_HTTP_SERVER_SESSION;
typedef void *SEC_HTTP_REQUEST_SESSION;

/*
 * This function creates a SEC_HTTP_SERVER_SESSION object. The implementer of a
 * specific HttpClient will allocate the necessary space, when this
 * function is called, and will free it when the corresponding FreeFcn
 * is called. The SEC_HTTP_SERVER_SESSION object is passed, as an opaque object,
 * to subsequent calls.
 *
 * If the function returns SECSuccess, the returned SEC_HTTP_SERVER_SESSION
 * must be cleaned up with a call to SEC_HttpServer_FreeSession,
 * after processing is finished.
 */
typedef SECStatus (*SEC_HttpServer_CreateSessionFcn)(
    const char *host,
    PRUint16 portnum,
    SEC_HTTP_SERVER_SESSION *pSession);

/*
 * This function is called to allow the implementation to attempt to keep
 * the connection alive. Depending on the underlying platform, it might
 * immediately return SECSuccess without having performed any operations.
 * (If a connection has not been kept alive, a subsequent call to
 * SEC_HttpRequest_TrySendAndReceiveFcn should reopen the connection
 * automatically.)
 *
 * If the connection uses nonblocking I/O, this function may return
 * SECWouldBlock and store a nonzero value at "pPollDesc". In that case
 * the caller may wait on the poll descriptor, and should call this function
 * again until SECSuccess (and a zero value at "pPollDesc") is obtained.
 */
typedef SECStatus (*SEC_HttpServer_KeepAliveSessionFcn)(
    SEC_HTTP_SERVER_SESSION session,
    PRPollDesc **pPollDesc);

/*
 * This function frees the client SEC_HTTP_SERVER_SESSION object, closes all
 * SEC_HTTP_REQUEST_SESSIONs created for that server, discards all partial results,
 * frees any memory that was allocated by the client, and invalidates any
 * response pointers that might have been returned by prior server or request
 * functions.
 */
typedef SECStatus (*SEC_HttpServer_FreeSessionFcn)(
    SEC_HTTP_SERVER_SESSION session);

/*
 * This function creates a SEC_HTTP_REQUEST_SESSION object. The implementer of a
 * specific HttpClient will allocate the necessary space, when this
 * function is called, and will free it when the corresponding FreeFcn
 * is called. The SEC_HTTP_REQUEST_SESSION object is passed, as an opaque object,
 * to subsequent calls.
 *
 * An implementation that does not support the requested protocol variant
 * (usually "http", but could eventually allow "https") or request method
 * should return SECFailure.
 *
 * Timeout values may include the constants PR_INTERVAL_NO_TIMEOUT (wait
 * forever) or PR_INTERVAL_NO_WAIT (nonblocking I/O).
 *
 * If the function returns SECSuccess, the returned SEC_HTTP_REQUEST_SESSION
 * must be cleaned up with a call to SEC_HttpRequest_FreeSession,
 * after processing is finished.
 */
typedef SECStatus (*SEC_HttpRequest_CreateFcn)(
    SEC_HTTP_SERVER_SESSION session,
    const char *http_protocol_variant, /* usually "http" */
    const char *path_and_query_string,
    const char *http_request_method,
    const PRIntervalTime timeout,
    SEC_HTTP_REQUEST_SESSION *pRequest);

/*
 * This function sets data to be sent to the server for an HTTP request
 * of http_request_method == POST. If a particular implementation
 * supports it, the details for the POST request can be set by calling
 * this function, prior to activating the request with TrySendAndReceiveFcn.
 *
 * An implementation that does not support the POST method should
 * implement a SetPostDataFcn function that returns immediately.
 *
 * Setting http_content_type is optional, the parameter may
 * by NULL or the empty string.
 */
typedef SECStatus (*SEC_HttpRequest_SetPostDataFcn)(
    SEC_HTTP_REQUEST_SESSION request,
    const char *http_data,
    const PRUint32 http_data_len,
    const char *http_content_type);

/*
 * This function sets an additional HTTP protocol request header.
 * If a particular implementation supports it, one or multiple headers
 * can be added to the request by calling this function once or multiple
 * times, prior to activating the request with TryFcn.
 *
 * An implementation that does not support setting additional headers
 * should implement an AddRequestHeaderFcn function that returns immediately.
 */
typedef SECStatus (*SEC_HttpRequest_AddHeaderFcn)(
    SEC_HTTP_REQUEST_SESSION request,
    const char *http_header_name,
    const char *http_header_value);

/*
 * This function initiates or continues an HTTP request. After
 * parameters have been set with the Create function and, optionally,
 * modified or enhanced with the AddParams function, this call creates
 * the socket connection and initiates the communication.
 *
 * If a timeout value of zero is specified, indicating non-blocking
 * I/O, the client creates a non-blocking socket, and returns a status
 * of SECWouldBlock and a non-NULL PRPollDesc if the operation is not
 * complete. In that case all other return parameters are undefined.
 * The caller is expected to repeat the call, possibly after using
 * PRPoll to determine that a completion has occurred, until a return
 * value of SECSuccess (and a NULL value for pPollDesc) or a return
 * value of SECFailure (indicating failure on the network level)
 * is obtained.
 *
 * http_response_data_len is both input and output parameter.
 * If a pointer to a PRUint32 is supplied, the http client is
 * expected to check the given integer value and always set an out
 * value, even on failure.
 * An input value of zero means, the caller will accept any response len.
 * A different input value indicates the maximum response value acceptable
 * to the caller.
 * If data is successfully read and the size is acceptable to the caller,
 * the function will return SECSuccess and set http_response_data_len to
 * the size of the block returned in http_response_data.
 * If the data read from the http server is larger than the acceptable
 * size, the function will return SECFailure.
 * http_response_data_len will be set to a value different from zero to
 * indicate the reason of the failure.
 * An out value of "0" means, the failure was unrelated to the
 * acceptable size.
 * An out value of "1" means, the result data is larger than the
 * accpeptable size, but the real size is not yet known to the http client
 * implementation and it stopped retrieving it,
 * Any other out value combined with a return value of SECFailure
 * will indicate the actual size of the server data.
 *
 * The caller is permitted to provide NULL values for any of the
 * http_response arguments, indicating the caller is not interested in
 * those values. If the caller does provide an address, the HttpClient
 * stores at that address a pointer to the corresponding argument, at
 * the completion of the operation.
 *
 * All returned pointers will be owned by the the HttpClient
 * implementation and will remain valid until the call to
 * SEC_HttpRequest_FreeFcn.
 */
typedef SECStatus (*SEC_HttpRequest_TrySendAndReceiveFcn)(
    SEC_HTTP_REQUEST_SESSION request,
    PRPollDesc **pPollDesc,
    PRUint16 *http_response_code,
    const char **http_response_content_type,
    const char **http_response_headers,
    const char **http_response_data,
    PRUint32 *http_response_data_len);

/*
 * Calling CancelFcn asks for premature termination of the request.
 *
 * Future calls to SEC_HttpRequest_TrySendAndReceive should
 * by avoided, but in this case the HttpClient implementation
 * is expected to return immediately with SECFailure.
 *
 * After calling CancelFcn, a separate call to SEC_HttpRequest_FreeFcn
 * is still necessary to free resources.
 */
typedef SECStatus (*SEC_HttpRequest_CancelFcn)(
    SEC_HTTP_REQUEST_SESSION request);

/*
 * Before calling this function, it must be assured the request
 * has been completed, i.e. either SEC_HttpRequest_TrySendAndReceiveFcn has
 * returned SECSuccess, or the request has been canceled with
 * a call to SEC_HttpRequest_CancelFcn.
 *
 * This function frees the client state object, closes all sockets,
 * discards all partial results, frees any memory that was allocated
 * by the client, and invalidates all response pointers that might
 * have been returned by SEC_HttpRequest_TrySendAndReceiveFcn
 */
typedef SECStatus (*SEC_HttpRequest_FreeFcn)(
    SEC_HTTP_REQUEST_SESSION request);

typedef struct SEC_HttpClientFcnV1Struct {
    SEC_HttpServer_CreateSessionFcn createSessionFcn;
    SEC_HttpServer_KeepAliveSessionFcn keepAliveSessionFcn;
    SEC_HttpServer_FreeSessionFcn freeSessionFcn;
    SEC_HttpRequest_CreateFcn createFcn;
    SEC_HttpRequest_SetPostDataFcn setPostDataFcn;
    SEC_HttpRequest_AddHeaderFcn addHeaderFcn;
    SEC_HttpRequest_TrySendAndReceiveFcn trySendAndReceiveFcn;
    SEC_HttpRequest_CancelFcn cancelFcn;
    SEC_HttpRequest_FreeFcn freeFcn;
} SEC_HttpClientFcnV1;

typedef struct SEC_HttpClientFcnStruct {
    PRInt16 version;
    union {
        SEC_HttpClientFcnV1 ftable1;
        /* SEC_HttpClientFcnV2 ftable2; */
        /* ...                      */
    } fcnTable;
} SEC_HttpClientFcn;

/*
 * ocspMode_FailureIsVerificationFailure:
 * This is the classic behaviour of NSS.
 * Any OCSP failure is a verification failure (classic mode, default).
 * Without a good response, OCSP networking will be retried each time
 * it is required for verifying a cert.
 *
 * ocspMode_FailureIsNotAVerificationFailure:
 * If we fail to obtain a valid OCSP response, consider the
 * cert as good.
 * Failed OCSP attempts might get cached and not retried until
 * minimumSecondsToNextFetchAttempt.
 * If we are able to obtain a valid response, the cert
 * will be considered good, if either status is "good"
 * or the cert was not yet revoked at verification time.
 *
 * Additional failure modes might be added in the future.
 */
typedef enum {
    ocspMode_FailureIsVerificationFailure = 0,
    ocspMode_FailureIsNotAVerificationFailure = 1
} SEC_OcspFailureMode;

/*
 * A ResponderID identifies the responder -- or more correctly, the
 * signer of the response.  The ASN.1 definition of a ResponderID is:
 *
 * ResponderID	::=	CHOICE {
 *	byName			[1] EXPLICIT Name,
 *	byKey			[2] EXPLICIT KeyHash }
 *
 * Because it is CHOICE, the type of identification used and the
 * identification itself are actually encoded together.  To represent
 * this same information internally, we explicitly define a type and
 * save it, along with the value, into a data structure.
 */

typedef enum {
    ocspResponderID_other = -1, /* unknown kind of responderID */
    ocspResponderID_byName = 1,
    ocspResponderID_byKey = 2
} CERTOCSPResponderIDType;

#endif /* _OCSPT_H_ */
