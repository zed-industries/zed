

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../third_party/isimpledom/ISimpleDOMNode.idl:
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

#ifndef __ISimpleDOMNode_h__
#define __ISimpleDOMNode_h__

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

#ifndef __ISimpleDOMNode_FWD_DEFINED__
#define __ISimpleDOMNode_FWD_DEFINED__
typedef interface ISimpleDOMNode ISimpleDOMNode;

#endif 	/* __ISimpleDOMNode_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ISimpleDOMNode_0000_0000 */
/* [local] */ 

///////////////////////////////////////////////////////////////////////////////////////////////////////
//
// ISimpleDOMNode
// ---------------------------------------------------------------------------------------------------=
// An interface that extends MSAA's IAccessible to provide readonly DOM node information via cross-process COM.
//
// @STATUS UNDER_REVIEW
//
// get_nodeInfo(
//  /* [out] */ BSTR  *nodeName,   // For elements, this is the tag name
//  /* [out] */ short  *nameSpaceID,
//  /* [out] */ BSTR  *nodeValue, 
//  /* [out] */ unsigned int    *numChildren); 
//  /* [out] */ unsigned int    *uniqueID;  // In Win32 accessible events we generate, the target's childID matches to this
//  /* [out] */ unsigned short  *nodeType,
// ---------------------------------------------------------------------------------------------------=
// Get the basic information about a node.
// The namespace ID can be mapped to an URI using nsISimpleDOMDocument::get_nameSpaceURIForID()
//
// get_attributes(
//  /* [in]  */ unsigned short maxAttribs,
//  /* [out] */ unsigned short  *numAttribs,
//  /* [out] */ BSTR  *attribNames,
//  /* [out] */ short *nameSpaceID,
//  /* [out] */ BSTR  *attribValues);
// ---------------------------------------------------------------------------------------------------=
// Returns 3 arrays - the attribute names and values, and a namespace ID for each
// If the namespace ID is 0, it's the same namespace as the node's namespace
//
// get_attributesForNames(
//  /* [in] */ unsigned short numAttribs,
//  /* [in] */ BSTR   *attribNames,
//  /* [in] */ short  *nameSpaceID,
//  /* [out] */ BSTR  *attribValues);
// ---------------------------------------------------------------------------------------------------=
// Takes 2 arrays - the attribute names and namespace IDs, and returns an array of corresponding values
// If the namespace ID is 0, it's the same namespace as the node's namespace
//
// computedStyle(  
//  /* [in]  */ unsigned short maxStyleProperties,
//  /* [out] */ unsigned short *numStyleProperties, 
//  /* [in]  */ boolean useAlternateView,  // If TRUE, returns properites for media as set in nsIDOMDocument::set_alternateViewMediaTypes
//  /* [out] */ BSTR *styleProperties, 
//  /* [out] */ BSTR *styleValues);
// ---------------------------------------------------------------------------------------------------=
// Returns 2 arrays -- the style properties and their values
//  useAlternateView=FALSE: gets properties for the default media type (usually screen)
//  useAlternateView=TRUE: properties for media types set w/ nsIDOMSimpleDocument::set_alternateViewMediaTypes()
//
// computedStyleForProperties(  
//  /* [in] */  unsigned short numStyleProperties, 
//  /* [in] */  boolean useAlternateView,  // If TRUE, returns properites for media as set in nsIDOMDocument::set_alternateViewMediaTypes
//  /* [in] */  BSTR *styleProperties, 
//  /* [out] */ BSTR *styleValues);
// ---------------------------------------------------------------------------------------------------=
// Scroll the current view so that this dom node is visible.
//  placeTopLeft=TRUE: scroll until the top left corner of the dom node is at the top left corner of the view.
//  placeTopLeft=FALSE: scroll minimally to make the dom node visible. Don't scroll at all if already visible.
//
// scrollTo( 
//  /* [in] */ boolean placeTopLeft); 
// ---------------------------------------------------------------------------------------------------=
// Returns style property values for those properties in the styleProperties [in] array
// Returns 2 arrays -- the style properties and their values
//  useAlternateView=FALSE: gets properties for the default media type (usually screen)
//  useAlternateView=TRUE: properties for media types set w/ nsIDOMSimpleDocument::set_alternateViewMediaTypes()
//
// get_parentNode     (/* [out] */ ISimpleDOMNode **newNodePtr);
// get_firstChild     (/* [out] */ ISimpleDOMNode **newNodePtr);
// get_lastChild      (/* [out] */ ISimpleDOMNode **newNodePtr);
// get_previousSibling(/* [out] */ ISimpleDOMNode **newNodePtr);
// get_nextSibling    (/* [out] */ ISimpleDOMNode **newNodePtr);
// get_childAt        (/* [in] */ unsigned childIndex, /* [out] */ ISimpleDOMNode **newNodePtr);
// ---------------------------------------------------------------------------------------------------=
// DOM navigation - get a different node.
//
// get_innerHTML(/* [out] */ BSTR *htmlText);
// ---------------------------------------------------------------------------------------------------=
// Returns HTML of this DOM node's subtree. Does not include the start and end tag for this node/element.
//
//
// get_localInterface(/* [out] */ void **localInterface);
// ---------------------------------------------------------------------------------------------------=
// Only available in Gecko's process - casts to an XPCOM nsIAccessNode interface pointer
//
//
// get_language(/* [out] */ BSTR *htmlText);
// ---------------------------------------------------------------------------------------------------=
// Returns the computed language for this node, or empty string if unknown.
//
//
///////////////////////////////////////////////////////////////////////////////////////////////////////


#define	DISPID_NODE_NODEINFO	( -5900 )

#define	DISPID_NODE_ATTRIBUTES	( -5901 )

#define	DISPID_NODE_ATTRIBUTESFORNAMES	( -5902 )

#define	DISPID_NODE_COMPSTYLE	( -5903 )

#define	DISPID_NODE_COMPSTYLEFORPROPS	( -5904 )

#define	DISPID_NODE_LANGUAGE	( -5905 )



extern RPC_IF_HANDLE __MIDL_itf_ISimpleDOMNode_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ISimpleDOMNode_0000_0000_v0_0_s_ifspec;

#ifndef __ISimpleDOMNode_INTERFACE_DEFINED__
#define __ISimpleDOMNode_INTERFACE_DEFINED__

/* interface ISimpleDOMNode */
/* [uuid][object] */ 

#define	NODETYPE_ELEMENT	( 1 )

#define	NODETYPE_ATTRIBUTE	( 2 )

#define	NODETYPE_TEXT	( 3 )

#define	NODETYPE_CDATA_SECTION	( 4 )

#define	NODETYPE_ENTITY_REFERENCE	( 5 )

#define	NODETYPE_ENTITY	( 6 )

#define	NODETYPE_PROCESSING_INSTRUCTION	( 7 )

#define	NODETYPE_COMMENT	( 8 )

#define	NODETYPE_DOCUMENT	( 9 )

#define	NODETYPE_DOCUMENT_TYPE	( 10 )

#define	NODETYPE_DOCUMENT_FRAGMENT	( 11 )

#define	NODETYPE_NOTATION	( 12 )


EXTERN_C const IID IID_ISimpleDOMNode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1814ceeb-49e2-407f-af99-fa755a7d2607")
    ISimpleDOMNode : public IUnknown
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_nodeInfo( 
            /* [out] */ BSTR *nodeName,
            /* [out] */ short *nameSpaceID,
            /* [out] */ BSTR *nodeValue,
            /* [out] */ unsigned int *numChildren,
            /* [out] */ unsigned int *uniqueID,
            /* [retval][out] */ unsigned short *nodeType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_attributes( 
            /* [in] */ unsigned short maxAttribs,
            /* [length_is][size_is][out] */ BSTR *attribNames,
            /* [length_is][size_is][out] */ short *nameSpaceID,
            /* [length_is][size_is][out] */ BSTR *attribValues,
            /* [retval][out] */ unsigned short *numAttribs) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_attributesForNames( 
            /* [in] */ unsigned short numAttribs,
            /* [length_is][size_is][in] */ BSTR *attribNames,
            /* [length_is][size_is][in] */ short *nameSpaceID,
            /* [length_is][size_is][retval][out] */ BSTR *attribValues) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_computedStyle( 
            /* [in] */ unsigned short maxStyleProperties,
            /* [in] */ boolean useAlternateView,
            /* [length_is][size_is][out] */ BSTR *styleProperties,
            /* [length_is][size_is][out] */ BSTR *styleValues,
            /* [retval][out] */ unsigned short *numStyleProperties) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_computedStyleForProperties( 
            /* [in] */ unsigned short numStyleProperties,
            /* [in] */ boolean useAlternateView,
            /* [length_is][size_is][in] */ BSTR *styleProperties,
            /* [length_is][size_is][retval][out] */ BSTR *styleValues) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE scrollTo( 
            /* [in] */ boolean placeTopLeft) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_parentNode( 
            /* [retval][out] */ ISimpleDOMNode **node) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_firstChild( 
            /* [retval][out] */ ISimpleDOMNode **node) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_lastChild( 
            /* [retval][out] */ ISimpleDOMNode **node) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_previousSibling( 
            /* [retval][out] */ ISimpleDOMNode **node) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nextSibling( 
            /* [retval][out] */ ISimpleDOMNode **node) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_childAt( 
            /* [in] */ unsigned int childIndex,
            /* [retval][out] */ ISimpleDOMNode **node) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_innerHTML( 
            /* [retval][out] */ BSTR *innerHTML) = 0;
        
        virtual /* [local][propget] */ HRESULT STDMETHODCALLTYPE get_localInterface( 
            /* [retval][out] */ void **localInterface) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_language( 
            /* [retval][out] */ BSTR *language) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISimpleDOMNodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISimpleDOMNode * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISimpleDOMNode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISimpleDOMNode * This);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_nodeInfo)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeInfo )( 
            ISimpleDOMNode * This,
            /* [out] */ BSTR *nodeName,
            /* [out] */ short *nameSpaceID,
            /* [out] */ BSTR *nodeValue,
            /* [out] */ unsigned int *numChildren,
            /* [out] */ unsigned int *uniqueID,
            /* [retval][out] */ unsigned short *nodeType);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_attributes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            ISimpleDOMNode * This,
            /* [in] */ unsigned short maxAttribs,
            /* [length_is][size_is][out] */ BSTR *attribNames,
            /* [length_is][size_is][out] */ short *nameSpaceID,
            /* [length_is][size_is][out] */ BSTR *attribValues,
            /* [retval][out] */ unsigned short *numAttribs);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_attributesForNames)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributesForNames )( 
            ISimpleDOMNode * This,
            /* [in] */ unsigned short numAttribs,
            /* [length_is][size_is][in] */ BSTR *attribNames,
            /* [length_is][size_is][in] */ short *nameSpaceID,
            /* [length_is][size_is][retval][out] */ BSTR *attribValues);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_computedStyle)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_computedStyle )( 
            ISimpleDOMNode * This,
            /* [in] */ unsigned short maxStyleProperties,
            /* [in] */ boolean useAlternateView,
            /* [length_is][size_is][out] */ BSTR *styleProperties,
            /* [length_is][size_is][out] */ BSTR *styleValues,
            /* [retval][out] */ unsigned short *numStyleProperties);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_computedStyleForProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_computedStyleForProperties )( 
            ISimpleDOMNode * This,
            /* [in] */ unsigned short numStyleProperties,
            /* [in] */ boolean useAlternateView,
            /* [length_is][size_is][in] */ BSTR *styleProperties,
            /* [length_is][size_is][retval][out] */ BSTR *styleValues);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, scrollTo)
        HRESULT ( STDMETHODCALLTYPE *scrollTo )( 
            ISimpleDOMNode * This,
            /* [in] */ boolean placeTopLeft);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_parentNode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            ISimpleDOMNode * This,
            /* [retval][out] */ ISimpleDOMNode **node);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_firstChild)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            ISimpleDOMNode * This,
            /* [retval][out] */ ISimpleDOMNode **node);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_lastChild)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            ISimpleDOMNode * This,
            /* [retval][out] */ ISimpleDOMNode **node);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_previousSibling)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            ISimpleDOMNode * This,
            /* [retval][out] */ ISimpleDOMNode **node);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_nextSibling)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            ISimpleDOMNode * This,
            /* [retval][out] */ ISimpleDOMNode **node);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_childAt)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_childAt )( 
            ISimpleDOMNode * This,
            /* [in] */ unsigned int childIndex,
            /* [retval][out] */ ISimpleDOMNode **node);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_innerHTML)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_innerHTML )( 
            ISimpleDOMNode * This,
            /* [retval][out] */ BSTR *innerHTML);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_localInterface)
        /* [local][propget] */ HRESULT ( STDMETHODCALLTYPE *get_localInterface )( 
            ISimpleDOMNode * This,
            /* [retval][out] */ void **localInterface);
        
        DECLSPEC_XFGVIRT(ISimpleDOMNode, get_language)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_language )( 
            ISimpleDOMNode * This,
            /* [retval][out] */ BSTR *language);
        
        END_INTERFACE
    } ISimpleDOMNodeVtbl;

    interface ISimpleDOMNode
    {
        CONST_VTBL struct ISimpleDOMNodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISimpleDOMNode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISimpleDOMNode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISimpleDOMNode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISimpleDOMNode_get_nodeInfo(This,nodeName,nameSpaceID,nodeValue,numChildren,uniqueID,nodeType)	\
    ( (This)->lpVtbl -> get_nodeInfo(This,nodeName,nameSpaceID,nodeValue,numChildren,uniqueID,nodeType) ) 

#define ISimpleDOMNode_get_attributes(This,maxAttribs,attribNames,nameSpaceID,attribValues,numAttribs)	\
    ( (This)->lpVtbl -> get_attributes(This,maxAttribs,attribNames,nameSpaceID,attribValues,numAttribs) ) 

#define ISimpleDOMNode_get_attributesForNames(This,numAttribs,attribNames,nameSpaceID,attribValues)	\
    ( (This)->lpVtbl -> get_attributesForNames(This,numAttribs,attribNames,nameSpaceID,attribValues) ) 

#define ISimpleDOMNode_get_computedStyle(This,maxStyleProperties,useAlternateView,styleProperties,styleValues,numStyleProperties)	\
    ( (This)->lpVtbl -> get_computedStyle(This,maxStyleProperties,useAlternateView,styleProperties,styleValues,numStyleProperties) ) 

#define ISimpleDOMNode_get_computedStyleForProperties(This,numStyleProperties,useAlternateView,styleProperties,styleValues)	\
    ( (This)->lpVtbl -> get_computedStyleForProperties(This,numStyleProperties,useAlternateView,styleProperties,styleValues) ) 

#define ISimpleDOMNode_scrollTo(This,placeTopLeft)	\
    ( (This)->lpVtbl -> scrollTo(This,placeTopLeft) ) 

#define ISimpleDOMNode_get_parentNode(This,node)	\
    ( (This)->lpVtbl -> get_parentNode(This,node) ) 

#define ISimpleDOMNode_get_firstChild(This,node)	\
    ( (This)->lpVtbl -> get_firstChild(This,node) ) 

#define ISimpleDOMNode_get_lastChild(This,node)	\
    ( (This)->lpVtbl -> get_lastChild(This,node) ) 

#define ISimpleDOMNode_get_previousSibling(This,node)	\
    ( (This)->lpVtbl -> get_previousSibling(This,node) ) 

#define ISimpleDOMNode_get_nextSibling(This,node)	\
    ( (This)->lpVtbl -> get_nextSibling(This,node) ) 

#define ISimpleDOMNode_get_childAt(This,childIndex,node)	\
    ( (This)->lpVtbl -> get_childAt(This,childIndex,node) ) 

#define ISimpleDOMNode_get_innerHTML(This,innerHTML)	\
    ( (This)->lpVtbl -> get_innerHTML(This,innerHTML) ) 

#define ISimpleDOMNode_get_localInterface(This,localInterface)	\
    ( (This)->lpVtbl -> get_localInterface(This,localInterface) ) 

#define ISimpleDOMNode_get_language(This,language)	\
    ( (This)->lpVtbl -> get_language(This,language) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISimpleDOMNode_INTERFACE_DEFINED__ */


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


