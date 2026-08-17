

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../third_party/iaccessible2/ia2_api_all.idl:
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

#ifndef __ia2_api_all_h__
#define __ia2_api_all_h__

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

#ifndef __IAccessibleRelation_FWD_DEFINED__
#define __IAccessibleRelation_FWD_DEFINED__
typedef interface IAccessibleRelation IAccessibleRelation;

#endif 	/* __IAccessibleRelation_FWD_DEFINED__ */


#ifndef __IAccessibleAction_FWD_DEFINED__
#define __IAccessibleAction_FWD_DEFINED__
typedef interface IAccessibleAction IAccessibleAction;

#endif 	/* __IAccessibleAction_FWD_DEFINED__ */


#ifndef __IAccessible2_FWD_DEFINED__
#define __IAccessible2_FWD_DEFINED__
typedef interface IAccessible2 IAccessible2;

#endif 	/* __IAccessible2_FWD_DEFINED__ */


#ifndef __IAccessible2_2_FWD_DEFINED__
#define __IAccessible2_2_FWD_DEFINED__
typedef interface IAccessible2_2 IAccessible2_2;

#endif 	/* __IAccessible2_2_FWD_DEFINED__ */


#ifndef __IAccessible2_3_FWD_DEFINED__
#define __IAccessible2_3_FWD_DEFINED__
typedef interface IAccessible2_3 IAccessible2_3;

#endif 	/* __IAccessible2_3_FWD_DEFINED__ */


#ifndef __IAccessible2_4_FWD_DEFINED__
#define __IAccessible2_4_FWD_DEFINED__
typedef interface IAccessible2_4 IAccessible2_4;

#endif 	/* __IAccessible2_4_FWD_DEFINED__ */


#ifndef __IAccessibleComponent_FWD_DEFINED__
#define __IAccessibleComponent_FWD_DEFINED__
typedef interface IAccessibleComponent IAccessibleComponent;

#endif 	/* __IAccessibleComponent_FWD_DEFINED__ */


#ifndef __IAccessibleValue_FWD_DEFINED__
#define __IAccessibleValue_FWD_DEFINED__
typedef interface IAccessibleValue IAccessibleValue;

#endif 	/* __IAccessibleValue_FWD_DEFINED__ */


#ifndef __IAccessibleText_FWD_DEFINED__
#define __IAccessibleText_FWD_DEFINED__
typedef interface IAccessibleText IAccessibleText;

#endif 	/* __IAccessibleText_FWD_DEFINED__ */


#ifndef __IAccessibleText2_FWD_DEFINED__
#define __IAccessibleText2_FWD_DEFINED__
typedef interface IAccessibleText2 IAccessibleText2;

#endif 	/* __IAccessibleText2_FWD_DEFINED__ */


#ifndef __IAccessibleTextSelectionContainer_FWD_DEFINED__
#define __IAccessibleTextSelectionContainer_FWD_DEFINED__
typedef interface IAccessibleTextSelectionContainer IAccessibleTextSelectionContainer;

#endif 	/* __IAccessibleTextSelectionContainer_FWD_DEFINED__ */


#ifndef __IAccessibleEditableText_FWD_DEFINED__
#define __IAccessibleEditableText_FWD_DEFINED__
typedef interface IAccessibleEditableText IAccessibleEditableText;

#endif 	/* __IAccessibleEditableText_FWD_DEFINED__ */


#ifndef __IAccessibleHyperlink_FWD_DEFINED__
#define __IAccessibleHyperlink_FWD_DEFINED__
typedef interface IAccessibleHyperlink IAccessibleHyperlink;

#endif 	/* __IAccessibleHyperlink_FWD_DEFINED__ */


#ifndef __IAccessibleHypertext_FWD_DEFINED__
#define __IAccessibleHypertext_FWD_DEFINED__
typedef interface IAccessibleHypertext IAccessibleHypertext;

#endif 	/* __IAccessibleHypertext_FWD_DEFINED__ */


#ifndef __IAccessibleHypertext2_FWD_DEFINED__
#define __IAccessibleHypertext2_FWD_DEFINED__
typedef interface IAccessibleHypertext2 IAccessibleHypertext2;

#endif 	/* __IAccessibleHypertext2_FWD_DEFINED__ */


#ifndef __IAccessibleTable_FWD_DEFINED__
#define __IAccessibleTable_FWD_DEFINED__
typedef interface IAccessibleTable IAccessibleTable;

#endif 	/* __IAccessibleTable_FWD_DEFINED__ */


#ifndef __IAccessibleTable2_FWD_DEFINED__
#define __IAccessibleTable2_FWD_DEFINED__
typedef interface IAccessibleTable2 IAccessibleTable2;

#endif 	/* __IAccessibleTable2_FWD_DEFINED__ */


#ifndef __IAccessibleTableCell_FWD_DEFINED__
#define __IAccessibleTableCell_FWD_DEFINED__
typedef interface IAccessibleTableCell IAccessibleTableCell;

#endif 	/* __IAccessibleTableCell_FWD_DEFINED__ */


#ifndef __IAccessibleImage_FWD_DEFINED__
#define __IAccessibleImage_FWD_DEFINED__
typedef interface IAccessibleImage IAccessibleImage;

#endif 	/* __IAccessibleImage_FWD_DEFINED__ */


#ifndef __IAccessibleApplication_FWD_DEFINED__
#define __IAccessibleApplication_FWD_DEFINED__
typedef interface IAccessibleApplication IAccessibleApplication;

#endif 	/* __IAccessibleApplication_FWD_DEFINED__ */


#ifndef __IAccessibleDocument_FWD_DEFINED__
#define __IAccessibleDocument_FWD_DEFINED__
typedef interface IAccessibleDocument IAccessibleDocument;

#endif 	/* __IAccessibleDocument_FWD_DEFINED__ */


#ifndef __IAccessible2_FWD_DEFINED__
#define __IAccessible2_FWD_DEFINED__
typedef interface IAccessible2 IAccessible2;

#endif 	/* __IAccessible2_FWD_DEFINED__ */


#ifndef __IAccessible2_2_FWD_DEFINED__
#define __IAccessible2_2_FWD_DEFINED__
typedef interface IAccessible2_2 IAccessible2_2;

#endif 	/* __IAccessible2_2_FWD_DEFINED__ */


#ifndef __IAccessible2_3_FWD_DEFINED__
#define __IAccessible2_3_FWD_DEFINED__
typedef interface IAccessible2_3 IAccessible2_3;

#endif 	/* __IAccessible2_3_FWD_DEFINED__ */


#ifndef __IAccessibleAction_FWD_DEFINED__
#define __IAccessibleAction_FWD_DEFINED__
typedef interface IAccessibleAction IAccessibleAction;

#endif 	/* __IAccessibleAction_FWD_DEFINED__ */


#ifndef __IAccessibleApplication_FWD_DEFINED__
#define __IAccessibleApplication_FWD_DEFINED__
typedef interface IAccessibleApplication IAccessibleApplication;

#endif 	/* __IAccessibleApplication_FWD_DEFINED__ */


#ifndef __IAccessibleComponent_FWD_DEFINED__
#define __IAccessibleComponent_FWD_DEFINED__
typedef interface IAccessibleComponent IAccessibleComponent;

#endif 	/* __IAccessibleComponent_FWD_DEFINED__ */


#ifndef __IAccessibleDocument_FWD_DEFINED__
#define __IAccessibleDocument_FWD_DEFINED__
typedef interface IAccessibleDocument IAccessibleDocument;

#endif 	/* __IAccessibleDocument_FWD_DEFINED__ */


#ifndef __IAccessibleEditableText_FWD_DEFINED__
#define __IAccessibleEditableText_FWD_DEFINED__
typedef interface IAccessibleEditableText IAccessibleEditableText;

#endif 	/* __IAccessibleEditableText_FWD_DEFINED__ */


#ifndef __IAccessibleHyperlink_FWD_DEFINED__
#define __IAccessibleHyperlink_FWD_DEFINED__
typedef interface IAccessibleHyperlink IAccessibleHyperlink;

#endif 	/* __IAccessibleHyperlink_FWD_DEFINED__ */


#ifndef __IAccessibleText_FWD_DEFINED__
#define __IAccessibleText_FWD_DEFINED__
typedef interface IAccessibleText IAccessibleText;

#endif 	/* __IAccessibleText_FWD_DEFINED__ */


#ifndef __IAccessibleHypertext_FWD_DEFINED__
#define __IAccessibleHypertext_FWD_DEFINED__
typedef interface IAccessibleHypertext IAccessibleHypertext;

#endif 	/* __IAccessibleHypertext_FWD_DEFINED__ */


#ifndef __IAccessibleHypertext2_FWD_DEFINED__
#define __IAccessibleHypertext2_FWD_DEFINED__
typedef interface IAccessibleHypertext2 IAccessibleHypertext2;

#endif 	/* __IAccessibleHypertext2_FWD_DEFINED__ */


#ifndef __IAccessibleImage_FWD_DEFINED__
#define __IAccessibleImage_FWD_DEFINED__
typedef interface IAccessibleImage IAccessibleImage;

#endif 	/* __IAccessibleImage_FWD_DEFINED__ */


#ifndef __IAccessibleRelation_FWD_DEFINED__
#define __IAccessibleRelation_FWD_DEFINED__
typedef interface IAccessibleRelation IAccessibleRelation;

#endif 	/* __IAccessibleRelation_FWD_DEFINED__ */


#ifndef __IAccessibleTable_FWD_DEFINED__
#define __IAccessibleTable_FWD_DEFINED__
typedef interface IAccessibleTable IAccessibleTable;

#endif 	/* __IAccessibleTable_FWD_DEFINED__ */


#ifndef __IAccessibleTable2_FWD_DEFINED__
#define __IAccessibleTable2_FWD_DEFINED__
typedef interface IAccessibleTable2 IAccessibleTable2;

#endif 	/* __IAccessibleTable2_FWD_DEFINED__ */


#ifndef __IAccessibleTableCell_FWD_DEFINED__
#define __IAccessibleTableCell_FWD_DEFINED__
typedef interface IAccessibleTableCell IAccessibleTableCell;

#endif 	/* __IAccessibleTableCell_FWD_DEFINED__ */


#ifndef __IAccessibleText2_FWD_DEFINED__
#define __IAccessibleText2_FWD_DEFINED__
typedef interface IAccessibleText2 IAccessibleText2;

#endif 	/* __IAccessibleText2_FWD_DEFINED__ */


#ifndef __IAccessibleTextSelectionContainer_FWD_DEFINED__
#define __IAccessibleTextSelectionContainer_FWD_DEFINED__
typedef interface IAccessibleTextSelectionContainer IAccessibleTextSelectionContainer;

#endif 	/* __IAccessibleTextSelectionContainer_FWD_DEFINED__ */


#ifndef __IAccessibleValue_FWD_DEFINED__
#define __IAccessibleValue_FWD_DEFINED__
typedef interface IAccessibleValue IAccessibleValue;

#endif 	/* __IAccessibleValue_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oaidl.h"
#include "oleacc.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ia2_api_all_0000_0000 */
/* [local] */ 


enum IA2ScrollType
    {
        IA2_SCROLL_TYPE_TOP_LEFT	= 0,
        IA2_SCROLL_TYPE_BOTTOM_RIGHT	= ( IA2_SCROLL_TYPE_TOP_LEFT + 1 ) ,
        IA2_SCROLL_TYPE_TOP_EDGE	= ( IA2_SCROLL_TYPE_BOTTOM_RIGHT + 1 ) ,
        IA2_SCROLL_TYPE_BOTTOM_EDGE	= ( IA2_SCROLL_TYPE_TOP_EDGE + 1 ) ,
        IA2_SCROLL_TYPE_LEFT_EDGE	= ( IA2_SCROLL_TYPE_BOTTOM_EDGE + 1 ) ,
        IA2_SCROLL_TYPE_RIGHT_EDGE	= ( IA2_SCROLL_TYPE_LEFT_EDGE + 1 ) ,
        IA2_SCROLL_TYPE_ANYWHERE	= ( IA2_SCROLL_TYPE_RIGHT_EDGE + 1 ) 
    } ;

enum IA2CoordinateType
    {
        IA2_COORDTYPE_SCREEN_RELATIVE	= 0,
        IA2_COORDTYPE_PARENT_RELATIVE	= ( IA2_COORDTYPE_SCREEN_RELATIVE + 1 ) 
    } ;

enum IA2TextSpecialOffsets
    {
        IA2_TEXT_OFFSET_LENGTH	= -1,
        IA2_TEXT_OFFSET_CARET	= -2
    } ;

enum IA2TableModelChangeType
    {
        IA2_TABLE_MODEL_CHANGE_INSERT	= 0,
        IA2_TABLE_MODEL_CHANGE_DELETE	= ( IA2_TABLE_MODEL_CHANGE_INSERT + 1 ) ,
        IA2_TABLE_MODEL_CHANGE_UPDATE	= ( IA2_TABLE_MODEL_CHANGE_DELETE + 1 ) 
    } ;
typedef struct IA2TableModelChange
    {
    enum IA2TableModelChangeType type;
    long firstRow;
    long lastRow;
    long firstColumn;
    long lastColumn;
    } 	IA2TableModelChange;

#define	IA2_RELATION_CONTAINING_APPLICATION	( L"containingApplication" )

#define	IA2_RELATION_CONTAINING_DOCUMENT	( L"containingDocument" )

#define	IA2_RELATION_CONTAINING_TAB_PANE	( L"containingTabPane" )

#define	IA2_RELATION_CONTAINING_WINDOW	( L"containingWindow" )

#define	IA2_RELATION_CONTROLLED_BY	( L"controlledBy" )

#define	IA2_RELATION_CONTROLLER_FOR	( L"controllerFor" )

#define	IA2_RELATION_DESCRIBED_BY	( L"describedBy" )

#define	IA2_RELATION_DESCRIPTION_FOR	( L"descriptionFor" )

#define	IA2_RELATION_EMBEDDED_BY	( L"embeddedBy" )

#define	IA2_RELATION_EMBEDS	( L"embeds" )

#define	IA2_RELATION_FLOWS_FROM	( L"flowsFrom" )

#define	IA2_RELATION_FLOWS_TO	( L"flowsTo" )

#define	IA2_RELATION_LABEL_FOR	( L"labelFor" )

#define	IA2_RELATION_LABELED_BY	( L"labelledBy" )

#define	IA2_RELATION_LABELLED_BY	( L"labelledBy" )

#define	IA2_RELATION_MEMBER_OF	( L"memberOf" )

#define	IA2_RELATION_NEXT_TABBABLE	( L"nextTabbable" )

#define	IA2_RELATION_NODE_CHILD_OF	( L"nodeChildOf" )

#define	IA2_RELATION_NODE_PARENT_OF	( L"nodeParentOf" )

#define	IA2_RELATION_PARENT_WINDOW_OF	( L"parentWindowOf" )

#define	IA2_RELATION_POPUP_FOR	( L"popupFor" )

#define	IA2_RELATION_PREVIOUS_TABBABLE	( L"previousTabbable" )

#define	IA2_RELATION_SUBWINDOW_OF	( L"subwindowOf" )

#define	IA2_RELATION_DETAILS	( L"details" )

#define	IA2_RELATION_DETAILS_FOR	( L"detailsFor" )

#define	IA2_RELATION_ERROR	( L"error" )

#define	IA2_RELATION_ERROR_FOR	( L"errorFor" )



extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0000_v0_0_s_ifspec;

#ifndef __IAccessibleRelation_INTERFACE_DEFINED__
#define __IAccessibleRelation_INTERFACE_DEFINED__

/* interface IAccessibleRelation */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleRelation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7CDF86EE-C3DA-496a-BDA4-281B336E1FDC")
    IAccessibleRelation : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_relationType( 
            /* [retval][out] */ BSTR *relationType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_localizedRelationType( 
            /* [retval][out] */ BSTR *localizedRelationType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nTargets( 
            /* [retval][out] */ long *nTargets) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_target( 
            /* [in] */ long targetIndex,
            /* [retval][out] */ IUnknown **target) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_targets( 
            /* [in] */ long maxTargets,
            /* [length_is][size_is][out] */ IUnknown **targets,
            /* [retval][out] */ long *nTargets) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleRelationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleRelation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleRelation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleRelation * This);
        
        DECLSPEC_XFGVIRT(IAccessibleRelation, get_relationType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relationType )( 
            IAccessibleRelation * This,
            /* [retval][out] */ BSTR *relationType);
        
        DECLSPEC_XFGVIRT(IAccessibleRelation, get_localizedRelationType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedRelationType )( 
            IAccessibleRelation * This,
            /* [retval][out] */ BSTR *localizedRelationType);
        
        DECLSPEC_XFGVIRT(IAccessibleRelation, get_nTargets)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nTargets )( 
            IAccessibleRelation * This,
            /* [retval][out] */ long *nTargets);
        
        DECLSPEC_XFGVIRT(IAccessibleRelation, get_target)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_target )( 
            IAccessibleRelation * This,
            /* [in] */ long targetIndex,
            /* [retval][out] */ IUnknown **target);
        
        DECLSPEC_XFGVIRT(IAccessibleRelation, get_targets)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_targets )( 
            IAccessibleRelation * This,
            /* [in] */ long maxTargets,
            /* [length_is][size_is][out] */ IUnknown **targets,
            /* [retval][out] */ long *nTargets);
        
        END_INTERFACE
    } IAccessibleRelationVtbl;

    interface IAccessibleRelation
    {
        CONST_VTBL struct IAccessibleRelationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleRelation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleRelation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleRelation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleRelation_get_relationType(This,relationType)	\
    ( (This)->lpVtbl -> get_relationType(This,relationType) ) 

#define IAccessibleRelation_get_localizedRelationType(This,localizedRelationType)	\
    ( (This)->lpVtbl -> get_localizedRelationType(This,localizedRelationType) ) 

#define IAccessibleRelation_get_nTargets(This,nTargets)	\
    ( (This)->lpVtbl -> get_nTargets(This,nTargets) ) 

#define IAccessibleRelation_get_target(This,targetIndex,target)	\
    ( (This)->lpVtbl -> get_target(This,targetIndex,target) ) 

#define IAccessibleRelation_get_targets(This,maxTargets,targets,nTargets)	\
    ( (This)->lpVtbl -> get_targets(This,maxTargets,targets,nTargets) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleRelation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ia2_api_all_0000_0001 */
/* [local] */ 


enum IA2Actions
    {
        IA2_ACTION_OPEN	= -1,
        IA2_ACTION_COMPLETE	= -2,
        IA2_ACTION_CLOSE	= -3
    } ;


extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0001_v0_0_s_ifspec;

#ifndef __IAccessibleAction_INTERFACE_DEFINED__
#define __IAccessibleAction_INTERFACE_DEFINED__

/* interface IAccessibleAction */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B70D9F59-3B5A-4dba-AB9E-22012F607DF5")
    IAccessibleAction : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE nActions( 
            /* [retval][out] */ long *nActions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE doAction( 
            /* [in] */ long actionIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_description( 
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_keyBinding( 
            /* [in] */ long actionIndex,
            /* [in] */ long nMaxBindings,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **keyBindings,
            /* [retval][out] */ long *nBindings) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_name( 
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_localizedName( 
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *localizedName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleAction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleAction * This);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, nActions)
        HRESULT ( STDMETHODCALLTYPE *nActions )( 
            IAccessibleAction * This,
            /* [retval][out] */ long *nActions);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, doAction)
        HRESULT ( STDMETHODCALLTYPE *doAction )( 
            IAccessibleAction * This,
            /* [in] */ long actionIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, get_description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_description )( 
            IAccessibleAction * This,
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, get_keyBinding)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_keyBinding )( 
            IAccessibleAction * This,
            /* [in] */ long actionIndex,
            /* [in] */ long nMaxBindings,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **keyBindings,
            /* [retval][out] */ long *nBindings);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, get_name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            IAccessibleAction * This,
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, get_localizedName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedName )( 
            IAccessibleAction * This,
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *localizedName);
        
        END_INTERFACE
    } IAccessibleActionVtbl;

    interface IAccessibleAction
    {
        CONST_VTBL struct IAccessibleActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleAction_nActions(This,nActions)	\
    ( (This)->lpVtbl -> nActions(This,nActions) ) 

#define IAccessibleAction_doAction(This,actionIndex)	\
    ( (This)->lpVtbl -> doAction(This,actionIndex) ) 

#define IAccessibleAction_get_description(This,actionIndex,description)	\
    ( (This)->lpVtbl -> get_description(This,actionIndex,description) ) 

#define IAccessibleAction_get_keyBinding(This,actionIndex,nMaxBindings,keyBindings,nBindings)	\
    ( (This)->lpVtbl -> get_keyBinding(This,actionIndex,nMaxBindings,keyBindings,nBindings) ) 

#define IAccessibleAction_get_name(This,actionIndex,name)	\
    ( (This)->lpVtbl -> get_name(This,actionIndex,name) ) 

#define IAccessibleAction_get_localizedName(This,actionIndex,localizedName)	\
    ( (This)->lpVtbl -> get_localizedName(This,actionIndex,localizedName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleAction_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ia2_api_all_0000_0002 */
/* [local] */ 


enum IA2Role
    {
        IA2_ROLE_UNKNOWN	= 0,
        IA2_ROLE_CANVAS	= 0x401,
        IA2_ROLE_CAPTION	= ( IA2_ROLE_CANVAS + 1 ) ,
        IA2_ROLE_CHECK_MENU_ITEM	= ( IA2_ROLE_CAPTION + 1 ) ,
        IA2_ROLE_COLOR_CHOOSER	= ( IA2_ROLE_CHECK_MENU_ITEM + 1 ) ,
        IA2_ROLE_DATE_EDITOR	= ( IA2_ROLE_COLOR_CHOOSER + 1 ) ,
        IA2_ROLE_DESKTOP_ICON	= ( IA2_ROLE_DATE_EDITOR + 1 ) ,
        IA2_ROLE_DESKTOP_PANE	= ( IA2_ROLE_DESKTOP_ICON + 1 ) ,
        IA2_ROLE_DIRECTORY_PANE	= ( IA2_ROLE_DESKTOP_PANE + 1 ) ,
        IA2_ROLE_EDITBAR	= ( IA2_ROLE_DIRECTORY_PANE + 1 ) ,
        IA2_ROLE_EMBEDDED_OBJECT	= ( IA2_ROLE_EDITBAR + 1 ) ,
        IA2_ROLE_ENDNOTE	= ( IA2_ROLE_EMBEDDED_OBJECT + 1 ) ,
        IA2_ROLE_FILE_CHOOSER	= ( IA2_ROLE_ENDNOTE + 1 ) ,
        IA2_ROLE_FONT_CHOOSER	= ( IA2_ROLE_FILE_CHOOSER + 1 ) ,
        IA2_ROLE_FOOTER	= ( IA2_ROLE_FONT_CHOOSER + 1 ) ,
        IA2_ROLE_FOOTNOTE	= ( IA2_ROLE_FOOTER + 1 ) ,
        IA2_ROLE_FORM	= ( IA2_ROLE_FOOTNOTE + 1 ) ,
        IA2_ROLE_FRAME	= ( IA2_ROLE_FORM + 1 ) ,
        IA2_ROLE_GLASS_PANE	= ( IA2_ROLE_FRAME + 1 ) ,
        IA2_ROLE_HEADER	= ( IA2_ROLE_GLASS_PANE + 1 ) ,
        IA2_ROLE_HEADING	= ( IA2_ROLE_HEADER + 1 ) ,
        IA2_ROLE_ICON	= ( IA2_ROLE_HEADING + 1 ) ,
        IA2_ROLE_IMAGE_MAP	= ( IA2_ROLE_ICON + 1 ) ,
        IA2_ROLE_INPUT_METHOD_WINDOW	= ( IA2_ROLE_IMAGE_MAP + 1 ) ,
        IA2_ROLE_INTERNAL_FRAME	= ( IA2_ROLE_INPUT_METHOD_WINDOW + 1 ) ,
        IA2_ROLE_LABEL	= ( IA2_ROLE_INTERNAL_FRAME + 1 ) ,
        IA2_ROLE_LAYERED_PANE	= ( IA2_ROLE_LABEL + 1 ) ,
        IA2_ROLE_NOTE	= ( IA2_ROLE_LAYERED_PANE + 1 ) ,
        IA2_ROLE_OPTION_PANE	= ( IA2_ROLE_NOTE + 1 ) ,
        IA2_ROLE_PAGE	= ( IA2_ROLE_OPTION_PANE + 1 ) ,
        IA2_ROLE_PARAGRAPH	= ( IA2_ROLE_PAGE + 1 ) ,
        IA2_ROLE_RADIO_MENU_ITEM	= ( IA2_ROLE_PARAGRAPH + 1 ) ,
        IA2_ROLE_REDUNDANT_OBJECT	= ( IA2_ROLE_RADIO_MENU_ITEM + 1 ) ,
        IA2_ROLE_ROOT_PANE	= ( IA2_ROLE_REDUNDANT_OBJECT + 1 ) ,
        IA2_ROLE_RULER	= ( IA2_ROLE_ROOT_PANE + 1 ) ,
        IA2_ROLE_SCROLL_PANE	= ( IA2_ROLE_RULER + 1 ) ,
        IA2_ROLE_SECTION	= ( IA2_ROLE_SCROLL_PANE + 1 ) ,
        IA2_ROLE_SHAPE	= ( IA2_ROLE_SECTION + 1 ) ,
        IA2_ROLE_SPLIT_PANE	= ( IA2_ROLE_SHAPE + 1 ) ,
        IA2_ROLE_TEAR_OFF_MENU	= ( IA2_ROLE_SPLIT_PANE + 1 ) ,
        IA2_ROLE_TERMINAL	= ( IA2_ROLE_TEAR_OFF_MENU + 1 ) ,
        IA2_ROLE_TEXT_FRAME	= ( IA2_ROLE_TERMINAL + 1 ) ,
        IA2_ROLE_TOGGLE_BUTTON	= ( IA2_ROLE_TEXT_FRAME + 1 ) ,
        IA2_ROLE_VIEW_PORT	= ( IA2_ROLE_TOGGLE_BUTTON + 1 ) ,
        IA2_ROLE_COMPLEMENTARY_CONTENT	= ( IA2_ROLE_VIEW_PORT + 1 ) ,
        IA2_ROLE_LANDMARK	= ( IA2_ROLE_COMPLEMENTARY_CONTENT + 1 ) ,
        IA2_ROLE_LEVEL_BAR	= ( IA2_ROLE_LANDMARK + 1 ) ,
        IA2_ROLE_CONTENT_DELETION	= ( IA2_ROLE_LEVEL_BAR + 1 ) ,
        IA2_ROLE_CONTENT_INSERTION	= ( IA2_ROLE_CONTENT_DELETION + 1 ) ,
        IA2_ROLE_BLOCK_QUOTE	= ( IA2_ROLE_CONTENT_INSERTION + 1 ) ,
        IA2_ROLE_MARK	= ( IA2_ROLE_BLOCK_QUOTE + 1 ) ,
        IA2_ROLE_SUGGESTION	= ( IA2_ROLE_MARK + 1 ) ,
        IA2_ROLE_COMMENT	= ( IA2_ROLE_SUGGESTION + 1 ) 
    } ;
typedef long AccessibleStates;


enum IA2States
    {
        IA2_STATE_ACTIVE	= 0x1,
        IA2_STATE_ARMED	= 0x2,
        IA2_STATE_DEFUNCT	= 0x4,
        IA2_STATE_EDITABLE	= 0x8,
        IA2_STATE_HORIZONTAL	= 0x10,
        IA2_STATE_ICONIFIED	= 0x20,
        IA2_STATE_INVALID_ENTRY	= 0x40,
        IA2_STATE_MANAGES_DESCENDANTS	= 0x80,
        IA2_STATE_MODAL	= 0x100,
        IA2_STATE_MULTI_LINE	= 0x200,
        IA2_STATE_OPAQUE	= 0x400,
        IA2_STATE_REQUIRED	= 0x800,
        IA2_STATE_SELECTABLE_TEXT	= 0x1000,
        IA2_STATE_SINGLE_LINE	= 0x2000,
        IA2_STATE_STALE	= 0x4000,
        IA2_STATE_SUPPORTS_AUTOCOMPLETION	= 0x8000,
        IA2_STATE_TRANSIENT	= 0x10000,
        IA2_STATE_VERTICAL	= 0x20000,
        IA2_STATE_CHECKABLE	= 0x40000,
        IA2_STATE_PINNED	= 0x80000
    } ;
typedef struct IA2Locale
    {
    BSTR language;
    BSTR country;
    BSTR variant;
    } 	IA2Locale;



extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0002_v0_0_s_ifspec;

#ifndef __IAccessible2_INTERFACE_DEFINED__
#define __IAccessible2_INTERFACE_DEFINED__

/* interface IAccessible2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessible2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E89F726E-C4F4-4c19-BB19-B647D7FA8478")
    IAccessible2 : public IAccessible
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nRelations( 
            /* [retval][out] */ long *nRelations) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_relation( 
            /* [in] */ long relationIndex,
            /* [retval][out] */ IAccessibleRelation **relation) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_relations( 
            /* [in] */ long maxRelations,
            /* [length_is][size_is][out] */ IAccessibleRelation **relations,
            /* [retval][out] */ long *nRelations) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE role( 
            /* [retval][out] */ long *role) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE scrollTo( 
            /* [in] */ enum IA2ScrollType scrollType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE scrollToPoint( 
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_groupPosition( 
            /* [out] */ long *groupLevel,
            /* [out] */ long *similarItemsInGroup,
            /* [retval][out] */ long *positionInGroup) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_states( 
            /* [retval][out] */ AccessibleStates *states) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_extendedRole( 
            /* [retval][out] */ BSTR *extendedRole) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_localizedExtendedRole( 
            /* [retval][out] */ BSTR *localizedExtendedRole) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nExtendedStates( 
            /* [retval][out] */ long *nExtendedStates) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_extendedStates( 
            /* [in] */ long maxExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **extendedStates,
            /* [retval][out] */ long *nExtendedStates) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_localizedExtendedStates( 
            /* [in] */ long maxLocalizedExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **localizedExtendedStates,
            /* [retval][out] */ long *nLocalizedExtendedStates) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_uniqueID( 
            /* [retval][out] */ long *uniqueID) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_windowHandle( 
            /* [retval][out] */ HWND *windowHandle) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_indexInParent( 
            /* [retval][out] */ long *indexInParent) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_locale( 
            /* [retval][out] */ IA2Locale *locale) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_attributes( 
            /* [retval][out] */ BSTR *attributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessible2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessible2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessible2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessible2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAccessible2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAccessible2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAccessible2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAccessible2 * This,
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
        
        DECLSPEC_XFGVIRT(IAccessible, get_accParent)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accParent )( 
            IAccessible2 * This,
            /* [retval][out] */ IDispatch **ppdispParent);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChildCount)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChildCount )( 
            IAccessible2 * This,
            /* [retval][out] */ long *pcountChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChild)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChild )( 
            IAccessible2 * This,
            /* [in] */ VARIANT varChild,
            /* [retval][out] */ IDispatch **ppdispChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accName)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accName )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszName);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accValue)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accValue )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszValue);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDescription)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDescription )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszDescription);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accRole)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accRole )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ VARIANT *pvarRole);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accState)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accState )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ VARIANT *pvarState);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelp)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelp )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszHelp);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelpTopic)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelpTopic )( 
            IAccessible2 * This,
            /* [out] */ BSTR *pszHelpFile,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ long *pidTopic);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accKeyboardShortcut)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accKeyboardShortcut )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszKeyboardShortcut);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accFocus)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accFocus )( 
            IAccessible2 * This,
            /* [retval][out] */ VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accSelection)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accSelection )( 
            IAccessible2 * This,
            /* [retval][out] */ VARIANT *pvarChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDefaultAction)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDefaultAction )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszDefaultAction);
        
        DECLSPEC_XFGVIRT(IAccessible, accSelect)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accSelect )( 
            IAccessible2 * This,
            /* [in] */ long flagsSelect,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accLocation)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accLocation )( 
            IAccessible2 * This,
            /* [out] */ long *pxLeft,
            /* [out] */ long *pyTop,
            /* [out] */ long *pcxWidth,
            /* [out] */ long *pcyHeight,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accNavigate)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accNavigate )( 
            IAccessible2 * This,
            /* [in] */ long navDir,
            /* [optional][in] */ VARIANT varStart,
            /* [retval][out] */ VARIANT *pvarEndUpAt);
        
        DECLSPEC_XFGVIRT(IAccessible, accHitTest)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accHitTest )( 
            IAccessible2 * This,
            /* [in] */ long xLeft,
            /* [in] */ long yTop,
            /* [retval][out] */ VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accDoDefaultAction)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accDoDefaultAction )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accName)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accName )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ BSTR szName);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accValue)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accValue )( 
            IAccessible2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ BSTR szValue);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_nRelations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nRelations )( 
            IAccessible2 * This,
            /* [retval][out] */ long *nRelations);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_relation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relation )( 
            IAccessible2 * This,
            /* [in] */ long relationIndex,
            /* [retval][out] */ IAccessibleRelation **relation);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_relations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relations )( 
            IAccessible2 * This,
            /* [in] */ long maxRelations,
            /* [length_is][size_is][out] */ IAccessibleRelation **relations,
            /* [retval][out] */ long *nRelations);
        
        DECLSPEC_XFGVIRT(IAccessible2, role)
        HRESULT ( STDMETHODCALLTYPE *role )( 
            IAccessible2 * This,
            /* [retval][out] */ long *role);
        
        DECLSPEC_XFGVIRT(IAccessible2, scrollTo)
        HRESULT ( STDMETHODCALLTYPE *scrollTo )( 
            IAccessible2 * This,
            /* [in] */ enum IA2ScrollType scrollType);
        
        DECLSPEC_XFGVIRT(IAccessible2, scrollToPoint)
        HRESULT ( STDMETHODCALLTYPE *scrollToPoint )( 
            IAccessible2 * This,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_groupPosition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_groupPosition )( 
            IAccessible2 * This,
            /* [out] */ long *groupLevel,
            /* [out] */ long *similarItemsInGroup,
            /* [retval][out] */ long *positionInGroup);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_states)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_states )( 
            IAccessible2 * This,
            /* [retval][out] */ AccessibleStates *states);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_extendedRole)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extendedRole )( 
            IAccessible2 * This,
            /* [retval][out] */ BSTR *extendedRole);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_localizedExtendedRole)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedExtendedRole )( 
            IAccessible2 * This,
            /* [retval][out] */ BSTR *localizedExtendedRole);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_nExtendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nExtendedStates )( 
            IAccessible2 * This,
            /* [retval][out] */ long *nExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_extendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extendedStates )( 
            IAccessible2 * This,
            /* [in] */ long maxExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **extendedStates,
            /* [retval][out] */ long *nExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_localizedExtendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedExtendedStates )( 
            IAccessible2 * This,
            /* [in] */ long maxLocalizedExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **localizedExtendedStates,
            /* [retval][out] */ long *nLocalizedExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_uniqueID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_uniqueID )( 
            IAccessible2 * This,
            /* [retval][out] */ long *uniqueID);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_windowHandle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_windowHandle )( 
            IAccessible2 * This,
            /* [retval][out] */ HWND *windowHandle);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_indexInParent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_indexInParent )( 
            IAccessible2 * This,
            /* [retval][out] */ long *indexInParent);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_locale)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_locale )( 
            IAccessible2 * This,
            /* [retval][out] */ IA2Locale *locale);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_attributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IAccessible2 * This,
            /* [retval][out] */ BSTR *attributes);
        
        END_INTERFACE
    } IAccessible2Vtbl;

    interface IAccessible2
    {
        CONST_VTBL struct IAccessible2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessible2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessible2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessible2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessible2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAccessible2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAccessible2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAccessible2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAccessible2_get_accParent(This,ppdispParent)	\
    ( (This)->lpVtbl -> get_accParent(This,ppdispParent) ) 

#define IAccessible2_get_accChildCount(This,pcountChildren)	\
    ( (This)->lpVtbl -> get_accChildCount(This,pcountChildren) ) 

#define IAccessible2_get_accChild(This,varChild,ppdispChild)	\
    ( (This)->lpVtbl -> get_accChild(This,varChild,ppdispChild) ) 

#define IAccessible2_get_accName(This,varChild,pszName)	\
    ( (This)->lpVtbl -> get_accName(This,varChild,pszName) ) 

#define IAccessible2_get_accValue(This,varChild,pszValue)	\
    ( (This)->lpVtbl -> get_accValue(This,varChild,pszValue) ) 

#define IAccessible2_get_accDescription(This,varChild,pszDescription)	\
    ( (This)->lpVtbl -> get_accDescription(This,varChild,pszDescription) ) 

#define IAccessible2_get_accRole(This,varChild,pvarRole)	\
    ( (This)->lpVtbl -> get_accRole(This,varChild,pvarRole) ) 

#define IAccessible2_get_accState(This,varChild,pvarState)	\
    ( (This)->lpVtbl -> get_accState(This,varChild,pvarState) ) 

#define IAccessible2_get_accHelp(This,varChild,pszHelp)	\
    ( (This)->lpVtbl -> get_accHelp(This,varChild,pszHelp) ) 

#define IAccessible2_get_accHelpTopic(This,pszHelpFile,varChild,pidTopic)	\
    ( (This)->lpVtbl -> get_accHelpTopic(This,pszHelpFile,varChild,pidTopic) ) 

#define IAccessible2_get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut)	\
    ( (This)->lpVtbl -> get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut) ) 

#define IAccessible2_get_accFocus(This,pvarChild)	\
    ( (This)->lpVtbl -> get_accFocus(This,pvarChild) ) 

#define IAccessible2_get_accSelection(This,pvarChildren)	\
    ( (This)->lpVtbl -> get_accSelection(This,pvarChildren) ) 

#define IAccessible2_get_accDefaultAction(This,varChild,pszDefaultAction)	\
    ( (This)->lpVtbl -> get_accDefaultAction(This,varChild,pszDefaultAction) ) 

#define IAccessible2_accSelect(This,flagsSelect,varChild)	\
    ( (This)->lpVtbl -> accSelect(This,flagsSelect,varChild) ) 

#define IAccessible2_accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild)	\
    ( (This)->lpVtbl -> accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild) ) 

#define IAccessible2_accNavigate(This,navDir,varStart,pvarEndUpAt)	\
    ( (This)->lpVtbl -> accNavigate(This,navDir,varStart,pvarEndUpAt) ) 

#define IAccessible2_accHitTest(This,xLeft,yTop,pvarChild)	\
    ( (This)->lpVtbl -> accHitTest(This,xLeft,yTop,pvarChild) ) 

#define IAccessible2_accDoDefaultAction(This,varChild)	\
    ( (This)->lpVtbl -> accDoDefaultAction(This,varChild) ) 

#define IAccessible2_put_accName(This,varChild,szName)	\
    ( (This)->lpVtbl -> put_accName(This,varChild,szName) ) 

#define IAccessible2_put_accValue(This,varChild,szValue)	\
    ( (This)->lpVtbl -> put_accValue(This,varChild,szValue) ) 


#define IAccessible2_get_nRelations(This,nRelations)	\
    ( (This)->lpVtbl -> get_nRelations(This,nRelations) ) 

#define IAccessible2_get_relation(This,relationIndex,relation)	\
    ( (This)->lpVtbl -> get_relation(This,relationIndex,relation) ) 

#define IAccessible2_get_relations(This,maxRelations,relations,nRelations)	\
    ( (This)->lpVtbl -> get_relations(This,maxRelations,relations,nRelations) ) 

#define IAccessible2_role(This,role)	\
    ( (This)->lpVtbl -> role(This,role) ) 

#define IAccessible2_scrollTo(This,scrollType)	\
    ( (This)->lpVtbl -> scrollTo(This,scrollType) ) 

#define IAccessible2_scrollToPoint(This,coordinateType,x,y)	\
    ( (This)->lpVtbl -> scrollToPoint(This,coordinateType,x,y) ) 

#define IAccessible2_get_groupPosition(This,groupLevel,similarItemsInGroup,positionInGroup)	\
    ( (This)->lpVtbl -> get_groupPosition(This,groupLevel,similarItemsInGroup,positionInGroup) ) 

#define IAccessible2_get_states(This,states)	\
    ( (This)->lpVtbl -> get_states(This,states) ) 

#define IAccessible2_get_extendedRole(This,extendedRole)	\
    ( (This)->lpVtbl -> get_extendedRole(This,extendedRole) ) 

#define IAccessible2_get_localizedExtendedRole(This,localizedExtendedRole)	\
    ( (This)->lpVtbl -> get_localizedExtendedRole(This,localizedExtendedRole) ) 

#define IAccessible2_get_nExtendedStates(This,nExtendedStates)	\
    ( (This)->lpVtbl -> get_nExtendedStates(This,nExtendedStates) ) 

#define IAccessible2_get_extendedStates(This,maxExtendedStates,extendedStates,nExtendedStates)	\
    ( (This)->lpVtbl -> get_extendedStates(This,maxExtendedStates,extendedStates,nExtendedStates) ) 

#define IAccessible2_get_localizedExtendedStates(This,maxLocalizedExtendedStates,localizedExtendedStates,nLocalizedExtendedStates)	\
    ( (This)->lpVtbl -> get_localizedExtendedStates(This,maxLocalizedExtendedStates,localizedExtendedStates,nLocalizedExtendedStates) ) 

#define IAccessible2_get_uniqueID(This,uniqueID)	\
    ( (This)->lpVtbl -> get_uniqueID(This,uniqueID) ) 

#define IAccessible2_get_windowHandle(This,windowHandle)	\
    ( (This)->lpVtbl -> get_windowHandle(This,windowHandle) ) 

#define IAccessible2_get_indexInParent(This,indexInParent)	\
    ( (This)->lpVtbl -> get_indexInParent(This,indexInParent) ) 

#define IAccessible2_get_locale(This,locale)	\
    ( (This)->lpVtbl -> get_locale(This,locale) ) 

#define IAccessible2_get_attributes(This,attributes)	\
    ( (This)->lpVtbl -> get_attributes(This,attributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessible2_INTERFACE_DEFINED__ */


#ifndef __IAccessible2_2_INTERFACE_DEFINED__
#define __IAccessible2_2_INTERFACE_DEFINED__

/* interface IAccessible2_2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessible2_2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6C9430E9-299D-4E6F-BD01-A82A1E88D3FF")
    IAccessible2_2 : public IAccessible2
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_attribute( 
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *attribute) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_accessibleWithCaret( 
            /* [out] */ IUnknown **accessible,
            /* [retval][out] */ long *caretOffset) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_relationTargetsOfType( 
            /* [in] */ BSTR type,
            /* [in] */ long maxTargets,
            /* [size_is][size_is][out] */ IUnknown ***targets,
            /* [retval][out] */ long *nTargets) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessible2_2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessible2_2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessible2_2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessible2_2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAccessible2_2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAccessible2_2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAccessible2_2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAccessible2_2 * This,
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
        
        DECLSPEC_XFGVIRT(IAccessible, get_accParent)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accParent )( 
            IAccessible2_2 * This,
            /* [retval][out] */ IDispatch **ppdispParent);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChildCount)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChildCount )( 
            IAccessible2_2 * This,
            /* [retval][out] */ long *pcountChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChild)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChild )( 
            IAccessible2_2 * This,
            /* [in] */ VARIANT varChild,
            /* [retval][out] */ IDispatch **ppdispChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accName)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accName )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszName);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accValue)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accValue )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszValue);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDescription)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDescription )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszDescription);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accRole)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accRole )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ VARIANT *pvarRole);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accState)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accState )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ VARIANT *pvarState);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelp)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelp )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszHelp);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelpTopic)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelpTopic )( 
            IAccessible2_2 * This,
            /* [out] */ BSTR *pszHelpFile,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ long *pidTopic);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accKeyboardShortcut)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accKeyboardShortcut )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszKeyboardShortcut);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accFocus)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accFocus )( 
            IAccessible2_2 * This,
            /* [retval][out] */ VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accSelection)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accSelection )( 
            IAccessible2_2 * This,
            /* [retval][out] */ VARIANT *pvarChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDefaultAction)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDefaultAction )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszDefaultAction);
        
        DECLSPEC_XFGVIRT(IAccessible, accSelect)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accSelect )( 
            IAccessible2_2 * This,
            /* [in] */ long flagsSelect,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accLocation)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accLocation )( 
            IAccessible2_2 * This,
            /* [out] */ long *pxLeft,
            /* [out] */ long *pyTop,
            /* [out] */ long *pcxWidth,
            /* [out] */ long *pcyHeight,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accNavigate)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accNavigate )( 
            IAccessible2_2 * This,
            /* [in] */ long navDir,
            /* [optional][in] */ VARIANT varStart,
            /* [retval][out] */ VARIANT *pvarEndUpAt);
        
        DECLSPEC_XFGVIRT(IAccessible, accHitTest)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accHitTest )( 
            IAccessible2_2 * This,
            /* [in] */ long xLeft,
            /* [in] */ long yTop,
            /* [retval][out] */ VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accDoDefaultAction)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accDoDefaultAction )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accName)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accName )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ BSTR szName);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accValue)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accValue )( 
            IAccessible2_2 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ BSTR szValue);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_nRelations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nRelations )( 
            IAccessible2_2 * This,
            /* [retval][out] */ long *nRelations);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_relation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relation )( 
            IAccessible2_2 * This,
            /* [in] */ long relationIndex,
            /* [retval][out] */ IAccessibleRelation **relation);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_relations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relations )( 
            IAccessible2_2 * This,
            /* [in] */ long maxRelations,
            /* [length_is][size_is][out] */ IAccessibleRelation **relations,
            /* [retval][out] */ long *nRelations);
        
        DECLSPEC_XFGVIRT(IAccessible2, role)
        HRESULT ( STDMETHODCALLTYPE *role )( 
            IAccessible2_2 * This,
            /* [retval][out] */ long *role);
        
        DECLSPEC_XFGVIRT(IAccessible2, scrollTo)
        HRESULT ( STDMETHODCALLTYPE *scrollTo )( 
            IAccessible2_2 * This,
            /* [in] */ enum IA2ScrollType scrollType);
        
        DECLSPEC_XFGVIRT(IAccessible2, scrollToPoint)
        HRESULT ( STDMETHODCALLTYPE *scrollToPoint )( 
            IAccessible2_2 * This,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_groupPosition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_groupPosition )( 
            IAccessible2_2 * This,
            /* [out] */ long *groupLevel,
            /* [out] */ long *similarItemsInGroup,
            /* [retval][out] */ long *positionInGroup);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_states)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_states )( 
            IAccessible2_2 * This,
            /* [retval][out] */ AccessibleStates *states);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_extendedRole)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extendedRole )( 
            IAccessible2_2 * This,
            /* [retval][out] */ BSTR *extendedRole);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_localizedExtendedRole)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedExtendedRole )( 
            IAccessible2_2 * This,
            /* [retval][out] */ BSTR *localizedExtendedRole);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_nExtendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nExtendedStates )( 
            IAccessible2_2 * This,
            /* [retval][out] */ long *nExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_extendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extendedStates )( 
            IAccessible2_2 * This,
            /* [in] */ long maxExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **extendedStates,
            /* [retval][out] */ long *nExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_localizedExtendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedExtendedStates )( 
            IAccessible2_2 * This,
            /* [in] */ long maxLocalizedExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **localizedExtendedStates,
            /* [retval][out] */ long *nLocalizedExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_uniqueID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_uniqueID )( 
            IAccessible2_2 * This,
            /* [retval][out] */ long *uniqueID);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_windowHandle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_windowHandle )( 
            IAccessible2_2 * This,
            /* [retval][out] */ HWND *windowHandle);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_indexInParent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_indexInParent )( 
            IAccessible2_2 * This,
            /* [retval][out] */ long *indexInParent);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_locale)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_locale )( 
            IAccessible2_2 * This,
            /* [retval][out] */ IA2Locale *locale);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_attributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IAccessible2_2 * This,
            /* [retval][out] */ BSTR *attributes);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_attribute)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attribute )( 
            IAccessible2_2 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *attribute);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_accessibleWithCaret)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_accessibleWithCaret )( 
            IAccessible2_2 * This,
            /* [out] */ IUnknown **accessible,
            /* [retval][out] */ long *caretOffset);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_relationTargetsOfType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relationTargetsOfType )( 
            IAccessible2_2 * This,
            /* [in] */ BSTR type,
            /* [in] */ long maxTargets,
            /* [size_is][size_is][out] */ IUnknown ***targets,
            /* [retval][out] */ long *nTargets);
        
        END_INTERFACE
    } IAccessible2_2Vtbl;

    interface IAccessible2_2
    {
        CONST_VTBL struct IAccessible2_2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessible2_2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessible2_2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessible2_2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessible2_2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAccessible2_2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAccessible2_2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAccessible2_2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAccessible2_2_get_accParent(This,ppdispParent)	\
    ( (This)->lpVtbl -> get_accParent(This,ppdispParent) ) 

#define IAccessible2_2_get_accChildCount(This,pcountChildren)	\
    ( (This)->lpVtbl -> get_accChildCount(This,pcountChildren) ) 

#define IAccessible2_2_get_accChild(This,varChild,ppdispChild)	\
    ( (This)->lpVtbl -> get_accChild(This,varChild,ppdispChild) ) 

#define IAccessible2_2_get_accName(This,varChild,pszName)	\
    ( (This)->lpVtbl -> get_accName(This,varChild,pszName) ) 

#define IAccessible2_2_get_accValue(This,varChild,pszValue)	\
    ( (This)->lpVtbl -> get_accValue(This,varChild,pszValue) ) 

#define IAccessible2_2_get_accDescription(This,varChild,pszDescription)	\
    ( (This)->lpVtbl -> get_accDescription(This,varChild,pszDescription) ) 

#define IAccessible2_2_get_accRole(This,varChild,pvarRole)	\
    ( (This)->lpVtbl -> get_accRole(This,varChild,pvarRole) ) 

#define IAccessible2_2_get_accState(This,varChild,pvarState)	\
    ( (This)->lpVtbl -> get_accState(This,varChild,pvarState) ) 

#define IAccessible2_2_get_accHelp(This,varChild,pszHelp)	\
    ( (This)->lpVtbl -> get_accHelp(This,varChild,pszHelp) ) 

#define IAccessible2_2_get_accHelpTopic(This,pszHelpFile,varChild,pidTopic)	\
    ( (This)->lpVtbl -> get_accHelpTopic(This,pszHelpFile,varChild,pidTopic) ) 

#define IAccessible2_2_get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut)	\
    ( (This)->lpVtbl -> get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut) ) 

#define IAccessible2_2_get_accFocus(This,pvarChild)	\
    ( (This)->lpVtbl -> get_accFocus(This,pvarChild) ) 

#define IAccessible2_2_get_accSelection(This,pvarChildren)	\
    ( (This)->lpVtbl -> get_accSelection(This,pvarChildren) ) 

#define IAccessible2_2_get_accDefaultAction(This,varChild,pszDefaultAction)	\
    ( (This)->lpVtbl -> get_accDefaultAction(This,varChild,pszDefaultAction) ) 

#define IAccessible2_2_accSelect(This,flagsSelect,varChild)	\
    ( (This)->lpVtbl -> accSelect(This,flagsSelect,varChild) ) 

#define IAccessible2_2_accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild)	\
    ( (This)->lpVtbl -> accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild) ) 

#define IAccessible2_2_accNavigate(This,navDir,varStart,pvarEndUpAt)	\
    ( (This)->lpVtbl -> accNavigate(This,navDir,varStart,pvarEndUpAt) ) 

#define IAccessible2_2_accHitTest(This,xLeft,yTop,pvarChild)	\
    ( (This)->lpVtbl -> accHitTest(This,xLeft,yTop,pvarChild) ) 

#define IAccessible2_2_accDoDefaultAction(This,varChild)	\
    ( (This)->lpVtbl -> accDoDefaultAction(This,varChild) ) 

#define IAccessible2_2_put_accName(This,varChild,szName)	\
    ( (This)->lpVtbl -> put_accName(This,varChild,szName) ) 

#define IAccessible2_2_put_accValue(This,varChild,szValue)	\
    ( (This)->lpVtbl -> put_accValue(This,varChild,szValue) ) 


#define IAccessible2_2_get_nRelations(This,nRelations)	\
    ( (This)->lpVtbl -> get_nRelations(This,nRelations) ) 

#define IAccessible2_2_get_relation(This,relationIndex,relation)	\
    ( (This)->lpVtbl -> get_relation(This,relationIndex,relation) ) 

#define IAccessible2_2_get_relations(This,maxRelations,relations,nRelations)	\
    ( (This)->lpVtbl -> get_relations(This,maxRelations,relations,nRelations) ) 

#define IAccessible2_2_role(This,role)	\
    ( (This)->lpVtbl -> role(This,role) ) 

#define IAccessible2_2_scrollTo(This,scrollType)	\
    ( (This)->lpVtbl -> scrollTo(This,scrollType) ) 

#define IAccessible2_2_scrollToPoint(This,coordinateType,x,y)	\
    ( (This)->lpVtbl -> scrollToPoint(This,coordinateType,x,y) ) 

#define IAccessible2_2_get_groupPosition(This,groupLevel,similarItemsInGroup,positionInGroup)	\
    ( (This)->lpVtbl -> get_groupPosition(This,groupLevel,similarItemsInGroup,positionInGroup) ) 

#define IAccessible2_2_get_states(This,states)	\
    ( (This)->lpVtbl -> get_states(This,states) ) 

#define IAccessible2_2_get_extendedRole(This,extendedRole)	\
    ( (This)->lpVtbl -> get_extendedRole(This,extendedRole) ) 

#define IAccessible2_2_get_localizedExtendedRole(This,localizedExtendedRole)	\
    ( (This)->lpVtbl -> get_localizedExtendedRole(This,localizedExtendedRole) ) 

#define IAccessible2_2_get_nExtendedStates(This,nExtendedStates)	\
    ( (This)->lpVtbl -> get_nExtendedStates(This,nExtendedStates) ) 

#define IAccessible2_2_get_extendedStates(This,maxExtendedStates,extendedStates,nExtendedStates)	\
    ( (This)->lpVtbl -> get_extendedStates(This,maxExtendedStates,extendedStates,nExtendedStates) ) 

#define IAccessible2_2_get_localizedExtendedStates(This,maxLocalizedExtendedStates,localizedExtendedStates,nLocalizedExtendedStates)	\
    ( (This)->lpVtbl -> get_localizedExtendedStates(This,maxLocalizedExtendedStates,localizedExtendedStates,nLocalizedExtendedStates) ) 

#define IAccessible2_2_get_uniqueID(This,uniqueID)	\
    ( (This)->lpVtbl -> get_uniqueID(This,uniqueID) ) 

#define IAccessible2_2_get_windowHandle(This,windowHandle)	\
    ( (This)->lpVtbl -> get_windowHandle(This,windowHandle) ) 

#define IAccessible2_2_get_indexInParent(This,indexInParent)	\
    ( (This)->lpVtbl -> get_indexInParent(This,indexInParent) ) 

#define IAccessible2_2_get_locale(This,locale)	\
    ( (This)->lpVtbl -> get_locale(This,locale) ) 

#define IAccessible2_2_get_attributes(This,attributes)	\
    ( (This)->lpVtbl -> get_attributes(This,attributes) ) 


#define IAccessible2_2_get_attribute(This,name,attribute)	\
    ( (This)->lpVtbl -> get_attribute(This,name,attribute) ) 

#define IAccessible2_2_get_accessibleWithCaret(This,accessible,caretOffset)	\
    ( (This)->lpVtbl -> get_accessibleWithCaret(This,accessible,caretOffset) ) 

#define IAccessible2_2_get_relationTargetsOfType(This,type,maxTargets,targets,nTargets)	\
    ( (This)->lpVtbl -> get_relationTargetsOfType(This,type,maxTargets,targets,nTargets) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessible2_2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ia2_api_all_0000_0004 */
/* [local] */ 

typedef struct IA2Range
    {
    IUnknown *anchor;
    long anchorOffset;
    IUnknown *active;
    long activeOffset;
    } 	IA2Range;



extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0004_v0_0_s_ifspec;

#ifndef __IAccessible2_3_INTERFACE_DEFINED__
#define __IAccessible2_3_INTERFACE_DEFINED__

/* interface IAccessible2_3 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessible2_3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5BE18059-762E-4E73-9476-ABA294FED411")
    IAccessible2_3 : public IAccessible2_2
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selectionRanges( 
            /* [size_is][size_is][out] */ IA2Range **ranges,
            /* [retval][out] */ long *nRanges) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessible2_3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessible2_3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessible2_3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessible2_3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAccessible2_3 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAccessible2_3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAccessible2_3 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAccessible2_3 * This,
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
        
        DECLSPEC_XFGVIRT(IAccessible, get_accParent)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accParent )( 
            IAccessible2_3 * This,
            /* [retval][out] */ IDispatch **ppdispParent);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChildCount)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChildCount )( 
            IAccessible2_3 * This,
            /* [retval][out] */ long *pcountChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChild)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChild )( 
            IAccessible2_3 * This,
            /* [in] */ VARIANT varChild,
            /* [retval][out] */ IDispatch **ppdispChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accName)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accName )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszName);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accValue)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accValue )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszValue);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDescription)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDescription )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszDescription);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accRole)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accRole )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ VARIANT *pvarRole);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accState)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accState )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ VARIANT *pvarState);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelp)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelp )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszHelp);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelpTopic)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelpTopic )( 
            IAccessible2_3 * This,
            /* [out] */ BSTR *pszHelpFile,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ long *pidTopic);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accKeyboardShortcut)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accKeyboardShortcut )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszKeyboardShortcut);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accFocus)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accFocus )( 
            IAccessible2_3 * This,
            /* [retval][out] */ VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accSelection)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accSelection )( 
            IAccessible2_3 * This,
            /* [retval][out] */ VARIANT *pvarChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDefaultAction)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDefaultAction )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszDefaultAction);
        
        DECLSPEC_XFGVIRT(IAccessible, accSelect)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accSelect )( 
            IAccessible2_3 * This,
            /* [in] */ long flagsSelect,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accLocation)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accLocation )( 
            IAccessible2_3 * This,
            /* [out] */ long *pxLeft,
            /* [out] */ long *pyTop,
            /* [out] */ long *pcxWidth,
            /* [out] */ long *pcyHeight,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accNavigate)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accNavigate )( 
            IAccessible2_3 * This,
            /* [in] */ long navDir,
            /* [optional][in] */ VARIANT varStart,
            /* [retval][out] */ VARIANT *pvarEndUpAt);
        
        DECLSPEC_XFGVIRT(IAccessible, accHitTest)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accHitTest )( 
            IAccessible2_3 * This,
            /* [in] */ long xLeft,
            /* [in] */ long yTop,
            /* [retval][out] */ VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accDoDefaultAction)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accDoDefaultAction )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accName)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accName )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ BSTR szName);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accValue)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accValue )( 
            IAccessible2_3 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ BSTR szValue);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_nRelations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nRelations )( 
            IAccessible2_3 * This,
            /* [retval][out] */ long *nRelations);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_relation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relation )( 
            IAccessible2_3 * This,
            /* [in] */ long relationIndex,
            /* [retval][out] */ IAccessibleRelation **relation);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_relations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relations )( 
            IAccessible2_3 * This,
            /* [in] */ long maxRelations,
            /* [length_is][size_is][out] */ IAccessibleRelation **relations,
            /* [retval][out] */ long *nRelations);
        
        DECLSPEC_XFGVIRT(IAccessible2, role)
        HRESULT ( STDMETHODCALLTYPE *role )( 
            IAccessible2_3 * This,
            /* [retval][out] */ long *role);
        
        DECLSPEC_XFGVIRT(IAccessible2, scrollTo)
        HRESULT ( STDMETHODCALLTYPE *scrollTo )( 
            IAccessible2_3 * This,
            /* [in] */ enum IA2ScrollType scrollType);
        
        DECLSPEC_XFGVIRT(IAccessible2, scrollToPoint)
        HRESULT ( STDMETHODCALLTYPE *scrollToPoint )( 
            IAccessible2_3 * This,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_groupPosition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_groupPosition )( 
            IAccessible2_3 * This,
            /* [out] */ long *groupLevel,
            /* [out] */ long *similarItemsInGroup,
            /* [retval][out] */ long *positionInGroup);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_states)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_states )( 
            IAccessible2_3 * This,
            /* [retval][out] */ AccessibleStates *states);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_extendedRole)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extendedRole )( 
            IAccessible2_3 * This,
            /* [retval][out] */ BSTR *extendedRole);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_localizedExtendedRole)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedExtendedRole )( 
            IAccessible2_3 * This,
            /* [retval][out] */ BSTR *localizedExtendedRole);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_nExtendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nExtendedStates )( 
            IAccessible2_3 * This,
            /* [retval][out] */ long *nExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_extendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extendedStates )( 
            IAccessible2_3 * This,
            /* [in] */ long maxExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **extendedStates,
            /* [retval][out] */ long *nExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_localizedExtendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedExtendedStates )( 
            IAccessible2_3 * This,
            /* [in] */ long maxLocalizedExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **localizedExtendedStates,
            /* [retval][out] */ long *nLocalizedExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_uniqueID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_uniqueID )( 
            IAccessible2_3 * This,
            /* [retval][out] */ long *uniqueID);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_windowHandle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_windowHandle )( 
            IAccessible2_3 * This,
            /* [retval][out] */ HWND *windowHandle);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_indexInParent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_indexInParent )( 
            IAccessible2_3 * This,
            /* [retval][out] */ long *indexInParent);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_locale)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_locale )( 
            IAccessible2_3 * This,
            /* [retval][out] */ IA2Locale *locale);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_attributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IAccessible2_3 * This,
            /* [retval][out] */ BSTR *attributes);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_attribute)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attribute )( 
            IAccessible2_3 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *attribute);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_accessibleWithCaret)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_accessibleWithCaret )( 
            IAccessible2_3 * This,
            /* [out] */ IUnknown **accessible,
            /* [retval][out] */ long *caretOffset);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_relationTargetsOfType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relationTargetsOfType )( 
            IAccessible2_3 * This,
            /* [in] */ BSTR type,
            /* [in] */ long maxTargets,
            /* [size_is][size_is][out] */ IUnknown ***targets,
            /* [retval][out] */ long *nTargets);
        
        DECLSPEC_XFGVIRT(IAccessible2_3, get_selectionRanges)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selectionRanges )( 
            IAccessible2_3 * This,
            /* [size_is][size_is][out] */ IA2Range **ranges,
            /* [retval][out] */ long *nRanges);
        
        END_INTERFACE
    } IAccessible2_3Vtbl;

    interface IAccessible2_3
    {
        CONST_VTBL struct IAccessible2_3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessible2_3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessible2_3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessible2_3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessible2_3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAccessible2_3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAccessible2_3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAccessible2_3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAccessible2_3_get_accParent(This,ppdispParent)	\
    ( (This)->lpVtbl -> get_accParent(This,ppdispParent) ) 

#define IAccessible2_3_get_accChildCount(This,pcountChildren)	\
    ( (This)->lpVtbl -> get_accChildCount(This,pcountChildren) ) 

#define IAccessible2_3_get_accChild(This,varChild,ppdispChild)	\
    ( (This)->lpVtbl -> get_accChild(This,varChild,ppdispChild) ) 

#define IAccessible2_3_get_accName(This,varChild,pszName)	\
    ( (This)->lpVtbl -> get_accName(This,varChild,pszName) ) 

#define IAccessible2_3_get_accValue(This,varChild,pszValue)	\
    ( (This)->lpVtbl -> get_accValue(This,varChild,pszValue) ) 

#define IAccessible2_3_get_accDescription(This,varChild,pszDescription)	\
    ( (This)->lpVtbl -> get_accDescription(This,varChild,pszDescription) ) 

#define IAccessible2_3_get_accRole(This,varChild,pvarRole)	\
    ( (This)->lpVtbl -> get_accRole(This,varChild,pvarRole) ) 

#define IAccessible2_3_get_accState(This,varChild,pvarState)	\
    ( (This)->lpVtbl -> get_accState(This,varChild,pvarState) ) 

#define IAccessible2_3_get_accHelp(This,varChild,pszHelp)	\
    ( (This)->lpVtbl -> get_accHelp(This,varChild,pszHelp) ) 

#define IAccessible2_3_get_accHelpTopic(This,pszHelpFile,varChild,pidTopic)	\
    ( (This)->lpVtbl -> get_accHelpTopic(This,pszHelpFile,varChild,pidTopic) ) 

#define IAccessible2_3_get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut)	\
    ( (This)->lpVtbl -> get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut) ) 

#define IAccessible2_3_get_accFocus(This,pvarChild)	\
    ( (This)->lpVtbl -> get_accFocus(This,pvarChild) ) 

#define IAccessible2_3_get_accSelection(This,pvarChildren)	\
    ( (This)->lpVtbl -> get_accSelection(This,pvarChildren) ) 

#define IAccessible2_3_get_accDefaultAction(This,varChild,pszDefaultAction)	\
    ( (This)->lpVtbl -> get_accDefaultAction(This,varChild,pszDefaultAction) ) 

#define IAccessible2_3_accSelect(This,flagsSelect,varChild)	\
    ( (This)->lpVtbl -> accSelect(This,flagsSelect,varChild) ) 

#define IAccessible2_3_accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild)	\
    ( (This)->lpVtbl -> accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild) ) 

#define IAccessible2_3_accNavigate(This,navDir,varStart,pvarEndUpAt)	\
    ( (This)->lpVtbl -> accNavigate(This,navDir,varStart,pvarEndUpAt) ) 

#define IAccessible2_3_accHitTest(This,xLeft,yTop,pvarChild)	\
    ( (This)->lpVtbl -> accHitTest(This,xLeft,yTop,pvarChild) ) 

#define IAccessible2_3_accDoDefaultAction(This,varChild)	\
    ( (This)->lpVtbl -> accDoDefaultAction(This,varChild) ) 

#define IAccessible2_3_put_accName(This,varChild,szName)	\
    ( (This)->lpVtbl -> put_accName(This,varChild,szName) ) 

#define IAccessible2_3_put_accValue(This,varChild,szValue)	\
    ( (This)->lpVtbl -> put_accValue(This,varChild,szValue) ) 


#define IAccessible2_3_get_nRelations(This,nRelations)	\
    ( (This)->lpVtbl -> get_nRelations(This,nRelations) ) 

#define IAccessible2_3_get_relation(This,relationIndex,relation)	\
    ( (This)->lpVtbl -> get_relation(This,relationIndex,relation) ) 

#define IAccessible2_3_get_relations(This,maxRelations,relations,nRelations)	\
    ( (This)->lpVtbl -> get_relations(This,maxRelations,relations,nRelations) ) 

#define IAccessible2_3_role(This,role)	\
    ( (This)->lpVtbl -> role(This,role) ) 

#define IAccessible2_3_scrollTo(This,scrollType)	\
    ( (This)->lpVtbl -> scrollTo(This,scrollType) ) 

#define IAccessible2_3_scrollToPoint(This,coordinateType,x,y)	\
    ( (This)->lpVtbl -> scrollToPoint(This,coordinateType,x,y) ) 

#define IAccessible2_3_get_groupPosition(This,groupLevel,similarItemsInGroup,positionInGroup)	\
    ( (This)->lpVtbl -> get_groupPosition(This,groupLevel,similarItemsInGroup,positionInGroup) ) 

#define IAccessible2_3_get_states(This,states)	\
    ( (This)->lpVtbl -> get_states(This,states) ) 

#define IAccessible2_3_get_extendedRole(This,extendedRole)	\
    ( (This)->lpVtbl -> get_extendedRole(This,extendedRole) ) 

#define IAccessible2_3_get_localizedExtendedRole(This,localizedExtendedRole)	\
    ( (This)->lpVtbl -> get_localizedExtendedRole(This,localizedExtendedRole) ) 

#define IAccessible2_3_get_nExtendedStates(This,nExtendedStates)	\
    ( (This)->lpVtbl -> get_nExtendedStates(This,nExtendedStates) ) 

#define IAccessible2_3_get_extendedStates(This,maxExtendedStates,extendedStates,nExtendedStates)	\
    ( (This)->lpVtbl -> get_extendedStates(This,maxExtendedStates,extendedStates,nExtendedStates) ) 

#define IAccessible2_3_get_localizedExtendedStates(This,maxLocalizedExtendedStates,localizedExtendedStates,nLocalizedExtendedStates)	\
    ( (This)->lpVtbl -> get_localizedExtendedStates(This,maxLocalizedExtendedStates,localizedExtendedStates,nLocalizedExtendedStates) ) 

#define IAccessible2_3_get_uniqueID(This,uniqueID)	\
    ( (This)->lpVtbl -> get_uniqueID(This,uniqueID) ) 

#define IAccessible2_3_get_windowHandle(This,windowHandle)	\
    ( (This)->lpVtbl -> get_windowHandle(This,windowHandle) ) 

#define IAccessible2_3_get_indexInParent(This,indexInParent)	\
    ( (This)->lpVtbl -> get_indexInParent(This,indexInParent) ) 

#define IAccessible2_3_get_locale(This,locale)	\
    ( (This)->lpVtbl -> get_locale(This,locale) ) 

#define IAccessible2_3_get_attributes(This,attributes)	\
    ( (This)->lpVtbl -> get_attributes(This,attributes) ) 


#define IAccessible2_3_get_attribute(This,name,attribute)	\
    ( (This)->lpVtbl -> get_attribute(This,name,attribute) ) 

#define IAccessible2_3_get_accessibleWithCaret(This,accessible,caretOffset)	\
    ( (This)->lpVtbl -> get_accessibleWithCaret(This,accessible,caretOffset) ) 

#define IAccessible2_3_get_relationTargetsOfType(This,type,maxTargets,targets,nTargets)	\
    ( (This)->lpVtbl -> get_relationTargetsOfType(This,type,maxTargets,targets,nTargets) ) 


#define IAccessible2_3_get_selectionRanges(This,ranges,nRanges)	\
    ( (This)->lpVtbl -> get_selectionRanges(This,ranges,nRanges) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessible2_3_INTERFACE_DEFINED__ */


#ifndef __IAccessible2_4_INTERFACE_DEFINED__
#define __IAccessible2_4_INTERFACE_DEFINED__

/* interface IAccessible2_4 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessible2_4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("610a7bec-91bb-444d-a336-a0daf13c4c29")
    IAccessible2_4 : public IAccessible2_3
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE setSelectionRanges( 
            /* [in] */ long nRanges,
            /* [size_is][in] */ IA2Range *ranges) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessible2_4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessible2_4 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessible2_4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessible2_4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAccessible2_4 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAccessible2_4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAccessible2_4 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAccessible2_4 * This,
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
        
        DECLSPEC_XFGVIRT(IAccessible, get_accParent)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accParent )( 
            IAccessible2_4 * This,
            /* [retval][out] */ IDispatch **ppdispParent);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChildCount)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChildCount )( 
            IAccessible2_4 * This,
            /* [retval][out] */ long *pcountChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChild)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChild )( 
            IAccessible2_4 * This,
            /* [in] */ VARIANT varChild,
            /* [retval][out] */ IDispatch **ppdispChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accName)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accName )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszName);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accValue)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accValue )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszValue);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDescription)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDescription )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszDescription);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accRole)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accRole )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ VARIANT *pvarRole);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accState)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accState )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ VARIANT *pvarState);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelp)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelp )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszHelp);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelpTopic)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelpTopic )( 
            IAccessible2_4 * This,
            /* [out] */ BSTR *pszHelpFile,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ long *pidTopic);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accKeyboardShortcut)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accKeyboardShortcut )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszKeyboardShortcut);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accFocus)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accFocus )( 
            IAccessible2_4 * This,
            /* [retval][out] */ VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accSelection)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accSelection )( 
            IAccessible2_4 * This,
            /* [retval][out] */ VARIANT *pvarChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDefaultAction)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDefaultAction )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ BSTR *pszDefaultAction);
        
        DECLSPEC_XFGVIRT(IAccessible, accSelect)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accSelect )( 
            IAccessible2_4 * This,
            /* [in] */ long flagsSelect,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accLocation)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accLocation )( 
            IAccessible2_4 * This,
            /* [out] */ long *pxLeft,
            /* [out] */ long *pyTop,
            /* [out] */ long *pcxWidth,
            /* [out] */ long *pcyHeight,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accNavigate)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accNavigate )( 
            IAccessible2_4 * This,
            /* [in] */ long navDir,
            /* [optional][in] */ VARIANT varStart,
            /* [retval][out] */ VARIANT *pvarEndUpAt);
        
        DECLSPEC_XFGVIRT(IAccessible, accHitTest)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accHitTest )( 
            IAccessible2_4 * This,
            /* [in] */ long xLeft,
            /* [in] */ long yTop,
            /* [retval][out] */ VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accDoDefaultAction)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accDoDefaultAction )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accName)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accName )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ BSTR szName);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accValue)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accValue )( 
            IAccessible2_4 * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ BSTR szValue);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_nRelations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nRelations )( 
            IAccessible2_4 * This,
            /* [retval][out] */ long *nRelations);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_relation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relation )( 
            IAccessible2_4 * This,
            /* [in] */ long relationIndex,
            /* [retval][out] */ IAccessibleRelation **relation);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_relations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relations )( 
            IAccessible2_4 * This,
            /* [in] */ long maxRelations,
            /* [length_is][size_is][out] */ IAccessibleRelation **relations,
            /* [retval][out] */ long *nRelations);
        
        DECLSPEC_XFGVIRT(IAccessible2, role)
        HRESULT ( STDMETHODCALLTYPE *role )( 
            IAccessible2_4 * This,
            /* [retval][out] */ long *role);
        
        DECLSPEC_XFGVIRT(IAccessible2, scrollTo)
        HRESULT ( STDMETHODCALLTYPE *scrollTo )( 
            IAccessible2_4 * This,
            /* [in] */ enum IA2ScrollType scrollType);
        
        DECLSPEC_XFGVIRT(IAccessible2, scrollToPoint)
        HRESULT ( STDMETHODCALLTYPE *scrollToPoint )( 
            IAccessible2_4 * This,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_groupPosition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_groupPosition )( 
            IAccessible2_4 * This,
            /* [out] */ long *groupLevel,
            /* [out] */ long *similarItemsInGroup,
            /* [retval][out] */ long *positionInGroup);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_states)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_states )( 
            IAccessible2_4 * This,
            /* [retval][out] */ AccessibleStates *states);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_extendedRole)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extendedRole )( 
            IAccessible2_4 * This,
            /* [retval][out] */ BSTR *extendedRole);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_localizedExtendedRole)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedExtendedRole )( 
            IAccessible2_4 * This,
            /* [retval][out] */ BSTR *localizedExtendedRole);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_nExtendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nExtendedStates )( 
            IAccessible2_4 * This,
            /* [retval][out] */ long *nExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_extendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_extendedStates )( 
            IAccessible2_4 * This,
            /* [in] */ long maxExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **extendedStates,
            /* [retval][out] */ long *nExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_localizedExtendedStates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedExtendedStates )( 
            IAccessible2_4 * This,
            /* [in] */ long maxLocalizedExtendedStates,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **localizedExtendedStates,
            /* [retval][out] */ long *nLocalizedExtendedStates);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_uniqueID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_uniqueID )( 
            IAccessible2_4 * This,
            /* [retval][out] */ long *uniqueID);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_windowHandle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_windowHandle )( 
            IAccessible2_4 * This,
            /* [retval][out] */ HWND *windowHandle);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_indexInParent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_indexInParent )( 
            IAccessible2_4 * This,
            /* [retval][out] */ long *indexInParent);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_locale)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_locale )( 
            IAccessible2_4 * This,
            /* [retval][out] */ IA2Locale *locale);
        
        DECLSPEC_XFGVIRT(IAccessible2, get_attributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IAccessible2_4 * This,
            /* [retval][out] */ BSTR *attributes);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_attribute)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attribute )( 
            IAccessible2_4 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *attribute);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_accessibleWithCaret)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_accessibleWithCaret )( 
            IAccessible2_4 * This,
            /* [out] */ IUnknown **accessible,
            /* [retval][out] */ long *caretOffset);
        
        DECLSPEC_XFGVIRT(IAccessible2_2, get_relationTargetsOfType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_relationTargetsOfType )( 
            IAccessible2_4 * This,
            /* [in] */ BSTR type,
            /* [in] */ long maxTargets,
            /* [size_is][size_is][out] */ IUnknown ***targets,
            /* [retval][out] */ long *nTargets);
        
        DECLSPEC_XFGVIRT(IAccessible2_3, get_selectionRanges)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selectionRanges )( 
            IAccessible2_4 * This,
            /* [size_is][size_is][out] */ IA2Range **ranges,
            /* [retval][out] */ long *nRanges);
        
        DECLSPEC_XFGVIRT(IAccessible2_4, setSelectionRanges)
        HRESULT ( STDMETHODCALLTYPE *setSelectionRanges )( 
            IAccessible2_4 * This,
            /* [in] */ long nRanges,
            /* [size_is][in] */ IA2Range *ranges);
        
        END_INTERFACE
    } IAccessible2_4Vtbl;

    interface IAccessible2_4
    {
        CONST_VTBL struct IAccessible2_4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessible2_4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessible2_4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessible2_4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessible2_4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAccessible2_4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAccessible2_4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAccessible2_4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAccessible2_4_get_accParent(This,ppdispParent)	\
    ( (This)->lpVtbl -> get_accParent(This,ppdispParent) ) 

#define IAccessible2_4_get_accChildCount(This,pcountChildren)	\
    ( (This)->lpVtbl -> get_accChildCount(This,pcountChildren) ) 

#define IAccessible2_4_get_accChild(This,varChild,ppdispChild)	\
    ( (This)->lpVtbl -> get_accChild(This,varChild,ppdispChild) ) 

#define IAccessible2_4_get_accName(This,varChild,pszName)	\
    ( (This)->lpVtbl -> get_accName(This,varChild,pszName) ) 

#define IAccessible2_4_get_accValue(This,varChild,pszValue)	\
    ( (This)->lpVtbl -> get_accValue(This,varChild,pszValue) ) 

#define IAccessible2_4_get_accDescription(This,varChild,pszDescription)	\
    ( (This)->lpVtbl -> get_accDescription(This,varChild,pszDescription) ) 

#define IAccessible2_4_get_accRole(This,varChild,pvarRole)	\
    ( (This)->lpVtbl -> get_accRole(This,varChild,pvarRole) ) 

#define IAccessible2_4_get_accState(This,varChild,pvarState)	\
    ( (This)->lpVtbl -> get_accState(This,varChild,pvarState) ) 

#define IAccessible2_4_get_accHelp(This,varChild,pszHelp)	\
    ( (This)->lpVtbl -> get_accHelp(This,varChild,pszHelp) ) 

#define IAccessible2_4_get_accHelpTopic(This,pszHelpFile,varChild,pidTopic)	\
    ( (This)->lpVtbl -> get_accHelpTopic(This,pszHelpFile,varChild,pidTopic) ) 

#define IAccessible2_4_get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut)	\
    ( (This)->lpVtbl -> get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut) ) 

#define IAccessible2_4_get_accFocus(This,pvarChild)	\
    ( (This)->lpVtbl -> get_accFocus(This,pvarChild) ) 

#define IAccessible2_4_get_accSelection(This,pvarChildren)	\
    ( (This)->lpVtbl -> get_accSelection(This,pvarChildren) ) 

#define IAccessible2_4_get_accDefaultAction(This,varChild,pszDefaultAction)	\
    ( (This)->lpVtbl -> get_accDefaultAction(This,varChild,pszDefaultAction) ) 

#define IAccessible2_4_accSelect(This,flagsSelect,varChild)	\
    ( (This)->lpVtbl -> accSelect(This,flagsSelect,varChild) ) 

#define IAccessible2_4_accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild)	\
    ( (This)->lpVtbl -> accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild) ) 

#define IAccessible2_4_accNavigate(This,navDir,varStart,pvarEndUpAt)	\
    ( (This)->lpVtbl -> accNavigate(This,navDir,varStart,pvarEndUpAt) ) 

#define IAccessible2_4_accHitTest(This,xLeft,yTop,pvarChild)	\
    ( (This)->lpVtbl -> accHitTest(This,xLeft,yTop,pvarChild) ) 

#define IAccessible2_4_accDoDefaultAction(This,varChild)	\
    ( (This)->lpVtbl -> accDoDefaultAction(This,varChild) ) 

#define IAccessible2_4_put_accName(This,varChild,szName)	\
    ( (This)->lpVtbl -> put_accName(This,varChild,szName) ) 

#define IAccessible2_4_put_accValue(This,varChild,szValue)	\
    ( (This)->lpVtbl -> put_accValue(This,varChild,szValue) ) 


#define IAccessible2_4_get_nRelations(This,nRelations)	\
    ( (This)->lpVtbl -> get_nRelations(This,nRelations) ) 

#define IAccessible2_4_get_relation(This,relationIndex,relation)	\
    ( (This)->lpVtbl -> get_relation(This,relationIndex,relation) ) 

#define IAccessible2_4_get_relations(This,maxRelations,relations,nRelations)	\
    ( (This)->lpVtbl -> get_relations(This,maxRelations,relations,nRelations) ) 

#define IAccessible2_4_role(This,role)	\
    ( (This)->lpVtbl -> role(This,role) ) 

#define IAccessible2_4_scrollTo(This,scrollType)	\
    ( (This)->lpVtbl -> scrollTo(This,scrollType) ) 

#define IAccessible2_4_scrollToPoint(This,coordinateType,x,y)	\
    ( (This)->lpVtbl -> scrollToPoint(This,coordinateType,x,y) ) 

#define IAccessible2_4_get_groupPosition(This,groupLevel,similarItemsInGroup,positionInGroup)	\
    ( (This)->lpVtbl -> get_groupPosition(This,groupLevel,similarItemsInGroup,positionInGroup) ) 

#define IAccessible2_4_get_states(This,states)	\
    ( (This)->lpVtbl -> get_states(This,states) ) 

#define IAccessible2_4_get_extendedRole(This,extendedRole)	\
    ( (This)->lpVtbl -> get_extendedRole(This,extendedRole) ) 

#define IAccessible2_4_get_localizedExtendedRole(This,localizedExtendedRole)	\
    ( (This)->lpVtbl -> get_localizedExtendedRole(This,localizedExtendedRole) ) 

#define IAccessible2_4_get_nExtendedStates(This,nExtendedStates)	\
    ( (This)->lpVtbl -> get_nExtendedStates(This,nExtendedStates) ) 

#define IAccessible2_4_get_extendedStates(This,maxExtendedStates,extendedStates,nExtendedStates)	\
    ( (This)->lpVtbl -> get_extendedStates(This,maxExtendedStates,extendedStates,nExtendedStates) ) 

#define IAccessible2_4_get_localizedExtendedStates(This,maxLocalizedExtendedStates,localizedExtendedStates,nLocalizedExtendedStates)	\
    ( (This)->lpVtbl -> get_localizedExtendedStates(This,maxLocalizedExtendedStates,localizedExtendedStates,nLocalizedExtendedStates) ) 

#define IAccessible2_4_get_uniqueID(This,uniqueID)	\
    ( (This)->lpVtbl -> get_uniqueID(This,uniqueID) ) 

#define IAccessible2_4_get_windowHandle(This,windowHandle)	\
    ( (This)->lpVtbl -> get_windowHandle(This,windowHandle) ) 

#define IAccessible2_4_get_indexInParent(This,indexInParent)	\
    ( (This)->lpVtbl -> get_indexInParent(This,indexInParent) ) 

#define IAccessible2_4_get_locale(This,locale)	\
    ( (This)->lpVtbl -> get_locale(This,locale) ) 

#define IAccessible2_4_get_attributes(This,attributes)	\
    ( (This)->lpVtbl -> get_attributes(This,attributes) ) 


#define IAccessible2_4_get_attribute(This,name,attribute)	\
    ( (This)->lpVtbl -> get_attribute(This,name,attribute) ) 

#define IAccessible2_4_get_accessibleWithCaret(This,accessible,caretOffset)	\
    ( (This)->lpVtbl -> get_accessibleWithCaret(This,accessible,caretOffset) ) 

#define IAccessible2_4_get_relationTargetsOfType(This,type,maxTargets,targets,nTargets)	\
    ( (This)->lpVtbl -> get_relationTargetsOfType(This,type,maxTargets,targets,nTargets) ) 


#define IAccessible2_4_get_selectionRanges(This,ranges,nRanges)	\
    ( (This)->lpVtbl -> get_selectionRanges(This,ranges,nRanges) ) 


#define IAccessible2_4_setSelectionRanges(This,nRanges,ranges)	\
    ( (This)->lpVtbl -> setSelectionRanges(This,nRanges,ranges) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessible2_4_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ia2_api_all_0000_0006 */
/* [local] */ 

typedef long IA2Color;



extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0006_v0_0_s_ifspec;

#ifndef __IAccessibleComponent_INTERFACE_DEFINED__
#define __IAccessibleComponent_INTERFACE_DEFINED__

/* interface IAccessibleComponent */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleComponent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1546D4B0-4C98-4bda-89AE-9A64748BDDE4")
    IAccessibleComponent : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_locationInParent( 
            /* [out] */ long *x,
            /* [retval][out] */ long *y) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_foreground( 
            /* [retval][out] */ IA2Color *foreground) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_background( 
            /* [retval][out] */ IA2Color *background) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleComponentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleComponent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleComponent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleComponent * This);
        
        DECLSPEC_XFGVIRT(IAccessibleComponent, get_locationInParent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_locationInParent )( 
            IAccessibleComponent * This,
            /* [out] */ long *x,
            /* [retval][out] */ long *y);
        
        DECLSPEC_XFGVIRT(IAccessibleComponent, get_foreground)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_foreground )( 
            IAccessibleComponent * This,
            /* [retval][out] */ IA2Color *foreground);
        
        DECLSPEC_XFGVIRT(IAccessibleComponent, get_background)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_background )( 
            IAccessibleComponent * This,
            /* [retval][out] */ IA2Color *background);
        
        END_INTERFACE
    } IAccessibleComponentVtbl;

    interface IAccessibleComponent
    {
        CONST_VTBL struct IAccessibleComponentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleComponent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleComponent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleComponent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleComponent_get_locationInParent(This,x,y)	\
    ( (This)->lpVtbl -> get_locationInParent(This,x,y) ) 

#define IAccessibleComponent_get_foreground(This,foreground)	\
    ( (This)->lpVtbl -> get_foreground(This,foreground) ) 

#define IAccessibleComponent_get_background(This,background)	\
    ( (This)->lpVtbl -> get_background(This,background) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleComponent_INTERFACE_DEFINED__ */


#ifndef __IAccessibleValue_INTERFACE_DEFINED__
#define __IAccessibleValue_INTERFACE_DEFINED__

/* interface IAccessibleValue */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleValue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("35855B5B-C566-4fd0-A7B1-E65465600394")
    IAccessibleValue : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_currentValue( 
            /* [retval][out] */ VARIANT *currentValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setCurrentValue( 
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_maximumValue( 
            /* [retval][out] */ VARIANT *maximumValue) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_minimumValue( 
            /* [retval][out] */ VARIANT *minimumValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleValueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleValue * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleValue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleValue * This);
        
        DECLSPEC_XFGVIRT(IAccessibleValue, get_currentValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_currentValue )( 
            IAccessibleValue * This,
            /* [retval][out] */ VARIANT *currentValue);
        
        DECLSPEC_XFGVIRT(IAccessibleValue, setCurrentValue)
        HRESULT ( STDMETHODCALLTYPE *setCurrentValue )( 
            IAccessibleValue * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IAccessibleValue, get_maximumValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_maximumValue )( 
            IAccessibleValue * This,
            /* [retval][out] */ VARIANT *maximumValue);
        
        DECLSPEC_XFGVIRT(IAccessibleValue, get_minimumValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_minimumValue )( 
            IAccessibleValue * This,
            /* [retval][out] */ VARIANT *minimumValue);
        
        END_INTERFACE
    } IAccessibleValueVtbl;

    interface IAccessibleValue
    {
        CONST_VTBL struct IAccessibleValueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleValue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleValue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleValue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleValue_get_currentValue(This,currentValue)	\
    ( (This)->lpVtbl -> get_currentValue(This,currentValue) ) 

#define IAccessibleValue_setCurrentValue(This,value)	\
    ( (This)->lpVtbl -> setCurrentValue(This,value) ) 

#define IAccessibleValue_get_maximumValue(This,maximumValue)	\
    ( (This)->lpVtbl -> get_maximumValue(This,maximumValue) ) 

#define IAccessibleValue_get_minimumValue(This,minimumValue)	\
    ( (This)->lpVtbl -> get_minimumValue(This,minimumValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleValue_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ia2_api_all_0000_0008 */
/* [local] */ 

typedef struct IA2TextSegment
    {
    BSTR text;
    long start;
    long end;
    } 	IA2TextSegment;


enum IA2TextBoundaryType
    {
        IA2_TEXT_BOUNDARY_CHAR	= 0,
        IA2_TEXT_BOUNDARY_WORD	= ( IA2_TEXT_BOUNDARY_CHAR + 1 ) ,
        IA2_TEXT_BOUNDARY_SENTENCE	= ( IA2_TEXT_BOUNDARY_WORD + 1 ) ,
        IA2_TEXT_BOUNDARY_PARAGRAPH	= ( IA2_TEXT_BOUNDARY_SENTENCE + 1 ) ,
        IA2_TEXT_BOUNDARY_LINE	= ( IA2_TEXT_BOUNDARY_PARAGRAPH + 1 ) ,
        IA2_TEXT_BOUNDARY_ALL	= ( IA2_TEXT_BOUNDARY_LINE + 1 ) 
    } ;


extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0008_v0_0_s_ifspec;

#ifndef __IAccessibleText_INTERFACE_DEFINED__
#define __IAccessibleText_INTERFACE_DEFINED__

/* interface IAccessibleText */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleText;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24FD2FFB-3AAD-4a08-8335-A3AD89C0FB4B")
    IAccessibleText : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE addSelection( 
            /* [in] */ long startOffset,
            /* [in] */ long endOffset) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_attributes( 
            /* [in] */ long offset,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *textAttributes) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_caretOffset( 
            /* [retval][out] */ long *offset) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_characterExtents( 
            /* [in] */ long offset,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [out] */ long *x,
            /* [out] */ long *y,
            /* [out] */ long *width,
            /* [retval][out] */ long *height) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nSelections( 
            /* [retval][out] */ long *nSelections) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_offsetAtPoint( 
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [retval][out] */ long *offset) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selection( 
            /* [in] */ long selectionIndex,
            /* [out] */ long *startOffset,
            /* [retval][out] */ long *endOffset) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_text( 
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [retval][out] */ BSTR *text) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_textBeforeOffset( 
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_textAfterOffset( 
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_textAtOffset( 
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE removeSelection( 
            /* [in] */ long selectionIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setCaretOffset( 
            /* [in] */ long offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setSelection( 
            /* [in] */ long selectionIndex,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nCharacters( 
            /* [retval][out] */ long *nCharacters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE scrollSubstringTo( 
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2ScrollType scrollType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE scrollSubstringToPoint( 
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_newText( 
            /* [retval][out] */ IA2TextSegment *newText) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_oldText( 
            /* [retval][out] */ IA2TextSegment *oldText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleTextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleText * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleText * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleText * This);
        
        DECLSPEC_XFGVIRT(IAccessibleText, addSelection)
        HRESULT ( STDMETHODCALLTYPE *addSelection )( 
            IAccessibleText * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_attributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IAccessibleText * This,
            /* [in] */ long offset,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *textAttributes);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_caretOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_caretOffset )( 
            IAccessibleText * This,
            /* [retval][out] */ long *offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_characterExtents)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_characterExtents )( 
            IAccessibleText * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [out] */ long *x,
            /* [out] */ long *y,
            /* [out] */ long *width,
            /* [retval][out] */ long *height);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_nSelections)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelections )( 
            IAccessibleText * This,
            /* [retval][out] */ long *nSelections);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_offsetAtPoint)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_offsetAtPoint )( 
            IAccessibleText * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [retval][out] */ long *offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_selection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selection )( 
            IAccessibleText * This,
            /* [in] */ long selectionIndex,
            /* [out] */ long *startOffset,
            /* [retval][out] */ long *endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_text)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IAccessibleText * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textBeforeOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textBeforeOffset )( 
            IAccessibleText * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textAfterOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textAfterOffset )( 
            IAccessibleText * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textAtOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textAtOffset )( 
            IAccessibleText * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, removeSelection)
        HRESULT ( STDMETHODCALLTYPE *removeSelection )( 
            IAccessibleText * This,
            /* [in] */ long selectionIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleText, setCaretOffset)
        HRESULT ( STDMETHODCALLTYPE *setCaretOffset )( 
            IAccessibleText * This,
            /* [in] */ long offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, setSelection)
        HRESULT ( STDMETHODCALLTYPE *setSelection )( 
            IAccessibleText * This,
            /* [in] */ long selectionIndex,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_nCharacters)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nCharacters )( 
            IAccessibleText * This,
            /* [retval][out] */ long *nCharacters);
        
        DECLSPEC_XFGVIRT(IAccessibleText, scrollSubstringTo)
        HRESULT ( STDMETHODCALLTYPE *scrollSubstringTo )( 
            IAccessibleText * This,
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2ScrollType scrollType);
        
        DECLSPEC_XFGVIRT(IAccessibleText, scrollSubstringToPoint)
        HRESULT ( STDMETHODCALLTYPE *scrollSubstringToPoint )( 
            IAccessibleText * This,
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_newText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_newText )( 
            IAccessibleText * This,
            /* [retval][out] */ IA2TextSegment *newText);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_oldText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_oldText )( 
            IAccessibleText * This,
            /* [retval][out] */ IA2TextSegment *oldText);
        
        END_INTERFACE
    } IAccessibleTextVtbl;

    interface IAccessibleText
    {
        CONST_VTBL struct IAccessibleTextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleText_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleText_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleText_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleText_addSelection(This,startOffset,endOffset)	\
    ( (This)->lpVtbl -> addSelection(This,startOffset,endOffset) ) 

#define IAccessibleText_get_attributes(This,offset,startOffset,endOffset,textAttributes)	\
    ( (This)->lpVtbl -> get_attributes(This,offset,startOffset,endOffset,textAttributes) ) 

#define IAccessibleText_get_caretOffset(This,offset)	\
    ( (This)->lpVtbl -> get_caretOffset(This,offset) ) 

#define IAccessibleText_get_characterExtents(This,offset,coordType,x,y,width,height)	\
    ( (This)->lpVtbl -> get_characterExtents(This,offset,coordType,x,y,width,height) ) 

#define IAccessibleText_get_nSelections(This,nSelections)	\
    ( (This)->lpVtbl -> get_nSelections(This,nSelections) ) 

#define IAccessibleText_get_offsetAtPoint(This,x,y,coordType,offset)	\
    ( (This)->lpVtbl -> get_offsetAtPoint(This,x,y,coordType,offset) ) 

#define IAccessibleText_get_selection(This,selectionIndex,startOffset,endOffset)	\
    ( (This)->lpVtbl -> get_selection(This,selectionIndex,startOffset,endOffset) ) 

#define IAccessibleText_get_text(This,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_text(This,startOffset,endOffset,text) ) 

#define IAccessibleText_get_textBeforeOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textBeforeOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleText_get_textAfterOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textAfterOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleText_get_textAtOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textAtOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleText_removeSelection(This,selectionIndex)	\
    ( (This)->lpVtbl -> removeSelection(This,selectionIndex) ) 

#define IAccessibleText_setCaretOffset(This,offset)	\
    ( (This)->lpVtbl -> setCaretOffset(This,offset) ) 

#define IAccessibleText_setSelection(This,selectionIndex,startOffset,endOffset)	\
    ( (This)->lpVtbl -> setSelection(This,selectionIndex,startOffset,endOffset) ) 

#define IAccessibleText_get_nCharacters(This,nCharacters)	\
    ( (This)->lpVtbl -> get_nCharacters(This,nCharacters) ) 

#define IAccessibleText_scrollSubstringTo(This,startIndex,endIndex,scrollType)	\
    ( (This)->lpVtbl -> scrollSubstringTo(This,startIndex,endIndex,scrollType) ) 

#define IAccessibleText_scrollSubstringToPoint(This,startIndex,endIndex,coordinateType,x,y)	\
    ( (This)->lpVtbl -> scrollSubstringToPoint(This,startIndex,endIndex,coordinateType,x,y) ) 

#define IAccessibleText_get_newText(This,newText)	\
    ( (This)->lpVtbl -> get_newText(This,newText) ) 

#define IAccessibleText_get_oldText(This,oldText)	\
    ( (This)->lpVtbl -> get_oldText(This,oldText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleText_INTERFACE_DEFINED__ */


#ifndef __IAccessibleText2_INTERFACE_DEFINED__
#define __IAccessibleText2_INTERFACE_DEFINED__

/* interface IAccessibleText2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleText2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9690A9CC-5C80-4DF5-852E-2D5AE4189A54")
    IAccessibleText2 : public IAccessibleText
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_attributeRange( 
            /* [in] */ long offset,
            /* [in] */ BSTR filter,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *attributeValues) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleText2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleText2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleText2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleText2 * This);
        
        DECLSPEC_XFGVIRT(IAccessibleText, addSelection)
        HRESULT ( STDMETHODCALLTYPE *addSelection )( 
            IAccessibleText2 * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_attributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IAccessibleText2 * This,
            /* [in] */ long offset,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *textAttributes);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_caretOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_caretOffset )( 
            IAccessibleText2 * This,
            /* [retval][out] */ long *offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_characterExtents)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_characterExtents )( 
            IAccessibleText2 * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [out] */ long *x,
            /* [out] */ long *y,
            /* [out] */ long *width,
            /* [retval][out] */ long *height);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_nSelections)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelections )( 
            IAccessibleText2 * This,
            /* [retval][out] */ long *nSelections);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_offsetAtPoint)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_offsetAtPoint )( 
            IAccessibleText2 * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [retval][out] */ long *offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_selection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selection )( 
            IAccessibleText2 * This,
            /* [in] */ long selectionIndex,
            /* [out] */ long *startOffset,
            /* [retval][out] */ long *endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_text)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IAccessibleText2 * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textBeforeOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textBeforeOffset )( 
            IAccessibleText2 * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textAfterOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textAfterOffset )( 
            IAccessibleText2 * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textAtOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textAtOffset )( 
            IAccessibleText2 * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, removeSelection)
        HRESULT ( STDMETHODCALLTYPE *removeSelection )( 
            IAccessibleText2 * This,
            /* [in] */ long selectionIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleText, setCaretOffset)
        HRESULT ( STDMETHODCALLTYPE *setCaretOffset )( 
            IAccessibleText2 * This,
            /* [in] */ long offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, setSelection)
        HRESULT ( STDMETHODCALLTYPE *setSelection )( 
            IAccessibleText2 * This,
            /* [in] */ long selectionIndex,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_nCharacters)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nCharacters )( 
            IAccessibleText2 * This,
            /* [retval][out] */ long *nCharacters);
        
        DECLSPEC_XFGVIRT(IAccessibleText, scrollSubstringTo)
        HRESULT ( STDMETHODCALLTYPE *scrollSubstringTo )( 
            IAccessibleText2 * This,
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2ScrollType scrollType);
        
        DECLSPEC_XFGVIRT(IAccessibleText, scrollSubstringToPoint)
        HRESULT ( STDMETHODCALLTYPE *scrollSubstringToPoint )( 
            IAccessibleText2 * This,
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_newText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_newText )( 
            IAccessibleText2 * This,
            /* [retval][out] */ IA2TextSegment *newText);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_oldText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_oldText )( 
            IAccessibleText2 * This,
            /* [retval][out] */ IA2TextSegment *oldText);
        
        DECLSPEC_XFGVIRT(IAccessibleText2, get_attributeRange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributeRange )( 
            IAccessibleText2 * This,
            /* [in] */ long offset,
            /* [in] */ BSTR filter,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *attributeValues);
        
        END_INTERFACE
    } IAccessibleText2Vtbl;

    interface IAccessibleText2
    {
        CONST_VTBL struct IAccessibleText2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleText2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleText2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleText2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleText2_addSelection(This,startOffset,endOffset)	\
    ( (This)->lpVtbl -> addSelection(This,startOffset,endOffset) ) 

#define IAccessibleText2_get_attributes(This,offset,startOffset,endOffset,textAttributes)	\
    ( (This)->lpVtbl -> get_attributes(This,offset,startOffset,endOffset,textAttributes) ) 

#define IAccessibleText2_get_caretOffset(This,offset)	\
    ( (This)->lpVtbl -> get_caretOffset(This,offset) ) 

#define IAccessibleText2_get_characterExtents(This,offset,coordType,x,y,width,height)	\
    ( (This)->lpVtbl -> get_characterExtents(This,offset,coordType,x,y,width,height) ) 

#define IAccessibleText2_get_nSelections(This,nSelections)	\
    ( (This)->lpVtbl -> get_nSelections(This,nSelections) ) 

#define IAccessibleText2_get_offsetAtPoint(This,x,y,coordType,offset)	\
    ( (This)->lpVtbl -> get_offsetAtPoint(This,x,y,coordType,offset) ) 

#define IAccessibleText2_get_selection(This,selectionIndex,startOffset,endOffset)	\
    ( (This)->lpVtbl -> get_selection(This,selectionIndex,startOffset,endOffset) ) 

#define IAccessibleText2_get_text(This,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_text(This,startOffset,endOffset,text) ) 

#define IAccessibleText2_get_textBeforeOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textBeforeOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleText2_get_textAfterOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textAfterOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleText2_get_textAtOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textAtOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleText2_removeSelection(This,selectionIndex)	\
    ( (This)->lpVtbl -> removeSelection(This,selectionIndex) ) 

#define IAccessibleText2_setCaretOffset(This,offset)	\
    ( (This)->lpVtbl -> setCaretOffset(This,offset) ) 

#define IAccessibleText2_setSelection(This,selectionIndex,startOffset,endOffset)	\
    ( (This)->lpVtbl -> setSelection(This,selectionIndex,startOffset,endOffset) ) 

#define IAccessibleText2_get_nCharacters(This,nCharacters)	\
    ( (This)->lpVtbl -> get_nCharacters(This,nCharacters) ) 

#define IAccessibleText2_scrollSubstringTo(This,startIndex,endIndex,scrollType)	\
    ( (This)->lpVtbl -> scrollSubstringTo(This,startIndex,endIndex,scrollType) ) 

#define IAccessibleText2_scrollSubstringToPoint(This,startIndex,endIndex,coordinateType,x,y)	\
    ( (This)->lpVtbl -> scrollSubstringToPoint(This,startIndex,endIndex,coordinateType,x,y) ) 

#define IAccessibleText2_get_newText(This,newText)	\
    ( (This)->lpVtbl -> get_newText(This,newText) ) 

#define IAccessibleText2_get_oldText(This,oldText)	\
    ( (This)->lpVtbl -> get_oldText(This,oldText) ) 


#define IAccessibleText2_get_attributeRange(This,offset,filter,startOffset,endOffset,attributeValues)	\
    ( (This)->lpVtbl -> get_attributeRange(This,offset,filter,startOffset,endOffset,attributeValues) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleText2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ia2_api_all_0000_0010 */
/* [local] */ 

typedef struct IA2TextSelection
    {
    IAccessibleText *startObj;
    long startOffset;
    IAccessibleText *endObj;
    long endOffset;
    boolean startIsActive;
    } 	IA2TextSelection;



extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0010_v0_0_s_ifspec;

#ifndef __IAccessibleTextSelectionContainer_INTERFACE_DEFINED__
#define __IAccessibleTextSelectionContainer_INTERFACE_DEFINED__

/* interface IAccessibleTextSelectionContainer */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleTextSelectionContainer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2118B599-733F-43D0-A569-0B31D125ED9A")
    IAccessibleTextSelectionContainer : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selections( 
            /* [size_is][size_is][out] */ IA2TextSelection **selections,
            /* [retval][out] */ long *nSelections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setSelections( 
            /* [in] */ long nSelections,
            /* [size_is][in] */ IA2TextSelection *selections) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleTextSelectionContainerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleTextSelectionContainer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleTextSelectionContainer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleTextSelectionContainer * This);
        
        DECLSPEC_XFGVIRT(IAccessibleTextSelectionContainer, get_selections)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selections )( 
            IAccessibleTextSelectionContainer * This,
            /* [size_is][size_is][out] */ IA2TextSelection **selections,
            /* [retval][out] */ long *nSelections);
        
        DECLSPEC_XFGVIRT(IAccessibleTextSelectionContainer, setSelections)
        HRESULT ( STDMETHODCALLTYPE *setSelections )( 
            IAccessibleTextSelectionContainer * This,
            /* [in] */ long nSelections,
            /* [size_is][in] */ IA2TextSelection *selections);
        
        END_INTERFACE
    } IAccessibleTextSelectionContainerVtbl;

    interface IAccessibleTextSelectionContainer
    {
        CONST_VTBL struct IAccessibleTextSelectionContainerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleTextSelectionContainer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleTextSelectionContainer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleTextSelectionContainer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleTextSelectionContainer_get_selections(This,selections,nSelections)	\
    ( (This)->lpVtbl -> get_selections(This,selections,nSelections) ) 

#define IAccessibleTextSelectionContainer_setSelections(This,nSelections,selections)	\
    ( (This)->lpVtbl -> setSelections(This,nSelections,selections) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleTextSelectionContainer_INTERFACE_DEFINED__ */


#ifndef __IAccessibleEditableText_INTERFACE_DEFINED__
#define __IAccessibleEditableText_INTERFACE_DEFINED__

/* interface IAccessibleEditableText */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleEditableText;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A59AA09A-7011-4b65-939D-32B1FB5547E3")
    IAccessibleEditableText : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE copyText( 
            /* [in] */ long startOffset,
            /* [in] */ long endOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE deleteText( 
            /* [in] */ long startOffset,
            /* [in] */ long endOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE insertText( 
            /* [in] */ long offset,
            /* [in] */ BSTR *text) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE cutText( 
            /* [in] */ long startOffset,
            /* [in] */ long endOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE pasteText( 
            /* [in] */ long offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE replaceText( 
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [in] */ BSTR *text) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setAttributes( 
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [in] */ BSTR *attributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleEditableTextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleEditableText * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleEditableText * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleEditableText * This);
        
        DECLSPEC_XFGVIRT(IAccessibleEditableText, copyText)
        HRESULT ( STDMETHODCALLTYPE *copyText )( 
            IAccessibleEditableText * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleEditableText, deleteText)
        HRESULT ( STDMETHODCALLTYPE *deleteText )( 
            IAccessibleEditableText * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleEditableText, insertText)
        HRESULT ( STDMETHODCALLTYPE *insertText )( 
            IAccessibleEditableText * This,
            /* [in] */ long offset,
            /* [in] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleEditableText, cutText)
        HRESULT ( STDMETHODCALLTYPE *cutText )( 
            IAccessibleEditableText * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleEditableText, pasteText)
        HRESULT ( STDMETHODCALLTYPE *pasteText )( 
            IAccessibleEditableText * This,
            /* [in] */ long offset);
        
        DECLSPEC_XFGVIRT(IAccessibleEditableText, replaceText)
        HRESULT ( STDMETHODCALLTYPE *replaceText )( 
            IAccessibleEditableText * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [in] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleEditableText, setAttributes)
        HRESULT ( STDMETHODCALLTYPE *setAttributes )( 
            IAccessibleEditableText * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [in] */ BSTR *attributes);
        
        END_INTERFACE
    } IAccessibleEditableTextVtbl;

    interface IAccessibleEditableText
    {
        CONST_VTBL struct IAccessibleEditableTextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleEditableText_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleEditableText_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleEditableText_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleEditableText_copyText(This,startOffset,endOffset)	\
    ( (This)->lpVtbl -> copyText(This,startOffset,endOffset) ) 

#define IAccessibleEditableText_deleteText(This,startOffset,endOffset)	\
    ( (This)->lpVtbl -> deleteText(This,startOffset,endOffset) ) 

#define IAccessibleEditableText_insertText(This,offset,text)	\
    ( (This)->lpVtbl -> insertText(This,offset,text) ) 

#define IAccessibleEditableText_cutText(This,startOffset,endOffset)	\
    ( (This)->lpVtbl -> cutText(This,startOffset,endOffset) ) 

#define IAccessibleEditableText_pasteText(This,offset)	\
    ( (This)->lpVtbl -> pasteText(This,offset) ) 

#define IAccessibleEditableText_replaceText(This,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> replaceText(This,startOffset,endOffset,text) ) 

#define IAccessibleEditableText_setAttributes(This,startOffset,endOffset,attributes)	\
    ( (This)->lpVtbl -> setAttributes(This,startOffset,endOffset,attributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleEditableText_INTERFACE_DEFINED__ */


#ifndef __IAccessibleHyperlink_INTERFACE_DEFINED__
#define __IAccessibleHyperlink_INTERFACE_DEFINED__

/* interface IAccessibleHyperlink */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleHyperlink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("01C20F2B-3DD2-400f-949F-AD00BDAB1D41")
    IAccessibleHyperlink : public IAccessibleAction
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_anchor( 
            /* [in] */ long index,
            /* [retval][out] */ VARIANT *anchor) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_anchorTarget( 
            /* [in] */ long index,
            /* [retval][out] */ VARIANT *anchorTarget) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_startIndex( 
            /* [retval][out] */ long *index) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_endIndex( 
            /* [retval][out] */ long *index) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_valid( 
            /* [retval][out] */ boolean *valid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleHyperlinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleHyperlink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleHyperlink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleHyperlink * This);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, nActions)
        HRESULT ( STDMETHODCALLTYPE *nActions )( 
            IAccessibleHyperlink * This,
            /* [retval][out] */ long *nActions);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, doAction)
        HRESULT ( STDMETHODCALLTYPE *doAction )( 
            IAccessibleHyperlink * This,
            /* [in] */ long actionIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, get_description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_description )( 
            IAccessibleHyperlink * This,
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, get_keyBinding)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_keyBinding )( 
            IAccessibleHyperlink * This,
            /* [in] */ long actionIndex,
            /* [in] */ long nMaxBindings,
            /* [length_is][length_is][size_is][size_is][out] */ BSTR **keyBindings,
            /* [retval][out] */ long *nBindings);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, get_name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            IAccessibleHyperlink * This,
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IAccessibleAction, get_localizedName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_localizedName )( 
            IAccessibleHyperlink * This,
            /* [in] */ long actionIndex,
            /* [retval][out] */ BSTR *localizedName);
        
        DECLSPEC_XFGVIRT(IAccessibleHyperlink, get_anchor)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_anchor )( 
            IAccessibleHyperlink * This,
            /* [in] */ long index,
            /* [retval][out] */ VARIANT *anchor);
        
        DECLSPEC_XFGVIRT(IAccessibleHyperlink, get_anchorTarget)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_anchorTarget )( 
            IAccessibleHyperlink * This,
            /* [in] */ long index,
            /* [retval][out] */ VARIANT *anchorTarget);
        
        DECLSPEC_XFGVIRT(IAccessibleHyperlink, get_startIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_startIndex )( 
            IAccessibleHyperlink * This,
            /* [retval][out] */ long *index);
        
        DECLSPEC_XFGVIRT(IAccessibleHyperlink, get_endIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_endIndex )( 
            IAccessibleHyperlink * This,
            /* [retval][out] */ long *index);
        
        DECLSPEC_XFGVIRT(IAccessibleHyperlink, get_valid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_valid )( 
            IAccessibleHyperlink * This,
            /* [retval][out] */ boolean *valid);
        
        END_INTERFACE
    } IAccessibleHyperlinkVtbl;

    interface IAccessibleHyperlink
    {
        CONST_VTBL struct IAccessibleHyperlinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleHyperlink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleHyperlink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleHyperlink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleHyperlink_nActions(This,nActions)	\
    ( (This)->lpVtbl -> nActions(This,nActions) ) 

#define IAccessibleHyperlink_doAction(This,actionIndex)	\
    ( (This)->lpVtbl -> doAction(This,actionIndex) ) 

#define IAccessibleHyperlink_get_description(This,actionIndex,description)	\
    ( (This)->lpVtbl -> get_description(This,actionIndex,description) ) 

#define IAccessibleHyperlink_get_keyBinding(This,actionIndex,nMaxBindings,keyBindings,nBindings)	\
    ( (This)->lpVtbl -> get_keyBinding(This,actionIndex,nMaxBindings,keyBindings,nBindings) ) 

#define IAccessibleHyperlink_get_name(This,actionIndex,name)	\
    ( (This)->lpVtbl -> get_name(This,actionIndex,name) ) 

#define IAccessibleHyperlink_get_localizedName(This,actionIndex,localizedName)	\
    ( (This)->lpVtbl -> get_localizedName(This,actionIndex,localizedName) ) 


#define IAccessibleHyperlink_get_anchor(This,index,anchor)	\
    ( (This)->lpVtbl -> get_anchor(This,index,anchor) ) 

#define IAccessibleHyperlink_get_anchorTarget(This,index,anchorTarget)	\
    ( (This)->lpVtbl -> get_anchorTarget(This,index,anchorTarget) ) 

#define IAccessibleHyperlink_get_startIndex(This,index)	\
    ( (This)->lpVtbl -> get_startIndex(This,index) ) 

#define IAccessibleHyperlink_get_endIndex(This,index)	\
    ( (This)->lpVtbl -> get_endIndex(This,index) ) 

#define IAccessibleHyperlink_get_valid(This,valid)	\
    ( (This)->lpVtbl -> get_valid(This,valid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleHyperlink_INTERFACE_DEFINED__ */


#ifndef __IAccessibleHypertext_INTERFACE_DEFINED__
#define __IAccessibleHypertext_INTERFACE_DEFINED__

/* interface IAccessibleHypertext */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleHypertext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6B4F8BBF-F1F2-418a-B35E-A195BC4103B9")
    IAccessibleHypertext : public IAccessibleText
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nHyperlinks( 
            /* [retval][out] */ long *hyperlinkCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_hyperlink( 
            /* [in] */ long index,
            /* [retval][out] */ IAccessibleHyperlink **hyperlink) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_hyperlinkIndex( 
            /* [in] */ long charIndex,
            /* [retval][out] */ long *hyperlinkIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleHypertextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleHypertext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleHypertext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleHypertext * This);
        
        DECLSPEC_XFGVIRT(IAccessibleText, addSelection)
        HRESULT ( STDMETHODCALLTYPE *addSelection )( 
            IAccessibleHypertext * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_attributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IAccessibleHypertext * This,
            /* [in] */ long offset,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *textAttributes);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_caretOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_caretOffset )( 
            IAccessibleHypertext * This,
            /* [retval][out] */ long *offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_characterExtents)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_characterExtents )( 
            IAccessibleHypertext * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [out] */ long *x,
            /* [out] */ long *y,
            /* [out] */ long *width,
            /* [retval][out] */ long *height);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_nSelections)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelections )( 
            IAccessibleHypertext * This,
            /* [retval][out] */ long *nSelections);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_offsetAtPoint)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_offsetAtPoint )( 
            IAccessibleHypertext * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [retval][out] */ long *offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_selection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selection )( 
            IAccessibleHypertext * This,
            /* [in] */ long selectionIndex,
            /* [out] */ long *startOffset,
            /* [retval][out] */ long *endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_text)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IAccessibleHypertext * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textBeforeOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textBeforeOffset )( 
            IAccessibleHypertext * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textAfterOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textAfterOffset )( 
            IAccessibleHypertext * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textAtOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textAtOffset )( 
            IAccessibleHypertext * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, removeSelection)
        HRESULT ( STDMETHODCALLTYPE *removeSelection )( 
            IAccessibleHypertext * This,
            /* [in] */ long selectionIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleText, setCaretOffset)
        HRESULT ( STDMETHODCALLTYPE *setCaretOffset )( 
            IAccessibleHypertext * This,
            /* [in] */ long offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, setSelection)
        HRESULT ( STDMETHODCALLTYPE *setSelection )( 
            IAccessibleHypertext * This,
            /* [in] */ long selectionIndex,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_nCharacters)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nCharacters )( 
            IAccessibleHypertext * This,
            /* [retval][out] */ long *nCharacters);
        
        DECLSPEC_XFGVIRT(IAccessibleText, scrollSubstringTo)
        HRESULT ( STDMETHODCALLTYPE *scrollSubstringTo )( 
            IAccessibleHypertext * This,
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2ScrollType scrollType);
        
        DECLSPEC_XFGVIRT(IAccessibleText, scrollSubstringToPoint)
        HRESULT ( STDMETHODCALLTYPE *scrollSubstringToPoint )( 
            IAccessibleHypertext * This,
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_newText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_newText )( 
            IAccessibleHypertext * This,
            /* [retval][out] */ IA2TextSegment *newText);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_oldText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_oldText )( 
            IAccessibleHypertext * This,
            /* [retval][out] */ IA2TextSegment *oldText);
        
        DECLSPEC_XFGVIRT(IAccessibleHypertext, get_nHyperlinks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nHyperlinks )( 
            IAccessibleHypertext * This,
            /* [retval][out] */ long *hyperlinkCount);
        
        DECLSPEC_XFGVIRT(IAccessibleHypertext, get_hyperlink)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_hyperlink )( 
            IAccessibleHypertext * This,
            /* [in] */ long index,
            /* [retval][out] */ IAccessibleHyperlink **hyperlink);
        
        DECLSPEC_XFGVIRT(IAccessibleHypertext, get_hyperlinkIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_hyperlinkIndex )( 
            IAccessibleHypertext * This,
            /* [in] */ long charIndex,
            /* [retval][out] */ long *hyperlinkIndex);
        
        END_INTERFACE
    } IAccessibleHypertextVtbl;

    interface IAccessibleHypertext
    {
        CONST_VTBL struct IAccessibleHypertextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleHypertext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleHypertext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleHypertext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleHypertext_addSelection(This,startOffset,endOffset)	\
    ( (This)->lpVtbl -> addSelection(This,startOffset,endOffset) ) 

#define IAccessibleHypertext_get_attributes(This,offset,startOffset,endOffset,textAttributes)	\
    ( (This)->lpVtbl -> get_attributes(This,offset,startOffset,endOffset,textAttributes) ) 

#define IAccessibleHypertext_get_caretOffset(This,offset)	\
    ( (This)->lpVtbl -> get_caretOffset(This,offset) ) 

#define IAccessibleHypertext_get_characterExtents(This,offset,coordType,x,y,width,height)	\
    ( (This)->lpVtbl -> get_characterExtents(This,offset,coordType,x,y,width,height) ) 

#define IAccessibleHypertext_get_nSelections(This,nSelections)	\
    ( (This)->lpVtbl -> get_nSelections(This,nSelections) ) 

#define IAccessibleHypertext_get_offsetAtPoint(This,x,y,coordType,offset)	\
    ( (This)->lpVtbl -> get_offsetAtPoint(This,x,y,coordType,offset) ) 

#define IAccessibleHypertext_get_selection(This,selectionIndex,startOffset,endOffset)	\
    ( (This)->lpVtbl -> get_selection(This,selectionIndex,startOffset,endOffset) ) 

#define IAccessibleHypertext_get_text(This,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_text(This,startOffset,endOffset,text) ) 

#define IAccessibleHypertext_get_textBeforeOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textBeforeOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleHypertext_get_textAfterOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textAfterOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleHypertext_get_textAtOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textAtOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleHypertext_removeSelection(This,selectionIndex)	\
    ( (This)->lpVtbl -> removeSelection(This,selectionIndex) ) 

#define IAccessibleHypertext_setCaretOffset(This,offset)	\
    ( (This)->lpVtbl -> setCaretOffset(This,offset) ) 

#define IAccessibleHypertext_setSelection(This,selectionIndex,startOffset,endOffset)	\
    ( (This)->lpVtbl -> setSelection(This,selectionIndex,startOffset,endOffset) ) 

#define IAccessibleHypertext_get_nCharacters(This,nCharacters)	\
    ( (This)->lpVtbl -> get_nCharacters(This,nCharacters) ) 

#define IAccessibleHypertext_scrollSubstringTo(This,startIndex,endIndex,scrollType)	\
    ( (This)->lpVtbl -> scrollSubstringTo(This,startIndex,endIndex,scrollType) ) 

#define IAccessibleHypertext_scrollSubstringToPoint(This,startIndex,endIndex,coordinateType,x,y)	\
    ( (This)->lpVtbl -> scrollSubstringToPoint(This,startIndex,endIndex,coordinateType,x,y) ) 

#define IAccessibleHypertext_get_newText(This,newText)	\
    ( (This)->lpVtbl -> get_newText(This,newText) ) 

#define IAccessibleHypertext_get_oldText(This,oldText)	\
    ( (This)->lpVtbl -> get_oldText(This,oldText) ) 


#define IAccessibleHypertext_get_nHyperlinks(This,hyperlinkCount)	\
    ( (This)->lpVtbl -> get_nHyperlinks(This,hyperlinkCount) ) 

#define IAccessibleHypertext_get_hyperlink(This,index,hyperlink)	\
    ( (This)->lpVtbl -> get_hyperlink(This,index,hyperlink) ) 

#define IAccessibleHypertext_get_hyperlinkIndex(This,charIndex,hyperlinkIndex)	\
    ( (This)->lpVtbl -> get_hyperlinkIndex(This,charIndex,hyperlinkIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleHypertext_INTERFACE_DEFINED__ */


#ifndef __IAccessibleHypertext2_INTERFACE_DEFINED__
#define __IAccessibleHypertext2_INTERFACE_DEFINED__

/* interface IAccessibleHypertext2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleHypertext2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CF64D89F-8287-4B44-8501-A827453A6077")
    IAccessibleHypertext2 : public IAccessibleHypertext
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_hyperlinks( 
            /* [size_is][size_is][out] */ IAccessibleHyperlink ***hyperlinks,
            /* [retval][out] */ long *nHyperlinks) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleHypertext2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleHypertext2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleHypertext2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleHypertext2 * This);
        
        DECLSPEC_XFGVIRT(IAccessibleText, addSelection)
        HRESULT ( STDMETHODCALLTYPE *addSelection )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_attributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long offset,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *textAttributes);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_caretOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_caretOffset )( 
            IAccessibleHypertext2 * This,
            /* [retval][out] */ long *offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_characterExtents)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_characterExtents )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [out] */ long *x,
            /* [out] */ long *y,
            /* [out] */ long *width,
            /* [retval][out] */ long *height);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_nSelections)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelections )( 
            IAccessibleHypertext2 * This,
            /* [retval][out] */ long *nSelections);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_offsetAtPoint)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_offsetAtPoint )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ enum IA2CoordinateType coordType,
            /* [retval][out] */ long *offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_selection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selection )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long selectionIndex,
            /* [out] */ long *startOffset,
            /* [retval][out] */ long *endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_text)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textBeforeOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textBeforeOffset )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textAfterOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textAfterOffset )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_textAtOffset)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_textAtOffset )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long offset,
            /* [in] */ enum IA2TextBoundaryType boundaryType,
            /* [out] */ long *startOffset,
            /* [out] */ long *endOffset,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IAccessibleText, removeSelection)
        HRESULT ( STDMETHODCALLTYPE *removeSelection )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long selectionIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleText, setCaretOffset)
        HRESULT ( STDMETHODCALLTYPE *setCaretOffset )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long offset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, setSelection)
        HRESULT ( STDMETHODCALLTYPE *setSelection )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long selectionIndex,
            /* [in] */ long startOffset,
            /* [in] */ long endOffset);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_nCharacters)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nCharacters )( 
            IAccessibleHypertext2 * This,
            /* [retval][out] */ long *nCharacters);
        
        DECLSPEC_XFGVIRT(IAccessibleText, scrollSubstringTo)
        HRESULT ( STDMETHODCALLTYPE *scrollSubstringTo )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2ScrollType scrollType);
        
        DECLSPEC_XFGVIRT(IAccessibleText, scrollSubstringToPoint)
        HRESULT ( STDMETHODCALLTYPE *scrollSubstringToPoint )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long startIndex,
            /* [in] */ long endIndex,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [in] */ long x,
            /* [in] */ long y);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_newText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_newText )( 
            IAccessibleHypertext2 * This,
            /* [retval][out] */ IA2TextSegment *newText);
        
        DECLSPEC_XFGVIRT(IAccessibleText, get_oldText)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_oldText )( 
            IAccessibleHypertext2 * This,
            /* [retval][out] */ IA2TextSegment *oldText);
        
        DECLSPEC_XFGVIRT(IAccessibleHypertext, get_nHyperlinks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nHyperlinks )( 
            IAccessibleHypertext2 * This,
            /* [retval][out] */ long *hyperlinkCount);
        
        DECLSPEC_XFGVIRT(IAccessibleHypertext, get_hyperlink)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_hyperlink )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long index,
            /* [retval][out] */ IAccessibleHyperlink **hyperlink);
        
        DECLSPEC_XFGVIRT(IAccessibleHypertext, get_hyperlinkIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_hyperlinkIndex )( 
            IAccessibleHypertext2 * This,
            /* [in] */ long charIndex,
            /* [retval][out] */ long *hyperlinkIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleHypertext2, get_hyperlinks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_hyperlinks )( 
            IAccessibleHypertext2 * This,
            /* [size_is][size_is][out] */ IAccessibleHyperlink ***hyperlinks,
            /* [retval][out] */ long *nHyperlinks);
        
        END_INTERFACE
    } IAccessibleHypertext2Vtbl;

    interface IAccessibleHypertext2
    {
        CONST_VTBL struct IAccessibleHypertext2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleHypertext2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleHypertext2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleHypertext2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleHypertext2_addSelection(This,startOffset,endOffset)	\
    ( (This)->lpVtbl -> addSelection(This,startOffset,endOffset) ) 

#define IAccessibleHypertext2_get_attributes(This,offset,startOffset,endOffset,textAttributes)	\
    ( (This)->lpVtbl -> get_attributes(This,offset,startOffset,endOffset,textAttributes) ) 

#define IAccessibleHypertext2_get_caretOffset(This,offset)	\
    ( (This)->lpVtbl -> get_caretOffset(This,offset) ) 

#define IAccessibleHypertext2_get_characterExtents(This,offset,coordType,x,y,width,height)	\
    ( (This)->lpVtbl -> get_characterExtents(This,offset,coordType,x,y,width,height) ) 

#define IAccessibleHypertext2_get_nSelections(This,nSelections)	\
    ( (This)->lpVtbl -> get_nSelections(This,nSelections) ) 

#define IAccessibleHypertext2_get_offsetAtPoint(This,x,y,coordType,offset)	\
    ( (This)->lpVtbl -> get_offsetAtPoint(This,x,y,coordType,offset) ) 

#define IAccessibleHypertext2_get_selection(This,selectionIndex,startOffset,endOffset)	\
    ( (This)->lpVtbl -> get_selection(This,selectionIndex,startOffset,endOffset) ) 

#define IAccessibleHypertext2_get_text(This,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_text(This,startOffset,endOffset,text) ) 

#define IAccessibleHypertext2_get_textBeforeOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textBeforeOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleHypertext2_get_textAfterOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textAfterOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleHypertext2_get_textAtOffset(This,offset,boundaryType,startOffset,endOffset,text)	\
    ( (This)->lpVtbl -> get_textAtOffset(This,offset,boundaryType,startOffset,endOffset,text) ) 

#define IAccessibleHypertext2_removeSelection(This,selectionIndex)	\
    ( (This)->lpVtbl -> removeSelection(This,selectionIndex) ) 

#define IAccessibleHypertext2_setCaretOffset(This,offset)	\
    ( (This)->lpVtbl -> setCaretOffset(This,offset) ) 

#define IAccessibleHypertext2_setSelection(This,selectionIndex,startOffset,endOffset)	\
    ( (This)->lpVtbl -> setSelection(This,selectionIndex,startOffset,endOffset) ) 

#define IAccessibleHypertext2_get_nCharacters(This,nCharacters)	\
    ( (This)->lpVtbl -> get_nCharacters(This,nCharacters) ) 

#define IAccessibleHypertext2_scrollSubstringTo(This,startIndex,endIndex,scrollType)	\
    ( (This)->lpVtbl -> scrollSubstringTo(This,startIndex,endIndex,scrollType) ) 

#define IAccessibleHypertext2_scrollSubstringToPoint(This,startIndex,endIndex,coordinateType,x,y)	\
    ( (This)->lpVtbl -> scrollSubstringToPoint(This,startIndex,endIndex,coordinateType,x,y) ) 

#define IAccessibleHypertext2_get_newText(This,newText)	\
    ( (This)->lpVtbl -> get_newText(This,newText) ) 

#define IAccessibleHypertext2_get_oldText(This,oldText)	\
    ( (This)->lpVtbl -> get_oldText(This,oldText) ) 


#define IAccessibleHypertext2_get_nHyperlinks(This,hyperlinkCount)	\
    ( (This)->lpVtbl -> get_nHyperlinks(This,hyperlinkCount) ) 

#define IAccessibleHypertext2_get_hyperlink(This,index,hyperlink)	\
    ( (This)->lpVtbl -> get_hyperlink(This,index,hyperlink) ) 

#define IAccessibleHypertext2_get_hyperlinkIndex(This,charIndex,hyperlinkIndex)	\
    ( (This)->lpVtbl -> get_hyperlinkIndex(This,charIndex,hyperlinkIndex) ) 


#define IAccessibleHypertext2_get_hyperlinks(This,hyperlinks,nHyperlinks)	\
    ( (This)->lpVtbl -> get_hyperlinks(This,hyperlinks,nHyperlinks) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleHypertext2_INTERFACE_DEFINED__ */


#ifndef __IAccessibleTable_INTERFACE_DEFINED__
#define __IAccessibleTable_INTERFACE_DEFINED__

/* interface IAccessibleTable */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleTable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("35AD8070-C20C-4fb4-B094-F4F7275DD469")
    IAccessibleTable : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_accessibleAt( 
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ IUnknown **accessible) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_caption( 
            /* [retval][out] */ IUnknown **accessible) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_childIndex( 
            /* [in] */ long rowIndex,
            /* [in] */ long columnIndex,
            /* [retval][out] */ long *cellIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_columnDescription( 
            /* [in] */ long column,
            /* [retval][out] */ BSTR *description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_columnExtentAt( 
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ long *nColumnsSpanned) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_columnHeader( 
            /* [out] */ IAccessibleTable **accessibleTable,
            /* [retval][out] */ long *startingRowIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_columnIndex( 
            /* [in] */ long cellIndex,
            /* [retval][out] */ long *columnIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nColumns( 
            /* [retval][out] */ long *columnCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nRows( 
            /* [retval][out] */ long *rowCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nSelectedChildren( 
            /* [retval][out] */ long *cellCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nSelectedColumns( 
            /* [retval][out] */ long *columnCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nSelectedRows( 
            /* [retval][out] */ long *rowCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowDescription( 
            /* [in] */ long row,
            /* [retval][out] */ BSTR *description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowExtentAt( 
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ long *nRowsSpanned) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowHeader( 
            /* [out] */ IAccessibleTable **accessibleTable,
            /* [retval][out] */ long *startingColumnIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowIndex( 
            /* [in] */ long cellIndex,
            /* [retval][out] */ long *rowIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selectedChildren( 
            /* [in] */ long maxChildren,
            /* [length_is][length_is][size_is][size_is][out] */ long **children,
            /* [retval][out] */ long *nChildren) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selectedColumns( 
            /* [in] */ long maxColumns,
            /* [length_is][length_is][size_is][size_is][out] */ long **columns,
            /* [retval][out] */ long *nColumns) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selectedRows( 
            /* [in] */ long maxRows,
            /* [length_is][length_is][size_is][size_is][out] */ long **rows,
            /* [retval][out] */ long *nRows) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_summary( 
            /* [retval][out] */ IUnknown **accessible) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isColumnSelected( 
            /* [in] */ long column,
            /* [retval][out] */ boolean *isSelected) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isRowSelected( 
            /* [in] */ long row,
            /* [retval][out] */ boolean *isSelected) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isSelected( 
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ boolean *isSelected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE selectRow( 
            /* [in] */ long row) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE selectColumn( 
            /* [in] */ long column) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE unselectRow( 
            /* [in] */ long row) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE unselectColumn( 
            /* [in] */ long column) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowColumnExtentsAtIndex( 
            /* [in] */ long index,
            /* [out] */ long *row,
            /* [out] */ long *column,
            /* [out] */ long *rowExtents,
            /* [out] */ long *columnExtents,
            /* [retval][out] */ boolean *isSelected) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_modelChange( 
            /* [retval][out] */ IA2TableModelChange *modelChange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleTableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleTable * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleTable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleTable * This);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_accessibleAt)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_accessibleAt )( 
            IAccessibleTable * This,
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ IUnknown **accessible);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_caption)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_caption )( 
            IAccessibleTable * This,
            /* [retval][out] */ IUnknown **accessible);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_childIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_childIndex )( 
            IAccessibleTable * This,
            /* [in] */ long rowIndex,
            /* [in] */ long columnIndex,
            /* [retval][out] */ long *cellIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_columnDescription)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnDescription )( 
            IAccessibleTable * This,
            /* [in] */ long column,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_columnExtentAt)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnExtentAt )( 
            IAccessibleTable * This,
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ long *nColumnsSpanned);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_columnHeader)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnHeader )( 
            IAccessibleTable * This,
            /* [out] */ IAccessibleTable **accessibleTable,
            /* [retval][out] */ long *startingRowIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_columnIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnIndex )( 
            IAccessibleTable * This,
            /* [in] */ long cellIndex,
            /* [retval][out] */ long *columnIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_nColumns)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nColumns )( 
            IAccessibleTable * This,
            /* [retval][out] */ long *columnCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_nRows)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nRows )( 
            IAccessibleTable * This,
            /* [retval][out] */ long *rowCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_nSelectedChildren)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelectedChildren )( 
            IAccessibleTable * This,
            /* [retval][out] */ long *cellCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_nSelectedColumns)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelectedColumns )( 
            IAccessibleTable * This,
            /* [retval][out] */ long *columnCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_nSelectedRows)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelectedRows )( 
            IAccessibleTable * This,
            /* [retval][out] */ long *rowCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_rowDescription)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowDescription )( 
            IAccessibleTable * This,
            /* [in] */ long row,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_rowExtentAt)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowExtentAt )( 
            IAccessibleTable * This,
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ long *nRowsSpanned);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_rowHeader)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowHeader )( 
            IAccessibleTable * This,
            /* [out] */ IAccessibleTable **accessibleTable,
            /* [retval][out] */ long *startingColumnIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_rowIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowIndex )( 
            IAccessibleTable * This,
            /* [in] */ long cellIndex,
            /* [retval][out] */ long *rowIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_selectedChildren)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selectedChildren )( 
            IAccessibleTable * This,
            /* [in] */ long maxChildren,
            /* [length_is][length_is][size_is][size_is][out] */ long **children,
            /* [retval][out] */ long *nChildren);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_selectedColumns)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selectedColumns )( 
            IAccessibleTable * This,
            /* [in] */ long maxColumns,
            /* [length_is][length_is][size_is][size_is][out] */ long **columns,
            /* [retval][out] */ long *nColumns);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_selectedRows)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selectedRows )( 
            IAccessibleTable * This,
            /* [in] */ long maxRows,
            /* [length_is][length_is][size_is][size_is][out] */ long **rows,
            /* [retval][out] */ long *nRows);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_summary)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_summary )( 
            IAccessibleTable * This,
            /* [retval][out] */ IUnknown **accessible);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_isColumnSelected)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isColumnSelected )( 
            IAccessibleTable * This,
            /* [in] */ long column,
            /* [retval][out] */ boolean *isSelected);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_isRowSelected)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRowSelected )( 
            IAccessibleTable * This,
            /* [in] */ long row,
            /* [retval][out] */ boolean *isSelected);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_isSelected)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isSelected )( 
            IAccessibleTable * This,
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ boolean *isSelected);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, selectRow)
        HRESULT ( STDMETHODCALLTYPE *selectRow )( 
            IAccessibleTable * This,
            /* [in] */ long row);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, selectColumn)
        HRESULT ( STDMETHODCALLTYPE *selectColumn )( 
            IAccessibleTable * This,
            /* [in] */ long column);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, unselectRow)
        HRESULT ( STDMETHODCALLTYPE *unselectRow )( 
            IAccessibleTable * This,
            /* [in] */ long row);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, unselectColumn)
        HRESULT ( STDMETHODCALLTYPE *unselectColumn )( 
            IAccessibleTable * This,
            /* [in] */ long column);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_rowColumnExtentsAtIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowColumnExtentsAtIndex )( 
            IAccessibleTable * This,
            /* [in] */ long index,
            /* [out] */ long *row,
            /* [out] */ long *column,
            /* [out] */ long *rowExtents,
            /* [out] */ long *columnExtents,
            /* [retval][out] */ boolean *isSelected);
        
        DECLSPEC_XFGVIRT(IAccessibleTable, get_modelChange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_modelChange )( 
            IAccessibleTable * This,
            /* [retval][out] */ IA2TableModelChange *modelChange);
        
        END_INTERFACE
    } IAccessibleTableVtbl;

    interface IAccessibleTable
    {
        CONST_VTBL struct IAccessibleTableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleTable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleTable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleTable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleTable_get_accessibleAt(This,row,column,accessible)	\
    ( (This)->lpVtbl -> get_accessibleAt(This,row,column,accessible) ) 

#define IAccessibleTable_get_caption(This,accessible)	\
    ( (This)->lpVtbl -> get_caption(This,accessible) ) 

#define IAccessibleTable_get_childIndex(This,rowIndex,columnIndex,cellIndex)	\
    ( (This)->lpVtbl -> get_childIndex(This,rowIndex,columnIndex,cellIndex) ) 

#define IAccessibleTable_get_columnDescription(This,column,description)	\
    ( (This)->lpVtbl -> get_columnDescription(This,column,description) ) 

#define IAccessibleTable_get_columnExtentAt(This,row,column,nColumnsSpanned)	\
    ( (This)->lpVtbl -> get_columnExtentAt(This,row,column,nColumnsSpanned) ) 

#define IAccessibleTable_get_columnHeader(This,accessibleTable,startingRowIndex)	\
    ( (This)->lpVtbl -> get_columnHeader(This,accessibleTable,startingRowIndex) ) 

#define IAccessibleTable_get_columnIndex(This,cellIndex,columnIndex)	\
    ( (This)->lpVtbl -> get_columnIndex(This,cellIndex,columnIndex) ) 

#define IAccessibleTable_get_nColumns(This,columnCount)	\
    ( (This)->lpVtbl -> get_nColumns(This,columnCount) ) 

#define IAccessibleTable_get_nRows(This,rowCount)	\
    ( (This)->lpVtbl -> get_nRows(This,rowCount) ) 

#define IAccessibleTable_get_nSelectedChildren(This,cellCount)	\
    ( (This)->lpVtbl -> get_nSelectedChildren(This,cellCount) ) 

#define IAccessibleTable_get_nSelectedColumns(This,columnCount)	\
    ( (This)->lpVtbl -> get_nSelectedColumns(This,columnCount) ) 

#define IAccessibleTable_get_nSelectedRows(This,rowCount)	\
    ( (This)->lpVtbl -> get_nSelectedRows(This,rowCount) ) 

#define IAccessibleTable_get_rowDescription(This,row,description)	\
    ( (This)->lpVtbl -> get_rowDescription(This,row,description) ) 

#define IAccessibleTable_get_rowExtentAt(This,row,column,nRowsSpanned)	\
    ( (This)->lpVtbl -> get_rowExtentAt(This,row,column,nRowsSpanned) ) 

#define IAccessibleTable_get_rowHeader(This,accessibleTable,startingColumnIndex)	\
    ( (This)->lpVtbl -> get_rowHeader(This,accessibleTable,startingColumnIndex) ) 

#define IAccessibleTable_get_rowIndex(This,cellIndex,rowIndex)	\
    ( (This)->lpVtbl -> get_rowIndex(This,cellIndex,rowIndex) ) 

#define IAccessibleTable_get_selectedChildren(This,maxChildren,children,nChildren)	\
    ( (This)->lpVtbl -> get_selectedChildren(This,maxChildren,children,nChildren) ) 

#define IAccessibleTable_get_selectedColumns(This,maxColumns,columns,nColumns)	\
    ( (This)->lpVtbl -> get_selectedColumns(This,maxColumns,columns,nColumns) ) 

#define IAccessibleTable_get_selectedRows(This,maxRows,rows,nRows)	\
    ( (This)->lpVtbl -> get_selectedRows(This,maxRows,rows,nRows) ) 

#define IAccessibleTable_get_summary(This,accessible)	\
    ( (This)->lpVtbl -> get_summary(This,accessible) ) 

#define IAccessibleTable_get_isColumnSelected(This,column,isSelected)	\
    ( (This)->lpVtbl -> get_isColumnSelected(This,column,isSelected) ) 

#define IAccessibleTable_get_isRowSelected(This,row,isSelected)	\
    ( (This)->lpVtbl -> get_isRowSelected(This,row,isSelected) ) 

#define IAccessibleTable_get_isSelected(This,row,column,isSelected)	\
    ( (This)->lpVtbl -> get_isSelected(This,row,column,isSelected) ) 

#define IAccessibleTable_selectRow(This,row)	\
    ( (This)->lpVtbl -> selectRow(This,row) ) 

#define IAccessibleTable_selectColumn(This,column)	\
    ( (This)->lpVtbl -> selectColumn(This,column) ) 

#define IAccessibleTable_unselectRow(This,row)	\
    ( (This)->lpVtbl -> unselectRow(This,row) ) 

#define IAccessibleTable_unselectColumn(This,column)	\
    ( (This)->lpVtbl -> unselectColumn(This,column) ) 

#define IAccessibleTable_get_rowColumnExtentsAtIndex(This,index,row,column,rowExtents,columnExtents,isSelected)	\
    ( (This)->lpVtbl -> get_rowColumnExtentsAtIndex(This,index,row,column,rowExtents,columnExtents,isSelected) ) 

#define IAccessibleTable_get_modelChange(This,modelChange)	\
    ( (This)->lpVtbl -> get_modelChange(This,modelChange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleTable_INTERFACE_DEFINED__ */


#ifndef __IAccessibleTable2_INTERFACE_DEFINED__
#define __IAccessibleTable2_INTERFACE_DEFINED__

/* interface IAccessibleTable2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleTable2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6167f295-06f0-4cdd-a1fa-02e25153d869")
    IAccessibleTable2 : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_cellAt( 
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ IUnknown **cell) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_caption( 
            /* [retval][out] */ IUnknown **accessible) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_columnDescription( 
            /* [in] */ long column,
            /* [retval][out] */ BSTR *description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nColumns( 
            /* [retval][out] */ long *columnCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nRows( 
            /* [retval][out] */ long *rowCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nSelectedCells( 
            /* [retval][out] */ long *cellCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nSelectedColumns( 
            /* [retval][out] */ long *columnCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_nSelectedRows( 
            /* [retval][out] */ long *rowCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowDescription( 
            /* [in] */ long row,
            /* [retval][out] */ BSTR *description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selectedCells( 
            /* [size_is][size_is][out] */ IUnknown ***cells,
            /* [retval][out] */ long *nSelectedCells) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selectedColumns( 
            /* [size_is][size_is][out] */ long **selectedColumns,
            /* [retval][out] */ long *nColumns) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_selectedRows( 
            /* [size_is][size_is][out] */ long **selectedRows,
            /* [retval][out] */ long *nRows) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_summary( 
            /* [retval][out] */ IUnknown **accessible) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isColumnSelected( 
            /* [in] */ long column,
            /* [retval][out] */ boolean *isSelected) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isRowSelected( 
            /* [in] */ long row,
            /* [retval][out] */ boolean *isSelected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE selectRow( 
            /* [in] */ long row) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE selectColumn( 
            /* [in] */ long column) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE unselectRow( 
            /* [in] */ long row) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE unselectColumn( 
            /* [in] */ long column) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_modelChange( 
            /* [retval][out] */ IA2TableModelChange *modelChange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleTable2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleTable2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleTable2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleTable2 * This);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_cellAt)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_cellAt )( 
            IAccessibleTable2 * This,
            /* [in] */ long row,
            /* [in] */ long column,
            /* [retval][out] */ IUnknown **cell);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_caption)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_caption )( 
            IAccessibleTable2 * This,
            /* [retval][out] */ IUnknown **accessible);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_columnDescription)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnDescription )( 
            IAccessibleTable2 * This,
            /* [in] */ long column,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_nColumns)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nColumns )( 
            IAccessibleTable2 * This,
            /* [retval][out] */ long *columnCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_nRows)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nRows )( 
            IAccessibleTable2 * This,
            /* [retval][out] */ long *rowCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_nSelectedCells)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelectedCells )( 
            IAccessibleTable2 * This,
            /* [retval][out] */ long *cellCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_nSelectedColumns)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelectedColumns )( 
            IAccessibleTable2 * This,
            /* [retval][out] */ long *columnCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_nSelectedRows)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_nSelectedRows )( 
            IAccessibleTable2 * This,
            /* [retval][out] */ long *rowCount);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_rowDescription)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowDescription )( 
            IAccessibleTable2 * This,
            /* [in] */ long row,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_selectedCells)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selectedCells )( 
            IAccessibleTable2 * This,
            /* [size_is][size_is][out] */ IUnknown ***cells,
            /* [retval][out] */ long *nSelectedCells);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_selectedColumns)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selectedColumns )( 
            IAccessibleTable2 * This,
            /* [size_is][size_is][out] */ long **selectedColumns,
            /* [retval][out] */ long *nColumns);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_selectedRows)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_selectedRows )( 
            IAccessibleTable2 * This,
            /* [size_is][size_is][out] */ long **selectedRows,
            /* [retval][out] */ long *nRows);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_summary)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_summary )( 
            IAccessibleTable2 * This,
            /* [retval][out] */ IUnknown **accessible);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_isColumnSelected)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isColumnSelected )( 
            IAccessibleTable2 * This,
            /* [in] */ long column,
            /* [retval][out] */ boolean *isSelected);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_isRowSelected)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isRowSelected )( 
            IAccessibleTable2 * This,
            /* [in] */ long row,
            /* [retval][out] */ boolean *isSelected);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, selectRow)
        HRESULT ( STDMETHODCALLTYPE *selectRow )( 
            IAccessibleTable2 * This,
            /* [in] */ long row);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, selectColumn)
        HRESULT ( STDMETHODCALLTYPE *selectColumn )( 
            IAccessibleTable2 * This,
            /* [in] */ long column);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, unselectRow)
        HRESULT ( STDMETHODCALLTYPE *unselectRow )( 
            IAccessibleTable2 * This,
            /* [in] */ long row);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, unselectColumn)
        HRESULT ( STDMETHODCALLTYPE *unselectColumn )( 
            IAccessibleTable2 * This,
            /* [in] */ long column);
        
        DECLSPEC_XFGVIRT(IAccessibleTable2, get_modelChange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_modelChange )( 
            IAccessibleTable2 * This,
            /* [retval][out] */ IA2TableModelChange *modelChange);
        
        END_INTERFACE
    } IAccessibleTable2Vtbl;

    interface IAccessibleTable2
    {
        CONST_VTBL struct IAccessibleTable2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleTable2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleTable2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleTable2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleTable2_get_cellAt(This,row,column,cell)	\
    ( (This)->lpVtbl -> get_cellAt(This,row,column,cell) ) 

#define IAccessibleTable2_get_caption(This,accessible)	\
    ( (This)->lpVtbl -> get_caption(This,accessible) ) 

#define IAccessibleTable2_get_columnDescription(This,column,description)	\
    ( (This)->lpVtbl -> get_columnDescription(This,column,description) ) 

#define IAccessibleTable2_get_nColumns(This,columnCount)	\
    ( (This)->lpVtbl -> get_nColumns(This,columnCount) ) 

#define IAccessibleTable2_get_nRows(This,rowCount)	\
    ( (This)->lpVtbl -> get_nRows(This,rowCount) ) 

#define IAccessibleTable2_get_nSelectedCells(This,cellCount)	\
    ( (This)->lpVtbl -> get_nSelectedCells(This,cellCount) ) 

#define IAccessibleTable2_get_nSelectedColumns(This,columnCount)	\
    ( (This)->lpVtbl -> get_nSelectedColumns(This,columnCount) ) 

#define IAccessibleTable2_get_nSelectedRows(This,rowCount)	\
    ( (This)->lpVtbl -> get_nSelectedRows(This,rowCount) ) 

#define IAccessibleTable2_get_rowDescription(This,row,description)	\
    ( (This)->lpVtbl -> get_rowDescription(This,row,description) ) 

#define IAccessibleTable2_get_selectedCells(This,cells,nSelectedCells)	\
    ( (This)->lpVtbl -> get_selectedCells(This,cells,nSelectedCells) ) 

#define IAccessibleTable2_get_selectedColumns(This,selectedColumns,nColumns)	\
    ( (This)->lpVtbl -> get_selectedColumns(This,selectedColumns,nColumns) ) 

#define IAccessibleTable2_get_selectedRows(This,selectedRows,nRows)	\
    ( (This)->lpVtbl -> get_selectedRows(This,selectedRows,nRows) ) 

#define IAccessibleTable2_get_summary(This,accessible)	\
    ( (This)->lpVtbl -> get_summary(This,accessible) ) 

#define IAccessibleTable2_get_isColumnSelected(This,column,isSelected)	\
    ( (This)->lpVtbl -> get_isColumnSelected(This,column,isSelected) ) 

#define IAccessibleTable2_get_isRowSelected(This,row,isSelected)	\
    ( (This)->lpVtbl -> get_isRowSelected(This,row,isSelected) ) 

#define IAccessibleTable2_selectRow(This,row)	\
    ( (This)->lpVtbl -> selectRow(This,row) ) 

#define IAccessibleTable2_selectColumn(This,column)	\
    ( (This)->lpVtbl -> selectColumn(This,column) ) 

#define IAccessibleTable2_unselectRow(This,row)	\
    ( (This)->lpVtbl -> unselectRow(This,row) ) 

#define IAccessibleTable2_unselectColumn(This,column)	\
    ( (This)->lpVtbl -> unselectColumn(This,column) ) 

#define IAccessibleTable2_get_modelChange(This,modelChange)	\
    ( (This)->lpVtbl -> get_modelChange(This,modelChange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleTable2_INTERFACE_DEFINED__ */


#ifndef __IAccessibleTableCell_INTERFACE_DEFINED__
#define __IAccessibleTableCell_INTERFACE_DEFINED__

/* interface IAccessibleTableCell */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleTableCell;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("594116B1-C99F-4847-AD06-0A7A86ECE645")
    IAccessibleTableCell : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_columnExtent( 
            /* [retval][out] */ long *nColumnsSpanned) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_columnHeaderCells( 
            /* [size_is][size_is][out] */ IUnknown ***cellAccessibles,
            /* [retval][out] */ long *nColumnHeaderCells) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_columnIndex( 
            /* [retval][out] */ long *columnIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowExtent( 
            /* [retval][out] */ long *nRowsSpanned) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowHeaderCells( 
            /* [size_is][size_is][out] */ IUnknown ***cellAccessibles,
            /* [retval][out] */ long *nRowHeaderCells) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowIndex( 
            /* [retval][out] */ long *rowIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_isSelected( 
            /* [retval][out] */ boolean *isSelected) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_rowColumnExtents( 
            /* [out] */ long *row,
            /* [out] */ long *column,
            /* [out] */ long *rowExtents,
            /* [out] */ long *columnExtents,
            /* [retval][out] */ boolean *isSelected) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_table( 
            /* [retval][out] */ IUnknown **table) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleTableCellVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleTableCell * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleTableCell * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleTableCell * This);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_columnExtent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnExtent )( 
            IAccessibleTableCell * This,
            /* [retval][out] */ long *nColumnsSpanned);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_columnHeaderCells)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnHeaderCells )( 
            IAccessibleTableCell * This,
            /* [size_is][size_is][out] */ IUnknown ***cellAccessibles,
            /* [retval][out] */ long *nColumnHeaderCells);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_columnIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnIndex )( 
            IAccessibleTableCell * This,
            /* [retval][out] */ long *columnIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_rowExtent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowExtent )( 
            IAccessibleTableCell * This,
            /* [retval][out] */ long *nRowsSpanned);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_rowHeaderCells)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowHeaderCells )( 
            IAccessibleTableCell * This,
            /* [size_is][size_is][out] */ IUnknown ***cellAccessibles,
            /* [retval][out] */ long *nRowHeaderCells);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_rowIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowIndex )( 
            IAccessibleTableCell * This,
            /* [retval][out] */ long *rowIndex);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_isSelected)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_isSelected )( 
            IAccessibleTableCell * This,
            /* [retval][out] */ boolean *isSelected);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_rowColumnExtents)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_rowColumnExtents )( 
            IAccessibleTableCell * This,
            /* [out] */ long *row,
            /* [out] */ long *column,
            /* [out] */ long *rowExtents,
            /* [out] */ long *columnExtents,
            /* [retval][out] */ boolean *isSelected);
        
        DECLSPEC_XFGVIRT(IAccessibleTableCell, get_table)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_table )( 
            IAccessibleTableCell * This,
            /* [retval][out] */ IUnknown **table);
        
        END_INTERFACE
    } IAccessibleTableCellVtbl;

    interface IAccessibleTableCell
    {
        CONST_VTBL struct IAccessibleTableCellVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleTableCell_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleTableCell_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleTableCell_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleTableCell_get_columnExtent(This,nColumnsSpanned)	\
    ( (This)->lpVtbl -> get_columnExtent(This,nColumnsSpanned) ) 

#define IAccessibleTableCell_get_columnHeaderCells(This,cellAccessibles,nColumnHeaderCells)	\
    ( (This)->lpVtbl -> get_columnHeaderCells(This,cellAccessibles,nColumnHeaderCells) ) 

#define IAccessibleTableCell_get_columnIndex(This,columnIndex)	\
    ( (This)->lpVtbl -> get_columnIndex(This,columnIndex) ) 

#define IAccessibleTableCell_get_rowExtent(This,nRowsSpanned)	\
    ( (This)->lpVtbl -> get_rowExtent(This,nRowsSpanned) ) 

#define IAccessibleTableCell_get_rowHeaderCells(This,cellAccessibles,nRowHeaderCells)	\
    ( (This)->lpVtbl -> get_rowHeaderCells(This,cellAccessibles,nRowHeaderCells) ) 

#define IAccessibleTableCell_get_rowIndex(This,rowIndex)	\
    ( (This)->lpVtbl -> get_rowIndex(This,rowIndex) ) 

#define IAccessibleTableCell_get_isSelected(This,isSelected)	\
    ( (This)->lpVtbl -> get_isSelected(This,isSelected) ) 

#define IAccessibleTableCell_get_rowColumnExtents(This,row,column,rowExtents,columnExtents,isSelected)	\
    ( (This)->lpVtbl -> get_rowColumnExtents(This,row,column,rowExtents,columnExtents,isSelected) ) 

#define IAccessibleTableCell_get_table(This,table)	\
    ( (This)->lpVtbl -> get_table(This,table) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleTableCell_INTERFACE_DEFINED__ */


#ifndef __IAccessibleImage_INTERFACE_DEFINED__
#define __IAccessibleImage_INTERFACE_DEFINED__

/* interface IAccessibleImage */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleImage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FE5ABB3D-615E-4f7b-909F-5F0EDA9E8DDE")
    IAccessibleImage : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_description( 
            /* [retval][out] */ BSTR *description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_imagePosition( 
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [out] */ long *x,
            /* [retval][out] */ long *y) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_imageSize( 
            /* [out] */ long *height,
            /* [retval][out] */ long *width) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleImageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleImage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleImage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleImage * This);
        
        DECLSPEC_XFGVIRT(IAccessibleImage, get_description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_description )( 
            IAccessibleImage * This,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(IAccessibleImage, get_imagePosition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_imagePosition )( 
            IAccessibleImage * This,
            /* [in] */ enum IA2CoordinateType coordinateType,
            /* [out] */ long *x,
            /* [retval][out] */ long *y);
        
        DECLSPEC_XFGVIRT(IAccessibleImage, get_imageSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_imageSize )( 
            IAccessibleImage * This,
            /* [out] */ long *height,
            /* [retval][out] */ long *width);
        
        END_INTERFACE
    } IAccessibleImageVtbl;

    interface IAccessibleImage
    {
        CONST_VTBL struct IAccessibleImageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleImage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleImage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleImage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleImage_get_description(This,description)	\
    ( (This)->lpVtbl -> get_description(This,description) ) 

#define IAccessibleImage_get_imagePosition(This,coordinateType,x,y)	\
    ( (This)->lpVtbl -> get_imagePosition(This,coordinateType,x,y) ) 

#define IAccessibleImage_get_imageSize(This,height,width)	\
    ( (This)->lpVtbl -> get_imageSize(This,height,width) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleImage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ia2_api_all_0000_0019 */
/* [local] */ 


enum IA2EventID
    {
        IA2_EVENT_ACTION_CHANGED	= 0x101,
        IA2_EVENT_ACTIVE_DECENDENT_CHANGED	= ( IA2_EVENT_ACTION_CHANGED + 1 ) ,
        IA2_EVENT_ACTIVE_DESCENDANT_CHANGED	= IA2_EVENT_ACTIVE_DECENDENT_CHANGED,
        IA2_EVENT_DOCUMENT_ATTRIBUTE_CHANGED	= ( IA2_EVENT_ACTIVE_DESCENDANT_CHANGED + 1 ) ,
        IA2_EVENT_DOCUMENT_CONTENT_CHANGED	= ( IA2_EVENT_DOCUMENT_ATTRIBUTE_CHANGED + 1 ) ,
        IA2_EVENT_DOCUMENT_LOAD_COMPLETE	= ( IA2_EVENT_DOCUMENT_CONTENT_CHANGED + 1 ) ,
        IA2_EVENT_DOCUMENT_LOAD_STOPPED	= ( IA2_EVENT_DOCUMENT_LOAD_COMPLETE + 1 ) ,
        IA2_EVENT_DOCUMENT_RELOAD	= ( IA2_EVENT_DOCUMENT_LOAD_STOPPED + 1 ) ,
        IA2_EVENT_HYPERLINK_END_INDEX_CHANGED	= ( IA2_EVENT_DOCUMENT_RELOAD + 1 ) ,
        IA2_EVENT_HYPERLINK_NUMBER_OF_ANCHORS_CHANGED	= ( IA2_EVENT_HYPERLINK_END_INDEX_CHANGED + 1 ) ,
        IA2_EVENT_HYPERLINK_SELECTED_LINK_CHANGED	= ( IA2_EVENT_HYPERLINK_NUMBER_OF_ANCHORS_CHANGED + 1 ) ,
        IA2_EVENT_HYPERTEXT_LINK_ACTIVATED	= ( IA2_EVENT_HYPERLINK_SELECTED_LINK_CHANGED + 1 ) ,
        IA2_EVENT_HYPERTEXT_LINK_SELECTED	= ( IA2_EVENT_HYPERTEXT_LINK_ACTIVATED + 1 ) ,
        IA2_EVENT_HYPERLINK_START_INDEX_CHANGED	= ( IA2_EVENT_HYPERTEXT_LINK_SELECTED + 1 ) ,
        IA2_EVENT_HYPERTEXT_CHANGED	= ( IA2_EVENT_HYPERLINK_START_INDEX_CHANGED + 1 ) ,
        IA2_EVENT_HYPERTEXT_NLINKS_CHANGED	= ( IA2_EVENT_HYPERTEXT_CHANGED + 1 ) ,
        IA2_EVENT_OBJECT_ATTRIBUTE_CHANGED	= ( IA2_EVENT_HYPERTEXT_NLINKS_CHANGED + 1 ) ,
        IA2_EVENT_PAGE_CHANGED	= ( IA2_EVENT_OBJECT_ATTRIBUTE_CHANGED + 1 ) ,
        IA2_EVENT_SECTION_CHANGED	= ( IA2_EVENT_PAGE_CHANGED + 1 ) ,
        IA2_EVENT_TABLE_CAPTION_CHANGED	= ( IA2_EVENT_SECTION_CHANGED + 1 ) ,
        IA2_EVENT_TABLE_COLUMN_DESCRIPTION_CHANGED	= ( IA2_EVENT_TABLE_CAPTION_CHANGED + 1 ) ,
        IA2_EVENT_TABLE_COLUMN_HEADER_CHANGED	= ( IA2_EVENT_TABLE_COLUMN_DESCRIPTION_CHANGED + 1 ) ,
        IA2_EVENT_TABLE_MODEL_CHANGED	= ( IA2_EVENT_TABLE_COLUMN_HEADER_CHANGED + 1 ) ,
        IA2_EVENT_TABLE_ROW_DESCRIPTION_CHANGED	= ( IA2_EVENT_TABLE_MODEL_CHANGED + 1 ) ,
        IA2_EVENT_TABLE_ROW_HEADER_CHANGED	= ( IA2_EVENT_TABLE_ROW_DESCRIPTION_CHANGED + 1 ) ,
        IA2_EVENT_TABLE_SUMMARY_CHANGED	= ( IA2_EVENT_TABLE_ROW_HEADER_CHANGED + 1 ) ,
        IA2_EVENT_TEXT_ATTRIBUTE_CHANGED	= ( IA2_EVENT_TABLE_SUMMARY_CHANGED + 1 ) ,
        IA2_EVENT_TEXT_CARET_MOVED	= ( IA2_EVENT_TEXT_ATTRIBUTE_CHANGED + 1 ) ,
        IA2_EVENT_TEXT_CHANGED	= ( IA2_EVENT_TEXT_CARET_MOVED + 1 ) ,
        IA2_EVENT_TEXT_COLUMN_CHANGED	= ( IA2_EVENT_TEXT_CHANGED + 1 ) ,
        IA2_EVENT_TEXT_INSERTED	= ( IA2_EVENT_TEXT_COLUMN_CHANGED + 1 ) ,
        IA2_EVENT_TEXT_REMOVED	= ( IA2_EVENT_TEXT_INSERTED + 1 ) ,
        IA2_EVENT_TEXT_UPDATED	= ( IA2_EVENT_TEXT_REMOVED + 1 ) ,
        IA2_EVENT_TEXT_SELECTION_CHANGED	= ( IA2_EVENT_TEXT_UPDATED + 1 ) ,
        IA2_EVENT_VISIBLE_DATA_CHANGED	= ( IA2_EVENT_TEXT_SELECTION_CHANGED + 1 ) ,
        IA2_EVENT_ROLE_CHANGED	= ( IA2_EVENT_VISIBLE_DATA_CHANGED + 1 ) 
    } ;


extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0019_v0_0_s_ifspec;

#ifndef __IAccessibleApplication_INTERFACE_DEFINED__
#define __IAccessibleApplication_INTERFACE_DEFINED__

/* interface IAccessibleApplication */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleApplication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D49DED83-5B25-43F4-9B95-93B44595979E")
    IAccessibleApplication : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appName( 
            /* [retval][out] */ BSTR *name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_appVersion( 
            /* [retval][out] */ BSTR *version) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_toolkitName( 
            /* [retval][out] */ BSTR *name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_toolkitVersion( 
            /* [retval][out] */ BSTR *version) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleApplicationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleApplication * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleApplication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleApplication * This);
        
        DECLSPEC_XFGVIRT(IAccessibleApplication, get_appName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appName )( 
            IAccessibleApplication * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IAccessibleApplication, get_appVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_appVersion )( 
            IAccessibleApplication * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(IAccessibleApplication, get_toolkitName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_toolkitName )( 
            IAccessibleApplication * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IAccessibleApplication, get_toolkitVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_toolkitVersion )( 
            IAccessibleApplication * This,
            /* [retval][out] */ BSTR *version);
        
        END_INTERFACE
    } IAccessibleApplicationVtbl;

    interface IAccessibleApplication
    {
        CONST_VTBL struct IAccessibleApplicationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleApplication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleApplication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleApplication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleApplication_get_appName(This,name)	\
    ( (This)->lpVtbl -> get_appName(This,name) ) 

#define IAccessibleApplication_get_appVersion(This,version)	\
    ( (This)->lpVtbl -> get_appVersion(This,version) ) 

#define IAccessibleApplication_get_toolkitName(This,name)	\
    ( (This)->lpVtbl -> get_toolkitName(This,name) ) 

#define IAccessibleApplication_get_toolkitVersion(This,version)	\
    ( (This)->lpVtbl -> get_toolkitVersion(This,version) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleApplication_INTERFACE_DEFINED__ */


#ifndef __IAccessibleDocument_INTERFACE_DEFINED__
#define __IAccessibleDocument_INTERFACE_DEFINED__

/* interface IAccessibleDocument */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C48C7FCF-4AB5-4056-AFA6-902D6E1D1149")
    IAccessibleDocument : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_anchorTarget( 
            /* [retval][out] */ IUnknown **accessible) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleDocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccessibleDocument * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccessibleDocument * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccessibleDocument * This);
        
        DECLSPEC_XFGVIRT(IAccessibleDocument, get_anchorTarget)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_anchorTarget )( 
            IAccessibleDocument * This,
            /* [retval][out] */ IUnknown **accessible);
        
        END_INTERFACE
    } IAccessibleDocumentVtbl;

    interface IAccessibleDocument
    {
        CONST_VTBL struct IAccessibleDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleDocument_get_anchorTarget(This,accessible)	\
    ( (This)->lpVtbl -> get_anchorTarget(This,accessible) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleDocument_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ia2_api_all_0000_0021 */
/* [local] */ 


// Type Library Definitions



extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ia2_api_all_0000_0021_v0_0_s_ifspec;


#ifndef __IAccessible2Lib_LIBRARY_DEFINED__
#define __IAccessible2Lib_LIBRARY_DEFINED__

/* library IAccessible2Lib */
/* [hidden][version][helpstring][uuid] */ 






























EXTERN_C const IID LIBID_IAccessible2Lib;
#endif /* __IAccessible2Lib_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long *, unsigned long            , BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  unsigned long *, unsigned char *, BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(unsigned long *, unsigned char *, BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long *, BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     unsigned long *, unsigned long            , HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  unsigned long *, unsigned char *, HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(unsigned long *, unsigned char *, HWND * ); 
void                      __RPC_USER  HWND_UserFree(     unsigned long *, HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     unsigned long *, unsigned long            , VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  unsigned long *, unsigned char *, VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(unsigned long *, unsigned char *, VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     unsigned long *, VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


