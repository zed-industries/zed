

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../chrome/windows_services/service_program/test_service_idl.idl:
    Oicf, W1, Zp8, env=Win32 (32b run), target_arch=X86 8.xx.xxxx 
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

#ifndef __test_service_idl_h__
#define __test_service_idl_h__

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

#ifndef __ITestService_FWD_DEFINED__
#define __ITestService_FWD_DEFINED__
typedef interface ITestService ITestService;

#endif 	/* __ITestService_FWD_DEFINED__ */


#ifndef __ITestService_FWD_DEFINED__
#define __ITestService_FWD_DEFINED__
typedef interface ITestService ITestService;

#endif 	/* __ITestService_FWD_DEFINED__ */


#ifndef __TestService_FWD_DEFINED__
#define __TestService_FWD_DEFINED__

#ifdef __cplusplus
typedef class TestService TestService;
#else
typedef struct TestService TestService;
#endif /* __cplusplus */

#endif 	/* __TestService_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


#ifndef __ITestService_INTERFACE_DEFINED__
#define __ITestService_INTERFACE_DEFINED__

/* interface ITestService */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ITestService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C8760DFF-5BE9-45E0-86E5-80B9C9F27DF7")
    ITestService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProcessHandle( 
            /* [out] */ unsigned long *handle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRunningUnattended( 
            /* [out] */ VARIANT_BOOL *is_running_unattended) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCrashpadDatabasePath( 
            /* [out] */ BSTR *database_path) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InduceCrash( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InduceCrashSoon( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITestServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITestService * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITestService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITestService * This);
        
        DECLSPEC_XFGVIRT(ITestService, GetProcessHandle)
        HRESULT ( STDMETHODCALLTYPE *GetProcessHandle )( 
            ITestService * This,
            /* [out] */ unsigned long *handle);
        
        DECLSPEC_XFGVIRT(ITestService, IsRunningUnattended)
        HRESULT ( STDMETHODCALLTYPE *IsRunningUnattended )( 
            ITestService * This,
            /* [out] */ VARIANT_BOOL *is_running_unattended);
        
        DECLSPEC_XFGVIRT(ITestService, GetCrashpadDatabasePath)
        HRESULT ( STDMETHODCALLTYPE *GetCrashpadDatabasePath )( 
            ITestService * This,
            /* [out] */ BSTR *database_path);
        
        DECLSPEC_XFGVIRT(ITestService, InduceCrash)
        HRESULT ( STDMETHODCALLTYPE *InduceCrash )( 
            ITestService * This);
        
        DECLSPEC_XFGVIRT(ITestService, InduceCrashSoon)
        HRESULT ( STDMETHODCALLTYPE *InduceCrashSoon )( 
            ITestService * This);
        
        END_INTERFACE
    } ITestServiceVtbl;

    interface ITestService
    {
        CONST_VTBL struct ITestServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITestService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITestService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITestService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITestService_GetProcessHandle(This,handle)	\
    ( (This)->lpVtbl -> GetProcessHandle(This,handle) ) 

#define ITestService_IsRunningUnattended(This,is_running_unattended)	\
    ( (This)->lpVtbl -> IsRunningUnattended(This,is_running_unattended) ) 

#define ITestService_GetCrashpadDatabasePath(This,database_path)	\
    ( (This)->lpVtbl -> GetCrashpadDatabasePath(This,database_path) ) 

#define ITestService_InduceCrash(This)	\
    ( (This)->lpVtbl -> InduceCrash(This) ) 

#define ITestService_InduceCrashSoon(This)	\
    ( (This)->lpVtbl -> InduceCrashSoon(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITestService_INTERFACE_DEFINED__ */



#ifndef __TestServiceLib_LIBRARY_DEFINED__
#define __TestServiceLib_LIBRARY_DEFINED__

/* library TestServiceLib */
/* [helpstring][version][uuid] */ 



EXTERN_C const IID LIBID_TestServiceLib;

EXTERN_C const CLSID CLSID_TestService;

#ifdef __cplusplus

class DECLSPEC_UUID("4428C238-EBE7-4B6C-8DAB-A1A76886A991")
TestService;
#endif
#endif /* __TestServiceLib_LIBRARY_DEFINED__ */

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


