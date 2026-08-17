

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for gen/remoting/host/win/chromoting_lib.idl:
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

#ifndef __chromoting_lib_h__
#define __chromoting_lib_h__

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

#ifndef __IRdpDesktopSessionEventHandler_FWD_DEFINED__
#define __IRdpDesktopSessionEventHandler_FWD_DEFINED__
typedef interface IRdpDesktopSessionEventHandler IRdpDesktopSessionEventHandler;

#endif 	/* __IRdpDesktopSessionEventHandler_FWD_DEFINED__ */


#ifndef __IRdpDesktopSession_FWD_DEFINED__
#define __IRdpDesktopSession_FWD_DEFINED__
typedef interface IRdpDesktopSession IRdpDesktopSession;

#endif 	/* __IRdpDesktopSession_FWD_DEFINED__ */


#ifndef __RdpDesktopSession_FWD_DEFINED__
#define __RdpDesktopSession_FWD_DEFINED__

#ifdef __cplusplus
typedef class RdpDesktopSession RdpDesktopSession;
#else
typedef struct RdpDesktopSession RdpDesktopSession;
#endif /* __cplusplus */

#endif 	/* __RdpDesktopSession_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


#ifndef __IRdpDesktopSessionEventHandler_INTERFACE_DEFINED__
#define __IRdpDesktopSessionEventHandler_INTERFACE_DEFINED__

/* interface IRdpDesktopSessionEventHandler */
/* [unique][helpstring][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IRdpDesktopSessionEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b59b96da-83cb-40ee-9b91-c377400fc3e3")
    IRdpDesktopSessionEventHandler : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnRdpConnected( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnRdpClosed( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRdpDesktopSessionEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRdpDesktopSessionEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRdpDesktopSessionEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRdpDesktopSessionEventHandler * This);
        
        DECLSPEC_XFGVIRT(IRdpDesktopSessionEventHandler, OnRdpConnected)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnRdpConnected )( 
            IRdpDesktopSessionEventHandler * This);
        
        DECLSPEC_XFGVIRT(IRdpDesktopSessionEventHandler, OnRdpClosed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnRdpClosed )( 
            IRdpDesktopSessionEventHandler * This);
        
        END_INTERFACE
    } IRdpDesktopSessionEventHandlerVtbl;

    interface IRdpDesktopSessionEventHandler
    {
        CONST_VTBL struct IRdpDesktopSessionEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRdpDesktopSessionEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRdpDesktopSessionEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRdpDesktopSessionEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRdpDesktopSessionEventHandler_OnRdpConnected(This)	\
    ( (This)->lpVtbl -> OnRdpConnected(This) ) 

#define IRdpDesktopSessionEventHandler_OnRdpClosed(This)	\
    ( (This)->lpVtbl -> OnRdpClosed(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRdpDesktopSessionEventHandler_INTERFACE_DEFINED__ */


#ifndef __IRdpDesktopSession_INTERFACE_DEFINED__
#define __IRdpDesktopSession_INTERFACE_DEFINED__

/* interface IRdpDesktopSession */
/* [unique][helpstring][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IRdpDesktopSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6a7699f0-ee43-43e7-aa30-a6738f9bd470")
    IRdpDesktopSession : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ long width,
            /* [in] */ long height,
            /* [in] */ long dpi_x,
            /* [in] */ long dpi_y,
            /* [in] */ BSTR terminal_id,
            /* [in] */ DWORD port_number,
            /* [in] */ IRdpDesktopSessionEventHandler *event_handler) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ChangeResolution( 
            /* [in] */ long width,
            /* [in] */ long height,
            /* [in] */ long dpi_x,
            /* [in] */ long dpi_y) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InjectSas( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRdpDesktopSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRdpDesktopSession * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRdpDesktopSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRdpDesktopSession * This);
        
        DECLSPEC_XFGVIRT(IRdpDesktopSession, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IRdpDesktopSession * This,
            /* [in] */ long width,
            /* [in] */ long height,
            /* [in] */ long dpi_x,
            /* [in] */ long dpi_y,
            /* [in] */ BSTR terminal_id,
            /* [in] */ DWORD port_number,
            /* [in] */ IRdpDesktopSessionEventHandler *event_handler);
        
        DECLSPEC_XFGVIRT(IRdpDesktopSession, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            IRdpDesktopSession * This);
        
        DECLSPEC_XFGVIRT(IRdpDesktopSession, ChangeResolution)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ChangeResolution )( 
            IRdpDesktopSession * This,
            /* [in] */ long width,
            /* [in] */ long height,
            /* [in] */ long dpi_x,
            /* [in] */ long dpi_y);
        
        DECLSPEC_XFGVIRT(IRdpDesktopSession, InjectSas)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InjectSas )( 
            IRdpDesktopSession * This);
        
        END_INTERFACE
    } IRdpDesktopSessionVtbl;

    interface IRdpDesktopSession
    {
        CONST_VTBL struct IRdpDesktopSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRdpDesktopSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRdpDesktopSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRdpDesktopSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRdpDesktopSession_Connect(This,width,height,dpi_x,dpi_y,terminal_id,port_number,event_handler)	\
    ( (This)->lpVtbl -> Connect(This,width,height,dpi_x,dpi_y,terminal_id,port_number,event_handler) ) 

#define IRdpDesktopSession_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IRdpDesktopSession_ChangeResolution(This,width,height,dpi_x,dpi_y)	\
    ( (This)->lpVtbl -> ChangeResolution(This,width,height,dpi_x,dpi_y) ) 

#define IRdpDesktopSession_InjectSas(This)	\
    ( (This)->lpVtbl -> InjectSas(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRdpDesktopSession_INTERFACE_DEFINED__ */



#ifndef __ChromotingLib_LIBRARY_DEFINED__
#define __ChromotingLib_LIBRARY_DEFINED__

/* library ChromotingLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_ChromotingLib;

EXTERN_C const CLSID CLSID_RdpDesktopSession;

#ifdef __cplusplus

class DECLSPEC_UUID("6741fd0a-6a8a-5838-a35e-8088697e2088")
RdpDesktopSession;
#endif
#endif /* __ChromotingLib_LIBRARY_DEFINED__ */

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


