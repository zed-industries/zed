

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for gen/chrome/updater/app/server/win/updater_idl.idl:
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

#ifndef __updater_idl_h__
#define __updater_idl_h__

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

#ifndef __IUpdateState_FWD_DEFINED__
#define __IUpdateState_FWD_DEFINED__
typedef interface IUpdateState IUpdateState;

#endif 	/* __IUpdateState_FWD_DEFINED__ */


#ifndef __IUpdateStateUser_FWD_DEFINED__
#define __IUpdateStateUser_FWD_DEFINED__
typedef interface IUpdateStateUser IUpdateStateUser;

#endif 	/* __IUpdateStateUser_FWD_DEFINED__ */


#ifndef __IUpdateStateSystem_FWD_DEFINED__
#define __IUpdateStateSystem_FWD_DEFINED__
typedef interface IUpdateStateSystem IUpdateStateSystem;

#endif 	/* __IUpdateStateSystem_FWD_DEFINED__ */


#ifndef __ICompleteStatus_FWD_DEFINED__
#define __ICompleteStatus_FWD_DEFINED__
typedef interface ICompleteStatus ICompleteStatus;

#endif 	/* __ICompleteStatus_FWD_DEFINED__ */


#ifndef __ICompleteStatusUser_FWD_DEFINED__
#define __ICompleteStatusUser_FWD_DEFINED__
typedef interface ICompleteStatusUser ICompleteStatusUser;

#endif 	/* __ICompleteStatusUser_FWD_DEFINED__ */


#ifndef __ICompleteStatusSystem_FWD_DEFINED__
#define __ICompleteStatusSystem_FWD_DEFINED__
typedef interface ICompleteStatusSystem ICompleteStatusSystem;

#endif 	/* __ICompleteStatusSystem_FWD_DEFINED__ */


#ifndef __IUpdaterObserver_FWD_DEFINED__
#define __IUpdaterObserver_FWD_DEFINED__
typedef interface IUpdaterObserver IUpdaterObserver;

#endif 	/* __IUpdaterObserver_FWD_DEFINED__ */


#ifndef __IUpdaterObserverUser_FWD_DEFINED__
#define __IUpdaterObserverUser_FWD_DEFINED__
typedef interface IUpdaterObserverUser IUpdaterObserverUser;

#endif 	/* __IUpdaterObserverUser_FWD_DEFINED__ */


#ifndef __IUpdaterObserverSystem_FWD_DEFINED__
#define __IUpdaterObserverSystem_FWD_DEFINED__
typedef interface IUpdaterObserverSystem IUpdaterObserverSystem;

#endif 	/* __IUpdaterObserverSystem_FWD_DEFINED__ */


#ifndef __IUpdaterCallback_FWD_DEFINED__
#define __IUpdaterCallback_FWD_DEFINED__
typedef interface IUpdaterCallback IUpdaterCallback;

#endif 	/* __IUpdaterCallback_FWD_DEFINED__ */


#ifndef __IUpdaterCallbackUser_FWD_DEFINED__
#define __IUpdaterCallbackUser_FWD_DEFINED__
typedef interface IUpdaterCallbackUser IUpdaterCallbackUser;

#endif 	/* __IUpdaterCallbackUser_FWD_DEFINED__ */


#ifndef __IUpdaterCallbackSystem_FWD_DEFINED__
#define __IUpdaterCallbackSystem_FWD_DEFINED__
typedef interface IUpdaterCallbackSystem IUpdaterCallbackSystem;

#endif 	/* __IUpdaterCallbackSystem_FWD_DEFINED__ */


#ifndef __IUpdaterAppState_FWD_DEFINED__
#define __IUpdaterAppState_FWD_DEFINED__
typedef interface IUpdaterAppState IUpdaterAppState;

#endif 	/* __IUpdaterAppState_FWD_DEFINED__ */


#ifndef __IUpdaterAppStateUser_FWD_DEFINED__
#define __IUpdaterAppStateUser_FWD_DEFINED__
typedef interface IUpdaterAppStateUser IUpdaterAppStateUser;

#endif 	/* __IUpdaterAppStateUser_FWD_DEFINED__ */


#ifndef __IUpdaterAppStateSystem_FWD_DEFINED__
#define __IUpdaterAppStateSystem_FWD_DEFINED__
typedef interface IUpdaterAppStateSystem IUpdaterAppStateSystem;

#endif 	/* __IUpdaterAppStateSystem_FWD_DEFINED__ */


#ifndef __IUpdaterAppStatesCallback_FWD_DEFINED__
#define __IUpdaterAppStatesCallback_FWD_DEFINED__
typedef interface IUpdaterAppStatesCallback IUpdaterAppStatesCallback;

#endif 	/* __IUpdaterAppStatesCallback_FWD_DEFINED__ */


#ifndef __IUpdaterAppStatesCallbackUser_FWD_DEFINED__
#define __IUpdaterAppStatesCallbackUser_FWD_DEFINED__
typedef interface IUpdaterAppStatesCallbackUser IUpdaterAppStatesCallbackUser;

#endif 	/* __IUpdaterAppStatesCallbackUser_FWD_DEFINED__ */


#ifndef __IUpdaterAppStatesCallbackSystem_FWD_DEFINED__
#define __IUpdaterAppStatesCallbackSystem_FWD_DEFINED__
typedef interface IUpdaterAppStatesCallbackSystem IUpdaterAppStatesCallbackSystem;

#endif 	/* __IUpdaterAppStatesCallbackSystem_FWD_DEFINED__ */


#ifndef __IUpdater_FWD_DEFINED__
#define __IUpdater_FWD_DEFINED__
typedef interface IUpdater IUpdater;

#endif 	/* __IUpdater_FWD_DEFINED__ */


#ifndef __IUpdaterUser_FWD_DEFINED__
#define __IUpdaterUser_FWD_DEFINED__
typedef interface IUpdaterUser IUpdaterUser;

#endif 	/* __IUpdaterUser_FWD_DEFINED__ */


#ifndef __IUpdaterSystem_FWD_DEFINED__
#define __IUpdaterSystem_FWD_DEFINED__
typedef interface IUpdaterSystem IUpdaterSystem;

#endif 	/* __IUpdaterSystem_FWD_DEFINED__ */


#ifndef __IUpdater2_FWD_DEFINED__
#define __IUpdater2_FWD_DEFINED__
typedef interface IUpdater2 IUpdater2;

#endif 	/* __IUpdater2_FWD_DEFINED__ */


#ifndef __IUpdater2User_FWD_DEFINED__
#define __IUpdater2User_FWD_DEFINED__
typedef interface IUpdater2User IUpdater2User;

#endif 	/* __IUpdater2User_FWD_DEFINED__ */


#ifndef __IUpdater2System_FWD_DEFINED__
#define __IUpdater2System_FWD_DEFINED__
typedef interface IUpdater2System IUpdater2System;

#endif 	/* __IUpdater2System_FWD_DEFINED__ */


#ifndef __UpdaterUserClass_FWD_DEFINED__
#define __UpdaterUserClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class UpdaterUserClass UpdaterUserClass;
#else
typedef struct UpdaterUserClass UpdaterUserClass;
#endif /* __cplusplus */

#endif 	/* __UpdaterUserClass_FWD_DEFINED__ */


#ifndef __UpdaterSystemClass_FWD_DEFINED__
#define __UpdaterSystemClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class UpdaterSystemClass UpdaterSystemClass;
#else
typedef struct UpdaterSystemClass UpdaterSystemClass;
#endif /* __cplusplus */

#endif 	/* __UpdaterSystemClass_FWD_DEFINED__ */


#ifndef __IUpdateStateUser_FWD_DEFINED__
#define __IUpdateStateUser_FWD_DEFINED__
typedef interface IUpdateStateUser IUpdateStateUser;

#endif 	/* __IUpdateStateUser_FWD_DEFINED__ */


#ifndef __IUpdateStateSystem_FWD_DEFINED__
#define __IUpdateStateSystem_FWD_DEFINED__
typedef interface IUpdateStateSystem IUpdateStateSystem;

#endif 	/* __IUpdateStateSystem_FWD_DEFINED__ */


#ifndef __ICompleteStatusUser_FWD_DEFINED__
#define __ICompleteStatusUser_FWD_DEFINED__
typedef interface ICompleteStatusUser ICompleteStatusUser;

#endif 	/* __ICompleteStatusUser_FWD_DEFINED__ */


#ifndef __ICompleteStatusSystem_FWD_DEFINED__
#define __ICompleteStatusSystem_FWD_DEFINED__
typedef interface ICompleteStatusSystem ICompleteStatusSystem;

#endif 	/* __ICompleteStatusSystem_FWD_DEFINED__ */


#ifndef __IUpdaterObserverUser_FWD_DEFINED__
#define __IUpdaterObserverUser_FWD_DEFINED__
typedef interface IUpdaterObserverUser IUpdaterObserverUser;

#endif 	/* __IUpdaterObserverUser_FWD_DEFINED__ */


#ifndef __IUpdaterObserverSystem_FWD_DEFINED__
#define __IUpdaterObserverSystem_FWD_DEFINED__
typedef interface IUpdaterObserverSystem IUpdaterObserverSystem;

#endif 	/* __IUpdaterObserverSystem_FWD_DEFINED__ */


#ifndef __IUpdaterCallbackUser_FWD_DEFINED__
#define __IUpdaterCallbackUser_FWD_DEFINED__
typedef interface IUpdaterCallbackUser IUpdaterCallbackUser;

#endif 	/* __IUpdaterCallbackUser_FWD_DEFINED__ */


#ifndef __IUpdaterCallbackSystem_FWD_DEFINED__
#define __IUpdaterCallbackSystem_FWD_DEFINED__
typedef interface IUpdaterCallbackSystem IUpdaterCallbackSystem;

#endif 	/* __IUpdaterCallbackSystem_FWD_DEFINED__ */


#ifndef __IUpdaterAppState_FWD_DEFINED__
#define __IUpdaterAppState_FWD_DEFINED__
typedef interface IUpdaterAppState IUpdaterAppState;

#endif 	/* __IUpdaterAppState_FWD_DEFINED__ */


#ifndef __IUpdaterAppStateUser_FWD_DEFINED__
#define __IUpdaterAppStateUser_FWD_DEFINED__
typedef interface IUpdaterAppStateUser IUpdaterAppStateUser;

#endif 	/* __IUpdaterAppStateUser_FWD_DEFINED__ */


#ifndef __IUpdaterAppStateSystem_FWD_DEFINED__
#define __IUpdaterAppStateSystem_FWD_DEFINED__
typedef interface IUpdaterAppStateSystem IUpdaterAppStateSystem;

#endif 	/* __IUpdaterAppStateSystem_FWD_DEFINED__ */


#ifndef __IUpdaterAppStatesCallbackUser_FWD_DEFINED__
#define __IUpdaterAppStatesCallbackUser_FWD_DEFINED__
typedef interface IUpdaterAppStatesCallbackUser IUpdaterAppStatesCallbackUser;

#endif 	/* __IUpdaterAppStatesCallbackUser_FWD_DEFINED__ */


#ifndef __IUpdaterAppStatesCallbackSystem_FWD_DEFINED__
#define __IUpdaterAppStatesCallbackSystem_FWD_DEFINED__
typedef interface IUpdaterAppStatesCallbackSystem IUpdaterAppStatesCallbackSystem;

#endif 	/* __IUpdaterAppStatesCallbackSystem_FWD_DEFINED__ */


#ifndef __IUpdaterUser_FWD_DEFINED__
#define __IUpdaterUser_FWD_DEFINED__
typedef interface IUpdaterUser IUpdaterUser;

#endif 	/* __IUpdaterUser_FWD_DEFINED__ */


#ifndef __IUpdaterSystem_FWD_DEFINED__
#define __IUpdaterSystem_FWD_DEFINED__
typedef interface IUpdaterSystem IUpdaterSystem;

#endif 	/* __IUpdaterSystem_FWD_DEFINED__ */


#ifndef __IUpdater2User_FWD_DEFINED__
#define __IUpdater2User_FWD_DEFINED__
typedef interface IUpdater2User IUpdater2User;

#endif 	/* __IUpdater2User_FWD_DEFINED__ */


#ifndef __IUpdater2System_FWD_DEFINED__
#define __IUpdater2System_FWD_DEFINED__
typedef interface IUpdater2System IUpdater2System;

#endif 	/* __IUpdater2System_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


#ifndef __IUpdateState_INTERFACE_DEFINED__
#define __IUpdateState_INTERFACE_DEFINED__

/* interface IUpdateState */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdateState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("46ACF70B-AC13-406D-B53B-B2C4BF091FF6")
    IUpdateState : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_state( 
            /* [retval][out] */ LONG *__MIDL__IUpdateState0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appId( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateState0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nextVersion( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateState0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadedBytes( 
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateState0003) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_totalBytes( 
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateState0004) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installProgress( 
            /* [retval][out] */ LONG *__MIDL__IUpdateState0005) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_errorCategory( 
            /* [retval][out] */ LONG *__MIDL__IUpdateState0006) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_errorCode( 
            /* [retval][out] */ LONG *__MIDL__IUpdateState0007) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_extraCode1( 
            /* [retval][out] */ LONG *__MIDL__IUpdateState0008) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerText( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateState0009) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerCommandLine( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateState0010) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdateStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdateState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdateState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdateState * This);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_state)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_state )( 
            IUpdateState * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateState0000);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_appId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appId )( 
            IUpdateState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateState0001);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_nextVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextVersion )( 
            IUpdateState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateState0002);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_downloadedBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadedBytes )( 
            IUpdateState * This,
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateState0003);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_totalBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_totalBytes )( 
            IUpdateState * This,
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateState0004);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_installProgress)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installProgress )( 
            IUpdateState * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateState0005);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_errorCategory)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCategory )( 
            IUpdateState * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateState0006);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_errorCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCode )( 
            IUpdateState * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateState0007);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_extraCode1)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extraCode1 )( 
            IUpdateState * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateState0008);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_installerText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerText )( 
            IUpdateState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateState0009);
        
        DECLSPEC_XFGVIRT(IUpdateState, get_installerCommandLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerCommandLine )( 
            IUpdateState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateState0010);
        
        END_INTERFACE
    } IUpdateStateVtbl;

    interface IUpdateState
    {
        CONST_VTBL struct IUpdateStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdateState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdateState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdateState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdateState_get_state(This,__MIDL__IUpdateState0000)	\
    ( (This)->lpVtbl -> get_state(This,__MIDL__IUpdateState0000) ) 

#define IUpdateState_get_appId(This,__MIDL__IUpdateState0001)	\
    ( (This)->lpVtbl -> get_appId(This,__MIDL__IUpdateState0001) ) 

#define IUpdateState_get_nextVersion(This,__MIDL__IUpdateState0002)	\
    ( (This)->lpVtbl -> get_nextVersion(This,__MIDL__IUpdateState0002) ) 

#define IUpdateState_get_downloadedBytes(This,__MIDL__IUpdateState0003)	\
    ( (This)->lpVtbl -> get_downloadedBytes(This,__MIDL__IUpdateState0003) ) 

#define IUpdateState_get_totalBytes(This,__MIDL__IUpdateState0004)	\
    ( (This)->lpVtbl -> get_totalBytes(This,__MIDL__IUpdateState0004) ) 

#define IUpdateState_get_installProgress(This,__MIDL__IUpdateState0005)	\
    ( (This)->lpVtbl -> get_installProgress(This,__MIDL__IUpdateState0005) ) 

#define IUpdateState_get_errorCategory(This,__MIDL__IUpdateState0006)	\
    ( (This)->lpVtbl -> get_errorCategory(This,__MIDL__IUpdateState0006) ) 

#define IUpdateState_get_errorCode(This,__MIDL__IUpdateState0007)	\
    ( (This)->lpVtbl -> get_errorCode(This,__MIDL__IUpdateState0007) ) 

#define IUpdateState_get_extraCode1(This,__MIDL__IUpdateState0008)	\
    ( (This)->lpVtbl -> get_extraCode1(This,__MIDL__IUpdateState0008) ) 

#define IUpdateState_get_installerText(This,__MIDL__IUpdateState0009)	\
    ( (This)->lpVtbl -> get_installerText(This,__MIDL__IUpdateState0009) ) 

#define IUpdateState_get_installerCommandLine(This,__MIDL__IUpdateState0010)	\
    ( (This)->lpVtbl -> get_installerCommandLine(This,__MIDL__IUpdateState0010) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdateState_INTERFACE_DEFINED__ */


#ifndef __IUpdateStateUser_INTERFACE_DEFINED__
#define __IUpdateStateUser_INTERFACE_DEFINED__

/* interface IUpdateStateUser */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdateStateUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C3485D9F-C684-4C43-B85B-E339EA395C29")
    IUpdateStateUser : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_state( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appId( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateUser0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nextVersion( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateUser0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadedBytes( 
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateStateUser0003) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_totalBytes( 
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateStateUser0004) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installProgress( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0005) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_errorCategory( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0006) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_errorCode( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0007) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_extraCode1( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0008) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerText( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateUser0009) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerCommandLine( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateUser0010) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdateStateUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdateStateUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdateStateUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdateStateUser * This);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_state)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_state )( 
            IUpdateStateUser * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0000);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_appId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appId )( 
            IUpdateStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateUser0001);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_nextVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextVersion )( 
            IUpdateStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateUser0002);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_downloadedBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadedBytes )( 
            IUpdateStateUser * This,
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateStateUser0003);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_totalBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_totalBytes )( 
            IUpdateStateUser * This,
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateStateUser0004);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_installProgress)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installProgress )( 
            IUpdateStateUser * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0005);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_errorCategory)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCategory )( 
            IUpdateStateUser * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0006);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_errorCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCode )( 
            IUpdateStateUser * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0007);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_extraCode1)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extraCode1 )( 
            IUpdateStateUser * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateUser0008);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_installerText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerText )( 
            IUpdateStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateUser0009);
        
        DECLSPEC_XFGVIRT(IUpdateStateUser, get_installerCommandLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerCommandLine )( 
            IUpdateStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateUser0010);
        
        END_INTERFACE
    } IUpdateStateUserVtbl;

    interface IUpdateStateUser
    {
        CONST_VTBL struct IUpdateStateUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdateStateUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdateStateUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdateStateUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdateStateUser_get_state(This,__MIDL__IUpdateStateUser0000)	\
    ( (This)->lpVtbl -> get_state(This,__MIDL__IUpdateStateUser0000) ) 

#define IUpdateStateUser_get_appId(This,__MIDL__IUpdateStateUser0001)	\
    ( (This)->lpVtbl -> get_appId(This,__MIDL__IUpdateStateUser0001) ) 

#define IUpdateStateUser_get_nextVersion(This,__MIDL__IUpdateStateUser0002)	\
    ( (This)->lpVtbl -> get_nextVersion(This,__MIDL__IUpdateStateUser0002) ) 

#define IUpdateStateUser_get_downloadedBytes(This,__MIDL__IUpdateStateUser0003)	\
    ( (This)->lpVtbl -> get_downloadedBytes(This,__MIDL__IUpdateStateUser0003) ) 

#define IUpdateStateUser_get_totalBytes(This,__MIDL__IUpdateStateUser0004)	\
    ( (This)->lpVtbl -> get_totalBytes(This,__MIDL__IUpdateStateUser0004) ) 

#define IUpdateStateUser_get_installProgress(This,__MIDL__IUpdateStateUser0005)	\
    ( (This)->lpVtbl -> get_installProgress(This,__MIDL__IUpdateStateUser0005) ) 

#define IUpdateStateUser_get_errorCategory(This,__MIDL__IUpdateStateUser0006)	\
    ( (This)->lpVtbl -> get_errorCategory(This,__MIDL__IUpdateStateUser0006) ) 

#define IUpdateStateUser_get_errorCode(This,__MIDL__IUpdateStateUser0007)	\
    ( (This)->lpVtbl -> get_errorCode(This,__MIDL__IUpdateStateUser0007) ) 

#define IUpdateStateUser_get_extraCode1(This,__MIDL__IUpdateStateUser0008)	\
    ( (This)->lpVtbl -> get_extraCode1(This,__MIDL__IUpdateStateUser0008) ) 

#define IUpdateStateUser_get_installerText(This,__MIDL__IUpdateStateUser0009)	\
    ( (This)->lpVtbl -> get_installerText(This,__MIDL__IUpdateStateUser0009) ) 

#define IUpdateStateUser_get_installerCommandLine(This,__MIDL__IUpdateStateUser0010)	\
    ( (This)->lpVtbl -> get_installerCommandLine(This,__MIDL__IUpdateStateUser0010) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdateStateUser_INTERFACE_DEFINED__ */


#ifndef __IUpdateStateSystem_INTERFACE_DEFINED__
#define __IUpdateStateSystem_INTERFACE_DEFINED__

/* interface IUpdateStateSystem */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdateStateSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EA6FDC05-CDC5-4EA4-AB41-CCBD1040A2B5")
    IUpdateStateSystem : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_state( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appId( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateSystem0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nextVersion( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateSystem0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_downloadedBytes( 
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateStateSystem0003) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_totalBytes( 
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateStateSystem0004) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installProgress( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0005) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_errorCategory( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0006) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_errorCode( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0007) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_extraCode1( 
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0008) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerText( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateSystem0009) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_installerCommandLine( 
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateSystem0010) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdateStateSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdateStateSystem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdateStateSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdateStateSystem * This);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_state)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_state )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0000);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_appId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appId )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateSystem0001);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_nextVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextVersion )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateSystem0002);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_downloadedBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_downloadedBytes )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateStateSystem0003);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_totalBytes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_totalBytes )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ LONGLONG *__MIDL__IUpdateStateSystem0004);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_installProgress)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installProgress )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0005);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_errorCategory)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCategory )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0006);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_errorCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCode )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0007);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_extraCode1)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extraCode1 )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ LONG *__MIDL__IUpdateStateSystem0008);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_installerText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerText )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateSystem0009);
        
        DECLSPEC_XFGVIRT(IUpdateStateSystem, get_installerCommandLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_installerCommandLine )( 
            IUpdateStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdateStateSystem0010);
        
        END_INTERFACE
    } IUpdateStateSystemVtbl;

    interface IUpdateStateSystem
    {
        CONST_VTBL struct IUpdateStateSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdateStateSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdateStateSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdateStateSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdateStateSystem_get_state(This,__MIDL__IUpdateStateSystem0000)	\
    ( (This)->lpVtbl -> get_state(This,__MIDL__IUpdateStateSystem0000) ) 

#define IUpdateStateSystem_get_appId(This,__MIDL__IUpdateStateSystem0001)	\
    ( (This)->lpVtbl -> get_appId(This,__MIDL__IUpdateStateSystem0001) ) 

#define IUpdateStateSystem_get_nextVersion(This,__MIDL__IUpdateStateSystem0002)	\
    ( (This)->lpVtbl -> get_nextVersion(This,__MIDL__IUpdateStateSystem0002) ) 

#define IUpdateStateSystem_get_downloadedBytes(This,__MIDL__IUpdateStateSystem0003)	\
    ( (This)->lpVtbl -> get_downloadedBytes(This,__MIDL__IUpdateStateSystem0003) ) 

#define IUpdateStateSystem_get_totalBytes(This,__MIDL__IUpdateStateSystem0004)	\
    ( (This)->lpVtbl -> get_totalBytes(This,__MIDL__IUpdateStateSystem0004) ) 

#define IUpdateStateSystem_get_installProgress(This,__MIDL__IUpdateStateSystem0005)	\
    ( (This)->lpVtbl -> get_installProgress(This,__MIDL__IUpdateStateSystem0005) ) 

#define IUpdateStateSystem_get_errorCategory(This,__MIDL__IUpdateStateSystem0006)	\
    ( (This)->lpVtbl -> get_errorCategory(This,__MIDL__IUpdateStateSystem0006) ) 

#define IUpdateStateSystem_get_errorCode(This,__MIDL__IUpdateStateSystem0007)	\
    ( (This)->lpVtbl -> get_errorCode(This,__MIDL__IUpdateStateSystem0007) ) 

#define IUpdateStateSystem_get_extraCode1(This,__MIDL__IUpdateStateSystem0008)	\
    ( (This)->lpVtbl -> get_extraCode1(This,__MIDL__IUpdateStateSystem0008) ) 

#define IUpdateStateSystem_get_installerText(This,__MIDL__IUpdateStateSystem0009)	\
    ( (This)->lpVtbl -> get_installerText(This,__MIDL__IUpdateStateSystem0009) ) 

#define IUpdateStateSystem_get_installerCommandLine(This,__MIDL__IUpdateStateSystem0010)	\
    ( (This)->lpVtbl -> get_installerCommandLine(This,__MIDL__IUpdateStateSystem0010) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdateStateSystem_INTERFACE_DEFINED__ */


#ifndef __ICompleteStatus_INTERFACE_DEFINED__
#define __ICompleteStatus_INTERFACE_DEFINED__

/* interface ICompleteStatus */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ICompleteStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2FCD14AF-B645-4351-8359-E80A0E202A0B")
    ICompleteStatus : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_statusCode( 
            /* [retval][out] */ LONG *__MIDL__ICompleteStatus0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_statusMessage( 
            /* [retval][out] */ BSTR *__MIDL__ICompleteStatus0001) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICompleteStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICompleteStatus * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICompleteStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICompleteStatus * This);
        
        DECLSPEC_XFGVIRT(ICompleteStatus, get_statusCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusCode )( 
            ICompleteStatus * This,
            /* [retval][out] */ LONG *__MIDL__ICompleteStatus0000);
        
        DECLSPEC_XFGVIRT(ICompleteStatus, get_statusMessage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusMessage )( 
            ICompleteStatus * This,
            /* [retval][out] */ BSTR *__MIDL__ICompleteStatus0001);
        
        END_INTERFACE
    } ICompleteStatusVtbl;

    interface ICompleteStatus
    {
        CONST_VTBL struct ICompleteStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICompleteStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICompleteStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICompleteStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICompleteStatus_get_statusCode(This,__MIDL__ICompleteStatus0000)	\
    ( (This)->lpVtbl -> get_statusCode(This,__MIDL__ICompleteStatus0000) ) 

#define ICompleteStatus_get_statusMessage(This,__MIDL__ICompleteStatus0001)	\
    ( (This)->lpVtbl -> get_statusMessage(This,__MIDL__ICompleteStatus0001) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICompleteStatus_INTERFACE_DEFINED__ */


#ifndef __ICompleteStatusUser_INTERFACE_DEFINED__
#define __ICompleteStatusUser_INTERFACE_DEFINED__

/* interface ICompleteStatusUser */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ICompleteStatusUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9AD1A645-5A4B-4D36-BC21-F0059482E6EA")
    ICompleteStatusUser : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_statusCode( 
            /* [retval][out] */ LONG *__MIDL__ICompleteStatusUser0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_statusMessage( 
            /* [retval][out] */ BSTR *__MIDL__ICompleteStatusUser0001) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICompleteStatusUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICompleteStatusUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICompleteStatusUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICompleteStatusUser * This);
        
        DECLSPEC_XFGVIRT(ICompleteStatusUser, get_statusCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusCode )( 
            ICompleteStatusUser * This,
            /* [retval][out] */ LONG *__MIDL__ICompleteStatusUser0000);
        
        DECLSPEC_XFGVIRT(ICompleteStatusUser, get_statusMessage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusMessage )( 
            ICompleteStatusUser * This,
            /* [retval][out] */ BSTR *__MIDL__ICompleteStatusUser0001);
        
        END_INTERFACE
    } ICompleteStatusUserVtbl;

    interface ICompleteStatusUser
    {
        CONST_VTBL struct ICompleteStatusUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICompleteStatusUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICompleteStatusUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICompleteStatusUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICompleteStatusUser_get_statusCode(This,__MIDL__ICompleteStatusUser0000)	\
    ( (This)->lpVtbl -> get_statusCode(This,__MIDL__ICompleteStatusUser0000) ) 

#define ICompleteStatusUser_get_statusMessage(This,__MIDL__ICompleteStatusUser0001)	\
    ( (This)->lpVtbl -> get_statusMessage(This,__MIDL__ICompleteStatusUser0001) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICompleteStatusUser_INTERFACE_DEFINED__ */


#ifndef __ICompleteStatusSystem_INTERFACE_DEFINED__
#define __ICompleteStatusSystem_INTERFACE_DEFINED__

/* interface ICompleteStatusSystem */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ICompleteStatusSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E2BD9A6B-0A19-4C89-AE8B-B7E9E51D9A07")
    ICompleteStatusSystem : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_statusCode( 
            /* [retval][out] */ LONG *__MIDL__ICompleteStatusSystem0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_statusMessage( 
            /* [retval][out] */ BSTR *__MIDL__ICompleteStatusSystem0001) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICompleteStatusSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICompleteStatusSystem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICompleteStatusSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICompleteStatusSystem * This);
        
        DECLSPEC_XFGVIRT(ICompleteStatusSystem, get_statusCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusCode )( 
            ICompleteStatusSystem * This,
            /* [retval][out] */ LONG *__MIDL__ICompleteStatusSystem0000);
        
        DECLSPEC_XFGVIRT(ICompleteStatusSystem, get_statusMessage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusMessage )( 
            ICompleteStatusSystem * This,
            /* [retval][out] */ BSTR *__MIDL__ICompleteStatusSystem0001);
        
        END_INTERFACE
    } ICompleteStatusSystemVtbl;

    interface ICompleteStatusSystem
    {
        CONST_VTBL struct ICompleteStatusSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICompleteStatusSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICompleteStatusSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICompleteStatusSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICompleteStatusSystem_get_statusCode(This,__MIDL__ICompleteStatusSystem0000)	\
    ( (This)->lpVtbl -> get_statusCode(This,__MIDL__ICompleteStatusSystem0000) ) 

#define ICompleteStatusSystem_get_statusMessage(This,__MIDL__ICompleteStatusSystem0001)	\
    ( (This)->lpVtbl -> get_statusMessage(This,__MIDL__ICompleteStatusSystem0001) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICompleteStatusSystem_INTERFACE_DEFINED__ */


#ifndef __IUpdaterObserver_INTERFACE_DEFINED__
#define __IUpdaterObserver_INTERFACE_DEFINED__

/* interface IUpdaterObserver */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterObserver;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7B416CFD-4216-4FD6-BD83-7C586054676E")
    IUpdaterObserver : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStateChange( 
            /* [in] */ IUpdateState *update_state) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnComplete( 
            /* [in] */ ICompleteStatus *status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterObserverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterObserver * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterObserver * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterObserver * This);
        
        DECLSPEC_XFGVIRT(IUpdaterObserver, OnStateChange)
        HRESULT ( STDMETHODCALLTYPE *OnStateChange )( 
            IUpdaterObserver * This,
            /* [in] */ IUpdateState *update_state);
        
        DECLSPEC_XFGVIRT(IUpdaterObserver, OnComplete)
        HRESULT ( STDMETHODCALLTYPE *OnComplete )( 
            IUpdaterObserver * This,
            /* [in] */ ICompleteStatus *status);
        
        END_INTERFACE
    } IUpdaterObserverVtbl;

    interface IUpdaterObserver
    {
        CONST_VTBL struct IUpdaterObserverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterObserver_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterObserver_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterObserver_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterObserver_OnStateChange(This,update_state)	\
    ( (This)->lpVtbl -> OnStateChange(This,update_state) ) 

#define IUpdaterObserver_OnComplete(This,status)	\
    ( (This)->lpVtbl -> OnComplete(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterObserver_INTERFACE_DEFINED__ */


#ifndef __IUpdaterObserverUser_INTERFACE_DEFINED__
#define __IUpdaterObserverUser_INTERFACE_DEFINED__

/* interface IUpdaterObserverUser */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterObserverUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B54493A0-65B7-408C-B650-06265D2182AC")
    IUpdaterObserverUser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStateChange( 
            /* [in] */ IUpdateStateUser *update_state) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnComplete( 
            /* [in] */ ICompleteStatusUser *status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterObserverUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterObserverUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterObserverUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterObserverUser * This);
        
        DECLSPEC_XFGVIRT(IUpdaterObserverUser, OnStateChange)
        HRESULT ( STDMETHODCALLTYPE *OnStateChange )( 
            IUpdaterObserverUser * This,
            /* [in] */ IUpdateStateUser *update_state);
        
        DECLSPEC_XFGVIRT(IUpdaterObserverUser, OnComplete)
        HRESULT ( STDMETHODCALLTYPE *OnComplete )( 
            IUpdaterObserverUser * This,
            /* [in] */ ICompleteStatusUser *status);
        
        END_INTERFACE
    } IUpdaterObserverUserVtbl;

    interface IUpdaterObserverUser
    {
        CONST_VTBL struct IUpdaterObserverUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterObserverUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterObserverUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterObserverUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterObserverUser_OnStateChange(This,update_state)	\
    ( (This)->lpVtbl -> OnStateChange(This,update_state) ) 

#define IUpdaterObserverUser_OnComplete(This,status)	\
    ( (This)->lpVtbl -> OnComplete(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterObserverUser_INTERFACE_DEFINED__ */


#ifndef __IUpdaterObserverSystem_INTERFACE_DEFINED__
#define __IUpdaterObserverSystem_INTERFACE_DEFINED__

/* interface IUpdaterObserverSystem */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterObserverSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("057B500A-4BA2-496A-B1CD-C5DED3CCC61B")
    IUpdaterObserverSystem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStateChange( 
            /* [in] */ IUpdateStateSystem *update_state) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnComplete( 
            /* [in] */ ICompleteStatusSystem *status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterObserverSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterObserverSystem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterObserverSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterObserverSystem * This);
        
        DECLSPEC_XFGVIRT(IUpdaterObserverSystem, OnStateChange)
        HRESULT ( STDMETHODCALLTYPE *OnStateChange )( 
            IUpdaterObserverSystem * This,
            /* [in] */ IUpdateStateSystem *update_state);
        
        DECLSPEC_XFGVIRT(IUpdaterObserverSystem, OnComplete)
        HRESULT ( STDMETHODCALLTYPE *OnComplete )( 
            IUpdaterObserverSystem * This,
            /* [in] */ ICompleteStatusSystem *status);
        
        END_INTERFACE
    } IUpdaterObserverSystemVtbl;

    interface IUpdaterObserverSystem
    {
        CONST_VTBL struct IUpdaterObserverSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterObserverSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterObserverSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterObserverSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterObserverSystem_OnStateChange(This,update_state)	\
    ( (This)->lpVtbl -> OnStateChange(This,update_state) ) 

#define IUpdaterObserverSystem_OnComplete(This,status)	\
    ( (This)->lpVtbl -> OnComplete(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterObserverSystem_INTERFACE_DEFINED__ */


#ifndef __IUpdaterCallback_INTERFACE_DEFINED__
#define __IUpdaterCallback_INTERFACE_DEFINED__

/* interface IUpdaterCallback */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8BAB6F84-AD67-4819-B846-CC890880FD3B")
    IUpdaterCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ LONG result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterCallback * This);
        
        DECLSPEC_XFGVIRT(IUpdaterCallback, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterCallback * This,
            /* [in] */ LONG result);
        
        END_INTERFACE
    } IUpdaterCallbackVtbl;

    interface IUpdaterCallback
    {
        CONST_VTBL struct IUpdaterCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterCallback_Run(This,result)	\
    ( (This)->lpVtbl -> Run(This,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterCallback_INTERFACE_DEFINED__ */


#ifndef __IUpdaterCallbackUser_INTERFACE_DEFINED__
#define __IUpdaterCallbackUser_INTERFACE_DEFINED__

/* interface IUpdaterCallbackUser */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterCallbackUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34ADC89D-552B-4102-8AE5-D613A691335B")
    IUpdaterCallbackUser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ LONG result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterCallbackUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterCallbackUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterCallbackUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterCallbackUser * This);
        
        DECLSPEC_XFGVIRT(IUpdaterCallbackUser, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterCallbackUser * This,
            /* [in] */ LONG result);
        
        END_INTERFACE
    } IUpdaterCallbackUserVtbl;

    interface IUpdaterCallbackUser
    {
        CONST_VTBL struct IUpdaterCallbackUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterCallbackUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterCallbackUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterCallbackUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterCallbackUser_Run(This,result)	\
    ( (This)->lpVtbl -> Run(This,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterCallbackUser_INTERFACE_DEFINED__ */


#ifndef __IUpdaterCallbackSystem_INTERFACE_DEFINED__
#define __IUpdaterCallbackSystem_INTERFACE_DEFINED__

/* interface IUpdaterCallbackSystem */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterCallbackSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F0D6763A-0182-4136-B1FA-508E334CFFC1")
    IUpdaterCallbackSystem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ LONG result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterCallbackSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterCallbackSystem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterCallbackSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterCallbackSystem * This);
        
        DECLSPEC_XFGVIRT(IUpdaterCallbackSystem, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterCallbackSystem * This,
            /* [in] */ LONG result);
        
        END_INTERFACE
    } IUpdaterCallbackSystemVtbl;

    interface IUpdaterCallbackSystem
    {
        CONST_VTBL struct IUpdaterCallbackSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterCallbackSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterCallbackSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterCallbackSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterCallbackSystem_Run(This,result)	\
    ( (This)->lpVtbl -> Run(This,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterCallbackSystem_INTERFACE_DEFINED__ */


#ifndef __IUpdaterAppState_INTERFACE_DEFINED__
#define __IUpdaterAppState_INTERFACE_DEFINED__

/* interface IUpdaterAppState */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IUpdaterAppState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A22AFC54-2DEF-4578-9187-DB3B24381090")
    IUpdaterAppState : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appId( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_version( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ap( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_brandCode( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0003) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_brandPath( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0004) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ecp( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0005) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterAppStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterAppState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterAppState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterAppState * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IUpdaterAppState * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IUpdaterAppState * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IUpdaterAppState * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IUpdaterAppState * This,
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
        
        DECLSPEC_XFGVIRT(IUpdaterAppState, get_appId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appId )( 
            IUpdaterAppState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0000);
        
        DECLSPEC_XFGVIRT(IUpdaterAppState, get_version)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_version )( 
            IUpdaterAppState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0001);
        
        DECLSPEC_XFGVIRT(IUpdaterAppState, get_ap)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ap )( 
            IUpdaterAppState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0002);
        
        DECLSPEC_XFGVIRT(IUpdaterAppState, get_brandCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_brandCode )( 
            IUpdaterAppState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0003);
        
        DECLSPEC_XFGVIRT(IUpdaterAppState, get_brandPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_brandPath )( 
            IUpdaterAppState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0004);
        
        DECLSPEC_XFGVIRT(IUpdaterAppState, get_ecp)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ecp )( 
            IUpdaterAppState * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppState0005);
        
        END_INTERFACE
    } IUpdaterAppStateVtbl;

    interface IUpdaterAppState
    {
        CONST_VTBL struct IUpdaterAppStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterAppState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterAppState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterAppState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterAppState_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IUpdaterAppState_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IUpdaterAppState_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IUpdaterAppState_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IUpdaterAppState_get_appId(This,__MIDL__IUpdaterAppState0000)	\
    ( (This)->lpVtbl -> get_appId(This,__MIDL__IUpdaterAppState0000) ) 

#define IUpdaterAppState_get_version(This,__MIDL__IUpdaterAppState0001)	\
    ( (This)->lpVtbl -> get_version(This,__MIDL__IUpdaterAppState0001) ) 

#define IUpdaterAppState_get_ap(This,__MIDL__IUpdaterAppState0002)	\
    ( (This)->lpVtbl -> get_ap(This,__MIDL__IUpdaterAppState0002) ) 

#define IUpdaterAppState_get_brandCode(This,__MIDL__IUpdaterAppState0003)	\
    ( (This)->lpVtbl -> get_brandCode(This,__MIDL__IUpdaterAppState0003) ) 

#define IUpdaterAppState_get_brandPath(This,__MIDL__IUpdaterAppState0004)	\
    ( (This)->lpVtbl -> get_brandPath(This,__MIDL__IUpdaterAppState0004) ) 

#define IUpdaterAppState_get_ecp(This,__MIDL__IUpdaterAppState0005)	\
    ( (This)->lpVtbl -> get_ecp(This,__MIDL__IUpdaterAppState0005) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterAppState_INTERFACE_DEFINED__ */


#ifndef __IUpdaterAppStateUser_INTERFACE_DEFINED__
#define __IUpdaterAppStateUser_INTERFACE_DEFINED__

/* interface IUpdaterAppStateUser */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IUpdaterAppStateUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("028FEB84-44BC-4A73-A0CD-603678155CC3")
    IUpdaterAppStateUser : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appId( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_version( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ap( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_brandCode( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0003) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_brandPath( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0004) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ecp( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0005) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterAppStateUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterAppStateUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterAppStateUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterAppStateUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IUpdaterAppStateUser * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IUpdaterAppStateUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IUpdaterAppStateUser * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IUpdaterAppStateUser * This,
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
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateUser, get_appId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appId )( 
            IUpdaterAppStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0000);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateUser, get_version)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_version )( 
            IUpdaterAppStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0001);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateUser, get_ap)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ap )( 
            IUpdaterAppStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0002);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateUser, get_brandCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_brandCode )( 
            IUpdaterAppStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0003);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateUser, get_brandPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_brandPath )( 
            IUpdaterAppStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0004);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateUser, get_ecp)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ecp )( 
            IUpdaterAppStateUser * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateUser0005);
        
        END_INTERFACE
    } IUpdaterAppStateUserVtbl;

    interface IUpdaterAppStateUser
    {
        CONST_VTBL struct IUpdaterAppStateUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterAppStateUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterAppStateUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterAppStateUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterAppStateUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IUpdaterAppStateUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IUpdaterAppStateUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IUpdaterAppStateUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IUpdaterAppStateUser_get_appId(This,__MIDL__IUpdaterAppStateUser0000)	\
    ( (This)->lpVtbl -> get_appId(This,__MIDL__IUpdaterAppStateUser0000) ) 

#define IUpdaterAppStateUser_get_version(This,__MIDL__IUpdaterAppStateUser0001)	\
    ( (This)->lpVtbl -> get_version(This,__MIDL__IUpdaterAppStateUser0001) ) 

#define IUpdaterAppStateUser_get_ap(This,__MIDL__IUpdaterAppStateUser0002)	\
    ( (This)->lpVtbl -> get_ap(This,__MIDL__IUpdaterAppStateUser0002) ) 

#define IUpdaterAppStateUser_get_brandCode(This,__MIDL__IUpdaterAppStateUser0003)	\
    ( (This)->lpVtbl -> get_brandCode(This,__MIDL__IUpdaterAppStateUser0003) ) 

#define IUpdaterAppStateUser_get_brandPath(This,__MIDL__IUpdaterAppStateUser0004)	\
    ( (This)->lpVtbl -> get_brandPath(This,__MIDL__IUpdaterAppStateUser0004) ) 

#define IUpdaterAppStateUser_get_ecp(This,__MIDL__IUpdaterAppStateUser0005)	\
    ( (This)->lpVtbl -> get_ecp(This,__MIDL__IUpdaterAppStateUser0005) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterAppStateUser_INTERFACE_DEFINED__ */


#ifndef __IUpdaterAppStateSystem_INTERFACE_DEFINED__
#define __IUpdaterAppStateSystem_INTERFACE_DEFINED__

/* interface IUpdaterAppStateSystem */
/* [unique][helpstring][uuid][dual][object] */ 


EXTERN_C const IID IID_IUpdaterAppStateSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92631531-8044-46F4-B645-CDFBCCC7FA3B")
    IUpdaterAppStateSystem : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appId( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0000) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_version( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0001) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ap( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0002) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_brandCode( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0003) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_brandPath( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0004) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ecp( 
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0005) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterAppStateSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterAppStateSystem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterAppStateSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterAppStateSystem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IUpdaterAppStateSystem * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IUpdaterAppStateSystem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IUpdaterAppStateSystem * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IUpdaterAppStateSystem * This,
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
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateSystem, get_appId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appId )( 
            IUpdaterAppStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0000);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateSystem, get_version)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_version )( 
            IUpdaterAppStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0001);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateSystem, get_ap)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ap )( 
            IUpdaterAppStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0002);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateSystem, get_brandCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_brandCode )( 
            IUpdaterAppStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0003);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateSystem, get_brandPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_brandPath )( 
            IUpdaterAppStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0004);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStateSystem, get_ecp)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ecp )( 
            IUpdaterAppStateSystem * This,
            /* [retval][out] */ BSTR *__MIDL__IUpdaterAppStateSystem0005);
        
        END_INTERFACE
    } IUpdaterAppStateSystemVtbl;

    interface IUpdaterAppStateSystem
    {
        CONST_VTBL struct IUpdaterAppStateSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterAppStateSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterAppStateSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterAppStateSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterAppStateSystem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IUpdaterAppStateSystem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IUpdaterAppStateSystem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IUpdaterAppStateSystem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IUpdaterAppStateSystem_get_appId(This,__MIDL__IUpdaterAppStateSystem0000)	\
    ( (This)->lpVtbl -> get_appId(This,__MIDL__IUpdaterAppStateSystem0000) ) 

#define IUpdaterAppStateSystem_get_version(This,__MIDL__IUpdaterAppStateSystem0001)	\
    ( (This)->lpVtbl -> get_version(This,__MIDL__IUpdaterAppStateSystem0001) ) 

#define IUpdaterAppStateSystem_get_ap(This,__MIDL__IUpdaterAppStateSystem0002)	\
    ( (This)->lpVtbl -> get_ap(This,__MIDL__IUpdaterAppStateSystem0002) ) 

#define IUpdaterAppStateSystem_get_brandCode(This,__MIDL__IUpdaterAppStateSystem0003)	\
    ( (This)->lpVtbl -> get_brandCode(This,__MIDL__IUpdaterAppStateSystem0003) ) 

#define IUpdaterAppStateSystem_get_brandPath(This,__MIDL__IUpdaterAppStateSystem0004)	\
    ( (This)->lpVtbl -> get_brandPath(This,__MIDL__IUpdaterAppStateSystem0004) ) 

#define IUpdaterAppStateSystem_get_ecp(This,__MIDL__IUpdaterAppStateSystem0005)	\
    ( (This)->lpVtbl -> get_ecp(This,__MIDL__IUpdaterAppStateSystem0005) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterAppStateSystem_INTERFACE_DEFINED__ */


#ifndef __IUpdaterAppStatesCallback_INTERFACE_DEFINED__
#define __IUpdaterAppStatesCallback_INTERFACE_DEFINED__

/* interface IUpdaterAppStatesCallback */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterAppStatesCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EFE903C0-E820-4136-9FAE-FDCD7F256302")
    IUpdaterAppStatesCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ VARIANT app_states) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterAppStatesCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterAppStatesCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterAppStatesCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterAppStatesCallback * This);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStatesCallback, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterAppStatesCallback * This,
            /* [in] */ VARIANT app_states);
        
        END_INTERFACE
    } IUpdaterAppStatesCallbackVtbl;

    interface IUpdaterAppStatesCallback
    {
        CONST_VTBL struct IUpdaterAppStatesCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterAppStatesCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterAppStatesCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterAppStatesCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterAppStatesCallback_Run(This,app_states)	\
    ( (This)->lpVtbl -> Run(This,app_states) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterAppStatesCallback_INTERFACE_DEFINED__ */


#ifndef __IUpdaterAppStatesCallbackUser_INTERFACE_DEFINED__
#define __IUpdaterAppStatesCallbackUser_INTERFACE_DEFINED__

/* interface IUpdaterAppStatesCallbackUser */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterAppStatesCallbackUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BCFCF95C-DE48-4F42-B0E9-D50DB407DB53")
    IUpdaterAppStatesCallbackUser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ VARIANT app_states) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterAppStatesCallbackUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterAppStatesCallbackUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterAppStatesCallbackUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterAppStatesCallbackUser * This);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStatesCallbackUser, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterAppStatesCallbackUser * This,
            /* [in] */ VARIANT app_states);
        
        END_INTERFACE
    } IUpdaterAppStatesCallbackUserVtbl;

    interface IUpdaterAppStatesCallbackUser
    {
        CONST_VTBL struct IUpdaterAppStatesCallbackUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterAppStatesCallbackUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterAppStatesCallbackUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterAppStatesCallbackUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterAppStatesCallbackUser_Run(This,app_states)	\
    ( (This)->lpVtbl -> Run(This,app_states) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterAppStatesCallbackUser_INTERFACE_DEFINED__ */


#ifndef __IUpdaterAppStatesCallbackSystem_INTERFACE_DEFINED__
#define __IUpdaterAppStatesCallbackSystem_INTERFACE_DEFINED__

/* interface IUpdaterAppStatesCallbackSystem */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterAppStatesCallbackSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2CB8867E-495E-459F-B1B6-2DD7FFDBD462")
    IUpdaterAppStatesCallbackSystem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ VARIANT app_states) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterAppStatesCallbackSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterAppStatesCallbackSystem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterAppStatesCallbackSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterAppStatesCallbackSystem * This);
        
        DECLSPEC_XFGVIRT(IUpdaterAppStatesCallbackSystem, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IUpdaterAppStatesCallbackSystem * This,
            /* [in] */ VARIANT app_states);
        
        END_INTERFACE
    } IUpdaterAppStatesCallbackSystemVtbl;

    interface IUpdaterAppStatesCallbackSystem
    {
        CONST_VTBL struct IUpdaterAppStatesCallbackSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterAppStatesCallbackSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterAppStatesCallbackSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterAppStatesCallbackSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterAppStatesCallbackSystem_Run(This,app_states)	\
    ( (This)->lpVtbl -> Run(This,app_states) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterAppStatesCallbackSystem_INTERFACE_DEFINED__ */


#ifndef __IUpdater_INTERFACE_DEFINED__
#define __IUpdater_INTERFACE_DEFINED__

/* interface IUpdater */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdater;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63B8FFB1-5314-48C9-9C57-93EC8BC6184B")
    IUpdater : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [retval][out] */ BSTR *version) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FetchPolicies( 
            /* [in] */ IUpdaterCallback *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterApp( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallback *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunPeriodicTasks( 
            /* [in] */ IUpdaterCallback *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckForUpdate( 
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserver *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserver *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateAll( 
            /* [in] */ IUpdaterObserver *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserver *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelInstalls( 
            /* [string][in] */ const WCHAR *app_id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunInstaller( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserver *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAppStates( 
            /* [in] */ IUpdaterAppStatesCallback *callback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdater * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdater * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdater * This);
        
        DECLSPEC_XFGVIRT(IUpdater, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            IUpdater * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IUpdater, FetchPolicies)
        HRESULT ( STDMETHODCALLTYPE *FetchPolicies )( 
            IUpdater * This,
            /* [in] */ IUpdaterCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdater, RegisterApp)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp )( 
            IUpdater * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdater, RunPeriodicTasks)
        HRESULT ( STDMETHODCALLTYPE *RunPeriodicTasks )( 
            IUpdater * This,
            /* [in] */ IUpdaterCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdater, CheckForUpdate)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate )( 
            IUpdater * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IUpdater * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, UpdateAll)
        HRESULT ( STDMETHODCALLTYPE *UpdateAll )( 
            IUpdater * This,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            IUpdater * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, CancelInstalls)
        HRESULT ( STDMETHODCALLTYPE *CancelInstalls )( 
            IUpdater * This,
            /* [string][in] */ const WCHAR *app_id);
        
        DECLSPEC_XFGVIRT(IUpdater, RunInstaller)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller )( 
            IUpdater * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, GetAppStates)
        HRESULT ( STDMETHODCALLTYPE *GetAppStates )( 
            IUpdater * This,
            /* [in] */ IUpdaterAppStatesCallback *callback);
        
        END_INTERFACE
    } IUpdaterVtbl;

    interface IUpdater
    {
        CONST_VTBL struct IUpdaterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdater_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdater_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdater_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdater_GetVersion(This,version)	\
    ( (This)->lpVtbl -> GetVersion(This,version) ) 

#define IUpdater_FetchPolicies(This,callback)	\
    ( (This)->lpVtbl -> FetchPolicies(This,callback) ) 

#define IUpdater_RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback)	\
    ( (This)->lpVtbl -> RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback) ) 

#define IUpdater_RunPeriodicTasks(This,callback)	\
    ( (This)->lpVtbl -> RunPeriodicTasks(This,callback) ) 

#define IUpdater_CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer) ) 

#define IUpdater_Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer) ) 

#define IUpdater_UpdateAll(This,observer)	\
    ( (This)->lpVtbl -> UpdateAll(This,observer) ) 

#define IUpdater_Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer)	\
    ( (This)->lpVtbl -> Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer) ) 

#define IUpdater_CancelInstalls(This,app_id)	\
    ( (This)->lpVtbl -> CancelInstalls(This,app_id) ) 

#define IUpdater_RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer)	\
    ( (This)->lpVtbl -> RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer) ) 

#define IUpdater_GetAppStates(This,callback)	\
    ( (This)->lpVtbl -> GetAppStates(This,callback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdater_INTERFACE_DEFINED__ */


#ifndef __IUpdaterUser_INTERFACE_DEFINED__
#define __IUpdaterUser_INTERFACE_DEFINED__

/* interface IUpdaterUser */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02AFCB67-0899-4676-91A9-67D92B3B7918")
    IUpdaterUser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [retval][out] */ BSTR *version) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FetchPolicies( 
            /* [in] */ IUpdaterCallbackUser *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterApp( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallbackUser *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunPeriodicTasks( 
            /* [in] */ IUpdaterCallbackUser *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckForUpdate( 
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateAll( 
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelInstalls( 
            /* [string][in] */ const WCHAR *app_id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunInstaller( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAppStates( 
            /* [in] */ IUpdaterAppStatesCallbackUser *callback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterUser * This);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            IUpdaterUser * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, FetchPolicies)
        HRESULT ( STDMETHODCALLTYPE *FetchPolicies )( 
            IUpdaterUser * This,
            /* [in] */ IUpdaterCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, RegisterApp)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp )( 
            IUpdaterUser * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, RunPeriodicTasks)
        HRESULT ( STDMETHODCALLTYPE *RunPeriodicTasks )( 
            IUpdaterUser * This,
            /* [in] */ IUpdaterCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, CheckForUpdate)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate )( 
            IUpdaterUser * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IUpdaterUser * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, UpdateAll)
        HRESULT ( STDMETHODCALLTYPE *UpdateAll )( 
            IUpdaterUser * This,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            IUpdaterUser * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, CancelInstalls)
        HRESULT ( STDMETHODCALLTYPE *CancelInstalls )( 
            IUpdaterUser * This,
            /* [string][in] */ const WCHAR *app_id);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, RunInstaller)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller )( 
            IUpdaterUser * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, GetAppStates)
        HRESULT ( STDMETHODCALLTYPE *GetAppStates )( 
            IUpdaterUser * This,
            /* [in] */ IUpdaterAppStatesCallbackUser *callback);
        
        END_INTERFACE
    } IUpdaterUserVtbl;

    interface IUpdaterUser
    {
        CONST_VTBL struct IUpdaterUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterUser_GetVersion(This,version)	\
    ( (This)->lpVtbl -> GetVersion(This,version) ) 

#define IUpdaterUser_FetchPolicies(This,callback)	\
    ( (This)->lpVtbl -> FetchPolicies(This,callback) ) 

#define IUpdaterUser_RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback)	\
    ( (This)->lpVtbl -> RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback) ) 

#define IUpdaterUser_RunPeriodicTasks(This,callback)	\
    ( (This)->lpVtbl -> RunPeriodicTasks(This,callback) ) 

#define IUpdaterUser_CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer) ) 

#define IUpdaterUser_Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer) ) 

#define IUpdaterUser_UpdateAll(This,observer)	\
    ( (This)->lpVtbl -> UpdateAll(This,observer) ) 

#define IUpdaterUser_Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer)	\
    ( (This)->lpVtbl -> Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer) ) 

#define IUpdaterUser_CancelInstalls(This,app_id)	\
    ( (This)->lpVtbl -> CancelInstalls(This,app_id) ) 

#define IUpdaterUser_RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer)	\
    ( (This)->lpVtbl -> RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer) ) 

#define IUpdaterUser_GetAppStates(This,callback)	\
    ( (This)->lpVtbl -> GetAppStates(This,callback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterUser_INTERFACE_DEFINED__ */


#ifndef __IUpdaterSystem_INTERFACE_DEFINED__
#define __IUpdaterSystem_INTERFACE_DEFINED__

/* interface IUpdaterSystem */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdaterSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FCE335F3-A55C-496E-814F-85971C9FA6F1")
    IUpdaterSystem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [retval][out] */ BSTR *version) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FetchPolicies( 
            /* [in] */ IUpdaterCallbackSystem *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterApp( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallbackSystem *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunPeriodicTasks( 
            /* [in] */ IUpdaterCallbackSystem *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckForUpdate( 
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateAll( 
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelInstalls( 
            /* [string][in] */ const WCHAR *app_id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunInstaller( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAppStates( 
            /* [in] */ IUpdaterAppStatesCallbackSystem *callback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdaterSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdaterSystem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdaterSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdaterSystem * This);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            IUpdaterSystem * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, FetchPolicies)
        HRESULT ( STDMETHODCALLTYPE *FetchPolicies )( 
            IUpdaterSystem * This,
            /* [in] */ IUpdaterCallbackSystem *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, RegisterApp)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp )( 
            IUpdaterSystem * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallbackSystem *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, RunPeriodicTasks)
        HRESULT ( STDMETHODCALLTYPE *RunPeriodicTasks )( 
            IUpdaterSystem * This,
            /* [in] */ IUpdaterCallbackSystem *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, CheckForUpdate)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate )( 
            IUpdaterSystem * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IUpdaterSystem * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, UpdateAll)
        HRESULT ( STDMETHODCALLTYPE *UpdateAll )( 
            IUpdaterSystem * This,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            IUpdaterSystem * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, CancelInstalls)
        HRESULT ( STDMETHODCALLTYPE *CancelInstalls )( 
            IUpdaterSystem * This,
            /* [string][in] */ const WCHAR *app_id);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, RunInstaller)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller )( 
            IUpdaterSystem * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, GetAppStates)
        HRESULT ( STDMETHODCALLTYPE *GetAppStates )( 
            IUpdaterSystem * This,
            /* [in] */ IUpdaterAppStatesCallbackSystem *callback);
        
        END_INTERFACE
    } IUpdaterSystemVtbl;

    interface IUpdaterSystem
    {
        CONST_VTBL struct IUpdaterSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdaterSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdaterSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdaterSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdaterSystem_GetVersion(This,version)	\
    ( (This)->lpVtbl -> GetVersion(This,version) ) 

#define IUpdaterSystem_FetchPolicies(This,callback)	\
    ( (This)->lpVtbl -> FetchPolicies(This,callback) ) 

#define IUpdaterSystem_RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback)	\
    ( (This)->lpVtbl -> RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback) ) 

#define IUpdaterSystem_RunPeriodicTasks(This,callback)	\
    ( (This)->lpVtbl -> RunPeriodicTasks(This,callback) ) 

#define IUpdaterSystem_CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer) ) 

#define IUpdaterSystem_Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer) ) 

#define IUpdaterSystem_UpdateAll(This,observer)	\
    ( (This)->lpVtbl -> UpdateAll(This,observer) ) 

#define IUpdaterSystem_Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer)	\
    ( (This)->lpVtbl -> Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer) ) 

#define IUpdaterSystem_CancelInstalls(This,app_id)	\
    ( (This)->lpVtbl -> CancelInstalls(This,app_id) ) 

#define IUpdaterSystem_RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer)	\
    ( (This)->lpVtbl -> RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer) ) 

#define IUpdaterSystem_GetAppStates(This,callback)	\
    ( (This)->lpVtbl -> GetAppStates(This,callback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdaterSystem_INTERFACE_DEFINED__ */


#ifndef __IUpdater2_INTERFACE_DEFINED__
#define __IUpdater2_INTERFACE_DEFINED__

/* interface IUpdater2 */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdater2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("295F15EF-23C8-4BC6-91EB-1E9DA4145250")
    IUpdater2 : public IUpdater
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterApp2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ IUpdaterCallback *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckForUpdate2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserver *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserver *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ LONG priority,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserver *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunInstaller2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserver *observer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdater2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdater2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdater2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdater2 * This);
        
        DECLSPEC_XFGVIRT(IUpdater, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            IUpdater2 * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IUpdater, FetchPolicies)
        HRESULT ( STDMETHODCALLTYPE *FetchPolicies )( 
            IUpdater2 * This,
            /* [in] */ IUpdaterCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdater, RegisterApp)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdater, RunPeriodicTasks)
        HRESULT ( STDMETHODCALLTYPE *RunPeriodicTasks )( 
            IUpdater2 * This,
            /* [in] */ IUpdaterCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdater, CheckForUpdate)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, UpdateAll)
        HRESULT ( STDMETHODCALLTYPE *UpdateAll )( 
            IUpdater2 * This,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, CancelInstalls)
        HRESULT ( STDMETHODCALLTYPE *CancelInstalls )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id);
        
        DECLSPEC_XFGVIRT(IUpdater, RunInstaller)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater, GetAppStates)
        HRESULT ( STDMETHODCALLTYPE *GetAppStates )( 
            IUpdater2 * This,
            /* [in] */ IUpdaterAppStatesCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdater2, RegisterApp2)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp2 )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ IUpdaterCallback *callback);
        
        DECLSPEC_XFGVIRT(IUpdater2, CheckForUpdate2)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate2 )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2, Update2)
        HRESULT ( STDMETHODCALLTYPE *Update2 )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2, Install2)
        HRESULT ( STDMETHODCALLTYPE *Install2 )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ LONG priority,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserver *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2, RunInstaller2)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller2 )( 
            IUpdater2 * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserver *observer);
        
        END_INTERFACE
    } IUpdater2Vtbl;

    interface IUpdater2
    {
        CONST_VTBL struct IUpdater2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdater2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdater2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdater2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdater2_GetVersion(This,version)	\
    ( (This)->lpVtbl -> GetVersion(This,version) ) 

#define IUpdater2_FetchPolicies(This,callback)	\
    ( (This)->lpVtbl -> FetchPolicies(This,callback) ) 

#define IUpdater2_RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback)	\
    ( (This)->lpVtbl -> RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback) ) 

#define IUpdater2_RunPeriodicTasks(This,callback)	\
    ( (This)->lpVtbl -> RunPeriodicTasks(This,callback) ) 

#define IUpdater2_CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer) ) 

#define IUpdater2_Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer) ) 

#define IUpdater2_UpdateAll(This,observer)	\
    ( (This)->lpVtbl -> UpdateAll(This,observer) ) 

#define IUpdater2_Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer)	\
    ( (This)->lpVtbl -> Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer) ) 

#define IUpdater2_CancelInstalls(This,app_id)	\
    ( (This)->lpVtbl -> CancelInstalls(This,app_id) ) 

#define IUpdater2_RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer)	\
    ( (This)->lpVtbl -> RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer) ) 

#define IUpdater2_GetAppStates(This,callback)	\
    ( (This)->lpVtbl -> GetAppStates(This,callback) ) 


#define IUpdater2_RegisterApp2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,install_id,callback)	\
    ( (This)->lpVtbl -> RegisterApp2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,install_id,callback) ) 

#define IUpdater2_CheckForUpdate2(This,app_id,priority,same_version_update_allowed,language,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate2(This,app_id,priority,same_version_update_allowed,language,observer) ) 

#define IUpdater2_Update2(This,app_id,install_data_index,priority,same_version_update_allowed,language,observer)	\
    ( (This)->lpVtbl -> Update2(This,app_id,install_data_index,priority,same_version_update_allowed,language,observer) ) 

#define IUpdater2_Install2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,install_id,priority,language,observer)	\
    ( (This)->lpVtbl -> Install2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,install_id,priority,language,observer) ) 

#define IUpdater2_RunInstaller2(This,app_id,installer_path,install_args,install_data,install_settings,language,observer)	\
    ( (This)->lpVtbl -> RunInstaller2(This,app_id,installer_path,install_args,install_data,install_settings,language,observer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdater2_INTERFACE_DEFINED__ */


#ifndef __IUpdater2User_INTERFACE_DEFINED__
#define __IUpdater2User_INTERFACE_DEFINED__

/* interface IUpdater2User */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdater2User;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0AB4AFBE-AEA1-4F22-9D45-E867423B8E01")
    IUpdater2User : public IUpdaterUser
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterApp2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ IUpdaterCallbackUser *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckForUpdate2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ LONG priority,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunInstaller2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverUser *observer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdater2UserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdater2User * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdater2User * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdater2User * This);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            IUpdater2User * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, FetchPolicies)
        HRESULT ( STDMETHODCALLTYPE *FetchPolicies )( 
            IUpdater2User * This,
            /* [in] */ IUpdaterCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, RegisterApp)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, RunPeriodicTasks)
        HRESULT ( STDMETHODCALLTYPE *RunPeriodicTasks )( 
            IUpdater2User * This,
            /* [in] */ IUpdaterCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, CheckForUpdate)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, UpdateAll)
        HRESULT ( STDMETHODCALLTYPE *UpdateAll )( 
            IUpdater2User * This,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, CancelInstalls)
        HRESULT ( STDMETHODCALLTYPE *CancelInstalls )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, RunInstaller)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterUser, GetAppStates)
        HRESULT ( STDMETHODCALLTYPE *GetAppStates )( 
            IUpdater2User * This,
            /* [in] */ IUpdaterAppStatesCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdater2User, RegisterApp2)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp2 )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ IUpdaterCallbackUser *callback);
        
        DECLSPEC_XFGVIRT(IUpdater2User, CheckForUpdate2)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate2 )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2User, Update2)
        HRESULT ( STDMETHODCALLTYPE *Update2 )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2User, Install2)
        HRESULT ( STDMETHODCALLTYPE *Install2 )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ LONG priority,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverUser *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2User, RunInstaller2)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller2 )( 
            IUpdater2User * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverUser *observer);
        
        END_INTERFACE
    } IUpdater2UserVtbl;

    interface IUpdater2User
    {
        CONST_VTBL struct IUpdater2UserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdater2User_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdater2User_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdater2User_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdater2User_GetVersion(This,version)	\
    ( (This)->lpVtbl -> GetVersion(This,version) ) 

#define IUpdater2User_FetchPolicies(This,callback)	\
    ( (This)->lpVtbl -> FetchPolicies(This,callback) ) 

#define IUpdater2User_RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback)	\
    ( (This)->lpVtbl -> RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback) ) 

#define IUpdater2User_RunPeriodicTasks(This,callback)	\
    ( (This)->lpVtbl -> RunPeriodicTasks(This,callback) ) 

#define IUpdater2User_CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer) ) 

#define IUpdater2User_Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer) ) 

#define IUpdater2User_UpdateAll(This,observer)	\
    ( (This)->lpVtbl -> UpdateAll(This,observer) ) 

#define IUpdater2User_Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer)	\
    ( (This)->lpVtbl -> Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer) ) 

#define IUpdater2User_CancelInstalls(This,app_id)	\
    ( (This)->lpVtbl -> CancelInstalls(This,app_id) ) 

#define IUpdater2User_RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer)	\
    ( (This)->lpVtbl -> RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer) ) 

#define IUpdater2User_GetAppStates(This,callback)	\
    ( (This)->lpVtbl -> GetAppStates(This,callback) ) 


#define IUpdater2User_RegisterApp2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,install_id,callback)	\
    ( (This)->lpVtbl -> RegisterApp2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,install_id,callback) ) 

#define IUpdater2User_CheckForUpdate2(This,app_id,priority,same_version_update_allowed,language,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate2(This,app_id,priority,same_version_update_allowed,language,observer) ) 

#define IUpdater2User_Update2(This,app_id,install_data_index,priority,same_version_update_allowed,language,observer)	\
    ( (This)->lpVtbl -> Update2(This,app_id,install_data_index,priority,same_version_update_allowed,language,observer) ) 

#define IUpdater2User_Install2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,install_id,priority,language,observer)	\
    ( (This)->lpVtbl -> Install2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,install_id,priority,language,observer) ) 

#define IUpdater2User_RunInstaller2(This,app_id,installer_path,install_args,install_data,install_settings,language,observer)	\
    ( (This)->lpVtbl -> RunInstaller2(This,app_id,installer_path,install_args,install_data,install_settings,language,observer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdater2User_INTERFACE_DEFINED__ */


#ifndef __IUpdater2System_INTERFACE_DEFINED__
#define __IUpdater2System_INTERFACE_DEFINED__

/* interface IUpdater2System */
/* [unique][helpstring][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IUpdater2System;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EEDAB506-FC1A-42B5-9264-BC6A053DDDF0")
    IUpdater2System : public IUpdaterSystem
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterApp2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ IUpdaterCallbackSystem *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckForUpdate2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ LONG priority,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RunInstaller2( 
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverSystem *observer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUpdater2SystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUpdater2System * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUpdater2System * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUpdater2System * This);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            IUpdater2System * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, FetchPolicies)
        HRESULT ( STDMETHODCALLTYPE *FetchPolicies )( 
            IUpdater2System * This,
            /* [in] */ IUpdaterCallbackSystem *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, RegisterApp)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [in] */ IUpdaterCallbackSystem *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, RunPeriodicTasks)
        HRESULT ( STDMETHODCALLTYPE *RunPeriodicTasks )( 
            IUpdater2System * This,
            /* [in] */ IUpdaterCallbackSystem *callback);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, CheckForUpdate)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, UpdateAll)
        HRESULT ( STDMETHODCALLTYPE *UpdateAll )( 
            IUpdater2System * This,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, CancelInstalls)
        HRESULT ( STDMETHODCALLTYPE *CancelInstalls )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, RunInstaller)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdaterSystem, GetAppStates)
        HRESULT ( STDMETHODCALLTYPE *GetAppStates )( 
            IUpdater2System * This,
            /* [in] */ IUpdaterAppStatesCallbackSystem *callback);
        
        DECLSPEC_XFGVIRT(IUpdater2System, RegisterApp2)
        HRESULT ( STDMETHODCALLTYPE *RegisterApp2 )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ IUpdaterCallbackSystem *callback);
        
        DECLSPEC_XFGVIRT(IUpdater2System, CheckForUpdate2)
        HRESULT ( STDMETHODCALLTYPE *CheckForUpdate2 )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2System, Update2)
        HRESULT ( STDMETHODCALLTYPE *Update2 )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [in] */ LONG priority,
            /* [in] */ BOOL same_version_update_allowed,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2System, Install2)
        HRESULT ( STDMETHODCALLTYPE *Install2 )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *brand_code,
            /* [string][in] */ const WCHAR *brand_path,
            /* [string][in] */ const WCHAR *tag,
            /* [string][in] */ const WCHAR *version,
            /* [string][in] */ const WCHAR *existence_checker_path,
            /* [string][in] */ const WCHAR *client_install_data,
            /* [string][in] */ const WCHAR *install_data_index,
            /* [string][in] */ const WCHAR *install_id,
            /* [in] */ LONG priority,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        DECLSPEC_XFGVIRT(IUpdater2System, RunInstaller2)
        HRESULT ( STDMETHODCALLTYPE *RunInstaller2 )( 
            IUpdater2System * This,
            /* [string][in] */ const WCHAR *app_id,
            /* [string][in] */ const WCHAR *installer_path,
            /* [string][in] */ const WCHAR *install_args,
            /* [string][in] */ const WCHAR *install_data,
            /* [string][in] */ const WCHAR *install_settings,
            /* [string][in] */ const WCHAR *language,
            /* [in] */ IUpdaterObserverSystem *observer);
        
        END_INTERFACE
    } IUpdater2SystemVtbl;

    interface IUpdater2System
    {
        CONST_VTBL struct IUpdater2SystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUpdater2System_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUpdater2System_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUpdater2System_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUpdater2System_GetVersion(This,version)	\
    ( (This)->lpVtbl -> GetVersion(This,version) ) 

#define IUpdater2System_FetchPolicies(This,callback)	\
    ( (This)->lpVtbl -> FetchPolicies(This,callback) ) 

#define IUpdater2System_RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback)	\
    ( (This)->lpVtbl -> RegisterApp(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,callback) ) 

#define IUpdater2System_RunPeriodicTasks(This,callback)	\
    ( (This)->lpVtbl -> RunPeriodicTasks(This,callback) ) 

#define IUpdater2System_CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate(This,app_id,priority,same_version_update_allowed,observer) ) 

#define IUpdater2System_Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer)	\
    ( (This)->lpVtbl -> Update(This,app_id,install_data_index,priority,same_version_update_allowed,observer) ) 

#define IUpdater2System_UpdateAll(This,observer)	\
    ( (This)->lpVtbl -> UpdateAll(This,observer) ) 

#define IUpdater2System_Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer)	\
    ( (This)->lpVtbl -> Install(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,priority,observer) ) 

#define IUpdater2System_CancelInstalls(This,app_id)	\
    ( (This)->lpVtbl -> CancelInstalls(This,app_id) ) 

#define IUpdater2System_RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer)	\
    ( (This)->lpVtbl -> RunInstaller(This,app_id,installer_path,install_args,install_data,install_settings,observer) ) 

#define IUpdater2System_GetAppStates(This,callback)	\
    ( (This)->lpVtbl -> GetAppStates(This,callback) ) 


#define IUpdater2System_RegisterApp2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,install_id,callback)	\
    ( (This)->lpVtbl -> RegisterApp2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,install_id,callback) ) 

#define IUpdater2System_CheckForUpdate2(This,app_id,priority,same_version_update_allowed,language,observer)	\
    ( (This)->lpVtbl -> CheckForUpdate2(This,app_id,priority,same_version_update_allowed,language,observer) ) 

#define IUpdater2System_Update2(This,app_id,install_data_index,priority,same_version_update_allowed,language,observer)	\
    ( (This)->lpVtbl -> Update2(This,app_id,install_data_index,priority,same_version_update_allowed,language,observer) ) 

#define IUpdater2System_Install2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,install_id,priority,language,observer)	\
    ( (This)->lpVtbl -> Install2(This,app_id,brand_code,brand_path,tag,version,existence_checker_path,client_install_data,install_data_index,install_id,priority,language,observer) ) 

#define IUpdater2System_RunInstaller2(This,app_id,installer_path,install_args,install_data,install_settings,language,observer)	\
    ( (This)->lpVtbl -> RunInstaller2(This,app_id,installer_path,install_args,install_data,install_settings,language,observer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUpdater2System_INTERFACE_DEFINED__ */



#ifndef __UpdaterLib_LIBRARY_DEFINED__
#define __UpdaterLib_LIBRARY_DEFINED__

/* library UpdaterLib */
/* [helpstring][version][uuid] */ 



















EXTERN_C const IID LIBID_UpdaterLib;

EXTERN_C const CLSID CLSID_UpdaterUserClass;

#ifdef __cplusplus

class DECLSPEC_UUID("158428a4-6014-4978-83ba-9fad0dabe791")
UpdaterUserClass;
#endif

EXTERN_C const CLSID CLSID_UpdaterSystemClass;

#ifdef __cplusplus

class DECLSPEC_UUID("415FD747-D79E-42D7-93AC-1BA6E5FD4E93")
UpdaterSystemClass;
#endif
#endif /* __UpdaterLib_LIBRARY_DEFINED__ */

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


