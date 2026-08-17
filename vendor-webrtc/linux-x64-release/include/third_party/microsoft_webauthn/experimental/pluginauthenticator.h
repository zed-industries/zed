

/* this ALWAYS GENERATED file contains the definitions for the interfaces */

/* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "ole2.h"
#include "windows.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __pluginauthenticator_h__
#define __pluginauthenticator_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */

#ifndef __EXPERIMENTAL_IPluginAuthenticator_FWD_DEFINED__
#define __EXPERIMENTAL_IPluginAuthenticator_FWD_DEFINED__
typedef interface EXPERIMENTAL_IPluginAuthenticator
    EXPERIMENTAL_IPluginAuthenticator;

#endif /* __EXPERIMENTAL_IPluginAuthenticator_FWD_DEFINED__ */

/* header files for imported files */
#include "oaidl.h"
#include "webauthn.h"

#ifdef __cplusplus
extern "C" {
#endif

/* interface __MIDL_itf_pluginauthenticator_0000_0000 */
/* [local] */

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST {
  HWND hWnd;
  GUID transactionId;
  DWORD cbRequestSignature;
  /* [size_is] */ byte* pbRequestSignature;
  DWORD cbEncodedRequest;
  /* [size_is] */ byte* pbEncodedRequest;
} EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST*
    EXPERIMENTAL_PWEBAUTHN_PLUGIN_OPERATION_REQUEST;

typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_OPERATION_REQUEST;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE {
  DWORD cbEncodedResponse;
  /* [size_is] */ byte* pbEncodedResponse;
} EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE*
    EXPERIMENTAL_PWEBAUTHN_PLUGIN_OPERATION_RESPONSE;

typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_OPERATION_RESPONSE;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
  GUID transactionId;
  DWORD cbRequestSignature;
  /* [size_is] */ byte* pbRequestSignature;
} EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST*
    EXPERIMENTAL_PWEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST;

typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST;

extern RPC_IF_HANDLE __MIDL_itf_pluginauthenticator_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_pluginauthenticator_0000_0000_v0_0_s_ifspec;

#ifndef __EXPERIMENTAL_IPluginAuthenticator_INTERFACE_DEFINED__
#define __EXPERIMENTAL_IPluginAuthenticator_INTERFACE_DEFINED__

/* interface EXPERIMENTAL_IPluginAuthenticator */
/* [unique][version][uuid][object] */

EXTERN_C const IID IID_EXPERIMENTAL_IPluginAuthenticator;

#if defined(__cplusplus) && !defined(CINTERFACE)

MIDL_INTERFACE("e6466e9a-b2f3-47c5-b88d-89bc14a8d998")
EXPERIMENTAL_IPluginAuthenticator : public IUnknown {
 public:
  virtual HRESULT STDMETHODCALLTYPE EXPERIMENTAL_PluginMakeCredential(
      /* [in] */ __RPC__in EXPERIMENTAL_PCWEBAUTHN_PLUGIN_OPERATION_REQUEST
          request,
      /* [out] */
      __RPC__deref_out_opt EXPERIMENTAL_PWEBAUTHN_PLUGIN_OPERATION_RESPONSE *
          response) = 0;

  virtual HRESULT STDMETHODCALLTYPE EXPERIMENTAL_PluginGetAssertion(
      /* [in] */ __RPC__in EXPERIMENTAL_PCWEBAUTHN_PLUGIN_OPERATION_REQUEST
          request,
      /* [out] */
      __RPC__deref_out_opt EXPERIMENTAL_PWEBAUTHN_PLUGIN_OPERATION_RESPONSE *
          response) = 0;

  virtual HRESULT STDMETHODCALLTYPE EXPERIMENTAL_PluginCancelOperation(
      /* [in] */ __RPC__in
          EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST request) = 0;
};

#else /* C style interface */

typedef struct EXPERIMENTAL_IPluginAuthenticatorVtbl {
  BEGIN_INTERFACE

  DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
  HRESULT(STDMETHODCALLTYPE* QueryInterface)(
      __RPC__in EXPERIMENTAL_IPluginAuthenticator* This,
      /* [in] */ __RPC__in REFIID riid,
      /* [annotation][iid_is][out] */
      _COM_Outptr_ void** ppvObject);

  DECLSPEC_XFGVIRT(IUnknown, AddRef)
  ULONG(STDMETHODCALLTYPE* AddRef)(
      __RPC__in EXPERIMENTAL_IPluginAuthenticator* This);

  DECLSPEC_XFGVIRT(IUnknown, Release)
  ULONG(STDMETHODCALLTYPE* Release)(
      __RPC__in EXPERIMENTAL_IPluginAuthenticator* This);

  DECLSPEC_XFGVIRT(EXPERIMENTAL_IPluginAuthenticator,
                   EXPERIMENTAL_PluginMakeCredential)
  HRESULT(STDMETHODCALLTYPE* EXPERIMENTAL_PluginMakeCredential)(
      __RPC__in EXPERIMENTAL_IPluginAuthenticator* This,
      /* [in] */
      __RPC__in EXPERIMENTAL_PCWEBAUTHN_PLUGIN_OPERATION_REQUEST request,
      /* [out] */
      __RPC__deref_out_opt EXPERIMENTAL_PWEBAUTHN_PLUGIN_OPERATION_RESPONSE*
          response);

  DECLSPEC_XFGVIRT(EXPERIMENTAL_IPluginAuthenticator,
                   EXPERIMENTAL_PluginGetAssertion)
  HRESULT(STDMETHODCALLTYPE* EXPERIMENTAL_PluginGetAssertion)(
      __RPC__in EXPERIMENTAL_IPluginAuthenticator* This,
      /* [in] */
      __RPC__in EXPERIMENTAL_PCWEBAUTHN_PLUGIN_OPERATION_REQUEST request,
      /* [out] */
      __RPC__deref_out_opt EXPERIMENTAL_PWEBAUTHN_PLUGIN_OPERATION_RESPONSE*
          response);

  DECLSPEC_XFGVIRT(EXPERIMENTAL_IPluginAuthenticator,
                   EXPERIMENTAL_PluginCancelOperation)
  HRESULT(STDMETHODCALLTYPE* EXPERIMENTAL_PluginCancelOperation)(
      __RPC__in EXPERIMENTAL_IPluginAuthenticator* This,
      /* [in] */ __RPC__in
          EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST request);

  END_INTERFACE
} EXPERIMENTAL_IPluginAuthenticatorVtbl;

interface EXPERIMENTAL_IPluginAuthenticator {
  CONST_VTBL struct EXPERIMENTAL_IPluginAuthenticatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define EXPERIMENTAL_IPluginAuthenticator_QueryInterface(This, riid, \
                                                         ppvObject)  \
  ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define EXPERIMENTAL_IPluginAuthenticator_AddRef(This) \
  ((This)->lpVtbl->AddRef(This))

#define EXPERIMENTAL_IPluginAuthenticator_Release(This) \
  ((This)->lpVtbl->Release(This))

#define EXPERIMENTAL_IPluginAuthenticator_EXPERIMENTAL_PluginMakeCredential( \
    This, request, response)                                                 \
  ((This)->lpVtbl->EXPERIMENTAL_PluginMakeCredential(This, request, response))

#define EXPERIMENTAL_IPluginAuthenticator_EXPERIMENTAL_PluginGetAssertion( \
    This, request, response)                                               \
  ((This)->lpVtbl->EXPERIMENTAL_PluginGetAssertion(This, request, response))

#define EXPERIMENTAL_IPluginAuthenticator_EXPERIMENTAL_PluginCancelOperation( \
    This, request)                                                            \
  ((This)->lpVtbl->EXPERIMENTAL_PluginCancelOperation(This, request))

#endif /* COBJMACROS */

#endif /* C style interface */

#endif /* __EXPERIMENTAL_IPluginAuthenticator_INTERFACE_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

unsigned long __RPC_USER HWND_UserSize(__RPC__in unsigned long*,
                                       unsigned long,
                                       __RPC__in HWND*);
unsigned char* __RPC_USER
HWND_UserMarshal(__RPC__in unsigned long*,
                 __RPC__inout_xcount(0) unsigned char*,
                 __RPC__in HWND*);
unsigned char* __RPC_USER HWND_UserUnmarshal(__RPC__in unsigned long*,
                                             __RPC__in_xcount(0) unsigned char*,
                                             __RPC__out HWND*);
void __RPC_USER HWND_UserFree(__RPC__in unsigned long*, __RPC__in HWND*);

unsigned long __RPC_USER HWND_UserSize64(__RPC__in unsigned long*,
                                         unsigned long,
                                         __RPC__in HWND*);
unsigned char* __RPC_USER
HWND_UserMarshal64(__RPC__in unsigned long*,
                   __RPC__inout_xcount(0) unsigned char*,
                   __RPC__in HWND*);
unsigned char* __RPC_USER
HWND_UserUnmarshal64(__RPC__in unsigned long*,
                     __RPC__in_xcount(0) unsigned char*,
                     __RPC__out HWND*);
void __RPC_USER HWND_UserFree64(__RPC__in unsigned long*, __RPC__in HWND*);

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif
