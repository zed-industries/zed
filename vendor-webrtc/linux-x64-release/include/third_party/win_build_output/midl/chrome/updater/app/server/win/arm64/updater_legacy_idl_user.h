

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for gen/chrome/updater/app/server/win/updater_legacy_idl_user.idl:
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

#ifndef __updater_legacy_idl_user_h__
#define __updater_legacy_idl_user_h__

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

#ifndef __IAppVersionWeb_FWD_DEFINED__
#define __IAppVersionWeb_FWD_DEFINED__
typedef interface IAppVersionWeb IAppVersionWeb;

#endif 	/* __IAppVersionWeb_FWD_DEFINED__ */


#ifndef __IAppVersionWebUser_FWD_DEFINED__
#define __IAppVersionWebUser_FWD_DEFINED__
typedef interface IAppVersionWebUser IAppVersionWebUser;

#endif 	/* __IAppVersionWebUser_FWD_DEFINED__ */


#ifndef __ICurrentState_FWD_DEFINED__
#define __ICurrentState_FWD_DEFINED__
typedef interface ICurrentState ICurrentState;

#endif 	/* __ICurrentState_FWD_DEFINED__ */


#ifndef __ICurrentStateUser_FWD_DEFINED__
#define __ICurrentStateUser_FWD_DEFINED__
typedef interface ICurrentStateUser ICurrentStateUser;

#endif 	/* __ICurrentStateUser_FWD_DEFINED__ */


#ifndef __IGoogleUpdate3Web_FWD_DEFINED__
#define __IGoogleUpdate3Web_FWD_DEFINED__
typedef interface IGoogleUpdate3Web IGoogleUpdate3Web;

#endif 	/* __IGoogleUpdate3Web_FWD_DEFINED__ */


#ifndef __IGoogleUpdate3WebUser_FWD_DEFINED__
#define __IGoogleUpdate3WebUser_FWD_DEFINED__
typedef interface IGoogleUpdate3WebUser IGoogleUpdate3WebUser;

#endif 	/* __IGoogleUpdate3WebUser_FWD_DEFINED__ */


#ifndef __IAppBundleWeb_FWD_DEFINED__
#define __IAppBundleWeb_FWD_DEFINED__
typedef interface IAppBundleWeb IAppBundleWeb;

#endif 	/* __IAppBundleWeb_FWD_DEFINED__ */


#ifndef __IAppBundleWebUser_FWD_DEFINED__
#define __IAppBundleWebUser_FWD_DEFINED__
typedef interface IAppBundleWebUser IAppBundleWebUser;

#endif 	/* __IAppBundleWebUser_FWD_DEFINED__ */


#ifndef __IAppWeb_FWD_DEFINED__
#define __IAppWeb_FWD_DEFINED__
typedef interface IAppWeb IAppWeb;

#endif 	/* __IAppWeb_FWD_DEFINED__ */


#ifndef __IAppWebUser_FWD_DEFINED__
#define __IAppWebUser_FWD_DEFINED__
typedef interface IAppWebUser IAppWebUser;

#endif 	/* __IAppWebUser_FWD_DEFINED__ */


#ifndef __IAppCommandWeb_FWD_DEFINED__
#define __IAppCommandWeb_FWD_DEFINED__
typedef interface IAppCommandWeb IAppCommandWeb;

#endif 	/* __IAppCommandWeb_FWD_DEFINED__ */


#ifndef __IAppCommandWebUser_FWD_DEFINED__
#define __IAppCommandWebUser_FWD_DEFINED__
typedef interface IAppCommandWebUser IAppCommandWebUser;

#endif 	/* __IAppCommandWebUser_FWD_DEFINED__ */


#ifndef __IPolicyStatus_FWD_DEFINED__
#define __IPolicyStatus_FWD_DEFINED__
typedef interface IPolicyStatus IPolicyStatus;

#endif 	/* __IPolicyStatus_FWD_DEFINED__ */


#ifndef __IPolicyStatusUser_FWD_DEFINED__
#define __IPolicyStatusUser_FWD_DEFINED__
typedef interface IPolicyStatusUser IPolicyStatusUser;

#endif 	/* __IPolicyStatusUser_FWD_DEFINED__ */


#ifndef __IPolicyStatusValue_FWD_DEFINED__
#define __IPolicyStatusValue_FWD_DEFINED__
typedef interface IPolicyStatusValue IPolicyStatusValue;

#endif 	/* __IPolicyStatusValue_FWD_DEFINED__ */


#ifndef __IPolicyStatusValueUser_FWD_DEFINED__
#define __IPolicyStatusValueUser_FWD_DEFINED__
typedef interface IPolicyStatusValueUser IPolicyStatusValueUser;

#endif 	/* __IPolicyStatusValueUser_FWD_DEFINED__ */


#ifndef __IPolicyStatus2_FWD_DEFINED__
#define __IPolicyStatus2_FWD_DEFINED__
typedef interface IPolicyStatus2 IPolicyStatus2;

#endif 	/* __IPolicyStatus2_FWD_DEFINED__ */


#ifndef __IPolicyStatus2User_FWD_DEFINED__
#define __IPolicyStatus2User_FWD_DEFINED__
typedef interface IPolicyStatus2User IPolicyStatus2User;

#endif 	/* __IPolicyStatus2User_FWD_DEFINED__ */


#ifndef __IPolicyStatus3_FWD_DEFINED__
#define __IPolicyStatus3_FWD_DEFINED__
typedef interface IPolicyStatus3 IPolicyStatus3;

#endif 	/* __IPolicyStatus3_FWD_DEFINED__ */


#ifndef __IPolicyStatus3User_FWD_DEFINED__
#define __IPolicyStatus3User_FWD_DEFINED__
typedef interface IPolicyStatus3User IPolicyStatus3User;

#endif 	/* __IPolicyStatus3User_FWD_DEFINED__ */


#ifndef __IPolicyStatus4_FWD_DEFINED__
#define __IPolicyStatus4_FWD_DEFINED__
typedef interface IPolicyStatus4 IPolicyStatus4;

#endif 	/* __IPolicyStatus4_FWD_DEFINED__ */


#ifndef __IPolicyStatus4User_FWD_DEFINED__
#define __IPolicyStatus4User_FWD_DEFINED__
typedef interface IPolicyStatus4User IPolicyStatus4User;

#endif 	/* __IPolicyStatus4User_FWD_DEFINED__ */


#ifndef __GoogleUpdate3WebUserClass_FWD_DEFINED__
#define __GoogleUpdate3WebUserClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class GoogleUpdate3WebUserClass GoogleUpdate3WebUserClass;
#else
typedef struct GoogleUpdate3WebUserClass GoogleUpdate3WebUserClass;
#endif /* __cplusplus */

#endif 	/* __GoogleUpdate3WebUserClass_FWD_DEFINED__ */


#ifndef __GoogleUpdate3WebSystemClass_FWD_DEFINED__
#define __GoogleUpdate3WebSystemClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class GoogleUpdate3WebSystemClass GoogleUpdate3WebSystemClass;
#else
typedef struct GoogleUpdate3WebSystemClass GoogleUpdate3WebSystemClass;
#endif /* __cplusplus */

#endif 	/* __GoogleUpdate3WebSystemClass_FWD_DEFINED__ */


#ifndef __GoogleUpdate3WebServiceClass_FWD_DEFINED__
#define __GoogleUpdate3WebServiceClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class GoogleUpdate3WebServiceClass GoogleUpdate3WebServiceClass;
#else
typedef struct GoogleUpdate3WebServiceClass GoogleUpdate3WebServiceClass;
#endif /* __cplusplus */

#endif 	/* __GoogleUpdate3WebServiceClass_FWD_DEFINED__ */


#ifndef __PolicyStatusUserClass_FWD_DEFINED__
#define __PolicyStatusUserClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class PolicyStatusUserClass PolicyStatusUserClass;
#else
typedef struct PolicyStatusUserClass PolicyStatusUserClass;
#endif /* __cplusplus */

#endif 	/* __PolicyStatusUserClass_FWD_DEFINED__ */


#ifndef __PolicyStatusSystemClass_FWD_DEFINED__
#define __PolicyStatusSystemClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class PolicyStatusSystemClass PolicyStatusSystemClass;
#else
typedef struct PolicyStatusSystemClass PolicyStatusSystemClass;
#endif /* __cplusplus */

#endif 	/* __PolicyStatusSystemClass_FWD_DEFINED__ */


#ifndef __ProcessLauncherClass_FWD_DEFINED__
#define __ProcessLauncherClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class ProcessLauncherClass ProcessLauncherClass;
#else
typedef struct ProcessLauncherClass ProcessLauncherClass;
#endif /* __cplusplus */

#endif 	/* __ProcessLauncherClass_FWD_DEFINED__ */


#ifndef __IAppVersionWeb_FWD_DEFINED__
#define __IAppVersionWeb_FWD_DEFINED__
typedef interface IAppVersionWeb IAppVersionWeb;

#endif 	/* __IAppVersionWeb_FWD_DEFINED__ */


#ifndef __IAppVersionWebUser_FWD_DEFINED__
#define __IAppVersionWebUser_FWD_DEFINED__
typedef interface IAppVersionWebUser IAppVersionWebUser;

#endif 	/* __IAppVersionWebUser_FWD_DEFINED__ */


#ifndef __ICurrentState_FWD_DEFINED__
#define __ICurrentState_FWD_DEFINED__
typedef interface ICurrentState ICurrentState;

#endif 	/* __ICurrentState_FWD_DEFINED__ */


#ifndef __ICurrentStateUser_FWD_DEFINED__
#define __ICurrentStateUser_FWD_DEFINED__
typedef interface ICurrentStateUser ICurrentStateUser;

#endif 	/* __ICurrentStateUser_FWD_DEFINED__ */


#ifndef __IGoogleUpdate3Web_FWD_DEFINED__
#define __IGoogleUpdate3Web_FWD_DEFINED__
typedef interface IGoogleUpdate3Web IGoogleUpdate3Web;

#endif 	/* __IGoogleUpdate3Web_FWD_DEFINED__ */


#ifndef __IGoogleUpdate3WebUser_FWD_DEFINED__
#define __IGoogleUpdate3WebUser_FWD_DEFINED__
typedef interface IGoogleUpdate3WebUser IGoogleUpdate3WebUser;

#endif 	/* __IGoogleUpdate3WebUser_FWD_DEFINED__ */


#ifndef __IAppBundleWeb_FWD_DEFINED__
#define __IAppBundleWeb_FWD_DEFINED__
typedef interface IAppBundleWeb IAppBundleWeb;

#endif 	/* __IAppBundleWeb_FWD_DEFINED__ */


#ifndef __IAppBundleWebUser_FWD_DEFINED__
#define __IAppBundleWebUser_FWD_DEFINED__
typedef interface IAppBundleWebUser IAppBundleWebUser;

#endif 	/* __IAppBundleWebUser_FWD_DEFINED__ */


#ifndef __IAppWeb_FWD_DEFINED__
#define __IAppWeb_FWD_DEFINED__
typedef interface IAppWeb IAppWeb;

#endif 	/* __IAppWeb_FWD_DEFINED__ */


#ifndef __IAppWebUser_FWD_DEFINED__
#define __IAppWebUser_FWD_DEFINED__
typedef interface IAppWebUser IAppWebUser;

#endif 	/* __IAppWebUser_FWD_DEFINED__ */


#ifndef __IAppCommandWeb_FWD_DEFINED__
#define __IAppCommandWeb_FWD_DEFINED__
typedef interface IAppCommandWeb IAppCommandWeb;

#endif 	/* __IAppCommandWeb_FWD_DEFINED__ */


#ifndef __IAppCommandWebUser_FWD_DEFINED__
#define __IAppCommandWebUser_FWD_DEFINED__
typedef interface IAppCommandWebUser IAppCommandWebUser;

#endif 	/* __IAppCommandWebUser_FWD_DEFINED__ */


#ifndef __IPolicyStatus_FWD_DEFINED__
#define __IPolicyStatus_FWD_DEFINED__
typedef interface IPolicyStatus IPolicyStatus;

#endif 	/* __IPolicyStatus_FWD_DEFINED__ */


#ifndef __IPolicyStatusUser_FWD_DEFINED__
#define __IPolicyStatusUser_FWD_DEFINED__
typedef interface IPolicyStatusUser IPolicyStatusUser;

#endif 	/* __IPolicyStatusUser_FWD_DEFINED__ */


#ifndef __IPolicyStatusValue_FWD_DEFINED__
#define __IPolicyStatusValue_FWD_DEFINED__
typedef interface IPolicyStatusValue IPolicyStatusValue;

#endif 	/* __IPolicyStatusValue_FWD_DEFINED__ */


#ifndef __IPolicyStatusValueUser_FWD_DEFINED__
#define __IPolicyStatusValueUser_FWD_DEFINED__
typedef interface IPolicyStatusValueUser IPolicyStatusValueUser;

#endif 	/* __IPolicyStatusValueUser_FWD_DEFINED__ */


#ifndef __IPolicyStatus2_FWD_DEFINED__
#define __IPolicyStatus2_FWD_DEFINED__
typedef interface IPolicyStatus2 IPolicyStatus2;

#endif 	/* __IPolicyStatus2_FWD_DEFINED__ */


#ifndef __IPolicyStatus2User_FWD_DEFINED__
#define __IPolicyStatus2User_FWD_DEFINED__
typedef interface IPolicyStatus2User IPolicyStatus2User;

#endif 	/* __IPolicyStatus2User_FWD_DEFINED__ */


#ifndef __IPolicyStatus3_FWD_DEFINED__
#define __IPolicyStatus3_FWD_DEFINED__
typedef interface IPolicyStatus3 IPolicyStatus3;

#endif 	/* __IPolicyStatus3_FWD_DEFINED__ */


#ifndef __IPolicyStatus3User_FWD_DEFINED__
#define __IPolicyStatus3User_FWD_DEFINED__
typedef interface IPolicyStatus3User IPolicyStatus3User;

#endif 	/* __IPolicyStatus3User_FWD_DEFINED__ */


#ifndef __IPolicyStatus4_FWD_DEFINED__
#define __IPolicyStatus4_FWD_DEFINED__
typedef interface IPolicyStatus4 IPolicyStatus4;

#endif 	/* __IPolicyStatus4_FWD_DEFINED__ */


#ifndef __IPolicyStatus4User_FWD_DEFINED__
#define __IPolicyStatus4User_FWD_DEFINED__
typedef interface IPolicyStatus4User IPolicyStatus4User;

#endif 	/* __IPolicyStatus4User_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_updater_legacy_idl_user_0000_0000 */
/* [local] */ 

typedef 
enum CurrentState
    {
        STATE_INIT	= 1,
        STATE_WAITING_TO_CHECK_FOR_UPDATE	= 2,
        STATE_CHECKING_FOR_UPDATE	= 3,
        STATE_UPDATE_AVAILABLE	= 4,
        STATE_WAITING_TO_DOWNLOAD	= 5,
        STATE_RETRYING_DOWNLOAD	= 6,
        STATE_DOWNLOADING	= 7,
        STATE_DOWNLOAD_COMPLETE	= 8,
        STATE_EXTRACTING	= 9,
        STATE_APPLYING_DIFFERENTIAL_PATCH	= 10,
        STATE_READY_TO_INSTALL	= 11,
        STATE_WAITING_TO_INSTALL	= 12,
        STATE_INSTALLING	= 13,
        STATE_INSTALL_COMPLETE	= 14,
        STATE_PAUSED	= 15,
        STATE_NO_UPDATE	= 16,
        STATE_ERROR	= 17
    } 	CurrentState;


enum AppCommandStatus
    {
        COMMAND_STATUS_INIT	= 1,
        COMMAND_STATUS_RUNNING	= 2,
        COMMAND_STATUS_ERROR	= 3,
        COMMAND_STATUS_COMPLETE	= 4
    } ;


extern RPC_IF_HANDLE __MIDL_itf_updater_legacy_idl_user_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_updater_legacy_idl_user_0000_0000_v0_0_s_ifspec;

#ifndef __IAppVersionWeb_INTERFACE_DEFINED__
#define __IAppVersionWeb_INTERFACE_DEFINED__

/* interface IAppVersionWeb */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IAppVersionWeb;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AA10D17D-7A09-48AC-B1E4-F124937E3D26")
    IAppVersionWeb : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_version( 
            /* [retval][out] */ BSTR *__MIDL__IAppVersionWeb0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCount( 
            /* [retval][out] */ long *count) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageWeb( 
            /* [in] */ long index,
            /* [retval][out] */ IDispatch **package) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppVersionWebVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppVersionWeb * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppVersionWeb * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppVersionWeb * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppVersionWeb * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppVersionWeb * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppVersionWeb * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppVersionWeb * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAppVersionWeb, get_version)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_version )( 
            IAppVersionWeb * This,
            /* [retval][out] */ BSTR *__MIDL__IAppVersionWeb0000);
        
        DECLSPEC_XFGVIRT(IAppVersionWeb, get_packageCount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCount )( 
            IAppVersionWeb * This,
            /* [retval][out] */ long *count);
        
        DECLSPEC_XFGVIRT(IAppVersionWeb, get_packageWeb)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageWeb )( 
            IAppVersionWeb * This,
            /* [in] */ long index,
            /* [retval][out] */ IDispatch **package);
        
        END_INTERFACE
    } IAppVersionWebVtbl;

    interface IAppVersionWeb
    {
        CONST_VTBL struct IAppVersionWebVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppVersionWeb_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppVersionWeb_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppVersionWeb_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppVersionWeb_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppVersionWeb_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppVersionWeb_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppVersionWeb_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppVersionWeb_get_version(This,__MIDL__IAppVersionWeb0000)	\
    ( (This)->lpVtbl -> get_version(This,__MIDL__IAppVersionWeb0000) ) 

#define IAppVersionWeb_get_packageCount(This,count)	\
    ( (This)->lpVtbl -> get_packageCount(This,count) ) 

#define IAppVersionWeb_get_packageWeb(This,index,package)	\
    ( (This)->lpVtbl -> get_packageWeb(This,index,package) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppVersionWeb_INTERFACE_DEFINED__ */


#ifndef __IAppVersionWebUser_INTERFACE_DEFINED__
#define __IAppVersionWebUser_INTERFACE_DEFINED__

/* interface IAppVersionWebUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IAppVersionWebUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AC817E10-993C-470F-8DCA-25F53D70EA8D")
    IAppVersionWebUser : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_version( 
            /* [retval][out] */ BSTR *__MIDL__IAppVersionWebUser0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCount( 
            /* [retval][out] */ long *count) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageWeb( 
            /* [in] */ long index,
            /* [retval][out] */ IDispatch **package) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppVersionWebUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppVersionWebUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppVersionWebUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppVersionWebUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppVersionWebUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppVersionWebUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppVersionWebUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppVersionWebUser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAppVersionWebUser, get_version)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_version )( 
            IAppVersionWebUser * This,
            /* [retval][out] */ BSTR *__MIDL__IAppVersionWebUser0000);
        
        DECLSPEC_XFGVIRT(IAppVersionWebUser, get_packageCount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCount )( 
            IAppVersionWebUser * This,
            /* [retval][out] */ long *count);
        
        DECLSPEC_XFGVIRT(IAppVersionWebUser, get_packageWeb)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageWeb )( 
            IAppVersionWebUser * This,
            /* [in] */ long index,
            /* [retval][out] */ IDispatch **package);
        
        END_INTERFACE
    } IAppVersionWebUserVtbl;

    interface IAppVersionWebUser
    {
        CONST_VTBL struct IAppVersionWebUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppVersionWebUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppVersionWebUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppVersionWebUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppVersionWebUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppVersionWebUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppVersionWebUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppVersionWebUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppVersionWebUser_get_version(This,__MIDL__IAppVersionWebUser0000)	\
    ( (This)->lpVtbl -> get_version(This,__MIDL__IAppVersionWebUser0000) ) 

#define IAppVersionWebUser_get_packageCount(This,count)	\
    ( (This)->lpVtbl -> get_packageCount(This,count) ) 

#define IAppVersionWebUser_get_packageWeb(This,index,package)	\
    ( (This)->lpVtbl -> get_packageWeb(This,index,package) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppVersionWebUser_INTERFACE_DEFINED__ */


#ifndef __ICurrentState_INTERFACE_DEFINED__
#define __ICurrentState_INTERFACE_DEFINED__

/* interface ICurrentState */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_ICurrentState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A643508B-B1E3-4457-9769-32C953BD1D57")
    ICurrentState : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_stateValue( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_availableVersion( 
            /* [retval][out] */ BSTR *__MIDL__ICurrentState0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_bytesDownloaded( 
            /* [retval][out] */ ULONG *__MIDL__ICurrentState0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_totalBytesToDownload( 
            /* [retval][out] */ ULONG *__MIDL__ICurrentState0003) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadTimeRemainingMs( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0004) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nextRetryTime( 
            /* [retval][out] */ ULONGLONG *__MIDL__ICurrentState0005) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installProgress( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0006) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installTimeRemainingMs( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0007) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isCanceled( 
            /* [retval][out] */ VARIANT_BOOL *is_canceled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_errorCode( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0008) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_extraCode1( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0009) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_completionMessage( 
            /* [retval][out] */ BSTR *__MIDL__ICurrentState0010) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerResultCode( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0011) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerResultExtraCode1( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0012) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_postInstallLaunchCommandLine( 
            /* [retval][out] */ BSTR *__MIDL__ICurrentState0013) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_postInstallUrl( 
            /* [retval][out] */ BSTR *__MIDL__ICurrentState0014) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_postInstallAction( 
            /* [retval][out] */ LONG *__MIDL__ICurrentState0015) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICurrentStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICurrentState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICurrentState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICurrentState * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ICurrentState * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ICurrentState * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ICurrentState * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICurrentState * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_stateValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_stateValue )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0000);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_availableVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_availableVersion )( 
            ICurrentState * This,
            /* [retval][out] */ BSTR *__MIDL__ICurrentState0001);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_bytesDownloaded)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_bytesDownloaded )( 
            ICurrentState * This,
            /* [retval][out] */ ULONG *__MIDL__ICurrentState0002);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_totalBytesToDownload)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_totalBytesToDownload )( 
            ICurrentState * This,
            /* [retval][out] */ ULONG *__MIDL__ICurrentState0003);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_downloadTimeRemainingMs)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadTimeRemainingMs )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0004);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_nextRetryTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextRetryTime )( 
            ICurrentState * This,
            /* [retval][out] */ ULONGLONG *__MIDL__ICurrentState0005);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_installProgress)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installProgress )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0006);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_installTimeRemainingMs)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installTimeRemainingMs )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0007);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_isCanceled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isCanceled )( 
            ICurrentState * This,
            /* [retval][out] */ VARIANT_BOOL *is_canceled);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_errorCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCode )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0008);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_extraCode1)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extraCode1 )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0009);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_completionMessage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_completionMessage )( 
            ICurrentState * This,
            /* [retval][out] */ BSTR *__MIDL__ICurrentState0010);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_installerResultCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerResultCode )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0011);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_installerResultExtraCode1)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerResultExtraCode1 )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0012);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_postInstallLaunchCommandLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_postInstallLaunchCommandLine )( 
            ICurrentState * This,
            /* [retval][out] */ BSTR *__MIDL__ICurrentState0013);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_postInstallUrl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_postInstallUrl )( 
            ICurrentState * This,
            /* [retval][out] */ BSTR *__MIDL__ICurrentState0014);
        
        DECLSPEC_XFGVIRT(ICurrentState, get_postInstallAction)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_postInstallAction )( 
            ICurrentState * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentState0015);
        
        END_INTERFACE
    } ICurrentStateVtbl;

    interface ICurrentState
    {
        CONST_VTBL struct ICurrentStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICurrentState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICurrentState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICurrentState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICurrentState_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICurrentState_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICurrentState_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICurrentState_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICurrentState_get_stateValue(This,__MIDL__ICurrentState0000)	\
    ( (This)->lpVtbl -> get_stateValue(This,__MIDL__ICurrentState0000) ) 

#define ICurrentState_get_availableVersion(This,__MIDL__ICurrentState0001)	\
    ( (This)->lpVtbl -> get_availableVersion(This,__MIDL__ICurrentState0001) ) 

#define ICurrentState_get_bytesDownloaded(This,__MIDL__ICurrentState0002)	\
    ( (This)->lpVtbl -> get_bytesDownloaded(This,__MIDL__ICurrentState0002) ) 

#define ICurrentState_get_totalBytesToDownload(This,__MIDL__ICurrentState0003)	\
    ( (This)->lpVtbl -> get_totalBytesToDownload(This,__MIDL__ICurrentState0003) ) 

#define ICurrentState_get_downloadTimeRemainingMs(This,__MIDL__ICurrentState0004)	\
    ( (This)->lpVtbl -> get_downloadTimeRemainingMs(This,__MIDL__ICurrentState0004) ) 

#define ICurrentState_get_nextRetryTime(This,__MIDL__ICurrentState0005)	\
    ( (This)->lpVtbl -> get_nextRetryTime(This,__MIDL__ICurrentState0005) ) 

#define ICurrentState_get_installProgress(This,__MIDL__ICurrentState0006)	\
    ( (This)->lpVtbl -> get_installProgress(This,__MIDL__ICurrentState0006) ) 

#define ICurrentState_get_installTimeRemainingMs(This,__MIDL__ICurrentState0007)	\
    ( (This)->lpVtbl -> get_installTimeRemainingMs(This,__MIDL__ICurrentState0007) ) 

#define ICurrentState_get_isCanceled(This,is_canceled)	\
    ( (This)->lpVtbl -> get_isCanceled(This,is_canceled) ) 

#define ICurrentState_get_errorCode(This,__MIDL__ICurrentState0008)	\
    ( (This)->lpVtbl -> get_errorCode(This,__MIDL__ICurrentState0008) ) 

#define ICurrentState_get_extraCode1(This,__MIDL__ICurrentState0009)	\
    ( (This)->lpVtbl -> get_extraCode1(This,__MIDL__ICurrentState0009) ) 

#define ICurrentState_get_completionMessage(This,__MIDL__ICurrentState0010)	\
    ( (This)->lpVtbl -> get_completionMessage(This,__MIDL__ICurrentState0010) ) 

#define ICurrentState_get_installerResultCode(This,__MIDL__ICurrentState0011)	\
    ( (This)->lpVtbl -> get_installerResultCode(This,__MIDL__ICurrentState0011) ) 

#define ICurrentState_get_installerResultExtraCode1(This,__MIDL__ICurrentState0012)	\
    ( (This)->lpVtbl -> get_installerResultExtraCode1(This,__MIDL__ICurrentState0012) ) 

#define ICurrentState_get_postInstallLaunchCommandLine(This,__MIDL__ICurrentState0013)	\
    ( (This)->lpVtbl -> get_postInstallLaunchCommandLine(This,__MIDL__ICurrentState0013) ) 

#define ICurrentState_get_postInstallUrl(This,__MIDL__ICurrentState0014)	\
    ( (This)->lpVtbl -> get_postInstallUrl(This,__MIDL__ICurrentState0014) ) 

#define ICurrentState_get_postInstallAction(This,__MIDL__ICurrentState0015)	\
    ( (This)->lpVtbl -> get_postInstallAction(This,__MIDL__ICurrentState0015) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICurrentState_INTERFACE_DEFINED__ */


#ifndef __ICurrentStateUser_INTERFACE_DEFINED__
#define __ICurrentStateUser_INTERFACE_DEFINED__

/* interface ICurrentStateUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_ICurrentStateUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31479718-D170-467B-9274-27FC3E88CB76")
    ICurrentStateUser : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_stateValue( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_availableVersion( 
            /* [retval][out] */ BSTR *__MIDL__ICurrentStateUser0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_bytesDownloaded( 
            /* [retval][out] */ ULONG *__MIDL__ICurrentStateUser0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_totalBytesToDownload( 
            /* [retval][out] */ ULONG *__MIDL__ICurrentStateUser0003) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadTimeRemainingMs( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0004) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nextRetryTime( 
            /* [retval][out] */ ULONGLONG *__MIDL__ICurrentStateUser0005) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installProgress( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0006) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installTimeRemainingMs( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0007) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isCanceled( 
            /* [retval][out] */ VARIANT_BOOL *is_canceled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_errorCode( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0008) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_extraCode1( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0009) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_completionMessage( 
            /* [retval][out] */ BSTR *__MIDL__ICurrentStateUser0010) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerResultCode( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0011) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerResultExtraCode1( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0012) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_postInstallLaunchCommandLine( 
            /* [retval][out] */ BSTR *__MIDL__ICurrentStateUser0013) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_postInstallUrl( 
            /* [retval][out] */ BSTR *__MIDL__ICurrentStateUser0014) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_postInstallAction( 
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0015) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICurrentStateUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICurrentStateUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICurrentStateUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICurrentStateUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ICurrentStateUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ICurrentStateUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ICurrentStateUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICurrentStateUser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_stateValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_stateValue )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0000);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_availableVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_availableVersion )( 
            ICurrentStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__ICurrentStateUser0001);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_bytesDownloaded)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_bytesDownloaded )( 
            ICurrentStateUser * This,
            /* [retval][out] */ ULONG *__MIDL__ICurrentStateUser0002);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_totalBytesToDownload)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_totalBytesToDownload )( 
            ICurrentStateUser * This,
            /* [retval][out] */ ULONG *__MIDL__ICurrentStateUser0003);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_downloadTimeRemainingMs)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadTimeRemainingMs )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0004);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_nextRetryTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextRetryTime )( 
            ICurrentStateUser * This,
            /* [retval][out] */ ULONGLONG *__MIDL__ICurrentStateUser0005);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_installProgress)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installProgress )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0006);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_installTimeRemainingMs)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installTimeRemainingMs )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0007);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_isCanceled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isCanceled )( 
            ICurrentStateUser * This,
            /* [retval][out] */ VARIANT_BOOL *is_canceled);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_errorCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCode )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0008);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_extraCode1)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extraCode1 )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0009);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_completionMessage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_completionMessage )( 
            ICurrentStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__ICurrentStateUser0010);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_installerResultCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerResultCode )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0011);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_installerResultExtraCode1)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerResultExtraCode1 )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0012);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_postInstallLaunchCommandLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_postInstallLaunchCommandLine )( 
            ICurrentStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__ICurrentStateUser0013);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_postInstallUrl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_postInstallUrl )( 
            ICurrentStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__ICurrentStateUser0014);
        
        DECLSPEC_XFGVIRT(ICurrentStateUser, get_postInstallAction)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_postInstallAction )( 
            ICurrentStateUser * This,
            /* [retval][out] */ LONG *__MIDL__ICurrentStateUser0015);
        
        END_INTERFACE
    } ICurrentStateUserVtbl;

    interface ICurrentStateUser
    {
        CONST_VTBL struct ICurrentStateUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICurrentStateUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICurrentStateUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICurrentStateUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICurrentStateUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICurrentStateUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICurrentStateUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICurrentStateUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICurrentStateUser_get_stateValue(This,__MIDL__ICurrentStateUser0000)	\
    ( (This)->lpVtbl -> get_stateValue(This,__MIDL__ICurrentStateUser0000) ) 

#define ICurrentStateUser_get_availableVersion(This,__MIDL__ICurrentStateUser0001)	\
    ( (This)->lpVtbl -> get_availableVersion(This,__MIDL__ICurrentStateUser0001) ) 

#define ICurrentStateUser_get_bytesDownloaded(This,__MIDL__ICurrentStateUser0002)	\
    ( (This)->lpVtbl -> get_bytesDownloaded(This,__MIDL__ICurrentStateUser0002) ) 

#define ICurrentStateUser_get_totalBytesToDownload(This,__MIDL__ICurrentStateUser0003)	\
    ( (This)->lpVtbl -> get_totalBytesToDownload(This,__MIDL__ICurrentStateUser0003) ) 

#define ICurrentStateUser_get_downloadTimeRemainingMs(This,__MIDL__ICurrentStateUser0004)	\
    ( (This)->lpVtbl -> get_downloadTimeRemainingMs(This,__MIDL__ICurrentStateUser0004) ) 

#define ICurrentStateUser_get_nextRetryTime(This,__MIDL__ICurrentStateUser0005)	\
    ( (This)->lpVtbl -> get_nextRetryTime(This,__MIDL__ICurrentStateUser0005) ) 

#define ICurrentStateUser_get_installProgress(This,__MIDL__ICurrentStateUser0006)	\
    ( (This)->lpVtbl -> get_installProgress(This,__MIDL__ICurrentStateUser0006) ) 

#define ICurrentStateUser_get_installTimeRemainingMs(This,__MIDL__ICurrentStateUser0007)	\
    ( (This)->lpVtbl -> get_installTimeRemainingMs(This,__MIDL__ICurrentStateUser0007) ) 

#define ICurrentStateUser_get_isCanceled(This,is_canceled)	\
    ( (This)->lpVtbl -> get_isCanceled(This,is_canceled) ) 

#define ICurrentStateUser_get_errorCode(This,__MIDL__ICurrentStateUser0008)	\
    ( (This)->lpVtbl -> get_errorCode(This,__MIDL__ICurrentStateUser0008) ) 

#define ICurrentStateUser_get_extraCode1(This,__MIDL__ICurrentStateUser0009)	\
    ( (This)->lpVtbl -> get_extraCode1(This,__MIDL__ICurrentStateUser0009) ) 

#define ICurrentStateUser_get_completionMessage(This,__MIDL__ICurrentStateUser0010)	\
    ( (This)->lpVtbl -> get_completionMessage(This,__MIDL__ICurrentStateUser0010) ) 

#define ICurrentStateUser_get_installerResultCode(This,__MIDL__ICurrentStateUser0011)	\
    ( (This)->lpVtbl -> get_installerResultCode(This,__MIDL__ICurrentStateUser0011) ) 

#define ICurrentStateUser_get_installerResultExtraCode1(This,__MIDL__ICurrentStateUser0012)	\
    ( (This)->lpVtbl -> get_installerResultExtraCode1(This,__MIDL__ICurrentStateUser0012) ) 

#define ICurrentStateUser_get_postInstallLaunchCommandLine(This,__MIDL__ICurrentStateUser0013)	\
    ( (This)->lpVtbl -> get_postInstallLaunchCommandLine(This,__MIDL__ICurrentStateUser0013) ) 

#define ICurrentStateUser_get_postInstallUrl(This,__MIDL__ICurrentStateUser0014)	\
    ( (This)->lpVtbl -> get_postInstallUrl(This,__MIDL__ICurrentStateUser0014) ) 

#define ICurrentStateUser_get_postInstallAction(This,__MIDL__ICurrentStateUser0015)	\
    ( (This)->lpVtbl -> get_postInstallAction(This,__MIDL__ICurrentStateUser0015) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICurrentStateUser_INTERFACE_DEFINED__ */


#ifndef __IGoogleUpdate3Web_INTERFACE_DEFINED__
#define __IGoogleUpdate3Web_INTERFACE_DEFINED__

/* interface IGoogleUpdate3Web */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IGoogleUpdate3Web;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A35E1C5E-0A18-4FF1-8C4D-DD8ED07B0BD0")
    IGoogleUpdate3Web : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE createAppBundleWeb( 
            /* [retval][out] */ IDispatch **app_bundle_web) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGoogleUpdate3WebVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGoogleUpdate3Web * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGoogleUpdate3Web * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGoogleUpdate3Web * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IGoogleUpdate3Web * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IGoogleUpdate3Web * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IGoogleUpdate3Web * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGoogleUpdate3Web * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IGoogleUpdate3Web, createAppBundleWeb)
        HRESULT ( STDMETHODCALLTYPE *createAppBundleWeb )( 
            IGoogleUpdate3Web * This,
            /* [retval][out] */ IDispatch **app_bundle_web);
        
        END_INTERFACE
    } IGoogleUpdate3WebVtbl;

    interface IGoogleUpdate3Web
    {
        CONST_VTBL struct IGoogleUpdate3WebVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGoogleUpdate3Web_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGoogleUpdate3Web_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGoogleUpdate3Web_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGoogleUpdate3Web_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGoogleUpdate3Web_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGoogleUpdate3Web_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGoogleUpdate3Web_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGoogleUpdate3Web_createAppBundleWeb(This,app_bundle_web)	\
    ( (This)->lpVtbl -> createAppBundleWeb(This,app_bundle_web) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGoogleUpdate3Web_INTERFACE_DEFINED__ */


#ifndef __IGoogleUpdate3WebUser_INTERFACE_DEFINED__
#define __IGoogleUpdate3WebUser_INTERFACE_DEFINED__

/* interface IGoogleUpdate3WebUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IGoogleUpdate3WebUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE8EE731-C592-4A4F-9774-BB04337B8F46")
    IGoogleUpdate3WebUser : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE createAppBundleWeb( 
            /* [retval][out] */ IDispatch **app_bundle_web) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGoogleUpdate3WebUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGoogleUpdate3WebUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGoogleUpdate3WebUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGoogleUpdate3WebUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IGoogleUpdate3WebUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IGoogleUpdate3WebUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IGoogleUpdate3WebUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGoogleUpdate3WebUser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IGoogleUpdate3WebUser, createAppBundleWeb)
        HRESULT ( STDMETHODCALLTYPE *createAppBundleWeb )( 
            IGoogleUpdate3WebUser * This,
            /* [retval][out] */ IDispatch **app_bundle_web);
        
        END_INTERFACE
    } IGoogleUpdate3WebUserVtbl;

    interface IGoogleUpdate3WebUser
    {
        CONST_VTBL struct IGoogleUpdate3WebUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGoogleUpdate3WebUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGoogleUpdate3WebUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGoogleUpdate3WebUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGoogleUpdate3WebUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGoogleUpdate3WebUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGoogleUpdate3WebUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGoogleUpdate3WebUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGoogleUpdate3WebUser_createAppBundleWeb(This,app_bundle_web)	\
    ( (This)->lpVtbl -> createAppBundleWeb(This,app_bundle_web) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGoogleUpdate3WebUser_INTERFACE_DEFINED__ */


#ifndef __IAppBundleWeb_INTERFACE_DEFINED__
#define __IAppBundleWeb_INTERFACE_DEFINED__

/* interface IAppBundleWeb */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IAppBundleWeb;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0569DBB9-BAA0-48D5-8543-0F3BE30A1648")
    IAppBundleWeb : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE createApp( 
            /* [in] */ BSTR app_guid,
            /* [in] */ BSTR brand_code,
            /* [in] */ BSTR language,
            /* [in] */ BSTR ap) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE createInstalledApp( 
            /* [in] */ BSTR app_id) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE createAllInstalledApps( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_displayLanguage( 
            /* [retval][out] */ BSTR *__MIDL__IAppBundleWeb0000) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_displayLanguage( 
            /* [in] */ BSTR __MIDL__IAppBundleWeb0001) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_parentHWND( 
            /* [in] */ ULONG_PTR hwnd) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ int *index) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_appWeb( 
            /* [in] */ int index,
            /* [retval][out] */ IDispatch **app_web) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE initialize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE checkForUpdate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE download( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE install( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resume( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE downloadPackage( 
            /* [in] */ BSTR app_id,
            /* [in] */ BSTR package_name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_currentState( 
            /* [retval][out] */ VARIANT *current_state) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppBundleWebVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppBundleWeb * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppBundleWeb * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppBundleWeb * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppBundleWeb * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppBundleWeb * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, createApp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *createApp )( 
            IAppBundleWeb * This,
            /* [in] */ BSTR app_guid,
            /* [in] */ BSTR brand_code,
            /* [in] */ BSTR language,
            /* [in] */ BSTR ap);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, createInstalledApp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *createInstalledApp )( 
            IAppBundleWeb * This,
            /* [in] */ BSTR app_id);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, createAllInstalledApps)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *createAllInstalledApps )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, get_displayLanguage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_displayLanguage )( 
            IAppBundleWeb * This,
            /* [retval][out] */ BSTR *__MIDL__IAppBundleWeb0000);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, put_displayLanguage)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_displayLanguage )( 
            IAppBundleWeb * This,
            /* [in] */ BSTR __MIDL__IAppBundleWeb0001);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, put_parentHWND)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_parentHWND )( 
            IAppBundleWeb * This,
            /* [in] */ ULONG_PTR hwnd);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, get_length)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IAppBundleWeb * This,
            /* [retval][out] */ int *index);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, get_appWeb)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_appWeb )( 
            IAppBundleWeb * This,
            /* [in] */ int index,
            /* [retval][out] */ IDispatch **app_web);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, initialize)
        HRESULT ( STDMETHODCALLTYPE *initialize )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, checkForUpdate)
        HRESULT ( STDMETHODCALLTYPE *checkForUpdate )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, download)
        HRESULT ( STDMETHODCALLTYPE *download )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, install)
        HRESULT ( STDMETHODCALLTYPE *install )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, pause)
        HRESULT ( STDMETHODCALLTYPE *pause )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, resume)
        HRESULT ( STDMETHODCALLTYPE *resume )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, cancel)
        HRESULT ( STDMETHODCALLTYPE *cancel )( 
            IAppBundleWeb * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, downloadPackage)
        HRESULT ( STDMETHODCALLTYPE *downloadPackage )( 
            IAppBundleWeb * This,
            /* [in] */ BSTR app_id,
            /* [in] */ BSTR package_name);
        
        DECLSPEC_XFGVIRT(IAppBundleWeb, get_currentState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_currentState )( 
            IAppBundleWeb * This,
            /* [retval][out] */ VARIANT *current_state);
        
        END_INTERFACE
    } IAppBundleWebVtbl;

    interface IAppBundleWeb
    {
        CONST_VTBL struct IAppBundleWebVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppBundleWeb_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppBundleWeb_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppBundleWeb_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppBundleWeb_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppBundleWeb_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppBundleWeb_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppBundleWeb_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppBundleWeb_createApp(This,app_guid,brand_code,language,ap)	\
    ( (This)->lpVtbl -> createApp(This,app_guid,brand_code,language,ap) ) 

#define IAppBundleWeb_createInstalledApp(This,app_id)	\
    ( (This)->lpVtbl -> createInstalledApp(This,app_id) ) 

#define IAppBundleWeb_createAllInstalledApps(This)	\
    ( (This)->lpVtbl -> createAllInstalledApps(This) ) 

#define IAppBundleWeb_get_displayLanguage(This,__MIDL__IAppBundleWeb0000)	\
    ( (This)->lpVtbl -> get_displayLanguage(This,__MIDL__IAppBundleWeb0000) ) 

#define IAppBundleWeb_put_displayLanguage(This,__MIDL__IAppBundleWeb0001)	\
    ( (This)->lpVtbl -> put_displayLanguage(This,__MIDL__IAppBundleWeb0001) ) 

#define IAppBundleWeb_put_parentHWND(This,hwnd)	\
    ( (This)->lpVtbl -> put_parentHWND(This,hwnd) ) 

#define IAppBundleWeb_get_length(This,index)	\
    ( (This)->lpVtbl -> get_length(This,index) ) 

#define IAppBundleWeb_get_appWeb(This,index,app_web)	\
    ( (This)->lpVtbl -> get_appWeb(This,index,app_web) ) 

#define IAppBundleWeb_initialize(This)	\
    ( (This)->lpVtbl -> initialize(This) ) 

#define IAppBundleWeb_checkForUpdate(This)	\
    ( (This)->lpVtbl -> checkForUpdate(This) ) 

#define IAppBundleWeb_download(This)	\
    ( (This)->lpVtbl -> download(This) ) 

#define IAppBundleWeb_install(This)	\
    ( (This)->lpVtbl -> install(This) ) 

#define IAppBundleWeb_pause(This)	\
    ( (This)->lpVtbl -> pause(This) ) 

#define IAppBundleWeb_resume(This)	\
    ( (This)->lpVtbl -> resume(This) ) 

#define IAppBundleWeb_cancel(This)	\
    ( (This)->lpVtbl -> cancel(This) ) 

#define IAppBundleWeb_downloadPackage(This,app_id,package_name)	\
    ( (This)->lpVtbl -> downloadPackage(This,app_id,package_name) ) 

#define IAppBundleWeb_get_currentState(This,current_state)	\
    ( (This)->lpVtbl -> get_currentState(This,current_state) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppBundleWeb_INTERFACE_DEFINED__ */


#ifndef __IAppBundleWebUser_INTERFACE_DEFINED__
#define __IAppBundleWebUser_INTERFACE_DEFINED__

/* interface IAppBundleWebUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IAppBundleWebUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CE7A37FD-A255-460C-BAF1-708765EB76EC")
    IAppBundleWebUser : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE createApp( 
            /* [in] */ BSTR app_guid,
            /* [in] */ BSTR brand_code,
            /* [in] */ BSTR language,
            /* [in] */ BSTR ap) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE createInstalledApp( 
            /* [in] */ BSTR app_id) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE createAllInstalledApps( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_displayLanguage( 
            /* [retval][out] */ BSTR *__MIDL__IAppBundleWebUser0000) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_displayLanguage( 
            /* [in] */ BSTR __MIDL__IAppBundleWebUser0001) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_parentHWND( 
            /* [in] */ ULONG_PTR hwnd) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ int *index) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_appWeb( 
            /* [in] */ int index,
            /* [retval][out] */ IDispatch **app_web) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE initialize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE checkForUpdate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE download( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE install( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resume( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE downloadPackage( 
            /* [in] */ BSTR app_id,
            /* [in] */ BSTR package_name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_currentState( 
            /* [retval][out] */ VARIANT *current_state) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppBundleWebUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppBundleWebUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppBundleWebUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppBundleWebUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppBundleWebUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppBundleWebUser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, createApp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *createApp )( 
            IAppBundleWebUser * This,
            /* [in] */ BSTR app_guid,
            /* [in] */ BSTR brand_code,
            /* [in] */ BSTR language,
            /* [in] */ BSTR ap);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, createInstalledApp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *createInstalledApp )( 
            IAppBundleWebUser * This,
            /* [in] */ BSTR app_id);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, createAllInstalledApps)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *createAllInstalledApps )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, get_displayLanguage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_displayLanguage )( 
            IAppBundleWebUser * This,
            /* [retval][out] */ BSTR *__MIDL__IAppBundleWebUser0000);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, put_displayLanguage)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_displayLanguage )( 
            IAppBundleWebUser * This,
            /* [in] */ BSTR __MIDL__IAppBundleWebUser0001);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, put_parentHWND)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_parentHWND )( 
            IAppBundleWebUser * This,
            /* [in] */ ULONG_PTR hwnd);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, get_length)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IAppBundleWebUser * This,
            /* [retval][out] */ int *index);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, get_appWeb)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_appWeb )( 
            IAppBundleWebUser * This,
            /* [in] */ int index,
            /* [retval][out] */ IDispatch **app_web);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, initialize)
        HRESULT ( STDMETHODCALLTYPE *initialize )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, checkForUpdate)
        HRESULT ( STDMETHODCALLTYPE *checkForUpdate )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, download)
        HRESULT ( STDMETHODCALLTYPE *download )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, install)
        HRESULT ( STDMETHODCALLTYPE *install )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, pause)
        HRESULT ( STDMETHODCALLTYPE *pause )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, resume)
        HRESULT ( STDMETHODCALLTYPE *resume )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, cancel)
        HRESULT ( STDMETHODCALLTYPE *cancel )( 
            IAppBundleWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, downloadPackage)
        HRESULT ( STDMETHODCALLTYPE *downloadPackage )( 
            IAppBundleWebUser * This,
            /* [in] */ BSTR app_id,
            /* [in] */ BSTR package_name);
        
        DECLSPEC_XFGVIRT(IAppBundleWebUser, get_currentState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_currentState )( 
            IAppBundleWebUser * This,
            /* [retval][out] */ VARIANT *current_state);
        
        END_INTERFACE
    } IAppBundleWebUserVtbl;

    interface IAppBundleWebUser
    {
        CONST_VTBL struct IAppBundleWebUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppBundleWebUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppBundleWebUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppBundleWebUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppBundleWebUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppBundleWebUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppBundleWebUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppBundleWebUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppBundleWebUser_createApp(This,app_guid,brand_code,language,ap)	\
    ( (This)->lpVtbl -> createApp(This,app_guid,brand_code,language,ap) ) 

#define IAppBundleWebUser_createInstalledApp(This,app_id)	\
    ( (This)->lpVtbl -> createInstalledApp(This,app_id) ) 

#define IAppBundleWebUser_createAllInstalledApps(This)	\
    ( (This)->lpVtbl -> createAllInstalledApps(This) ) 

#define IAppBundleWebUser_get_displayLanguage(This,__MIDL__IAppBundleWebUser0000)	\
    ( (This)->lpVtbl -> get_displayLanguage(This,__MIDL__IAppBundleWebUser0000) ) 

#define IAppBundleWebUser_put_displayLanguage(This,__MIDL__IAppBundleWebUser0001)	\
    ( (This)->lpVtbl -> put_displayLanguage(This,__MIDL__IAppBundleWebUser0001) ) 

#define IAppBundleWebUser_put_parentHWND(This,hwnd)	\
    ( (This)->lpVtbl -> put_parentHWND(This,hwnd) ) 

#define IAppBundleWebUser_get_length(This,index)	\
    ( (This)->lpVtbl -> get_length(This,index) ) 

#define IAppBundleWebUser_get_appWeb(This,index,app_web)	\
    ( (This)->lpVtbl -> get_appWeb(This,index,app_web) ) 

#define IAppBundleWebUser_initialize(This)	\
    ( (This)->lpVtbl -> initialize(This) ) 

#define IAppBundleWebUser_checkForUpdate(This)	\
    ( (This)->lpVtbl -> checkForUpdate(This) ) 

#define IAppBundleWebUser_download(This)	\
    ( (This)->lpVtbl -> download(This) ) 

#define IAppBundleWebUser_install(This)	\
    ( (This)->lpVtbl -> install(This) ) 

#define IAppBundleWebUser_pause(This)	\
    ( (This)->lpVtbl -> pause(This) ) 

#define IAppBundleWebUser_resume(This)	\
    ( (This)->lpVtbl -> resume(This) ) 

#define IAppBundleWebUser_cancel(This)	\
    ( (This)->lpVtbl -> cancel(This) ) 

#define IAppBundleWebUser_downloadPackage(This,app_id,package_name)	\
    ( (This)->lpVtbl -> downloadPackage(This,app_id,package_name) ) 

#define IAppBundleWebUser_get_currentState(This,current_state)	\
    ( (This)->lpVtbl -> get_currentState(This,current_state) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppBundleWebUser_INTERFACE_DEFINED__ */


#ifndef __IAppWeb_INTERFACE_DEFINED__
#define __IAppWeb_INTERFACE_DEFINED__

/* interface IAppWeb */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IAppWeb;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63D941DE-F67B-4E15-8A90-27881DA9EF4A")
    IAppWeb : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appId( 
            /* [retval][out] */ BSTR *__MIDL__IAppWeb0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_currentVersionWeb( 
            /* [retval][out] */ IDispatch **current) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nextVersionWeb( 
            /* [retval][out] */ IDispatch **next) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_command( 
            /* [in] */ BSTR command_id,
            /* [retval][out] */ IDispatch **command) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE cancel( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_currentState( 
            /* [retval][out] */ IDispatch **current_state) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE launch( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE uninstall( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_serverInstallDataIndex( 
            /* [retval][out] */ BSTR *__MIDL__IAppWeb0001) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_serverInstallDataIndex( 
            /* [in] */ BSTR __MIDL__IAppWeb0002) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppWebVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppWeb * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppWeb * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppWeb * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppWeb * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppWeb * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppWeb * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppWeb * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAppWeb, get_appId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appId )( 
            IAppWeb * This,
            /* [retval][out] */ BSTR *__MIDL__IAppWeb0000);
        
        DECLSPEC_XFGVIRT(IAppWeb, get_currentVersionWeb)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_currentVersionWeb )( 
            IAppWeb * This,
            /* [retval][out] */ IDispatch **current);
        
        DECLSPEC_XFGVIRT(IAppWeb, get_nextVersionWeb)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextVersionWeb )( 
            IAppWeb * This,
            /* [retval][out] */ IDispatch **next);
        
        DECLSPEC_XFGVIRT(IAppWeb, get_command)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_command )( 
            IAppWeb * This,
            /* [in] */ BSTR command_id,
            /* [retval][out] */ IDispatch **command);
        
        DECLSPEC_XFGVIRT(IAppWeb, cancel)
        HRESULT ( STDMETHODCALLTYPE *cancel )( 
            IAppWeb * This);
        
        DECLSPEC_XFGVIRT(IAppWeb, get_currentState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_currentState )( 
            IAppWeb * This,
            /* [retval][out] */ IDispatch **current_state);
        
        DECLSPEC_XFGVIRT(IAppWeb, launch)
        HRESULT ( STDMETHODCALLTYPE *launch )( 
            IAppWeb * This);
        
        DECLSPEC_XFGVIRT(IAppWeb, uninstall)
        HRESULT ( STDMETHODCALLTYPE *uninstall )( 
            IAppWeb * This);
        
        DECLSPEC_XFGVIRT(IAppWeb, get_serverInstallDataIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_serverInstallDataIndex )( 
            IAppWeb * This,
            /* [retval][out] */ BSTR *__MIDL__IAppWeb0001);
        
        DECLSPEC_XFGVIRT(IAppWeb, put_serverInstallDataIndex)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_serverInstallDataIndex )( 
            IAppWeb * This,
            /* [in] */ BSTR __MIDL__IAppWeb0002);
        
        END_INTERFACE
    } IAppWebVtbl;

    interface IAppWeb
    {
        CONST_VTBL struct IAppWebVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppWeb_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppWeb_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppWeb_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppWeb_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppWeb_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppWeb_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppWeb_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppWeb_get_appId(This,__MIDL__IAppWeb0000)	\
    ( (This)->lpVtbl -> get_appId(This,__MIDL__IAppWeb0000) ) 

#define IAppWeb_get_currentVersionWeb(This,current)	\
    ( (This)->lpVtbl -> get_currentVersionWeb(This,current) ) 

#define IAppWeb_get_nextVersionWeb(This,next)	\
    ( (This)->lpVtbl -> get_nextVersionWeb(This,next) ) 

#define IAppWeb_get_command(This,command_id,command)	\
    ( (This)->lpVtbl -> get_command(This,command_id,command) ) 

#define IAppWeb_cancel(This)	\
    ( (This)->lpVtbl -> cancel(This) ) 

#define IAppWeb_get_currentState(This,current_state)	\
    ( (This)->lpVtbl -> get_currentState(This,current_state) ) 

#define IAppWeb_launch(This)	\
    ( (This)->lpVtbl -> launch(This) ) 

#define IAppWeb_uninstall(This)	\
    ( (This)->lpVtbl -> uninstall(This) ) 

#define IAppWeb_get_serverInstallDataIndex(This,__MIDL__IAppWeb0001)	\
    ( (This)->lpVtbl -> get_serverInstallDataIndex(This,__MIDL__IAppWeb0001) ) 

#define IAppWeb_put_serverInstallDataIndex(This,__MIDL__IAppWeb0002)	\
    ( (This)->lpVtbl -> put_serverInstallDataIndex(This,__MIDL__IAppWeb0002) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppWeb_INTERFACE_DEFINED__ */


#ifndef __IAppWebUser_INTERFACE_DEFINED__
#define __IAppWebUser_INTERFACE_DEFINED__

/* interface IAppWebUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IAppWebUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("47B9D508-CB72-4F8B-AF00-7D0143603B25")
    IAppWebUser : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appId( 
            /* [retval][out] */ BSTR *__MIDL__IAppWebUser0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_currentVersionWeb( 
            /* [retval][out] */ IDispatch **current) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nextVersionWeb( 
            /* [retval][out] */ IDispatch **next) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_command( 
            /* [in] */ BSTR command_id,
            /* [retval][out] */ IDispatch **command) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE cancel( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_currentState( 
            /* [retval][out] */ IDispatch **current_state) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE launch( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE uninstall( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_serverInstallDataIndex( 
            /* [retval][out] */ BSTR *__MIDL__IAppWebUser0001) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_serverInstallDataIndex( 
            /* [in] */ BSTR __MIDL__IAppWebUser0002) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppWebUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppWebUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppWebUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppWebUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppWebUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppWebUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppWebUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppWebUser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAppWebUser, get_appId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appId )( 
            IAppWebUser * This,
            /* [retval][out] */ BSTR *__MIDL__IAppWebUser0000);
        
        DECLSPEC_XFGVIRT(IAppWebUser, get_currentVersionWeb)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_currentVersionWeb )( 
            IAppWebUser * This,
            /* [retval][out] */ IDispatch **current);
        
        DECLSPEC_XFGVIRT(IAppWebUser, get_nextVersionWeb)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextVersionWeb )( 
            IAppWebUser * This,
            /* [retval][out] */ IDispatch **next);
        
        DECLSPEC_XFGVIRT(IAppWebUser, get_command)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_command )( 
            IAppWebUser * This,
            /* [in] */ BSTR command_id,
            /* [retval][out] */ IDispatch **command);
        
        DECLSPEC_XFGVIRT(IAppWebUser, cancel)
        HRESULT ( STDMETHODCALLTYPE *cancel )( 
            IAppWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppWebUser, get_currentState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_currentState )( 
            IAppWebUser * This,
            /* [retval][out] */ IDispatch **current_state);
        
        DECLSPEC_XFGVIRT(IAppWebUser, launch)
        HRESULT ( STDMETHODCALLTYPE *launch )( 
            IAppWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppWebUser, uninstall)
        HRESULT ( STDMETHODCALLTYPE *uninstall )( 
            IAppWebUser * This);
        
        DECLSPEC_XFGVIRT(IAppWebUser, get_serverInstallDataIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_serverInstallDataIndex )( 
            IAppWebUser * This,
            /* [retval][out] */ BSTR *__MIDL__IAppWebUser0001);
        
        DECLSPEC_XFGVIRT(IAppWebUser, put_serverInstallDataIndex)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_serverInstallDataIndex )( 
            IAppWebUser * This,
            /* [in] */ BSTR __MIDL__IAppWebUser0002);
        
        END_INTERFACE
    } IAppWebUserVtbl;

    interface IAppWebUser
    {
        CONST_VTBL struct IAppWebUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppWebUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppWebUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppWebUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppWebUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppWebUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppWebUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppWebUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppWebUser_get_appId(This,__MIDL__IAppWebUser0000)	\
    ( (This)->lpVtbl -> get_appId(This,__MIDL__IAppWebUser0000) ) 

#define IAppWebUser_get_currentVersionWeb(This,current)	\
    ( (This)->lpVtbl -> get_currentVersionWeb(This,current) ) 

#define IAppWebUser_get_nextVersionWeb(This,next)	\
    ( (This)->lpVtbl -> get_nextVersionWeb(This,next) ) 

#define IAppWebUser_get_command(This,command_id,command)	\
    ( (This)->lpVtbl -> get_command(This,command_id,command) ) 

#define IAppWebUser_cancel(This)	\
    ( (This)->lpVtbl -> cancel(This) ) 

#define IAppWebUser_get_currentState(This,current_state)	\
    ( (This)->lpVtbl -> get_currentState(This,current_state) ) 

#define IAppWebUser_launch(This)	\
    ( (This)->lpVtbl -> launch(This) ) 

#define IAppWebUser_uninstall(This)	\
    ( (This)->lpVtbl -> uninstall(This) ) 

#define IAppWebUser_get_serverInstallDataIndex(This,__MIDL__IAppWebUser0001)	\
    ( (This)->lpVtbl -> get_serverInstallDataIndex(This,__MIDL__IAppWebUser0001) ) 

#define IAppWebUser_put_serverInstallDataIndex(This,__MIDL__IAppWebUser0002)	\
    ( (This)->lpVtbl -> put_serverInstallDataIndex(This,__MIDL__IAppWebUser0002) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppWebUser_INTERFACE_DEFINED__ */


#ifndef __IAppCommandWeb_INTERFACE_DEFINED__
#define __IAppCommandWeb_INTERFACE_DEFINED__

/* interface IAppCommandWeb */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IAppCommandWeb;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("10A2D03F-8BC7-49DB-A21E-A7D4429D2759")
    IAppCommandWeb : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_status( 
            /* [retval][out] */ UINT *__MIDL__IAppCommandWeb0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_exitCode( 
            /* [retval][out] */ DWORD *__MIDL__IAppCommandWeb0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_output( 
            /* [retval][out] */ BSTR *__MIDL__IAppCommandWeb0002) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE execute( 
            /* [optional][in] */ VARIANT substitution1,
            /* [optional][in] */ VARIANT substitution2,
            /* [optional][in] */ VARIANT substitution3,
            /* [optional][in] */ VARIANT substitution4,
            /* [optional][in] */ VARIANT substitution5,
            /* [optional][in] */ VARIANT substitution6,
            /* [optional][in] */ VARIANT substitution7,
            /* [optional][in] */ VARIANT substitution8,
            /* [optional][in] */ VARIANT substitution9) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppCommandWebVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppCommandWeb * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppCommandWeb * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppCommandWeb * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppCommandWeb * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppCommandWeb * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppCommandWeb * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppCommandWeb * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAppCommandWeb, get_status)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_status )( 
            IAppCommandWeb * This,
            /* [retval][out] */ UINT *__MIDL__IAppCommandWeb0000);
        
        DECLSPEC_XFGVIRT(IAppCommandWeb, get_exitCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_exitCode )( 
            IAppCommandWeb * This,
            /* [retval][out] */ DWORD *__MIDL__IAppCommandWeb0001);
        
        DECLSPEC_XFGVIRT(IAppCommandWeb, get_output)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_output )( 
            IAppCommandWeb * This,
            /* [retval][out] */ BSTR *__MIDL__IAppCommandWeb0002);
        
        DECLSPEC_XFGVIRT(IAppCommandWeb, execute)
        HRESULT ( STDMETHODCALLTYPE *execute )( 
            IAppCommandWeb * This,
            /* [optional][in] */ VARIANT substitution1,
            /* [optional][in] */ VARIANT substitution2,
            /* [optional][in] */ VARIANT substitution3,
            /* [optional][in] */ VARIANT substitution4,
            /* [optional][in] */ VARIANT substitution5,
            /* [optional][in] */ VARIANT substitution6,
            /* [optional][in] */ VARIANT substitution7,
            /* [optional][in] */ VARIANT substitution8,
            /* [optional][in] */ VARIANT substitution9);
        
        END_INTERFACE
    } IAppCommandWebVtbl;

    interface IAppCommandWeb
    {
        CONST_VTBL struct IAppCommandWebVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppCommandWeb_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppCommandWeb_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppCommandWeb_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppCommandWeb_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppCommandWeb_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppCommandWeb_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppCommandWeb_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppCommandWeb_get_status(This,__MIDL__IAppCommandWeb0000)	\
    ( (This)->lpVtbl -> get_status(This,__MIDL__IAppCommandWeb0000) ) 

#define IAppCommandWeb_get_exitCode(This,__MIDL__IAppCommandWeb0001)	\
    ( (This)->lpVtbl -> get_exitCode(This,__MIDL__IAppCommandWeb0001) ) 

#define IAppCommandWeb_get_output(This,__MIDL__IAppCommandWeb0002)	\
    ( (This)->lpVtbl -> get_output(This,__MIDL__IAppCommandWeb0002) ) 

#define IAppCommandWeb_execute(This,substitution1,substitution2,substitution3,substitution4,substitution5,substitution6,substitution7,substitution8,substitution9)	\
    ( (This)->lpVtbl -> execute(This,substitution1,substitution2,substitution3,substitution4,substitution5,substitution6,substitution7,substitution8,substitution9) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppCommandWeb_INTERFACE_DEFINED__ */


#ifndef __IAppCommandWebUser_INTERFACE_DEFINED__
#define __IAppCommandWebUser_INTERFACE_DEFINED__

/* interface IAppCommandWebUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IAppCommandWebUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5515E66F-FA6F-4D74-B5EA-4FCFDA16FE12")
    IAppCommandWebUser : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_status( 
            /* [retval][out] */ UINT *__MIDL__IAppCommandWebUser0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_exitCode( 
            /* [retval][out] */ DWORD *__MIDL__IAppCommandWebUser0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_output( 
            /* [retval][out] */ BSTR *__MIDL__IAppCommandWebUser0002) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE execute( 
            /* [optional][in] */ VARIANT substitution1,
            /* [optional][in] */ VARIANT substitution2,
            /* [optional][in] */ VARIANT substitution3,
            /* [optional][in] */ VARIANT substitution4,
            /* [optional][in] */ VARIANT substitution5,
            /* [optional][in] */ VARIANT substitution6,
            /* [optional][in] */ VARIANT substitution7,
            /* [optional][in] */ VARIANT substitution8,
            /* [optional][in] */ VARIANT substitution9) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppCommandWebUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppCommandWebUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppCommandWebUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppCommandWebUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppCommandWebUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppCommandWebUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppCommandWebUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppCommandWebUser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAppCommandWebUser, get_status)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_status )( 
            IAppCommandWebUser * This,
            /* [retval][out] */ UINT *__MIDL__IAppCommandWebUser0000);
        
        DECLSPEC_XFGVIRT(IAppCommandWebUser, get_exitCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_exitCode )( 
            IAppCommandWebUser * This,
            /* [retval][out] */ DWORD *__MIDL__IAppCommandWebUser0001);
        
        DECLSPEC_XFGVIRT(IAppCommandWebUser, get_output)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_output )( 
            IAppCommandWebUser * This,
            /* [retval][out] */ BSTR *__MIDL__IAppCommandWebUser0002);
        
        DECLSPEC_XFGVIRT(IAppCommandWebUser, execute)
        HRESULT ( STDMETHODCALLTYPE *execute )( 
            IAppCommandWebUser * This,
            /* [optional][in] */ VARIANT substitution1,
            /* [optional][in] */ VARIANT substitution2,
            /* [optional][in] */ VARIANT substitution3,
            /* [optional][in] */ VARIANT substitution4,
            /* [optional][in] */ VARIANT substitution5,
            /* [optional][in] */ VARIANT substitution6,
            /* [optional][in] */ VARIANT substitution7,
            /* [optional][in] */ VARIANT substitution8,
            /* [optional][in] */ VARIANT substitution9);
        
        END_INTERFACE
    } IAppCommandWebUserVtbl;

    interface IAppCommandWebUser
    {
        CONST_VTBL struct IAppCommandWebUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppCommandWebUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppCommandWebUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppCommandWebUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppCommandWebUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppCommandWebUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppCommandWebUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppCommandWebUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppCommandWebUser_get_status(This,__MIDL__IAppCommandWebUser0000)	\
    ( (This)->lpVtbl -> get_status(This,__MIDL__IAppCommandWebUser0000) ) 

#define IAppCommandWebUser_get_exitCode(This,__MIDL__IAppCommandWebUser0001)	\
    ( (This)->lpVtbl -> get_exitCode(This,__MIDL__IAppCommandWebUser0001) ) 

#define IAppCommandWebUser_get_output(This,__MIDL__IAppCommandWebUser0002)	\
    ( (This)->lpVtbl -> get_output(This,__MIDL__IAppCommandWebUser0002) ) 

#define IAppCommandWebUser_execute(This,substitution1,substitution2,substitution3,substitution4,substitution5,substitution6,substitution7,substitution8,substitution9)	\
    ( (This)->lpVtbl -> execute(This,substitution1,substitution2,substitution3,substitution4,substitution5,substitution6,substitution7,substitution8,substitution9) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppCommandWebUser_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatus_INTERFACE_DEFINED__
#define __IPolicyStatus_INTERFACE_DEFINED__

/* interface IPolicyStatus */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6A54FE75-EDC8-404E-A41B-4278C0557151")
    IPolicyStatus : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_lastCheckPeriodMinutes( 
            /* [retval][out] */ DWORD *minutes) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_updatesSuppressedTimes( 
            /* [out] */ DWORD *start_hour,
            /* [out] */ DWORD *start_min,
            /* [out] */ DWORD *duration_min,
            /* [out] */ VARIANT_BOOL *are_updates_suppressed) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadPreferenceGroupPolicy( 
            /* [retval][out] */ BSTR *pref) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCacheSizeLimitMBytes( 
            /* [retval][out] */ DWORD *limit) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCacheExpirationTimeDays( 
            /* [retval][out] */ DWORD *days) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_effectivePolicyForAppInstalls( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ DWORD *policy) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_effectivePolicyForAppUpdates( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ DWORD *policy) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_targetVersionPrefix( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ BSTR *prefix) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isRollbackToTargetVersionAllowed( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ VARIANT_BOOL *rollback_allowed) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatus * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatus * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatus * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatus * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatus * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatus * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_lastCheckPeriodMinutes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckPeriodMinutes )( 
            IPolicyStatus * This,
            /* [retval][out] */ DWORD *minutes);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_updatesSuppressedTimes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updatesSuppressedTimes )( 
            IPolicyStatus * This,
            /* [out] */ DWORD *start_hour,
            /* [out] */ DWORD *start_min,
            /* [out] */ DWORD *duration_min,
            /* [out] */ VARIANT_BOOL *are_updates_suppressed);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_downloadPreferenceGroupPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadPreferenceGroupPolicy )( 
            IPolicyStatus * This,
            /* [retval][out] */ BSTR *pref);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_packageCacheSizeLimitMBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheSizeLimitMBytes )( 
            IPolicyStatus * This,
            /* [retval][out] */ DWORD *limit);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_packageCacheExpirationTimeDays)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheExpirationTimeDays )( 
            IPolicyStatus * This,
            /* [retval][out] */ DWORD *days);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_effectivePolicyForAppInstalls)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppInstalls )( 
            IPolicyStatus * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ DWORD *policy);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_effectivePolicyForAppUpdates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppUpdates )( 
            IPolicyStatus * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ DWORD *policy);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_targetVersionPrefix)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetVersionPrefix )( 
            IPolicyStatus * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ BSTR *prefix);
        
        DECLSPEC_XFGVIRT(IPolicyStatus, get_isRollbackToTargetVersionAllowed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRollbackToTargetVersionAllowed )( 
            IPolicyStatus * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ VARIANT_BOOL *rollback_allowed);
        
        END_INTERFACE
    } IPolicyStatusVtbl;

    interface IPolicyStatus
    {
        CONST_VTBL struct IPolicyStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatus_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatus_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatus_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatus_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatus_get_lastCheckPeriodMinutes(This,minutes)	\
    ( (This)->lpVtbl -> get_lastCheckPeriodMinutes(This,minutes) ) 

#define IPolicyStatus_get_updatesSuppressedTimes(This,start_hour,start_min,duration_min,are_updates_suppressed)	\
    ( (This)->lpVtbl -> get_updatesSuppressedTimes(This,start_hour,start_min,duration_min,are_updates_suppressed) ) 

#define IPolicyStatus_get_downloadPreferenceGroupPolicy(This,pref)	\
    ( (This)->lpVtbl -> get_downloadPreferenceGroupPolicy(This,pref) ) 

#define IPolicyStatus_get_packageCacheSizeLimitMBytes(This,limit)	\
    ( (This)->lpVtbl -> get_packageCacheSizeLimitMBytes(This,limit) ) 

#define IPolicyStatus_get_packageCacheExpirationTimeDays(This,days)	\
    ( (This)->lpVtbl -> get_packageCacheExpirationTimeDays(This,days) ) 

#define IPolicyStatus_get_effectivePolicyForAppInstalls(This,app_id,policy)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppInstalls(This,app_id,policy) ) 

#define IPolicyStatus_get_effectivePolicyForAppUpdates(This,app_id,policy)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppUpdates(This,app_id,policy) ) 

#define IPolicyStatus_get_targetVersionPrefix(This,app_id,prefix)	\
    ( (This)->lpVtbl -> get_targetVersionPrefix(This,app_id,prefix) ) 

#define IPolicyStatus_get_isRollbackToTargetVersionAllowed(This,app_id,rollback_allowed)	\
    ( (This)->lpVtbl -> get_isRollbackToTargetVersionAllowed(This,app_id,rollback_allowed) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatus_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatusUser_INTERFACE_DEFINED__
#define __IPolicyStatusUser_INTERFACE_DEFINED__

/* interface IPolicyStatusUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatusUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EF739C0C-40B0-478D-B76B-3659B8F2B0EB")
    IPolicyStatusUser : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_lastCheckPeriodMinutes( 
            /* [retval][out] */ DWORD *minutes) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_updatesSuppressedTimes( 
            /* [out] */ DWORD *start_hour,
            /* [out] */ DWORD *start_min,
            /* [out] */ DWORD *duration_min,
            /* [out] */ VARIANT_BOOL *are_updates_suppressed) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadPreferenceGroupPolicy( 
            /* [retval][out] */ BSTR *pref) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCacheSizeLimitMBytes( 
            /* [retval][out] */ DWORD *limit) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCacheExpirationTimeDays( 
            /* [retval][out] */ DWORD *days) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_effectivePolicyForAppInstalls( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ DWORD *policy) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_effectivePolicyForAppUpdates( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ DWORD *policy) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_targetVersionPrefix( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ BSTR *prefix) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isRollbackToTargetVersionAllowed( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ VARIANT_BOOL *rollback_allowed) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatusUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatusUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatusUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatusUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatusUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatusUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatusUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatusUser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_lastCheckPeriodMinutes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckPeriodMinutes )( 
            IPolicyStatusUser * This,
            /* [retval][out] */ DWORD *minutes);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_updatesSuppressedTimes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updatesSuppressedTimes )( 
            IPolicyStatusUser * This,
            /* [out] */ DWORD *start_hour,
            /* [out] */ DWORD *start_min,
            /* [out] */ DWORD *duration_min,
            /* [out] */ VARIANT_BOOL *are_updates_suppressed);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_downloadPreferenceGroupPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadPreferenceGroupPolicy )( 
            IPolicyStatusUser * This,
            /* [retval][out] */ BSTR *pref);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_packageCacheSizeLimitMBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheSizeLimitMBytes )( 
            IPolicyStatusUser * This,
            /* [retval][out] */ DWORD *limit);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_packageCacheExpirationTimeDays)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheExpirationTimeDays )( 
            IPolicyStatusUser * This,
            /* [retval][out] */ DWORD *days);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_effectivePolicyForAppInstalls)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppInstalls )( 
            IPolicyStatusUser * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ DWORD *policy);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_effectivePolicyForAppUpdates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppUpdates )( 
            IPolicyStatusUser * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ DWORD *policy);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_targetVersionPrefix)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetVersionPrefix )( 
            IPolicyStatusUser * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ BSTR *prefix);
        
        DECLSPEC_XFGVIRT(IPolicyStatusUser, get_isRollbackToTargetVersionAllowed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRollbackToTargetVersionAllowed )( 
            IPolicyStatusUser * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ VARIANT_BOOL *rollback_allowed);
        
        END_INTERFACE
    } IPolicyStatusUserVtbl;

    interface IPolicyStatusUser
    {
        CONST_VTBL struct IPolicyStatusUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatusUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatusUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatusUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatusUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatusUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatusUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatusUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatusUser_get_lastCheckPeriodMinutes(This,minutes)	\
    ( (This)->lpVtbl -> get_lastCheckPeriodMinutes(This,minutes) ) 

#define IPolicyStatusUser_get_updatesSuppressedTimes(This,start_hour,start_min,duration_min,are_updates_suppressed)	\
    ( (This)->lpVtbl -> get_updatesSuppressedTimes(This,start_hour,start_min,duration_min,are_updates_suppressed) ) 

#define IPolicyStatusUser_get_downloadPreferenceGroupPolicy(This,pref)	\
    ( (This)->lpVtbl -> get_downloadPreferenceGroupPolicy(This,pref) ) 

#define IPolicyStatusUser_get_packageCacheSizeLimitMBytes(This,limit)	\
    ( (This)->lpVtbl -> get_packageCacheSizeLimitMBytes(This,limit) ) 

#define IPolicyStatusUser_get_packageCacheExpirationTimeDays(This,days)	\
    ( (This)->lpVtbl -> get_packageCacheExpirationTimeDays(This,days) ) 

#define IPolicyStatusUser_get_effectivePolicyForAppInstalls(This,app_id,policy)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppInstalls(This,app_id,policy) ) 

#define IPolicyStatusUser_get_effectivePolicyForAppUpdates(This,app_id,policy)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppUpdates(This,app_id,policy) ) 

#define IPolicyStatusUser_get_targetVersionPrefix(This,app_id,prefix)	\
    ( (This)->lpVtbl -> get_targetVersionPrefix(This,app_id,prefix) ) 

#define IPolicyStatusUser_get_isRollbackToTargetVersionAllowed(This,app_id,rollback_allowed)	\
    ( (This)->lpVtbl -> get_isRollbackToTargetVersionAllowed(This,app_id,rollback_allowed) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatusUser_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatusValue_INTERFACE_DEFINED__
#define __IPolicyStatusValue_INTERFACE_DEFINED__

/* interface IPolicyStatusValue */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatusValue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A7D2AE7-8EEE-45B4-B17F-31DAAC82CCBB")
    IPolicyStatusValue : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_source( 
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValue0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_value( 
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValue0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_hasConflict( 
            /* [retval][out] */ VARIANT_BOOL *has_conflict) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_conflictSource( 
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValue0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_conflictValue( 
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValue0003) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatusValueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatusValue * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatusValue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatusValue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatusValue * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatusValue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatusValue * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatusValue * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValue, get_source)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_source )( 
            IPolicyStatusValue * This,
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValue0000);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValue, get_value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_value )( 
            IPolicyStatusValue * This,
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValue0001);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValue, get_hasConflict)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_hasConflict )( 
            IPolicyStatusValue * This,
            /* [retval][out] */ VARIANT_BOOL *has_conflict);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValue, get_conflictSource)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_conflictSource )( 
            IPolicyStatusValue * This,
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValue0002);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValue, get_conflictValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_conflictValue )( 
            IPolicyStatusValue * This,
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValue0003);
        
        END_INTERFACE
    } IPolicyStatusValueVtbl;

    interface IPolicyStatusValue
    {
        CONST_VTBL struct IPolicyStatusValueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatusValue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatusValue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatusValue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatusValue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatusValue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatusValue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatusValue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatusValue_get_source(This,__MIDL__IPolicyStatusValue0000)	\
    ( (This)->lpVtbl -> get_source(This,__MIDL__IPolicyStatusValue0000) ) 

#define IPolicyStatusValue_get_value(This,__MIDL__IPolicyStatusValue0001)	\
    ( (This)->lpVtbl -> get_value(This,__MIDL__IPolicyStatusValue0001) ) 

#define IPolicyStatusValue_get_hasConflict(This,has_conflict)	\
    ( (This)->lpVtbl -> get_hasConflict(This,has_conflict) ) 

#define IPolicyStatusValue_get_conflictSource(This,__MIDL__IPolicyStatusValue0002)	\
    ( (This)->lpVtbl -> get_conflictSource(This,__MIDL__IPolicyStatusValue0002) ) 

#define IPolicyStatusValue_get_conflictValue(This,__MIDL__IPolicyStatusValue0003)	\
    ( (This)->lpVtbl -> get_conflictValue(This,__MIDL__IPolicyStatusValue0003) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatusValue_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatusValueUser_INTERFACE_DEFINED__
#define __IPolicyStatusValueUser_INTERFACE_DEFINED__

/* interface IPolicyStatusValueUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatusValueUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7E0A6B39-7CEB-4944-ABFA-F419D201D6A0")
    IPolicyStatusValueUser : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_source( 
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValueUser0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_value( 
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValueUser0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_hasConflict( 
            /* [retval][out] */ VARIANT_BOOL *has_conflict) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_conflictSource( 
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValueUser0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_conflictValue( 
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValueUser0003) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatusValueUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatusValueUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatusValueUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatusValueUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatusValueUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatusValueUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatusValueUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatusValueUser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValueUser, get_source)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_source )( 
            IPolicyStatusValueUser * This,
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValueUser0000);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValueUser, get_value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_value )( 
            IPolicyStatusValueUser * This,
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValueUser0001);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValueUser, get_hasConflict)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_hasConflict )( 
            IPolicyStatusValueUser * This,
            /* [retval][out] */ VARIANT_BOOL *has_conflict);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValueUser, get_conflictSource)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_conflictSource )( 
            IPolicyStatusValueUser * This,
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValueUser0002);
        
        DECLSPEC_XFGVIRT(IPolicyStatusValueUser, get_conflictValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_conflictValue )( 
            IPolicyStatusValueUser * This,
            /* [retval][out] */ BSTR *__MIDL__IPolicyStatusValueUser0003);
        
        END_INTERFACE
    } IPolicyStatusValueUserVtbl;

    interface IPolicyStatusValueUser
    {
        CONST_VTBL struct IPolicyStatusValueUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatusValueUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatusValueUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatusValueUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatusValueUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatusValueUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatusValueUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatusValueUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatusValueUser_get_source(This,__MIDL__IPolicyStatusValueUser0000)	\
    ( (This)->lpVtbl -> get_source(This,__MIDL__IPolicyStatusValueUser0000) ) 

#define IPolicyStatusValueUser_get_value(This,__MIDL__IPolicyStatusValueUser0001)	\
    ( (This)->lpVtbl -> get_value(This,__MIDL__IPolicyStatusValueUser0001) ) 

#define IPolicyStatusValueUser_get_hasConflict(This,has_conflict)	\
    ( (This)->lpVtbl -> get_hasConflict(This,has_conflict) ) 

#define IPolicyStatusValueUser_get_conflictSource(This,__MIDL__IPolicyStatusValueUser0002)	\
    ( (This)->lpVtbl -> get_conflictSource(This,__MIDL__IPolicyStatusValueUser0002) ) 

#define IPolicyStatusValueUser_get_conflictValue(This,__MIDL__IPolicyStatusValueUser0003)	\
    ( (This)->lpVtbl -> get_conflictValue(This,__MIDL__IPolicyStatusValueUser0003) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatusValueUser_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatus2_INTERFACE_DEFINED__
#define __IPolicyStatus2_INTERFACE_DEFINED__

/* interface IPolicyStatus2 */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatus2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("06A6AA1E-2680-4076-A7CD-6053722CF454")
    IPolicyStatus2 : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_updaterVersion( 
            /* [retval][out] */ BSTR *version) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_lastCheckedTime( 
            /* [retval][out] */ DATE *last_checked) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE refreshPolicies( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_lastCheckPeriodMinutes( 
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_updatesSuppressedTimes( 
            /* [out] */ IPolicyStatusValue **value,
            VARIANT_BOOL *are_updates_suppressed) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadPreferenceGroupPolicy( 
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCacheSizeLimitMBytes( 
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCacheExpirationTimeDays( 
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_proxyMode( 
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_proxyPacUrl( 
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_proxyServer( 
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_effectivePolicyForAppInstalls( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_effectivePolicyForAppUpdates( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_targetVersionPrefix( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isRollbackToTargetVersionAllowed( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_targetChannel( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatus2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatus2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatus2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatus2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatus2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatus2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatus2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatus2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_updaterVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updaterVersion )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_lastCheckedTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckedTime )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ DATE *last_checked);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, refreshPolicies)
        HRESULT ( STDMETHODCALLTYPE *refreshPolicies )( 
            IPolicyStatus2 * This);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_lastCheckPeriodMinutes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckPeriodMinutes )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_updatesSuppressedTimes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updatesSuppressedTimes )( 
            IPolicyStatus2 * This,
            /* [out] */ IPolicyStatusValue **value,
            VARIANT_BOOL *are_updates_suppressed);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_downloadPreferenceGroupPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadPreferenceGroupPolicy )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_packageCacheSizeLimitMBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheSizeLimitMBytes )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_packageCacheExpirationTimeDays)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheExpirationTimeDays )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyMode )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyPacUrl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyPacUrl )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyServer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyServer )( 
            IPolicyStatus2 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_effectivePolicyForAppInstalls)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppInstalls )( 
            IPolicyStatus2 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_effectivePolicyForAppUpdates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppUpdates )( 
            IPolicyStatus2 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_targetVersionPrefix)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetVersionPrefix )( 
            IPolicyStatus2 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_isRollbackToTargetVersionAllowed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRollbackToTargetVersionAllowed )( 
            IPolicyStatus2 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_targetChannel)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetChannel )( 
            IPolicyStatus2 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        END_INTERFACE
    } IPolicyStatus2Vtbl;

    interface IPolicyStatus2
    {
        CONST_VTBL struct IPolicyStatus2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatus2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatus2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatus2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatus2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatus2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatus2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatus2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatus2_get_updaterVersion(This,version)	\
    ( (This)->lpVtbl -> get_updaterVersion(This,version) ) 

#define IPolicyStatus2_get_lastCheckedTime(This,last_checked)	\
    ( (This)->lpVtbl -> get_lastCheckedTime(This,last_checked) ) 

#define IPolicyStatus2_refreshPolicies(This)	\
    ( (This)->lpVtbl -> refreshPolicies(This) ) 

#define IPolicyStatus2_get_lastCheckPeriodMinutes(This,value)	\
    ( (This)->lpVtbl -> get_lastCheckPeriodMinutes(This,value) ) 

#define IPolicyStatus2_get_updatesSuppressedTimes(This,value,are_updates_suppressed)	\
    ( (This)->lpVtbl -> get_updatesSuppressedTimes(This,value,are_updates_suppressed) ) 

#define IPolicyStatus2_get_downloadPreferenceGroupPolicy(This,value)	\
    ( (This)->lpVtbl -> get_downloadPreferenceGroupPolicy(This,value) ) 

#define IPolicyStatus2_get_packageCacheSizeLimitMBytes(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheSizeLimitMBytes(This,value) ) 

#define IPolicyStatus2_get_packageCacheExpirationTimeDays(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheExpirationTimeDays(This,value) ) 

#define IPolicyStatus2_get_proxyMode(This,value)	\
    ( (This)->lpVtbl -> get_proxyMode(This,value) ) 

#define IPolicyStatus2_get_proxyPacUrl(This,value)	\
    ( (This)->lpVtbl -> get_proxyPacUrl(This,value) ) 

#define IPolicyStatus2_get_proxyServer(This,value)	\
    ( (This)->lpVtbl -> get_proxyServer(This,value) ) 

#define IPolicyStatus2_get_effectivePolicyForAppInstalls(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppInstalls(This,app_id,value) ) 

#define IPolicyStatus2_get_effectivePolicyForAppUpdates(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppUpdates(This,app_id,value) ) 

#define IPolicyStatus2_get_targetVersionPrefix(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetVersionPrefix(This,app_id,value) ) 

#define IPolicyStatus2_get_isRollbackToTargetVersionAllowed(This,app_id,value)	\
    ( (This)->lpVtbl -> get_isRollbackToTargetVersionAllowed(This,app_id,value) ) 

#define IPolicyStatus2_get_targetChannel(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetChannel(This,app_id,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatus2_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatus2User_INTERFACE_DEFINED__
#define __IPolicyStatus2User_INTERFACE_DEFINED__

/* interface IPolicyStatus2User */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatus2User;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AD91C851-86AC-499F-9BA9-9A561744AA4D")
    IPolicyStatus2User : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_updaterVersion( 
            /* [retval][out] */ BSTR *version) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_lastCheckedTime( 
            /* [retval][out] */ DATE *last_checked) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE refreshPolicies( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_lastCheckPeriodMinutes( 
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_updatesSuppressedTimes( 
            /* [out] */ IPolicyStatusValueUser **value,
            VARIANT_BOOL *are_updates_suppressed) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadPreferenceGroupPolicy( 
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCacheSizeLimitMBytes( 
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_packageCacheExpirationTimeDays( 
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_proxyMode( 
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_proxyPacUrl( 
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_proxyServer( 
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_effectivePolicyForAppInstalls( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_effectivePolicyForAppUpdates( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_targetVersionPrefix( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isRollbackToTargetVersionAllowed( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_targetChannel( 
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatus2UserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatus2User * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatus2User * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatus2User * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatus2User * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatus2User * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatus2User * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatus2User * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_updaterVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updaterVersion )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_lastCheckedTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckedTime )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ DATE *last_checked);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, refreshPolicies)
        HRESULT ( STDMETHODCALLTYPE *refreshPolicies )( 
            IPolicyStatus2User * This);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_lastCheckPeriodMinutes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckPeriodMinutes )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_updatesSuppressedTimes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updatesSuppressedTimes )( 
            IPolicyStatus2User * This,
            /* [out] */ IPolicyStatusValueUser **value,
            VARIANT_BOOL *are_updates_suppressed);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_downloadPreferenceGroupPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadPreferenceGroupPolicy )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_packageCacheSizeLimitMBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheSizeLimitMBytes )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_packageCacheExpirationTimeDays)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheExpirationTimeDays )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyMode )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyPacUrl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyPacUrl )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyServer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyServer )( 
            IPolicyStatus2User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_effectivePolicyForAppInstalls)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppInstalls )( 
            IPolicyStatus2User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_effectivePolicyForAppUpdates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppUpdates )( 
            IPolicyStatus2User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_targetVersionPrefix)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetVersionPrefix )( 
            IPolicyStatus2User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_isRollbackToTargetVersionAllowed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRollbackToTargetVersionAllowed )( 
            IPolicyStatus2User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_targetChannel)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetChannel )( 
            IPolicyStatus2User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        END_INTERFACE
    } IPolicyStatus2UserVtbl;

    interface IPolicyStatus2User
    {
        CONST_VTBL struct IPolicyStatus2UserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatus2User_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatus2User_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatus2User_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatus2User_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatus2User_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatus2User_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatus2User_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatus2User_get_updaterVersion(This,version)	\
    ( (This)->lpVtbl -> get_updaterVersion(This,version) ) 

#define IPolicyStatus2User_get_lastCheckedTime(This,last_checked)	\
    ( (This)->lpVtbl -> get_lastCheckedTime(This,last_checked) ) 

#define IPolicyStatus2User_refreshPolicies(This)	\
    ( (This)->lpVtbl -> refreshPolicies(This) ) 

#define IPolicyStatus2User_get_lastCheckPeriodMinutes(This,value)	\
    ( (This)->lpVtbl -> get_lastCheckPeriodMinutes(This,value) ) 

#define IPolicyStatus2User_get_updatesSuppressedTimes(This,value,are_updates_suppressed)	\
    ( (This)->lpVtbl -> get_updatesSuppressedTimes(This,value,are_updates_suppressed) ) 

#define IPolicyStatus2User_get_downloadPreferenceGroupPolicy(This,value)	\
    ( (This)->lpVtbl -> get_downloadPreferenceGroupPolicy(This,value) ) 

#define IPolicyStatus2User_get_packageCacheSizeLimitMBytes(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheSizeLimitMBytes(This,value) ) 

#define IPolicyStatus2User_get_packageCacheExpirationTimeDays(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheExpirationTimeDays(This,value) ) 

#define IPolicyStatus2User_get_proxyMode(This,value)	\
    ( (This)->lpVtbl -> get_proxyMode(This,value) ) 

#define IPolicyStatus2User_get_proxyPacUrl(This,value)	\
    ( (This)->lpVtbl -> get_proxyPacUrl(This,value) ) 

#define IPolicyStatus2User_get_proxyServer(This,value)	\
    ( (This)->lpVtbl -> get_proxyServer(This,value) ) 

#define IPolicyStatus2User_get_effectivePolicyForAppInstalls(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppInstalls(This,app_id,value) ) 

#define IPolicyStatus2User_get_effectivePolicyForAppUpdates(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppUpdates(This,app_id,value) ) 

#define IPolicyStatus2User_get_targetVersionPrefix(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetVersionPrefix(This,app_id,value) ) 

#define IPolicyStatus2User_get_isRollbackToTargetVersionAllowed(This,app_id,value)	\
    ( (This)->lpVtbl -> get_isRollbackToTargetVersionAllowed(This,app_id,value) ) 

#define IPolicyStatus2User_get_targetChannel(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetChannel(This,app_id,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatus2User_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatus3_INTERFACE_DEFINED__
#define __IPolicyStatus3_INTERFACE_DEFINED__

/* interface IPolicyStatus3 */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatus3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("029BD175-5035-4E2A-8724-C9D47F4FAEA3")
    IPolicyStatus3 : public IPolicyStatus2
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_forceInstallApps( 
            /* [in] */ VARIANT_BOOL is_machine,
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatus3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatus3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatus3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatus3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatus3 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatus3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatus3 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatus3 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_updaterVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updaterVersion )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_lastCheckedTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckedTime )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ DATE *last_checked);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, refreshPolicies)
        HRESULT ( STDMETHODCALLTYPE *refreshPolicies )( 
            IPolicyStatus3 * This);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_lastCheckPeriodMinutes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckPeriodMinutes )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_updatesSuppressedTimes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updatesSuppressedTimes )( 
            IPolicyStatus3 * This,
            /* [out] */ IPolicyStatusValue **value,
            VARIANT_BOOL *are_updates_suppressed);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_downloadPreferenceGroupPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadPreferenceGroupPolicy )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_packageCacheSizeLimitMBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheSizeLimitMBytes )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_packageCacheExpirationTimeDays)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheExpirationTimeDays )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyMode )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyPacUrl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyPacUrl )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyServer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyServer )( 
            IPolicyStatus3 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_effectivePolicyForAppInstalls)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppInstalls )( 
            IPolicyStatus3 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_effectivePolicyForAppUpdates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppUpdates )( 
            IPolicyStatus3 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_targetVersionPrefix)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetVersionPrefix )( 
            IPolicyStatus3 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_isRollbackToTargetVersionAllowed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRollbackToTargetVersionAllowed )( 
            IPolicyStatus3 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_targetChannel)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetChannel )( 
            IPolicyStatus3 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus3, get_forceInstallApps)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_forceInstallApps )( 
            IPolicyStatus3 * This,
            /* [in] */ VARIANT_BOOL is_machine,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        END_INTERFACE
    } IPolicyStatus3Vtbl;

    interface IPolicyStatus3
    {
        CONST_VTBL struct IPolicyStatus3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatus3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatus3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatus3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatus3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatus3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatus3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatus3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatus3_get_updaterVersion(This,version)	\
    ( (This)->lpVtbl -> get_updaterVersion(This,version) ) 

#define IPolicyStatus3_get_lastCheckedTime(This,last_checked)	\
    ( (This)->lpVtbl -> get_lastCheckedTime(This,last_checked) ) 

#define IPolicyStatus3_refreshPolicies(This)	\
    ( (This)->lpVtbl -> refreshPolicies(This) ) 

#define IPolicyStatus3_get_lastCheckPeriodMinutes(This,value)	\
    ( (This)->lpVtbl -> get_lastCheckPeriodMinutes(This,value) ) 

#define IPolicyStatus3_get_updatesSuppressedTimes(This,value,are_updates_suppressed)	\
    ( (This)->lpVtbl -> get_updatesSuppressedTimes(This,value,are_updates_suppressed) ) 

#define IPolicyStatus3_get_downloadPreferenceGroupPolicy(This,value)	\
    ( (This)->lpVtbl -> get_downloadPreferenceGroupPolicy(This,value) ) 

#define IPolicyStatus3_get_packageCacheSizeLimitMBytes(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheSizeLimitMBytes(This,value) ) 

#define IPolicyStatus3_get_packageCacheExpirationTimeDays(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheExpirationTimeDays(This,value) ) 

#define IPolicyStatus3_get_proxyMode(This,value)	\
    ( (This)->lpVtbl -> get_proxyMode(This,value) ) 

#define IPolicyStatus3_get_proxyPacUrl(This,value)	\
    ( (This)->lpVtbl -> get_proxyPacUrl(This,value) ) 

#define IPolicyStatus3_get_proxyServer(This,value)	\
    ( (This)->lpVtbl -> get_proxyServer(This,value) ) 

#define IPolicyStatus3_get_effectivePolicyForAppInstalls(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppInstalls(This,app_id,value) ) 

#define IPolicyStatus3_get_effectivePolicyForAppUpdates(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppUpdates(This,app_id,value) ) 

#define IPolicyStatus3_get_targetVersionPrefix(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetVersionPrefix(This,app_id,value) ) 

#define IPolicyStatus3_get_isRollbackToTargetVersionAllowed(This,app_id,value)	\
    ( (This)->lpVtbl -> get_isRollbackToTargetVersionAllowed(This,app_id,value) ) 

#define IPolicyStatus3_get_targetChannel(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetChannel(This,app_id,value) ) 


#define IPolicyStatus3_get_forceInstallApps(This,is_machine,value)	\
    ( (This)->lpVtbl -> get_forceInstallApps(This,is_machine,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatus3_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatus3User_INTERFACE_DEFINED__
#define __IPolicyStatus3User_INTERFACE_DEFINED__

/* interface IPolicyStatus3User */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatus3User;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BC39E1E1-E8FA-4E72-903F-3BF346E7E165")
    IPolicyStatus3User : public IPolicyStatus2User
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_forceInstallApps( 
            /* [in] */ VARIANT_BOOL is_machine,
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatus3UserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatus3User * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatus3User * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatus3User * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatus3User * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatus3User * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatus3User * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatus3User * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_updaterVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updaterVersion )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_lastCheckedTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckedTime )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ DATE *last_checked);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, refreshPolicies)
        HRESULT ( STDMETHODCALLTYPE *refreshPolicies )( 
            IPolicyStatus3User * This);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_lastCheckPeriodMinutes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckPeriodMinutes )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_updatesSuppressedTimes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updatesSuppressedTimes )( 
            IPolicyStatus3User * This,
            /* [out] */ IPolicyStatusValueUser **value,
            VARIANT_BOOL *are_updates_suppressed);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_downloadPreferenceGroupPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadPreferenceGroupPolicy )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_packageCacheSizeLimitMBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheSizeLimitMBytes )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_packageCacheExpirationTimeDays)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheExpirationTimeDays )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyMode )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyPacUrl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyPacUrl )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyServer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyServer )( 
            IPolicyStatus3User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_effectivePolicyForAppInstalls)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppInstalls )( 
            IPolicyStatus3User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_effectivePolicyForAppUpdates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppUpdates )( 
            IPolicyStatus3User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_targetVersionPrefix)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetVersionPrefix )( 
            IPolicyStatus3User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_isRollbackToTargetVersionAllowed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRollbackToTargetVersionAllowed )( 
            IPolicyStatus3User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_targetChannel)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetChannel )( 
            IPolicyStatus3User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus3User, get_forceInstallApps)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_forceInstallApps )( 
            IPolicyStatus3User * This,
            /* [in] */ VARIANT_BOOL is_machine,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        END_INTERFACE
    } IPolicyStatus3UserVtbl;

    interface IPolicyStatus3User
    {
        CONST_VTBL struct IPolicyStatus3UserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatus3User_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatus3User_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatus3User_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatus3User_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatus3User_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatus3User_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatus3User_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatus3User_get_updaterVersion(This,version)	\
    ( (This)->lpVtbl -> get_updaterVersion(This,version) ) 

#define IPolicyStatus3User_get_lastCheckedTime(This,last_checked)	\
    ( (This)->lpVtbl -> get_lastCheckedTime(This,last_checked) ) 

#define IPolicyStatus3User_refreshPolicies(This)	\
    ( (This)->lpVtbl -> refreshPolicies(This) ) 

#define IPolicyStatus3User_get_lastCheckPeriodMinutes(This,value)	\
    ( (This)->lpVtbl -> get_lastCheckPeriodMinutes(This,value) ) 

#define IPolicyStatus3User_get_updatesSuppressedTimes(This,value,are_updates_suppressed)	\
    ( (This)->lpVtbl -> get_updatesSuppressedTimes(This,value,are_updates_suppressed) ) 

#define IPolicyStatus3User_get_downloadPreferenceGroupPolicy(This,value)	\
    ( (This)->lpVtbl -> get_downloadPreferenceGroupPolicy(This,value) ) 

#define IPolicyStatus3User_get_packageCacheSizeLimitMBytes(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheSizeLimitMBytes(This,value) ) 

#define IPolicyStatus3User_get_packageCacheExpirationTimeDays(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheExpirationTimeDays(This,value) ) 

#define IPolicyStatus3User_get_proxyMode(This,value)	\
    ( (This)->lpVtbl -> get_proxyMode(This,value) ) 

#define IPolicyStatus3User_get_proxyPacUrl(This,value)	\
    ( (This)->lpVtbl -> get_proxyPacUrl(This,value) ) 

#define IPolicyStatus3User_get_proxyServer(This,value)	\
    ( (This)->lpVtbl -> get_proxyServer(This,value) ) 

#define IPolicyStatus3User_get_effectivePolicyForAppInstalls(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppInstalls(This,app_id,value) ) 

#define IPolicyStatus3User_get_effectivePolicyForAppUpdates(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppUpdates(This,app_id,value) ) 

#define IPolicyStatus3User_get_targetVersionPrefix(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetVersionPrefix(This,app_id,value) ) 

#define IPolicyStatus3User_get_isRollbackToTargetVersionAllowed(This,app_id,value)	\
    ( (This)->lpVtbl -> get_isRollbackToTargetVersionAllowed(This,app_id,value) ) 

#define IPolicyStatus3User_get_targetChannel(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetChannel(This,app_id,value) ) 


#define IPolicyStatus3User_get_forceInstallApps(This,is_machine,value)	\
    ( (This)->lpVtbl -> get_forceInstallApps(This,is_machine,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatus3User_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatus4_INTERFACE_DEFINED__
#define __IPolicyStatus4_INTERFACE_DEFINED__

/* interface IPolicyStatus4 */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatus4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C07BC046-32E0-4184-BC9F-13C4533C24AC")
    IPolicyStatus4 : public IPolicyStatus3
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_cloudPolicyOverridesPlatformPolicy( 
            /* [retval][out] */ IPolicyStatusValue **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatus4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatus4 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatus4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatus4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatus4 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatus4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatus4 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatus4 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_updaterVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updaterVersion )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_lastCheckedTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckedTime )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ DATE *last_checked);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, refreshPolicies)
        HRESULT ( STDMETHODCALLTYPE *refreshPolicies )( 
            IPolicyStatus4 * This);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_lastCheckPeriodMinutes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckPeriodMinutes )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_updatesSuppressedTimes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updatesSuppressedTimes )( 
            IPolicyStatus4 * This,
            /* [out] */ IPolicyStatusValue **value,
            VARIANT_BOOL *are_updates_suppressed);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_downloadPreferenceGroupPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadPreferenceGroupPolicy )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_packageCacheSizeLimitMBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheSizeLimitMBytes )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_packageCacheExpirationTimeDays)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheExpirationTimeDays )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyMode )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyPacUrl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyPacUrl )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_proxyServer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyServer )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_effectivePolicyForAppInstalls)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppInstalls )( 
            IPolicyStatus4 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_effectivePolicyForAppUpdates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppUpdates )( 
            IPolicyStatus4 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_targetVersionPrefix)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetVersionPrefix )( 
            IPolicyStatus4 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_isRollbackToTargetVersionAllowed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRollbackToTargetVersionAllowed )( 
            IPolicyStatus4 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2, get_targetChannel)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetChannel )( 
            IPolicyStatus4 * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus3, get_forceInstallApps)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_forceInstallApps )( 
            IPolicyStatus4 * This,
            /* [in] */ VARIANT_BOOL is_machine,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus4, get_cloudPolicyOverridesPlatformPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_cloudPolicyOverridesPlatformPolicy )( 
            IPolicyStatus4 * This,
            /* [retval][out] */ IPolicyStatusValue **value);
        
        END_INTERFACE
    } IPolicyStatus4Vtbl;

    interface IPolicyStatus4
    {
        CONST_VTBL struct IPolicyStatus4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatus4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatus4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatus4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatus4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatus4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatus4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatus4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatus4_get_updaterVersion(This,version)	\
    ( (This)->lpVtbl -> get_updaterVersion(This,version) ) 

#define IPolicyStatus4_get_lastCheckedTime(This,last_checked)	\
    ( (This)->lpVtbl -> get_lastCheckedTime(This,last_checked) ) 

#define IPolicyStatus4_refreshPolicies(This)	\
    ( (This)->lpVtbl -> refreshPolicies(This) ) 

#define IPolicyStatus4_get_lastCheckPeriodMinutes(This,value)	\
    ( (This)->lpVtbl -> get_lastCheckPeriodMinutes(This,value) ) 

#define IPolicyStatus4_get_updatesSuppressedTimes(This,value,are_updates_suppressed)	\
    ( (This)->lpVtbl -> get_updatesSuppressedTimes(This,value,are_updates_suppressed) ) 

#define IPolicyStatus4_get_downloadPreferenceGroupPolicy(This,value)	\
    ( (This)->lpVtbl -> get_downloadPreferenceGroupPolicy(This,value) ) 

#define IPolicyStatus4_get_packageCacheSizeLimitMBytes(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheSizeLimitMBytes(This,value) ) 

#define IPolicyStatus4_get_packageCacheExpirationTimeDays(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheExpirationTimeDays(This,value) ) 

#define IPolicyStatus4_get_proxyMode(This,value)	\
    ( (This)->lpVtbl -> get_proxyMode(This,value) ) 

#define IPolicyStatus4_get_proxyPacUrl(This,value)	\
    ( (This)->lpVtbl -> get_proxyPacUrl(This,value) ) 

#define IPolicyStatus4_get_proxyServer(This,value)	\
    ( (This)->lpVtbl -> get_proxyServer(This,value) ) 

#define IPolicyStatus4_get_effectivePolicyForAppInstalls(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppInstalls(This,app_id,value) ) 

#define IPolicyStatus4_get_effectivePolicyForAppUpdates(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppUpdates(This,app_id,value) ) 

#define IPolicyStatus4_get_targetVersionPrefix(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetVersionPrefix(This,app_id,value) ) 

#define IPolicyStatus4_get_isRollbackToTargetVersionAllowed(This,app_id,value)	\
    ( (This)->lpVtbl -> get_isRollbackToTargetVersionAllowed(This,app_id,value) ) 

#define IPolicyStatus4_get_targetChannel(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetChannel(This,app_id,value) ) 


#define IPolicyStatus4_get_forceInstallApps(This,is_machine,value)	\
    ( (This)->lpVtbl -> get_forceInstallApps(This,is_machine,value) ) 


#define IPolicyStatus4_get_cloudPolicyOverridesPlatformPolicy(This,value)	\
    ( (This)->lpVtbl -> get_cloudPolicyOverridesPlatformPolicy(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatus4_INTERFACE_DEFINED__ */


#ifndef __IPolicyStatus4User_INTERFACE_DEFINED__
#define __IPolicyStatus4User_INTERFACE_DEFINED__

/* interface IPolicyStatus4User */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IPolicyStatus4User;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0F6696F3-7F48-446B-97FA-6B34EC2ADB32")
    IPolicyStatus4User : public IPolicyStatus3User
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_cloudPolicyOverridesPlatformPolicy( 
            /* [retval][out] */ IPolicyStatusValueUser **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPolicyStatus4UserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPolicyStatus4User * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPolicyStatus4User * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPolicyStatus4User * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPolicyStatus4User * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPolicyStatus4User * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPolicyStatus4User * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPolicyStatus4User * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_updaterVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updaterVersion )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_lastCheckedTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckedTime )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ DATE *last_checked);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, refreshPolicies)
        HRESULT ( STDMETHODCALLTYPE *refreshPolicies )( 
            IPolicyStatus4User * This);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_lastCheckPeriodMinutes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastCheckPeriodMinutes )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_updatesSuppressedTimes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_updatesSuppressedTimes )( 
            IPolicyStatus4User * This,
            /* [out] */ IPolicyStatusValueUser **value,
            VARIANT_BOOL *are_updates_suppressed);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_downloadPreferenceGroupPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadPreferenceGroupPolicy )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_packageCacheSizeLimitMBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheSizeLimitMBytes )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_packageCacheExpirationTimeDays)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_packageCacheExpirationTimeDays )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyMode )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyPacUrl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyPacUrl )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_proxyServer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_proxyServer )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_effectivePolicyForAppInstalls)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppInstalls )( 
            IPolicyStatus4User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_effectivePolicyForAppUpdates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_effectivePolicyForAppUpdates )( 
            IPolicyStatus4User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_targetVersionPrefix)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetVersionPrefix )( 
            IPolicyStatus4User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_isRollbackToTargetVersionAllowed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRollbackToTargetVersionAllowed )( 
            IPolicyStatus4User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus2User, get_targetChannel)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targetChannel )( 
            IPolicyStatus4User * This,
            /* [in] */ BSTR app_id,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus3User, get_forceInstallApps)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_forceInstallApps )( 
            IPolicyStatus4User * This,
            /* [in] */ VARIANT_BOOL is_machine,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        DECLSPEC_XFGVIRT(IPolicyStatus4User, get_cloudPolicyOverridesPlatformPolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_cloudPolicyOverridesPlatformPolicy )( 
            IPolicyStatus4User * This,
            /* [retval][out] */ IPolicyStatusValueUser **value);
        
        END_INTERFACE
    } IPolicyStatus4UserVtbl;

    interface IPolicyStatus4User
    {
        CONST_VTBL struct IPolicyStatus4UserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPolicyStatus4User_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPolicyStatus4User_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPolicyStatus4User_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPolicyStatus4User_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPolicyStatus4User_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPolicyStatus4User_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPolicyStatus4User_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPolicyStatus4User_get_updaterVersion(This,version)	\
    ( (This)->lpVtbl -> get_updaterVersion(This,version) ) 

#define IPolicyStatus4User_get_lastCheckedTime(This,last_checked)	\
    ( (This)->lpVtbl -> get_lastCheckedTime(This,last_checked) ) 

#define IPolicyStatus4User_refreshPolicies(This)	\
    ( (This)->lpVtbl -> refreshPolicies(This) ) 

#define IPolicyStatus4User_get_lastCheckPeriodMinutes(This,value)	\
    ( (This)->lpVtbl -> get_lastCheckPeriodMinutes(This,value) ) 

#define IPolicyStatus4User_get_updatesSuppressedTimes(This,value,are_updates_suppressed)	\
    ( (This)->lpVtbl -> get_updatesSuppressedTimes(This,value,are_updates_suppressed) ) 

#define IPolicyStatus4User_get_downloadPreferenceGroupPolicy(This,value)	\
    ( (This)->lpVtbl -> get_downloadPreferenceGroupPolicy(This,value) ) 

#define IPolicyStatus4User_get_packageCacheSizeLimitMBytes(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheSizeLimitMBytes(This,value) ) 

#define IPolicyStatus4User_get_packageCacheExpirationTimeDays(This,value)	\
    ( (This)->lpVtbl -> get_packageCacheExpirationTimeDays(This,value) ) 

#define IPolicyStatus4User_get_proxyMode(This,value)	\
    ( (This)->lpVtbl -> get_proxyMode(This,value) ) 

#define IPolicyStatus4User_get_proxyPacUrl(This,value)	\
    ( (This)->lpVtbl -> get_proxyPacUrl(This,value) ) 

#define IPolicyStatus4User_get_proxyServer(This,value)	\
    ( (This)->lpVtbl -> get_proxyServer(This,value) ) 

#define IPolicyStatus4User_get_effectivePolicyForAppInstalls(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppInstalls(This,app_id,value) ) 

#define IPolicyStatus4User_get_effectivePolicyForAppUpdates(This,app_id,value)	\
    ( (This)->lpVtbl -> get_effectivePolicyForAppUpdates(This,app_id,value) ) 

#define IPolicyStatus4User_get_targetVersionPrefix(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetVersionPrefix(This,app_id,value) ) 

#define IPolicyStatus4User_get_isRollbackToTargetVersionAllowed(This,app_id,value)	\
    ( (This)->lpVtbl -> get_isRollbackToTargetVersionAllowed(This,app_id,value) ) 

#define IPolicyStatus4User_get_targetChannel(This,app_id,value)	\
    ( (This)->lpVtbl -> get_targetChannel(This,app_id,value) ) 


#define IPolicyStatus4User_get_forceInstallApps(This,is_machine,value)	\
    ( (This)->lpVtbl -> get_forceInstallApps(This,is_machine,value) ) 


#define IPolicyStatus4User_get_cloudPolicyOverridesPlatformPolicy(This,value)	\
    ( (This)->lpVtbl -> get_cloudPolicyOverridesPlatformPolicy(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPolicyStatus4User_INTERFACE_DEFINED__ */



#ifndef __UpdaterLegacyLib_LIBRARY_DEFINED__
#define __UpdaterLegacyLib_LIBRARY_DEFINED__

/* library UpdaterLegacyLib */
/* [helpstring][version][uuid] */ 
























EXTERN_C const IID LIBID_UpdaterLegacyLib;

EXTERN_C const CLSID CLSID_GoogleUpdate3WebUserClass;

#ifdef __cplusplus

class DECLSPEC_UUID("A0FEB7CB-E0D8-4035-A4C9-5620A8C725AD")
GoogleUpdate3WebUserClass;
#endif

EXTERN_C const CLSID CLSID_GoogleUpdate3WebSystemClass;

#ifdef __cplusplus

class DECLSPEC_UUID("FAC5C548-84EC-474C-A4B3-CD414E09B14C")
GoogleUpdate3WebSystemClass;
#endif

EXTERN_C const CLSID CLSID_GoogleUpdate3WebServiceClass;

#ifdef __cplusplus

class DECLSPEC_UUID("687DCE9A-57BE-4026-BEC4-C0A9ACBBCAF2")
GoogleUpdate3WebServiceClass;
#endif

EXTERN_C const CLSID CLSID_PolicyStatusUserClass;

#ifdef __cplusplus

class DECLSPEC_UUID("E432DCFE-6A32-4C07-B038-9D74AC80D6AB")
PolicyStatusUserClass;
#endif

EXTERN_C const CLSID CLSID_PolicyStatusSystemClass;

#ifdef __cplusplus

class DECLSPEC_UUID("F675D224-BD54-40E9-AECB-AA3B64EB9863")
PolicyStatusSystemClass;
#endif

EXTERN_C const CLSID CLSID_ProcessLauncherClass;

#ifdef __cplusplus

class DECLSPEC_UUID("CEC2877D-4856-460E-BE73-11DD7CC7C821")
ProcessLauncherClass;
#endif
#endif /* __UpdaterLegacyLib_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long *, unsigned long            , BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  unsigned long *, unsigned char *, BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(unsigned long *, unsigned char *, BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long *, BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     unsigned long *, unsigned long            , VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  unsigned long *, unsigned char *, VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(unsigned long *, unsigned char *, VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     unsigned long *, VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


