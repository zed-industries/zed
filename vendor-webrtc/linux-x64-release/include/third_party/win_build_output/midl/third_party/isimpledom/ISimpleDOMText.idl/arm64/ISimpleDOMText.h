

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../third_party/isimpledom/ISimpleDOMText.idl:
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

#ifndef __ISimpleDOMText_h__
#define __ISimpleDOMText_h__

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

#ifndef __ISimpleDOMText_FWD_DEFINED__
#define __ISimpleDOMText_FWD_DEFINED__
typedef interface ISimpleDOMText ISimpleDOMText;

#endif 	/* __ISimpleDOMText_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ISimpleDOMText_0000_0000 */
/* [local] */ 

///////////////////////////////////////////////////////////////////////////////////////////////////////
//
// ISimpleDOMText
// ---------------------------------------------------------------------------------------------------=
// An interface that extends MSAA's IAccessible to provide important additional capabilities on text nodes
//
// @STATUS UNDER_REVIEW
//
// [propget] domText(/* out,retval */ BSTR *domText
// ---------------------------------------------------------------------------------------------------=
// Similar to IAccessible::get_accName, but does not strip out whitespace characters.
// Important for retrieving the correct start/end substring indices to use with other
// methods in ISimpleDOMText.
//
//
// get_[un]clippedSubstringBounds(
//   /* [in] */ unsigned int startIndex,
//   /* [in] */ unsigned int endIndex,
//   /* [out] */ int *x,
//   /* [out] */ int *y,
//   /* [out] */ int *width,
//   /* [out] */ int *height);
// ---------------------------------------------------------------------------------------------------=
// Both methods get_clippedSubstringBounds and get_unclippedSubstringBounds return the screen pixel
// coordinates of the given text substring. The in parameters for start and end indices refer
// to the string returned by ISimpleDOMText::get_domText().
//
//
// scrollToSubstring(
//   /* [in] */ unsigned int startIndex,
//   /* [in] */ unsigned int endIndex);
// ---------------------------------------------------------------------------------------------------=
// In scrollable views, scrolls to ensure that the specified substring is visible onscreen.
// The in parameters for start and end indices refer to the string returned
// by ISimpleDOMText::get_domText().
//
//
// [propget] fontFamily(/* out,retval */ BSTR *fontFamily);
// ---------------------------------------------------------------------------------------------------=
// Return a single computed font family name, which is better than the comma delineated list
// that is returned by the ISimpleDOMNode computed style methods for font-family.
// In other words, return something like 'Arial' instead of 'Arial, Helvetica, Sans-serif'.
///////////////////////////////////////////////////////////////////////////////////////////////////////




extern RPC_IF_HANDLE __MIDL_itf_ISimpleDOMText_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ISimpleDOMText_0000_0000_v0_0_s_ifspec;

#ifndef __ISimpleDOMText_INTERFACE_DEFINED__
#define __ISimpleDOMText_INTERFACE_DEFINED__

/* interface ISimpleDOMText */
/* [uuid][object] */ 


EXTERN_C const IID IID_ISimpleDOMText;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4e747be5-2052-4265-8af0-8ecad7aad1c0")
    ISimpleDOMText : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_domText( 
            /* [retval][out] */ BSTR *domText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_clippedSubstringBounds( 
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int endIndex,
            /* [out] */ int *x,
            /* [out] */ int *y,
            /* [out] */ int *width,
            /* [out] */ int *height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_unclippedSubstringBounds( 
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int endIndex,
            /* [out] */ int *x,
            /* [out] */ int *y,
            /* [out] */ int *width,
            /* [out] */ int *height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE scrollToSubstring( 
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int endIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_fontFamily( 
            /* [retval][out] */ BSTR *fontFamily) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISimpleDOMTextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISimpleDOMText * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISimpleDOMText * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISimpleDOMText * This);
        
        DECLSPEC_XFGVIRT(ISimpleDOMText, get_domText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_domText )( 
            ISimpleDOMText * This,
            /* [retval][out] */ BSTR *domText);
        
        DECLSPEC_XFGVIRT(ISimpleDOMText, get_clippedSubstringBounds)
        HRESULT ( STDMETHODCALLTYPE *get_clippedSubstringBounds )( 
            ISimpleDOMText * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int endIndex,
            /* [out] */ int *x,
            /* [out] */ int *y,
            /* [out] */ int *width,
            /* [out] */ int *height);
        
        DECLSPEC_XFGVIRT(ISimpleDOMText, get_unclippedSubstringBounds)
        HRESULT ( STDMETHODCALLTYPE *get_unclippedSubstringBounds )( 
            ISimpleDOMText * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int endIndex,
            /* [out] */ int *x,
            /* [out] */ int *y,
            /* [out] */ int *width,
            /* [out] */ int *height);
        
        DECLSPEC_XFGVIRT(ISimpleDOMText, scrollToSubstring)
        HRESULT ( STDMETHODCALLTYPE *scrollToSubstring )( 
            ISimpleDOMText * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int endIndex);
        
        DECLSPEC_XFGVIRT(ISimpleDOMText, get_fontFamily)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_fontFamily )( 
            ISimpleDOMText * This,
            /* [retval][out] */ BSTR *fontFamily);
        
        END_INTERFACE
    } ISimpleDOMTextVtbl;

    interface ISimpleDOMText
    {
        CONST_VTBL struct ISimpleDOMTextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISimpleDOMText_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISimpleDOMText_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISimpleDOMText_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISimpleDOMText_get_domText(This,domText)	\
    ( (This)->lpVtbl -> get_domText(This,domText) ) 

#define ISimpleDOMText_get_clippedSubstringBounds(This,startIndex,endIndex,x,y,width,height)	\
    ( (This)->lpVtbl -> get_clippedSubstringBounds(This,startIndex,endIndex,x,y,width,height) ) 

#define ISimpleDOMText_get_unclippedSubstringBounds(This,startIndex,endIndex,x,y,width,height)	\
    ( (This)->lpVtbl -> get_unclippedSubstringBounds(This,startIndex,endIndex,x,y,width,height) ) 

#define ISimpleDOMText_scrollToSubstring(This,startIndex,endIndex)	\
    ( (This)->lpVtbl -> scrollToSubstring(This,startIndex,endIndex) ) 

#define ISimpleDOMText_get_fontFamily(This,fontFamily)	\
    ( (This)->lpVtbl -> get_fontFamily(This,fontFamily) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISimpleDOMText_INTERFACE_DEFINED__ */


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


