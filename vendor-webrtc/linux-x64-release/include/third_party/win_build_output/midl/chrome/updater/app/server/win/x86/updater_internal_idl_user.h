

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for gen/chrome/updater/app/server/win/updater_internal_idl_user.idl:
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

#ifndef __updater_internal_idl_user_h__
#define __updater_internal_idl_user_h__

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

#ifndef __IUpdaterInternalCallback_FWD_DEFINED__
#define __IUpdaterInternalCallback_FWD_DEFINED__
typedef interface IUpdaterInternalCallback IUpdaterInternalCallback;

#endif 	/* __IUpdaterInternalCallback_FWD_DEFINED__ */


#ifndef __IUpdaterInternalCallbackUser_FWD_DEFINED__
#define __IUpdaterInternalCallbackUser_FWD_DEFINED__
typedef interface IUpdaterInternalCallbackUser IUpdaterInternalCallbackUser;

#endif 	/* __IUpdaterInternalCallbackUser_FWD_DEFINED__ */


#ifndef __IUpdaterInternal_FWD_DEFINED__
#define __IUpdaterInternal_FWD_DEFINED__
typedef interface IUpdaterInternal IUpdaterInternal;

#endif 	/* __IUpdaterInternal_FWD_DEFINED__ */


#ifndef __IUpdaterInternalUser_FWD_DEFINED__
#define __IUpdaterInternalUser_FWD_DEFINED__
typedef interface IUpdaterInternalUser IUpdaterInternalUser;

#endif 	/* __IUpdaterInternalUser_FWD_DEFINED__ */


#ifndef __UpdaterInternalUserClass_FWD_DEFINED__
#define __UpdaterInternalUserClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class UpdaterInternalUserClass UpdaterInternalUserClass;
#else
typedef struct UpdaterInternalUserClass UpdaterInternalUserClass;
#endif /* __cplusplus */

#endif 	/* __UpdaterInternalUserClass_FWD_DEFINED__ */


#ifndef __UpdaterInternalSystemClass_FWD_DEFINED__
#define __UpdaterInternalSystemClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class UpdaterInternalSystemClass UpdaterInternalSystemClass;
#else
typedef struct UpdaterInternalSystemClass UpdaterInternalSystemClass;
#endif /* __cplusplus */

#endif 	/* __UpdaterInternalSystemClass_FWD_DEFINED__ */


#ifndef __IUpdaterInternalCallbackUser_FWD_DEFINED__
#define __IUpdaterInternalCallbackUser_FWD_DEFINED__
typedef interface IUpdaterInternalCallbackUser IUpdaterInternalCallbackUser;

#endif 	/* __IUpdaterInternalCallbackUser_FWD_DEFINED__ */


#ifndef __IUpdaterInternalUser_FWD_DEFINED__
#define __IUpdaterInternalUser_FWD_DEFINED__
typedef interface IUpdaterInternalUser IUpdaterInternalUser;

#endif 	/* __IUpdaterInternalUser_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


#ifndef __IUpdaterInternalCallback_INTERFACE_DEFINED__
#define __IUpdaterInternalCallback_INTERFACE_DEFINED__

/* interface IUpdaterInternalCallback */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterInternalCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D272C794-2ACE-4584-B993-3B90C622BE65")
    IUpdaterInternalCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ LONG result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterInternalCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterInternalCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterInternalCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterInternalCallback * This);
        
        DECLSPEC_XFGVIRT(IUpdaterInternalCallback, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterInternalCallback * This,
            /* [in] */ LONG result);
        
        END_INTERFACE
    } IUpdaterInternalCallbackVtbl;

    interface IUpdaterInternalCallback
    {
        CONST_VTBL struct IUpdaterInternalCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterInternalCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterInternalCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterInternalCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterInternalCallback_Run(This,result)	\
    ( (This)->lpVtbl -> Run(This,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterInternalCallback_INTERFACE_DEFINED__ */


#ifndef __IUpdaterInternalCallbackUser_INTERFACE_DEFINED__
#define __IUpdaterInternalCallbackUser_INTERFACE_DEFINED__

/* interface IUpdaterInternalCallbackUser */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterInternalCallbackUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("618D9B82-9F51-4490-AF24-BB80489E1537")
    IUpdaterInternalCallbackUser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ LONG result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterInternalCallbackUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterInternalCallbackUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterInternalCallbackUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterInternalCallbackUser * This);
        
        DECLSPEC_XFGVIRT(IUpdaterInternalCallbackUser, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterInternalCallbackUser * This,
            /* [in] */ LONG result);
        
        END_INTERFACE
    } IUpdaterInternalCallbackUserVtbl;

    interface IUpdaterInternalCallbackUser
    {
        CONST_VTBL struct IUpdaterInternalCallbackUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterInternalCallbackUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterInternalCallbackUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterInternalCallbackUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterInternalCallbackUser_Run(This,result)	\
    ( (This)->lpVtbl -> Run(This,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterInternalCallbackUser_INTERFACE_DEFINED__ */


#ifndef __IUpdaterInternal_INTERFACE_DEFINED__
#define __IUpdaterInternal_INTERFACE_DEFINED__

/* interface IUpdaterInternal */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterInternal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("526DA036-9BD3-4697-865A-DA12D37DFFCA")
    IUpdaterInternal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ IUpdaterInternalCallback *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Hello( 
            /* [in] */ IUpdaterInternalCallback *callback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterInternalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterInternal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterInternal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterInternal * This);
        
        DECLSPEC_XFGVIRT(IUpdaterInternal, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterInternal * This,
            /* [in] */ IUpdaterInternalCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterInternal, Hello)
        HRESULT ( STDMETHODCALLTYPE *Hello )( 
            IUpdaterInternal * This,
            /* [in] */ IUpdaterInternalCallback *callback);
        
        END_INTERFACE
    } IUpdaterInternalVtbl;

    interface IUpdaterInternal
    {
        CONST_VTBL struct IUpdaterInternalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterInternal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterInternal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterInternal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterInternal_Run(This,callback)	\
    ( (This)->lpVtbl -> Run(This,callback) ) 

#define IUpdaterInternal_Hello(This,callback)	\
    ( (This)->lpVtbl -> Hello(This,callback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterInternal_INTERFACE_DEFINED__ */


#ifndef __IUpdaterInternalUser_INTERFACE_DEFINED__
#define __IUpdaterInternalUser_INTERFACE_DEFINED__

/* interface IUpdaterInternalUser */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterInternalUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C82AFDA3-CA76-46EE-96E9-474717BFA7BA")
    IUpdaterInternalUser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ IUpdaterInternalCallbackUser *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Hello( 
            /* [in] */ IUpdaterInternalCallbackUser *callback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterInternalUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterInternalUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterInternalUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterInternalUser * This);
        
        DECLSPEC_XFGVIRT(IUpdaterInternalUser, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterInternalUser * This,
            /* [in] */ IUpdaterInternalCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterInternalUser, Hello)
        HRESULT ( STDMETHODCALLTYPE *Hello )( 
            IUpdaterInternalUser * This,
            /* [in] */ IUpdaterInternalCallbackUser *callback);
        
        END_INTERFACE
    } IUpdaterInternalUserVtbl;

    interface IUpdaterInternalUser
    {
        CONST_VTBL struct IUpdaterInternalUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterInternalUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterInternalUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterInternalUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterInternalUser_Run(This,callback)	\
    ( (This)->lpVtbl -> Run(This,callback) ) 

#define IUpdaterInternalUser_Hello(This,callback)	\
    ( (This)->lpVtbl -> Hello(This,callback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterInternalUser_INTERFACE_DEFINED__ */



#ifndef __UpdaterInternalLib_LIBRARY_DEFINED__
#define __UpdaterInternalLib_LIBRARY_DEFINED__

/* library UpdaterInternalLib */
/* [helpstring][version][uuid] */ 




EXTERN_C const IID LIBID_UpdaterInternalLib;

EXTERN_C const CLSID CLSID_UpdaterInternalUserClass;

#ifdef __cplusplus

class DECLSPEC_UUID("1F87FE2F-D6A9-4711-9D11-8187705F8457")
UpdaterInternalUserClass;
#endif

EXTERN_C const CLSID CLSID_UpdaterInternalSystemClass;

#ifdef __cplusplus

class DECLSPEC_UUID("4556BA55-517E-4F03-8016-331A43C269C9")
UpdaterInternalSystemClass;
#endif
#endif /* __UpdaterInternalLib_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


