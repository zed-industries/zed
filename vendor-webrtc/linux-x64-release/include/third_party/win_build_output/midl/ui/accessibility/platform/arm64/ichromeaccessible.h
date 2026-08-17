

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../ui/accessibility/platform/ichromeaccessible.idl:
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

#ifndef __ichromeaccessible_h__
#define __ichromeaccessible_h__

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

#ifndef __IChromeAccessibleDelegate_FWD_DEFINED__
#define __IChromeAccessibleDelegate_FWD_DEFINED__
typedef interface IChromeAccessibleDelegate IChromeAccessibleDelegate;

#endif 	/* __IChromeAccessibleDelegate_FWD_DEFINED__ */


#ifndef __IChromeAccessible_FWD_DEFINED__
#define __IChromeAccessible_FWD_DEFINED__
typedef interface IChromeAccessible IChromeAccessible;

#endif 	/* __IChromeAccessible_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ichromeaccessible_0000_0000 */
/* [local] */ 

#define	DISPID_CHROME_BULK_FETCH	( -1600 )

#define	DISPID_CHROME_ON_BULK_FETCH_RESULT	( -1601 )

#define	DISPID_CHROME_HIT_TEST	( -1602 )

#define	DISPID_CHROME_ON_HIT_TEST_RESULT	( -1603 )



extern RPC_IF_HANDLE __MIDL_itf_ichromeaccessible_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ichromeaccessible_0000_0000_v0_0_s_ifspec;

#ifndef __IChromeAccessibleDelegate_INTERFACE_DEFINED__
#define __IChromeAccessibleDelegate_INTERFACE_DEFINED__

/* interface IChromeAccessibleDelegate */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IChromeAccessibleDelegate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0e3edc14-79f4-413f-b854-d3b6860d74a2")
    IChromeAccessibleDelegate : public IUnknown
    {
    public:
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_bulkFetchResult( 
            /* [in] */ LONG requestID,
            /* [in] */ BSTR resultJson) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_hitTestResult( 
            /* [in] */ LONG requestID,
            /* [in] */ IUnknown *result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IChromeAccessibleDelegateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IChromeAccessibleDelegate * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IChromeAccessibleDelegate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IChromeAccessibleDelegate * This);
        
        DECLSPEC_XFGVIRT(IChromeAccessibleDelegate, put_bulkFetchResult)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_bulkFetchResult )( 
            IChromeAccessibleDelegate * This,
            /* [in] */ LONG requestID,
            /* [in] */ BSTR resultJson);
        
        DECLSPEC_XFGVIRT(IChromeAccessibleDelegate, put_hitTestResult)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_hitTestResult )( 
            IChromeAccessibleDelegate * This,
            /* [in] */ LONG requestID,
            /* [in] */ IUnknown *result);
        
        END_INTERFACE
    } IChromeAccessibleDelegateVtbl;

    interface IChromeAccessibleDelegate
    {
        CONST_VTBL struct IChromeAccessibleDelegateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IChromeAccessibleDelegate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IChromeAccessibleDelegate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IChromeAccessibleDelegate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IChromeAccessibleDelegate_put_bulkFetchResult(This,requestID,resultJson)	\
    ( (This)->lpVtbl -> put_bulkFetchResult(This,requestID,resultJson) ) 

#define IChromeAccessibleDelegate_put_hitTestResult(This,requestID,result)	\
    ( (This)->lpVtbl -> put_hitTestResult(This,requestID,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IChromeAccessibleDelegate_INTERFACE_DEFINED__ */


#ifndef __IChromeAccessible_INTERFACE_DEFINED__
#define __IChromeAccessible_INTERFACE_DEFINED__

/* interface IChromeAccessible */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IChromeAccessible;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6175bd95-3b2e-4ebc-bc51-9cab782bec92")
    IChromeAccessible : public IUnknown
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_bulkFetch( 
            /* [in] */ BSTR inputJson,
            /* [in] */ LONG requestID,
            /* [in] */ IChromeAccessibleDelegate *delegate) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_hitTest( 
            /* [in] */ LONG screenPhysicalPixelX,
            /* [in] */ LONG screenPhysicalPixelY,
            /* [in] */ LONG requestID,
            /* [in] */ IChromeAccessibleDelegate *delegate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IChromeAccessibleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IChromeAccessible * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IChromeAccessible * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IChromeAccessible * This);
        
        DECLSPEC_XFGVIRT(IChromeAccessible, get_bulkFetch)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_bulkFetch )( 
            IChromeAccessible * This,
            /* [in] */ BSTR inputJson,
            /* [in] */ LONG requestID,
            /* [in] */ IChromeAccessibleDelegate *delegate);
        
        DECLSPEC_XFGVIRT(IChromeAccessible, get_hitTest)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_hitTest )( 
            IChromeAccessible * This,
            /* [in] */ LONG screenPhysicalPixelX,
            /* [in] */ LONG screenPhysicalPixelY,
            /* [in] */ LONG requestID,
            /* [in] */ IChromeAccessibleDelegate *delegate);
        
        END_INTERFACE
    } IChromeAccessibleVtbl;

    interface IChromeAccessible
    {
        CONST_VTBL struct IChromeAccessibleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IChromeAccessible_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IChromeAccessible_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IChromeAccessible_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IChromeAccessible_get_bulkFetch(This,inputJson,requestID,delegate)	\
    ( (This)->lpVtbl -> get_bulkFetch(This,inputJson,requestID,delegate) ) 

#define IChromeAccessible_get_hitTest(This,screenPhysicalPixelX,screenPhysicalPixelY,requestID,delegate)	\
    ( (This)->lpVtbl -> get_hitTest(This,screenPhysicalPixelX,screenPhysicalPixelY,requestID,delegate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IChromeAccessible_INTERFACE_DEFINED__ */


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


