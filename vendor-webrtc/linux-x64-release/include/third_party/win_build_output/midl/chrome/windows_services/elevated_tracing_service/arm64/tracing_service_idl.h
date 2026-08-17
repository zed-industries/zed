

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../chrome/windows_services/elevated_tracing_service/tracing_service_idl.idl:
    Oicf, W1, Zp8, env=Win64 (32b run), target_arch=ARM64 8.01.0628 
    protocol : dce , ms_ext, c_ext, robust
    error checks: allocation ref bounds_check enum stub_data 
    VC __declspec() decoration level: 
         __declspec(uuid()), __declspec(selectany), __declspec(novtable)
         DECLSPEC_UUID(), MIDL_INTERFACE()
*/
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __tracing_service_idl_h__
#define __tracing_service_idl_h__

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

#ifndef __ISystemTraceSession_FWD_DEFINED__
#define __ISystemTraceSession_FWD_DEFINED__
typedef interface ISystemTraceSession ISystemTraceSession;

#endif 	/* __ISystemTraceSession_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChromium_FWD_DEFINED__
#define __ISystemTraceSessionChromium_FWD_DEFINED__
typedef interface ISystemTraceSessionChromium ISystemTraceSessionChromium;

#endif 	/* __ISystemTraceSessionChromium_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChrome_FWD_DEFINED__
#define __ISystemTraceSessionChrome_FWD_DEFINED__
typedef interface ISystemTraceSessionChrome ISystemTraceSessionChrome;

#endif 	/* __ISystemTraceSessionChrome_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChromeBeta_FWD_DEFINED__
#define __ISystemTraceSessionChromeBeta_FWD_DEFINED__
typedef interface ISystemTraceSessionChromeBeta ISystemTraceSessionChromeBeta;

#endif 	/* __ISystemTraceSessionChromeBeta_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChromeDev_FWD_DEFINED__
#define __ISystemTraceSessionChromeDev_FWD_DEFINED__
typedef interface ISystemTraceSessionChromeDev ISystemTraceSessionChromeDev;

#endif 	/* __ISystemTraceSessionChromeDev_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChromeCanary_FWD_DEFINED__
#define __ISystemTraceSessionChromeCanary_FWD_DEFINED__
typedef interface ISystemTraceSessionChromeCanary ISystemTraceSessionChromeCanary;

#endif 	/* __ISystemTraceSessionChromeCanary_FWD_DEFINED__ */


#ifndef __ISystemTraceSession_FWD_DEFINED__
#define __ISystemTraceSession_FWD_DEFINED__
typedef interface ISystemTraceSession ISystemTraceSession;

#endif 	/* __ISystemTraceSession_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChromium_FWD_DEFINED__
#define __ISystemTraceSessionChromium_FWD_DEFINED__
typedef interface ISystemTraceSessionChromium ISystemTraceSessionChromium;

#endif 	/* __ISystemTraceSessionChromium_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChrome_FWD_DEFINED__
#define __ISystemTraceSessionChrome_FWD_DEFINED__
typedef interface ISystemTraceSessionChrome ISystemTraceSessionChrome;

#endif 	/* __ISystemTraceSessionChrome_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChromeBeta_FWD_DEFINED__
#define __ISystemTraceSessionChromeBeta_FWD_DEFINED__
typedef interface ISystemTraceSessionChromeBeta ISystemTraceSessionChromeBeta;

#endif 	/* __ISystemTraceSessionChromeBeta_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChromeDev_FWD_DEFINED__
#define __ISystemTraceSessionChromeDev_FWD_DEFINED__
typedef interface ISystemTraceSessionChromeDev ISystemTraceSessionChromeDev;

#endif 	/* __ISystemTraceSessionChromeDev_FWD_DEFINED__ */


#ifndef __ISystemTraceSessionChromeCanary_FWD_DEFINED__
#define __ISystemTraceSessionChromeCanary_FWD_DEFINED__
typedef interface ISystemTraceSessionChromeCanary ISystemTraceSessionChromeCanary;

#endif 	/* __ISystemTraceSessionChromeCanary_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


#ifndef __ISystemTraceSession_INTERFACE_DEFINED__
#define __ISystemTraceSession_INTERFACE_DEFINED__

/* interface ISystemTraceSession */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISystemTraceSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB01E5CE-10CE-4A84-8FAE-DA5E46EEF1CF")
    ISystemTraceSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AcceptInvitation( 
            /* [string][in] */ const WCHAR *server_name,
            /* [out] */ DWORD *pid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemTraceSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISystemTraceSession * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISystemTraceSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISystemTraceSession * This);
        
        DECLSPEC_XFGVIRT(ISystemTraceSession, AcceptInvitation)
        HRESULT ( STDMETHODCALLTYPE *AcceptInvitation )( 
            ISystemTraceSession * This,
            /* [string][in] */ const WCHAR *server_name,
            /* [out] */ DWORD *pid);
        
        END_INTERFACE
    } ISystemTraceSessionVtbl;

    interface ISystemTraceSession
    {
        CONST_VTBL struct ISystemTraceSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemTraceSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemTraceSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemTraceSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemTraceSession_AcceptInvitation(This,server_name,pid)	\
    ( (This)->lpVtbl -> AcceptInvitation(This,server_name,pid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemTraceSession_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tracing_service_idl_0000_0001 */
/* [local] */ 

enum : HRESULT {
  kErrorSessionInProgress =
      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA001),
  kErrorCouldNotObtainCallingProcess =
      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA002),
  kErrorCouldNotGetCallingProcessPid =
      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA003),
  kErrorCouldNotOpenCallingProcess =
      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA004),
  kErrorCouldNotDuplicateHandleToClient =
      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA005),
  kErrorTooManyInvitations =
      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA006),
  kErrorSessionAlreadyActive =
      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA007),
  kErrorNotWaitingForInvitation =
      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA008),
};


extern RPC_IF_HANDLE __MIDL_itf_tracing_service_idl_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tracing_service_idl_0000_0001_v0_0_s_ifspec;

#ifndef __ISystemTraceSessionChromium_INTERFACE_DEFINED__
#define __ISystemTraceSessionChromium_INTERFACE_DEFINED__

/* interface ISystemTraceSessionChromium */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISystemTraceSessionChromium;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3FD580A-FFD4-4075-9174-75D0B199D3CB")
    ISystemTraceSessionChromium : public ISystemTraceSession
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemTraceSessionChromiumVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISystemTraceSessionChromium * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISystemTraceSessionChromium * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISystemTraceSessionChromium * This);
        
        DECLSPEC_XFGVIRT(ISystemTraceSession, AcceptInvitation)
        HRESULT ( STDMETHODCALLTYPE *AcceptInvitation )( 
            ISystemTraceSessionChromium * This,
            /* [string][in] */ const WCHAR *server_name,
            /* [out] */ DWORD *pid);
        
        END_INTERFACE
    } ISystemTraceSessionChromiumVtbl;

    interface ISystemTraceSessionChromium
    {
        CONST_VTBL struct ISystemTraceSessionChromiumVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemTraceSessionChromium_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemTraceSessionChromium_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemTraceSessionChromium_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemTraceSessionChromium_AcceptInvitation(This,server_name,pid)	\
    ( (This)->lpVtbl -> AcceptInvitation(This,server_name,pid) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemTraceSessionChromium_INTERFACE_DEFINED__ */


#ifndef __ISystemTraceSessionChrome_INTERFACE_DEFINED__
#define __ISystemTraceSessionChrome_INTERFACE_DEFINED__

/* interface ISystemTraceSessionChrome */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISystemTraceSessionChrome;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("056B3371-1C09-475B-A8D7-9E58BF45533E")
    ISystemTraceSessionChrome : public ISystemTraceSession
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemTraceSessionChromeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISystemTraceSessionChrome * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISystemTraceSessionChrome * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISystemTraceSessionChrome * This);
        
        DECLSPEC_XFGVIRT(ISystemTraceSession, AcceptInvitation)
        HRESULT ( STDMETHODCALLTYPE *AcceptInvitation )( 
            ISystemTraceSessionChrome * This,
            /* [string][in] */ const WCHAR *server_name,
            /* [out] */ DWORD *pid);
        
        END_INTERFACE
    } ISystemTraceSessionChromeVtbl;

    interface ISystemTraceSessionChrome
    {
        CONST_VTBL struct ISystemTraceSessionChromeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemTraceSessionChrome_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemTraceSessionChrome_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemTraceSessionChrome_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemTraceSessionChrome_AcceptInvitation(This,server_name,pid)	\
    ( (This)->lpVtbl -> AcceptInvitation(This,server_name,pid) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemTraceSessionChrome_INTERFACE_DEFINED__ */


#ifndef __ISystemTraceSessionChromeBeta_INTERFACE_DEFINED__
#define __ISystemTraceSessionChromeBeta_INTERFACE_DEFINED__

/* interface ISystemTraceSessionChromeBeta */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISystemTraceSessionChromeBeta;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A69D7D7D-9A08-422A-B6C6-B7B8D376A12C")
    ISystemTraceSessionChromeBeta : public ISystemTraceSession
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemTraceSessionChromeBetaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISystemTraceSessionChromeBeta * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISystemTraceSessionChromeBeta * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISystemTraceSessionChromeBeta * This);
        
        DECLSPEC_XFGVIRT(ISystemTraceSession, AcceptInvitation)
        HRESULT ( STDMETHODCALLTYPE *AcceptInvitation )( 
            ISystemTraceSessionChromeBeta * This,
            /* [string][in] */ const WCHAR *server_name,
            /* [out] */ DWORD *pid);
        
        END_INTERFACE
    } ISystemTraceSessionChromeBetaVtbl;

    interface ISystemTraceSessionChromeBeta
    {
        CONST_VTBL struct ISystemTraceSessionChromeBetaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemTraceSessionChromeBeta_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemTraceSessionChromeBeta_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemTraceSessionChromeBeta_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemTraceSessionChromeBeta_AcceptInvitation(This,server_name,pid)	\
    ( (This)->lpVtbl -> AcceptInvitation(This,server_name,pid) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemTraceSessionChromeBeta_INTERFACE_DEFINED__ */


#ifndef __ISystemTraceSessionChromeDev_INTERFACE_DEFINED__
#define __ISystemTraceSessionChromeDev_INTERFACE_DEFINED__

/* interface ISystemTraceSessionChromeDev */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISystemTraceSessionChromeDev;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E08ADAE8-9334-46ED-B0CF-DD1780158D55")
    ISystemTraceSessionChromeDev : public ISystemTraceSession
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemTraceSessionChromeDevVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISystemTraceSessionChromeDev * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISystemTraceSessionChromeDev * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISystemTraceSessionChromeDev * This);
        
        DECLSPEC_XFGVIRT(ISystemTraceSession, AcceptInvitation)
        HRESULT ( STDMETHODCALLTYPE *AcceptInvitation )( 
            ISystemTraceSessionChromeDev * This,
            /* [string][in] */ const WCHAR *server_name,
            /* [out] */ DWORD *pid);
        
        END_INTERFACE
    } ISystemTraceSessionChromeDevVtbl;

    interface ISystemTraceSessionChromeDev
    {
        CONST_VTBL struct ISystemTraceSessionChromeDevVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemTraceSessionChromeDev_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemTraceSessionChromeDev_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemTraceSessionChromeDev_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemTraceSessionChromeDev_AcceptInvitation(This,server_name,pid)	\
    ( (This)->lpVtbl -> AcceptInvitation(This,server_name,pid) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemTraceSessionChromeDev_INTERFACE_DEFINED__ */


#ifndef __ISystemTraceSessionChromeCanary_INTERFACE_DEFINED__
#define __ISystemTraceSessionChromeCanary_INTERFACE_DEFINED__

/* interface ISystemTraceSessionChromeCanary */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISystemTraceSessionChromeCanary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6EFB8558-68D1-4826-A612-A180B3570375")
    ISystemTraceSessionChromeCanary : public ISystemTraceSession
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemTraceSessionChromeCanaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISystemTraceSessionChromeCanary * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISystemTraceSessionChromeCanary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISystemTraceSessionChromeCanary * This);
        
        DECLSPEC_XFGVIRT(ISystemTraceSession, AcceptInvitation)
        HRESULT ( STDMETHODCALLTYPE *AcceptInvitation )( 
            ISystemTraceSessionChromeCanary * This,
            /* [string][in] */ const WCHAR *server_name,
            /* [out] */ DWORD *pid);
        
        END_INTERFACE
    } ISystemTraceSessionChromeCanaryVtbl;

    interface ISystemTraceSessionChromeCanary
    {
        CONST_VTBL struct ISystemTraceSessionChromeCanaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemTraceSessionChromeCanary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemTraceSessionChromeCanary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemTraceSessionChromeCanary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemTraceSessionChromeCanary_AcceptInvitation(This,server_name,pid)	\
    ( (This)->lpVtbl -> AcceptInvitation(This,server_name,pid) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemTraceSessionChromeCanary_INTERFACE_DEFINED__ */



#ifndef __SystemTraceSessionLib_LIBRARY_DEFINED__
#define __SystemTraceSessionLib_LIBRARY_DEFINED__

/* library SystemTraceSessionLib */
/* [helpstring][version][uuid] */ 








EXTERN_C const IID LIBID_SystemTraceSessionLib;
#endif /* __SystemTraceSessionLib_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


