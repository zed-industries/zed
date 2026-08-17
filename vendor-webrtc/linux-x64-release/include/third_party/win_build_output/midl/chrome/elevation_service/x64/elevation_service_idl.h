

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../chrome/elevation_service/elevation_service_idl.idl:
    Oicf, W1, Zp8, env=Win64 (32b run), target_arch=AMD64 8.xx.xxxx 
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

#ifndef __elevation_service_idl_h__
#define __elevation_service_idl_h__

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

#ifndef __IElevator_FWD_DEFINED__
#define __IElevator_FWD_DEFINED__
typedef interface IElevator IElevator;

#endif 	/* __IElevator_FWD_DEFINED__ */


#ifndef __IElevatorChromium_FWD_DEFINED__
#define __IElevatorChromium_FWD_DEFINED__
typedef interface IElevatorChromium IElevatorChromium;

#endif 	/* __IElevatorChromium_FWD_DEFINED__ */


#ifndef __IElevatorChrome_FWD_DEFINED__
#define __IElevatorChrome_FWD_DEFINED__
typedef interface IElevatorChrome IElevatorChrome;

#endif 	/* __IElevatorChrome_FWD_DEFINED__ */


#ifndef __IElevatorChromeBeta_FWD_DEFINED__
#define __IElevatorChromeBeta_FWD_DEFINED__
typedef interface IElevatorChromeBeta IElevatorChromeBeta;

#endif 	/* __IElevatorChromeBeta_FWD_DEFINED__ */


#ifndef __IElevatorChromeDev_FWD_DEFINED__
#define __IElevatorChromeDev_FWD_DEFINED__
typedef interface IElevatorChromeDev IElevatorChromeDev;

#endif 	/* __IElevatorChromeDev_FWD_DEFINED__ */


#ifndef __IElevatorChromeCanary_FWD_DEFINED__
#define __IElevatorChromeCanary_FWD_DEFINED__
typedef interface IElevatorChromeCanary IElevatorChromeCanary;

#endif 	/* __IElevatorChromeCanary_FWD_DEFINED__ */


#ifndef __IElevator_FWD_DEFINED__
#define __IElevator_FWD_DEFINED__
typedef interface IElevator IElevator;

#endif 	/* __IElevator_FWD_DEFINED__ */


#ifndef __IElevatorChromium_FWD_DEFINED__
#define __IElevatorChromium_FWD_DEFINED__
typedef interface IElevatorChromium IElevatorChromium;

#endif 	/* __IElevatorChromium_FWD_DEFINED__ */


#ifndef __IElevatorChrome_FWD_DEFINED__
#define __IElevatorChrome_FWD_DEFINED__
typedef interface IElevatorChrome IElevatorChrome;

#endif 	/* __IElevatorChrome_FWD_DEFINED__ */


#ifndef __IElevatorChromeBeta_FWD_DEFINED__
#define __IElevatorChromeBeta_FWD_DEFINED__
typedef interface IElevatorChromeBeta IElevatorChromeBeta;

#endif 	/* __IElevatorChromeBeta_FWD_DEFINED__ */


#ifndef __IElevatorChromeDev_FWD_DEFINED__
#define __IElevatorChromeDev_FWD_DEFINED__
typedef interface IElevatorChromeDev IElevatorChromeDev;

#endif 	/* __IElevatorChromeDev_FWD_DEFINED__ */


#ifndef __IElevatorChromeCanary_FWD_DEFINED__
#define __IElevatorChromeCanary_FWD_DEFINED__
typedef interface IElevatorChromeCanary IElevatorChromeCanary;

#endif 	/* __IElevatorChromeCanary_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_elevation_service_idl_0000_0000 */
/* [local] */ 

typedef 
enum ProtectionLevel
    {
        PROTECTION_NONE	= 0,
        PROTECTION_PATH_VALIDATION_OLD	= 1,
        PROTECTION_PATH_VALIDATION	= 2,
        PROTECTION_MAX	= 3
    } 	ProtectionLevel;



extern RPC_IF_HANDLE __MIDL_itf_elevation_service_idl_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_elevation_service_idl_0000_0000_v0_0_s_ifspec;

#ifndef __IElevator_INTERFACE_DEFINED__
#define __IElevator_INTERFACE_DEFINED__

/* interface IElevator */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IElevator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A949CB4E-C4F9-44C4-B213-6BF8AA9AC69C")
    IElevator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RunRecoveryCRXElevated( 
            /* [string][in] */ const WCHAR *crx_path,
            /* [string][in] */ const WCHAR *browser_appid,
            /* [string][in] */ const WCHAR *browser_version,
            /* [string][in] */ const WCHAR *session_id,
            /* [in] */ DWORD caller_proc_id,
            /* [out] */ ULONG_PTR *proc_handle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EncryptData( 
            /* [in] */ ProtectionLevel protection_level,
            /* [in] */ const BSTR plaintext,
            /* [out] */ BSTR *ciphertext,
            /* [out] */ DWORD *last_error) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DecryptData( 
            /* [in] */ const BSTR ciphertext,
            /* [out] */ BSTR *plaintext,
            /* [out] */ DWORD *last_error) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IElevatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IElevator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IElevator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IElevator * This);
        
        DECLSPEC_XFGVIRT(IElevator, RunRecoveryCRXElevated)
        HRESULT ( STDMETHODCALLTYPE *RunRecoveryCRXElevated )( 
            IElevator * This,
            /* [string][in] */ const WCHAR *crx_path,
            /* [string][in] */ const WCHAR *browser_appid,
            /* [string][in] */ const WCHAR *browser_version,
            /* [string][in] */ const WCHAR *session_id,
            /* [in] */ DWORD caller_proc_id,
            /* [out] */ ULONG_PTR *proc_handle);
        
        DECLSPEC_XFGVIRT(IElevator, EncryptData)
        HRESULT ( STDMETHODCALLTYPE *EncryptData )( 
            IElevator * This,
            /* [in] */ ProtectionLevel protection_level,
            /* [in] */ const BSTR plaintext,
            /* [out] */ BSTR *ciphertext,
            /* [out] */ DWORD *last_error);
        
        DECLSPEC_XFGVIRT(IElevator, DecryptData)
        HRESULT ( STDMETHODCALLTYPE *DecryptData )( 
            IElevator * This,
            /* [in] */ const BSTR ciphertext,
            /* [out] */ BSTR *plaintext,
            /* [out] */ DWORD *last_error);
        
        END_INTERFACE
    } IElevatorVtbl;

    interface IElevator
    {
        CONST_VTBL struct IElevatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IElevator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IElevator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IElevator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IElevator_RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle)	\
    ( (This)->lpVtbl -> RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle) ) 

#define IElevator_EncryptData(This,protection_level,plaintext,ciphertext,last_error)	\
    ( (This)->lpVtbl -> EncryptData(This,protection_level,plaintext,ciphertext,last_error) ) 

#define IElevator_DecryptData(This,ciphertext,plaintext,last_error)	\
    ( (This)->lpVtbl -> DecryptData(This,ciphertext,plaintext,last_error) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IElevator_INTERFACE_DEFINED__ */


#ifndef __IElevatorChromium_INTERFACE_DEFINED__
#define __IElevatorChromium_INTERFACE_DEFINED__

/* interface IElevatorChromium */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IElevatorChromium;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B88C45B9-8825-4629-B83E-77CC67D9CEED")
    IElevatorChromium : public IElevator
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IElevatorChromiumVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IElevatorChromium * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IElevatorChromium * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IElevatorChromium * This);
        
        DECLSPEC_XFGVIRT(IElevator, RunRecoveryCRXElevated)
        HRESULT ( STDMETHODCALLTYPE *RunRecoveryCRXElevated )( 
            IElevatorChromium * This,
            /* [string][in] */ const WCHAR *crx_path,
            /* [string][in] */ const WCHAR *browser_appid,
            /* [string][in] */ const WCHAR *browser_version,
            /* [string][in] */ const WCHAR *session_id,
            /* [in] */ DWORD caller_proc_id,
            /* [out] */ ULONG_PTR *proc_handle);
        
        DECLSPEC_XFGVIRT(IElevator, EncryptData)
        HRESULT ( STDMETHODCALLTYPE *EncryptData )( 
            IElevatorChromium * This,
            /* [in] */ ProtectionLevel protection_level,
            /* [in] */ const BSTR plaintext,
            /* [out] */ BSTR *ciphertext,
            /* [out] */ DWORD *last_error);
        
        DECLSPEC_XFGVIRT(IElevator, DecryptData)
        HRESULT ( STDMETHODCALLTYPE *DecryptData )( 
            IElevatorChromium * This,
            /* [in] */ const BSTR ciphertext,
            /* [out] */ BSTR *plaintext,
            /* [out] */ DWORD *last_error);
        
        END_INTERFACE
    } IElevatorChromiumVtbl;

    interface IElevatorChromium
    {
        CONST_VTBL struct IElevatorChromiumVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IElevatorChromium_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IElevatorChromium_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IElevatorChromium_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IElevatorChromium_RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle)	\
    ( (This)->lpVtbl -> RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle) ) 

#define IElevatorChromium_EncryptData(This,protection_level,plaintext,ciphertext,last_error)	\
    ( (This)->lpVtbl -> EncryptData(This,protection_level,plaintext,ciphertext,last_error) ) 

#define IElevatorChromium_DecryptData(This,ciphertext,plaintext,last_error)	\
    ( (This)->lpVtbl -> DecryptData(This,ciphertext,plaintext,last_error) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IElevatorChromium_INTERFACE_DEFINED__ */


#ifndef __IElevatorChrome_INTERFACE_DEFINED__
#define __IElevatorChrome_INTERFACE_DEFINED__

/* interface IElevatorChrome */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IElevatorChrome;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("463ABECF-410D-407F-8AF5-0DF35A005CC8")
    IElevatorChrome : public IElevator
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IElevatorChromeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IElevatorChrome * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IElevatorChrome * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IElevatorChrome * This);
        
        DECLSPEC_XFGVIRT(IElevator, RunRecoveryCRXElevated)
        HRESULT ( STDMETHODCALLTYPE *RunRecoveryCRXElevated )( 
            IElevatorChrome * This,
            /* [string][in] */ const WCHAR *crx_path,
            /* [string][in] */ const WCHAR *browser_appid,
            /* [string][in] */ const WCHAR *browser_version,
            /* [string][in] */ const WCHAR *session_id,
            /* [in] */ DWORD caller_proc_id,
            /* [out] */ ULONG_PTR *proc_handle);
        
        DECLSPEC_XFGVIRT(IElevator, EncryptData)
        HRESULT ( STDMETHODCALLTYPE *EncryptData )( 
            IElevatorChrome * This,
            /* [in] */ ProtectionLevel protection_level,
            /* [in] */ const BSTR plaintext,
            /* [out] */ BSTR *ciphertext,
            /* [out] */ DWORD *last_error);
        
        DECLSPEC_XFGVIRT(IElevator, DecryptData)
        HRESULT ( STDMETHODCALLTYPE *DecryptData )( 
            IElevatorChrome * This,
            /* [in] */ const BSTR ciphertext,
            /* [out] */ BSTR *plaintext,
            /* [out] */ DWORD *last_error);
        
        END_INTERFACE
    } IElevatorChromeVtbl;

    interface IElevatorChrome
    {
        CONST_VTBL struct IElevatorChromeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IElevatorChrome_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IElevatorChrome_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IElevatorChrome_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IElevatorChrome_RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle)	\
    ( (This)->lpVtbl -> RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle) ) 

#define IElevatorChrome_EncryptData(This,protection_level,plaintext,ciphertext,last_error)	\
    ( (This)->lpVtbl -> EncryptData(This,protection_level,plaintext,ciphertext,last_error) ) 

#define IElevatorChrome_DecryptData(This,ciphertext,plaintext,last_error)	\
    ( (This)->lpVtbl -> DecryptData(This,ciphertext,plaintext,last_error) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IElevatorChrome_INTERFACE_DEFINED__ */


#ifndef __IElevatorChromeBeta_INTERFACE_DEFINED__
#define __IElevatorChromeBeta_INTERFACE_DEFINED__

/* interface IElevatorChromeBeta */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IElevatorChromeBeta;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A2721D66-376E-4D2F-9F0F-9070E9A42B5F")
    IElevatorChromeBeta : public IElevator
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IElevatorChromeBetaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IElevatorChromeBeta * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IElevatorChromeBeta * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IElevatorChromeBeta * This);
        
        DECLSPEC_XFGVIRT(IElevator, RunRecoveryCRXElevated)
        HRESULT ( STDMETHODCALLTYPE *RunRecoveryCRXElevated )( 
            IElevatorChromeBeta * This,
            /* [string][in] */ const WCHAR *crx_path,
            /* [string][in] */ const WCHAR *browser_appid,
            /* [string][in] */ const WCHAR *browser_version,
            /* [string][in] */ const WCHAR *session_id,
            /* [in] */ DWORD caller_proc_id,
            /* [out] */ ULONG_PTR *proc_handle);
        
        DECLSPEC_XFGVIRT(IElevator, EncryptData)
        HRESULT ( STDMETHODCALLTYPE *EncryptData )( 
            IElevatorChromeBeta * This,
            /* [in] */ ProtectionLevel protection_level,
            /* [in] */ const BSTR plaintext,
            /* [out] */ BSTR *ciphertext,
            /* [out] */ DWORD *last_error);
        
        DECLSPEC_XFGVIRT(IElevator, DecryptData)
        HRESULT ( STDMETHODCALLTYPE *DecryptData )( 
            IElevatorChromeBeta * This,
            /* [in] */ const BSTR ciphertext,
            /* [out] */ BSTR *plaintext,
            /* [out] */ DWORD *last_error);
        
        END_INTERFACE
    } IElevatorChromeBetaVtbl;

    interface IElevatorChromeBeta
    {
        CONST_VTBL struct IElevatorChromeBetaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IElevatorChromeBeta_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IElevatorChromeBeta_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IElevatorChromeBeta_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IElevatorChromeBeta_RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle)	\
    ( (This)->lpVtbl -> RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle) ) 

#define IElevatorChromeBeta_EncryptData(This,protection_level,plaintext,ciphertext,last_error)	\
    ( (This)->lpVtbl -> EncryptData(This,protection_level,plaintext,ciphertext,last_error) ) 

#define IElevatorChromeBeta_DecryptData(This,ciphertext,plaintext,last_error)	\
    ( (This)->lpVtbl -> DecryptData(This,ciphertext,plaintext,last_error) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IElevatorChromeBeta_INTERFACE_DEFINED__ */


#ifndef __IElevatorChromeDev_INTERFACE_DEFINED__
#define __IElevatorChromeDev_INTERFACE_DEFINED__

/* interface IElevatorChromeDev */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IElevatorChromeDev;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB2AA26B-343A-4072-8B6F-80557B8CE571")
    IElevatorChromeDev : public IElevator
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IElevatorChromeDevVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IElevatorChromeDev * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IElevatorChromeDev * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IElevatorChromeDev * This);
        
        DECLSPEC_XFGVIRT(IElevator, RunRecoveryCRXElevated)
        HRESULT ( STDMETHODCALLTYPE *RunRecoveryCRXElevated )( 
            IElevatorChromeDev * This,
            /* [string][in] */ const WCHAR *crx_path,
            /* [string][in] */ const WCHAR *browser_appid,
            /* [string][in] */ const WCHAR *browser_version,
            /* [string][in] */ const WCHAR *session_id,
            /* [in] */ DWORD caller_proc_id,
            /* [out] */ ULONG_PTR *proc_handle);
        
        DECLSPEC_XFGVIRT(IElevator, EncryptData)
        HRESULT ( STDMETHODCALLTYPE *EncryptData )( 
            IElevatorChromeDev * This,
            /* [in] */ ProtectionLevel protection_level,
            /* [in] */ const BSTR plaintext,
            /* [out] */ BSTR *ciphertext,
            /* [out] */ DWORD *last_error);
        
        DECLSPEC_XFGVIRT(IElevator, DecryptData)
        HRESULT ( STDMETHODCALLTYPE *DecryptData )( 
            IElevatorChromeDev * This,
            /* [in] */ const BSTR ciphertext,
            /* [out] */ BSTR *plaintext,
            /* [out] */ DWORD *last_error);
        
        END_INTERFACE
    } IElevatorChromeDevVtbl;

    interface IElevatorChromeDev
    {
        CONST_VTBL struct IElevatorChromeDevVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IElevatorChromeDev_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IElevatorChromeDev_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IElevatorChromeDev_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IElevatorChromeDev_RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle)	\
    ( (This)->lpVtbl -> RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle) ) 

#define IElevatorChromeDev_EncryptData(This,protection_level,plaintext,ciphertext,last_error)	\
    ( (This)->lpVtbl -> EncryptData(This,protection_level,plaintext,ciphertext,last_error) ) 

#define IElevatorChromeDev_DecryptData(This,ciphertext,plaintext,last_error)	\
    ( (This)->lpVtbl -> DecryptData(This,ciphertext,plaintext,last_error) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IElevatorChromeDev_INTERFACE_DEFINED__ */


#ifndef __IElevatorChromeCanary_INTERFACE_DEFINED__
#define __IElevatorChromeCanary_INTERFACE_DEFINED__

/* interface IElevatorChromeCanary */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IElevatorChromeCanary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4F7CE041-28E9-484F-9DD0-61A8CACEFEE4")
    IElevatorChromeCanary : public IElevator
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IElevatorChromeCanaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IElevatorChromeCanary * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IElevatorChromeCanary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IElevatorChromeCanary * This);
        
        DECLSPEC_XFGVIRT(IElevator, RunRecoveryCRXElevated)
        HRESULT ( STDMETHODCALLTYPE *RunRecoveryCRXElevated )( 
            IElevatorChromeCanary * This,
            /* [string][in] */ const WCHAR *crx_path,
            /* [string][in] */ const WCHAR *browser_appid,
            /* [string][in] */ const WCHAR *browser_version,
            /* [string][in] */ const WCHAR *session_id,
            /* [in] */ DWORD caller_proc_id,
            /* [out] */ ULONG_PTR *proc_handle);
        
        DECLSPEC_XFGVIRT(IElevator, EncryptData)
        HRESULT ( STDMETHODCALLTYPE *EncryptData )( 
            IElevatorChromeCanary * This,
            /* [in] */ ProtectionLevel protection_level,
            /* [in] */ const BSTR plaintext,
            /* [out] */ BSTR *ciphertext,
            /* [out] */ DWORD *last_error);
        
        DECLSPEC_XFGVIRT(IElevator, DecryptData)
        HRESULT ( STDMETHODCALLTYPE *DecryptData )( 
            IElevatorChromeCanary * This,
            /* [in] */ const BSTR ciphertext,
            /* [out] */ BSTR *plaintext,
            /* [out] */ DWORD *last_error);
        
        END_INTERFACE
    } IElevatorChromeCanaryVtbl;

    interface IElevatorChromeCanary
    {
        CONST_VTBL struct IElevatorChromeCanaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IElevatorChromeCanary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IElevatorChromeCanary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IElevatorChromeCanary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IElevatorChromeCanary_RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle)	\
    ( (This)->lpVtbl -> RunRecoveryCRXElevated(This,crx_path,browser_appid,browser_version,session_id,caller_proc_id,proc_handle) ) 

#define IElevatorChromeCanary_EncryptData(This,protection_level,plaintext,ciphertext,last_error)	\
    ( (This)->lpVtbl -> EncryptData(This,protection_level,plaintext,ciphertext,last_error) ) 

#define IElevatorChromeCanary_DecryptData(This,ciphertext,plaintext,last_error)	\
    ( (This)->lpVtbl -> DecryptData(This,ciphertext,plaintext,last_error) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IElevatorChromeCanary_INTERFACE_DEFINED__ */



#ifndef __ElevatorLib_LIBRARY_DEFINED__
#define __ElevatorLib_LIBRARY_DEFINED__

/* library ElevatorLib */
/* [helpstring][version][uuid] */ 








EXTERN_C const IID LIBID_ElevatorLib;
#endif /* __ElevatorLib_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long *, unsigned long            , BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  unsigned long *, unsigned char *, BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(unsigned long *, unsigned char *, BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long *, BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


