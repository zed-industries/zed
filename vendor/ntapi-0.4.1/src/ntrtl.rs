use core::ptr::null_mut;
use crate::ntapi_base::{CLIENT_ID, PCLIENT_ID};
use crate::ntexapi::{RTL_PROCESS_BACKTRACES, RTL_PROCESS_LOCKS};
use crate::ntioapi::FILE_INFORMATION_CLASS;
use crate::ntldr::{RTL_PROCESS_MODULES, RTL_PROCESS_MODULE_INFORMATION_EX};
use crate::ntmmapi::SECTION_IMAGE_INFORMATION;
use crate::ntnls::{PCPTABLEINFO, PNLSTABLEINFO};
use crate::ntpebteb::{PPEB, PTEB_ACTIVE_FRAME};
use crate::ntpsapi::{PINITIAL_TEB, PPS_APC_ROUTINE, PS_PROTECTION};
use crate::ntapi_base::{PRTL_ATOM, RTL_ATOM};
use crate::string::UTF16Const;
use winapi::ctypes::c_void;
use winapi::shared::basetsd::{PULONG64, ULONG32, ULONG64, PSIZE_T, PULONG_PTR, SIZE_T, ULONG_PTR};
use winapi::shared::guiddef::GUID;
use winapi::shared::in6addr::in6_addr;
use winapi::shared::inaddr::in_addr;
use winapi::shared::minwindef::{BOOL, DWORD, PBOOL};
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use winapi::shared::ntdef::{LARGE_INTEGER, RTL_BALANCED_NODE};
use winapi::shared::ntdef::{
    BOOLEAN, CCHAR, CHAR, CLONG, CSHORT, HANDLE, LCID, LIST_ENTRY, LOGICAL, LONG, LUID, NTSTATUS,
    PANSI_STRING, PBOOLEAN, PCANSI_STRING, PCCH, PCH, PCHAR, PCOEM_STRING, PCSZ, PCUNICODE_STRING,
    PCWCH, PCWSTR, PHANDLE, PLARGE_INTEGER, PLCID, PLIST_ENTRY, PLONG, PLUID, PNT_PRODUCT_TYPE,
    POEM_STRING, PPROCESSOR_NUMBER, PRTL_BALANCED_NODE, PSINGLE_LIST_ENTRY, PSTR, PSTRING, PUCHAR,
    PULONG, PULONGLONG, PUNICODE_STRING, PUSHORT, PVOID, PWCH, PWCHAR, PWSTR, SINGLE_LIST_ENTRY,
    STRING, UCHAR, ULONG, ULONGLONG, UNICODE_STRING, USHORT, VOID, WCHAR,
};
use winapi::um::minwinbase::PTHREAD_START_ROUTINE;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use winapi::um::winnt::{PGET_RUNTIME_FUNCTION_CALLBACK, PRUNTIME_FUNCTION, PWOW64_CONTEXT};
use winapi::um::winnt::{
    ACCESS_MASK, ACL_INFORMATION_CLASS, APC_CALLBACK_FUNCTION, HEAP_INFORMATION_CLASS,
    HEAP_REALLOC_IN_PLACE_ONLY, HEAP_ZERO_MEMORY, OS_DEPLOYEMENT_STATE_VALUES, PACCESS_MASK, PACL,
    PCONTEXT, PEXCEPTION_POINTERS, PEXCEPTION_RECORD, PFLS_CALLBACK_FUNCTION, PGENERIC_MAPPING,
    PIMAGE_NT_HEADERS, PIMAGE_SECTION_HEADER, PLUID_AND_ATTRIBUTES, PMESSAGE_RESOURCE_ENTRY,
    PPERFORMANCE_DATA, PRTL_BARRIER, PRTL_CONDITION_VARIABLE, PRTL_CRITICAL_SECTION,
    PRTL_OSVERSIONINFOEXW, PRTL_OSVERSIONINFOW, PRTL_RESOURCE_DEBUG, PRTL_SRWLOCK,
    PSECURITY_DESCRIPTOR, PSECURITY_DESCRIPTOR_CONTROL, PSID, PSID_AND_ATTRIBUTES,
    PSID_AND_ATTRIBUTES_HASH, PSID_IDENTIFIER_AUTHORITY, PVECTORED_EXCEPTION_HANDLER,
    PXSAVE_AREA_HEADER, RTL_CRITICAL_SECTION, RTL_SRWLOCK, SECURITY_DESCRIPTOR_CONTROL,
    SECURITY_IMPERSONATION_LEVEL, SECURITY_INFORMATION, WAITORTIMERCALLBACKFUNC,
    WORKERCALLBACKFUNC,
};
use winapi::vc::vadefs::va_list;
#[inline]
pub fn InitializeListHead(ListHead: &mut LIST_ENTRY) {
    ListHead.Flink = ListHead;
    ListHead.Blink = ListHead;
}
#[inline]
pub fn IsListEmpty(ListHead: &LIST_ENTRY) -> bool {
    ListHead.Flink as *const _ == ListHead as *const _
}
#[inline]
pub unsafe fn RemoveEntryList(Entry: &mut LIST_ENTRY) -> bool {
    let (Blink, Flink) = (Entry.Blink, Entry.Flink);
    (*Blink).Flink = Flink;
    (*Flink).Blink = Blink;
    Flink == Blink
}
#[inline]
pub unsafe fn RemoveHeadList(ListHead: &mut LIST_ENTRY) -> PLIST_ENTRY {
    let Entry = ListHead.Flink;
    let Flink = (*Entry).Flink;
    ListHead.Flink = Flink;
    (*Flink).Blink = ListHead;
    Entry
}
#[inline]
pub unsafe fn RemoveTailList(ListHead: &mut LIST_ENTRY) -> PLIST_ENTRY {
    let Entry = ListHead.Blink;
    let Blink = (*Entry).Blink;
    ListHead.Blink = Blink;
    (*Blink).Flink = ListHead;
    Entry
}
#[inline]
pub unsafe fn InsertTailList(ListHead: &mut LIST_ENTRY, Entry: &mut LIST_ENTRY) {
    let Blink = ListHead.Blink;
    Entry.Flink = ListHead;
    Entry.Blink = Blink;
    (*Blink).Flink = Entry;
    ListHead.Blink = Entry;
}
#[inline]
pub unsafe fn InsertHeadList(ListHead: &mut LIST_ENTRY, Entry: &mut LIST_ENTRY) {
    let Flink = ListHead.Flink;
    Entry.Flink = Flink;
    Entry.Blink = ListHead;
    (*Flink).Blink = Entry;
    ListHead.Flink = Entry;
}
#[inline]
pub unsafe fn AppendTailList(ListHead: &mut LIST_ENTRY, ListToAppend: &mut LIST_ENTRY) {
    let ListEnd = ListHead.Blink;
    (*ListHead.Blink).Flink = ListToAppend;
    ListHead.Blink = ListToAppend.Blink;
    (*ListToAppend.Blink).Flink = ListHead;
    ListToAppend.Blink = ListEnd;
}
#[inline]
pub unsafe fn PopEntryList(ListHead: &mut SINGLE_LIST_ENTRY) -> PSINGLE_LIST_ENTRY {
    let FirstEntry = ListHead.Next;
    if !FirstEntry.is_null() {
        ListHead.Next = (*FirstEntry).Next;
    }
    FirstEntry
}
#[inline]
pub fn PushEntryList(ListHead: &mut SINGLE_LIST_ENTRY, Entry: &mut SINGLE_LIST_ENTRY) {
    Entry.Next = ListHead.Next;
    ListHead.Next = Entry;
}
ENUM!{enum TABLE_SEARCH_RESULT {
    TableEmptyTree = 0,
    TableFoundNode = 1,
    TableInsertAsLeft = 2,
    TableInsertAsRight = 3,
}}
ENUM!{enum RTL_GENERIC_COMPARE_RESULTS {
    GenericLessThan = 0,
    GenericGreaterThan = 1,
    GenericEqual = 2,
}}
FN!{stdcall PRTL_AVL_COMPARE_ROUTINE(
    Table: *mut RTL_AVL_TABLE,
    FirstStruct: PVOID,
    SecondStruct: PVOID,
) -> RTL_GENERIC_COMPARE_RESULTS}
FN!{stdcall PRTL_AVL_ALLOCATE_ROUTINE(
    Table: *mut RTL_AVL_TABLE,
    ByteSize: CLONG,
) -> PVOID}
FN!{stdcall PRTL_AVL_FREE_ROUTINE(
    Table: *mut RTL_AVL_TABLE,
    Buffer: PVOID,
) -> ()}
FN!{stdcall PRTL_AVL_MATCH_FUNCTION(
    Table: *mut RTL_AVL_TABLE,
    UserData: PVOID,
    MatchData: PVOID,
) -> NTSTATUS}
STRUCT!{struct RTL_BALANCED_LINKS {
    Parent: *mut RTL_BALANCED_LINKS,
    LeftChild: *mut RTL_BALANCED_LINKS,
    RightChild: *mut RTL_BALANCED_LINKS,
    Balance: CHAR,
    Reserved: [UCHAR; 3],
}}
pub type PRTL_BALANCED_LINKS = *mut RTL_BALANCED_LINKS;
STRUCT!{struct RTL_AVL_TABLE {
    BalancedRoot: RTL_BALANCED_LINKS,
    OrderedPointer: PVOID,
    WhichOrderedElement: ULONG,
    NumberGenericTableElements: ULONG,
    DepthOfTree: ULONG,
    RestartKey: PRTL_BALANCED_LINKS,
    DeleteCount: ULONG,
    CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
    AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
    FreeRoutine: PRTL_AVL_FREE_ROUTINE,
    TableContext: PVOID,
}}
pub type PRTL_AVL_TABLE = *mut RTL_AVL_TABLE;
EXTERN!{extern "system" {
    fn RtlInitializeGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
        AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
        FreeRoutine: PRTL_AVL_FREE_ROUTINE,
        TableContext: PVOID,
    );
    fn RtlInsertElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: PVOID,
        BufferSize: CLONG,
        NewElement: PBOOLEAN,
    ) -> PVOID;
    fn RtlInsertElementGenericTableFullAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: PVOID,
        BufferSize: CLONG,
        NewElement: PBOOLEAN,
        NodeOrParent: PVOID,
        SearchResult: TABLE_SEARCH_RESULT,
    ) -> PVOID;
    fn RtlDeleteElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: PVOID,
    ) -> BOOLEAN;
    fn RtlLookupElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: PVOID,
    ) -> PVOID;
    fn RtlLookupElementGenericTableFullAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: PVOID,
        NodeOrParent: *mut PVOID,
        SearchResult: *mut TABLE_SEARCH_RESULT,
    ) -> PVOID;
    fn RtlEnumerateGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Restart: BOOLEAN,
    ) -> PVOID;
    fn RtlEnumerateGenericTableWithoutSplayingAvl(
        Table: PRTL_AVL_TABLE,
        RestartKey: *mut PVOID,
    ) -> PVOID;
    fn RtlLookupFirstMatchingElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: PVOID,
        RestartKey: *mut PVOID,
    ) -> PVOID;
    fn RtlEnumerateGenericTableLikeADirectory(
        Table: PRTL_AVL_TABLE,
        MatchFunction: PRTL_AVL_MATCH_FUNCTION,
        MatchData: PVOID,
        NextFlag: ULONG,
        RestartKey: *mut PVOID,
        DeleteCount: PULONG,
        Buffer: PVOID,
    ) -> PVOID;
    fn RtlGetElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        I: ULONG,
    ) -> PVOID;
    fn RtlNumberGenericTableElementsAvl(
        Table: PRTL_AVL_TABLE,
    ) -> ULONG;
    fn RtlIsGenericTableEmptyAvl(
        Table: PRTL_AVL_TABLE,
    ) -> BOOLEAN;
}}
STRUCT!{struct RTL_SPLAY_LINKS {
    Parent: *mut RTL_SPLAY_LINKS,
    LeftChild: *mut RTL_SPLAY_LINKS,
    RightChild: *mut RTL_SPLAY_LINKS,
}}
pub type PRTL_SPLAY_LINKS = *mut RTL_SPLAY_LINKS;
#[inline]
pub fn RtlInitializeSplayLinks(Links: &mut RTL_SPLAY_LINKS) {
    Links.Parent = Links;
    Links.LeftChild = null_mut();
    Links.RightChild = null_mut();
}
#[inline]
pub const fn RtlParent(Links: &RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS {
    Links.Parent
}
#[inline]
pub const fn RtlLeftChild(Links: &RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS {
    Links.LeftChild
}
#[inline]
pub const fn RtlRightChild(Links: &RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS {
    Links.RightChild
}
#[inline]
pub unsafe fn RtlIsRoot(Links: *const RTL_SPLAY_LINKS) -> bool {
    (*Links).Parent as *const _ == Links
}
#[inline]
pub unsafe fn RtlIsLeftChild(Links: *const RTL_SPLAY_LINKS) -> bool {
    RtlLeftChild(&*RtlParent(&*Links)) as *const _ == Links
}
#[inline]
pub unsafe fn RtlIsRightChild(Links: *const RTL_SPLAY_LINKS) -> bool {
    RtlRightChild(&*RtlParent(&*Links)) as *const _ == Links
}
#[inline]
pub fn RtlInsertAsLeftChild(
    ParentLinks: &mut RTL_SPLAY_LINKS,
    ChildLinks: &mut RTL_SPLAY_LINKS,
) {
    ParentLinks.LeftChild = ChildLinks;
    ChildLinks.Parent = ParentLinks;
}
#[inline]
pub fn RtlInsertAsRightChild(
    ParentLinks: &mut RTL_SPLAY_LINKS,
    ChildLinks: &mut RTL_SPLAY_LINKS,
) {
    ParentLinks.RightChild = ChildLinks;
    ChildLinks.Parent = ParentLinks;
}
EXTERN!{extern "system" {
    fn RtlSplay(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlDelete(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlDeleteNoSplay(
        Links: PRTL_SPLAY_LINKS,
        Root: *mut PRTL_SPLAY_LINKS,
    );
    fn RtlSubtreeSuccessor(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlSubtreePredecessor(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlRealSuccessor(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlRealPredecessor(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
}}
FN!{stdcall PRTL_GENERIC_COMPARE_ROUTINE(
    Table: *mut RTL_GENERIC_TABLE,
    FirstStruct: PVOID,
    SecondStruct: PVOID,
) -> RTL_GENERIC_COMPARE_RESULTS}
FN!{stdcall PRTL_GENERIC_ALLOCATE_ROUTINE(
    Table: *mut RTL_GENERIC_TABLE,
    ByteSize: CLONG,
) -> PVOID}
FN!{stdcall PRTL_GENERIC_FREE_ROUTINE(
    Table: *mut RTL_GENERIC_TABLE,
    Buffer: PVOID,
) -> ()}
STRUCT!{struct RTL_GENERIC_TABLE {
    TableRoot: PRTL_SPLAY_LINKS,
    InsertOrderList: LIST_ENTRY,
    OrderedPointer: PLIST_ENTRY,
    WhichOrderedElement: ULONG,
    NumberGenericTableElements: ULONG,
    CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
    AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
    FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
    TableContext: PVOID,
}}
pub type PRTL_GENERIC_TABLE = *mut RTL_GENERIC_TABLE;
EXTERN!{extern "system" {
    fn RtlInitializeGenericTable(
        Table: PRTL_GENERIC_TABLE,
        CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
        AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
        FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
        TableContext: PVOID,
    );
    fn RtlInsertElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Buffer: PVOID,
        BufferSize: CLONG,
        NewElement: PBOOLEAN,
    ) -> PVOID;
    fn RtlInsertElementGenericTableFull(
        Table: PRTL_GENERIC_TABLE,
        Buffer: PVOID,
        BufferSize: CLONG,
        NewElement: PBOOLEAN,
        NodeOrParent: PVOID,
        SearchResult: TABLE_SEARCH_RESULT,
    ) -> PVOID;
    fn RtlDeleteElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Buffer: PVOID,
    ) -> BOOLEAN;
    fn RtlLookupElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Buffer: PVOID,
    ) -> PVOID;
    fn RtlLookupElementGenericTableFull(
        Table: PRTL_GENERIC_TABLE,
        Buffer: PVOID,
        NodeOrParent: *mut PVOID,
        SearchResult: *mut TABLE_SEARCH_RESULT,
    ) -> PVOID;
    fn RtlEnumerateGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Restart: BOOLEAN,
    ) -> PVOID;
    fn RtlEnumerateGenericTableWithoutSplaying(
        Table: PRTL_GENERIC_TABLE,
        RestartKey: *mut PVOID,
    ) -> PVOID;
    fn RtlGetElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        I: ULONG,
    ) -> PVOID;
    fn RtlNumberGenericTableElements(
        Table: PRTL_GENERIC_TABLE,
    ) -> ULONG;
    fn RtlIsGenericTableEmpty(
        Table: PRTL_GENERIC_TABLE,
    ) -> BOOLEAN;
}}
STRUCT!{struct RTL_RB_TREE {
    Root: PRTL_BALANCED_NODE,
    Min: PRTL_BALANCED_NODE,
}}
pub type PRTL_RB_TREE = *mut RTL_RB_TREE;
EXTERN!{extern "system" {
    fn RtlRbInsertNodeEx(
        Tree: PRTL_RB_TREE,
        Parent: PRTL_BALANCED_NODE,
        Right: BOOLEAN,
        Node: PRTL_BALANCED_NODE,
    );
    fn RtlRbRemoveNode(
        Tree: PRTL_RB_TREE,
        Node: PRTL_BALANCED_NODE,
    );
}}
pub const RTL_HASH_ALLOCATED_HEADER: u32 = 0x00000001;
pub const RTL_HASH_RESERVED_SIGNATURE: u32 = 0;
STRUCT!{struct RTL_DYNAMIC_HASH_TABLE_ENTRY {
    Linkage: LIST_ENTRY,
    Signature: ULONG_PTR,
}}
pub type PRTL_DYNAMIC_HASH_TABLE_ENTRY = *mut RTL_DYNAMIC_HASH_TABLE_ENTRY;
#[inline]
pub const fn HASH_ENTRY_KEY(x: &RTL_DYNAMIC_HASH_TABLE_ENTRY) -> ULONG_PTR {
    x.Signature
}
STRUCT!{struct RTL_DYNAMIC_HASH_TABLE_CONTEXT {
    ChainHead: PLIST_ENTRY,
    PrevLinkage: PLIST_ENTRY,
    Signature: ULONG_PTR,
}}
pub type PRTL_DYNAMIC_HASH_TABLE_CONTEXT = *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT;
STRUCT!{struct RTL_DYNAMIC_HASH_TABLE_ENUMERATOR {
    HashEntry: RTL_DYNAMIC_HASH_TABLE_ENTRY,
    ChainHead: PLIST_ENTRY,
    BucketIndex: ULONG,
}}
pub type PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR = *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR;
STRUCT!{struct RTL_DYNAMIC_HASH_TABLE {
    Flags: ULONG,
    Shift: ULONG,
    TableSize: ULONG,
    Pivot: ULONG,
    DivisorMask: ULONG,
    NumEntries: ULONG,
    NonEmptyBuckets: ULONG,
    NumEnumerators: ULONG,
    Directory: PVOID,
}}
pub type PRTL_DYNAMIC_HASH_TABLE = *mut RTL_DYNAMIC_HASH_TABLE;
#[inline]
pub fn RtlInitHashTableContext(Context: &mut RTL_DYNAMIC_HASH_TABLE_CONTEXT) {
    Context.ChainHead = null_mut();
    Context.PrevLinkage = null_mut();
}
#[inline]
pub fn RtlInitHashTableContextFromEnumerator(
    Context: &mut RTL_DYNAMIC_HASH_TABLE_CONTEXT,
    Enumerator: &RTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
) {
    Context.ChainHead = Enumerator.ChainHead;
    Context.PrevLinkage = Enumerator.HashEntry.Linkage.Blink;
}
// RtlReleaseHashTableContext
#[inline]
pub const fn RtlTotalBucketsHashTable(HashTable: &RTL_DYNAMIC_HASH_TABLE) -> ULONG {
    HashTable.TableSize
}
#[inline]
pub const fn RtlNonEmptyBucketsHashTable(HashTable: &RTL_DYNAMIC_HASH_TABLE) -> ULONG {
    HashTable.NonEmptyBuckets
}
#[inline]
pub const fn RtlEmptyBucketsHashTable(HashTable: &RTL_DYNAMIC_HASH_TABLE) -> ULONG {
    HashTable.TableSize - HashTable.NonEmptyBuckets
}
#[inline]
pub const fn RtlTotalEntriesHashTable(HashTable: &RTL_DYNAMIC_HASH_TABLE) -> ULONG {
    HashTable.NumEntries
}
#[inline]
pub const fn RtlActiveEnumeratorsHashTable(HashTable: &RTL_DYNAMIC_HASH_TABLE) -> ULONG {
    HashTable.NumEnumerators
}
EXTERN!{extern "system" {
    fn RtlCreateHashTable(
        HashTable: *mut PRTL_DYNAMIC_HASH_TABLE,
        Shift: ULONG,
        Flags: ULONG,
    ) -> BOOLEAN;
    fn RtlDeleteHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
    );
    fn RtlInsertEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Entry: PRTL_DYNAMIC_HASH_TABLE_ENTRY,
        Signature: ULONG_PTR,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> BOOLEAN;
    fn RtlRemoveEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Entry: PRTL_DYNAMIC_HASH_TABLE_ENTRY,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> BOOLEAN;
    fn RtlLookupEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Signature: ULONG_PTR,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlGetNextEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlInitEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> BOOLEAN;
    fn RtlEnumerateEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlEndEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    );
    fn RtlInitWeakEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> BOOLEAN;
    fn RtlWeaklyEnumerateEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlEndWeakEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    );
    fn RtlExpandHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
    ) -> BOOLEAN;
    fn RtlContractHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
    ) -> BOOLEAN;
    fn RtlInitStrongEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> BOOLEAN;
    fn RtlStronglyEnumerateEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlEndStrongEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    );
    fn RtlInitializeCriticalSection(
        CriticalSection: PRTL_CRITICAL_SECTION,
    ) -> NTSTATUS;
    fn RtlInitializeCriticalSectionAndSpinCount(
        CriticalSection: PRTL_CRITICAL_SECTION,
        SpinCount: ULONG,
    ) -> NTSTATUS;
    fn RtlDeleteCriticalSection(
        CriticalSection: PRTL_CRITICAL_SECTION,
    ) -> NTSTATUS;
    fn RtlEnterCriticalSection(
        CriticalSection: PRTL_CRITICAL_SECTION,
    ) -> NTSTATUS;
    fn RtlLeaveCriticalSection(
        CriticalSection: PRTL_CRITICAL_SECTION,
    ) -> NTSTATUS;
    fn RtlTryEnterCriticalSection(
        CriticalSection: PRTL_CRITICAL_SECTION,
    ) -> LOGICAL;
    fn RtlIsCriticalSectionLocked(
        CriticalSection: PRTL_CRITICAL_SECTION,
    ) -> LOGICAL;
    fn RtlIsCriticalSectionLockedByThread(
        CriticalSection: PRTL_CRITICAL_SECTION,
    ) -> LOGICAL;
    fn RtlGetCriticalSectionRecursionCount(
        CriticalSection: PRTL_CRITICAL_SECTION,
    ) -> ULONG;
    fn RtlSetCriticalSectionSpinCount(
        CriticalSection: PRTL_CRITICAL_SECTION,
        SpinCount: ULONG,
    ) -> ULONG;
    fn RtlQueryCriticalSectionOwner(
        EventHandle: HANDLE,
    ) -> HANDLE;
    fn RtlCheckForOrphanedCriticalSections(
        ThreadHandle: HANDLE,
    );
}}
STRUCT!{struct RTL_RESOURCE {
    CriticalSection: RTL_CRITICAL_SECTION,
    SharedSemaphore: HANDLE,
    NumberOfWaitingShared: ULONG,
    ExclusiveSemaphore: HANDLE,
    NumberOfWaitingExclusive: ULONG,
    NumberOfActive: LONG,
    ExclusiveOwnerThread: HANDLE,
    Flags: ULONG,
    DebugInfo: PRTL_RESOURCE_DEBUG,
}}
pub type PRTL_RESOURCE = *mut RTL_RESOURCE;
pub const RTL_RESOURCE_FLAG_LONG_TERM: ULONG = 0x00000001;
EXTERN!{extern "system" {
    fn RtlInitializeResource(
        Resource: PRTL_RESOURCE,
    );
    fn RtlDeleteResource(
        Resource: PRTL_RESOURCE,
    );
    fn RtlAcquireResourceShared(
        Resource: PRTL_RESOURCE,
        Wait: BOOLEAN,
    ) -> BOOLEAN;
    fn RtlAcquireResourceExclusive(
        Resource: PRTL_RESOURCE,
        Wait: BOOLEAN,
    ) -> BOOLEAN;
    fn RtlReleaseResource(
        Resource: PRTL_RESOURCE,
    );
    fn RtlConvertSharedToExclusive(
        Resource: PRTL_RESOURCE,
    );
    fn RtlConvertExclusiveToShared(
        Resource: PRTL_RESOURCE,
    );
    fn RtlInitializeSRWLock(
        SRWLock: PRTL_SRWLOCK,
    );
    fn RtlAcquireSRWLockExclusive(
        SRWLock: PRTL_SRWLOCK,
    );
    fn RtlAcquireSRWLockShared(
        SRWLock: PRTL_SRWLOCK,
    );
    fn RtlReleaseSRWLockExclusive(
        SRWLock: PRTL_SRWLOCK,
    );
    fn RtlReleaseSRWLockShared(
        SRWLock: PRTL_SRWLOCK,
    );
    fn RtlTryAcquireSRWLockExclusive(
        SRWLock: PRTL_SRWLOCK,
    ) -> BOOLEAN;
    fn RtlTryAcquireSRWLockShared(
        SRWLock: PRTL_SRWLOCK,
    ) -> BOOLEAN;
    fn RtlAcquireReleaseSRWLockExclusive(
        SRWLock: PRTL_SRWLOCK,
    );
    fn RtlInitializeConditionVariable(
        ConditionVariable: PRTL_CONDITION_VARIABLE,
    );
    fn RtlSleepConditionVariableCS(
        ConditionVariable: PRTL_CONDITION_VARIABLE,
        CriticalSection: PRTL_CRITICAL_SECTION,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlSleepConditionVariableSRW(
        ConditionVariable: PRTL_CONDITION_VARIABLE,
        SRWLock: PRTL_SRWLOCK,
        Timeout: PLARGE_INTEGER,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlWakeConditionVariable(
        ConditionVariable: PRTL_CONDITION_VARIABLE,
    );
    fn RtlWakeAllConditionVariable(
        ConditionVariable: PRTL_CONDITION_VARIABLE,
    );
}}
pub const RTL_BARRIER_FLAGS_SPIN_ONLY: ULONG = 0x00000001;
pub const RTL_BARRIER_FLAGS_BLOCK_ONLY: ULONG = 0x00000002;
pub const RTL_BARRIER_FLAGS_NO_DELETE: ULONG = 0x00000004;
EXTERN!{extern "system" {
    fn RtlInitBarrier(
        Barrier: PRTL_BARRIER,
        TotalThreads: ULONG,
        SpinCount: ULONG,
    ) -> NTSTATUS;
    fn RtlDeleteBarrier(
        Barrier: PRTL_BARRIER,
    ) -> NTSTATUS;
    fn RtlBarrier(
        Barrier: PRTL_BARRIER,
        Flags: ULONG,
    ) -> BOOLEAN;
    fn RtlBarrierForDelete(
        Barrier: PRTL_BARRIER,
        Flags: ULONG,
    ) -> BOOLEAN;
    fn RtlWaitOnAddress(
        Address: *mut VOID,
        CompareAddress: PVOID,
        AddressSize: SIZE_T,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlWakeAddressAll(
        Address: PVOID,
    );
    fn RtlWakeAddressSingle(
        Address: PVOID,
    );
    fn RtlInitString(
        DestinationString: PSTRING,
        SourceString: PCSZ,
    );
    fn RtlInitStringEx(
        DestinationString: PSTRING,
        SourceString: PCSZ,
    ) -> NTSTATUS;
    fn RtlInitAnsiString(
        DestinationString: PANSI_STRING,
        SourceString: PCSZ,
    );
    fn RtlInitAnsiStringEx(
        DestinationString: PANSI_STRING,
        SourceString: PCSZ,
    ) -> NTSTATUS;
    fn RtlFreeAnsiString(
        AnsiString: PANSI_STRING,
    );
    fn RtlFreeOemString(
        OemString: POEM_STRING,
    );
    fn RtlCopyString(
        DestinationString: PSTRING,
        SourceString: *const STRING,
    );
    fn RtlUpperChar(
        Character: CHAR,
    ) -> CHAR;
    fn RtlCompareString(
        String1: *const STRING,
        String2: *const STRING,
        CaseInSensitive: BOOLEAN,
    ) -> LONG;
    fn RtlEqualString(
        String1: *const STRING,
        String2: *const STRING,
        CaseInSensitive: BOOLEAN,
    ) -> BOOLEAN;
    fn RtlPrefixString(
        String1: *const STRING,
        String2: *const STRING,
        CaseInSensitive: BOOLEAN,
    ) -> BOOLEAN;
    fn RtlAppendStringToString(
        Destination: PSTRING,
        Source: *const STRING,
    ) -> NTSTATUS;
    fn RtlAppendAsciizToString(
        Destination: PSTRING,
        Source: PSTR,
    ) -> NTSTATUS;
    fn RtlUpperString(
        DestinationString: PSTRING,
        SourceString: *const STRING,
    );
}}
#[inline]
pub unsafe fn RtlIsNullOrEmptyUnicodeString(String: PUNICODE_STRING) -> bool {
    String.is_null() || (*String).Length == 0
}
#[inline]
pub fn RtlInitEmptyUnicodeString(
    UnicodeString: &mut UNICODE_STRING,
    Buffer: PWCHAR,
    MaximumLength: USHORT,
) {
    UnicodeString.Buffer = Buffer;
    UnicodeString.MaximumLength = MaximumLength;
    UnicodeString.Length = 0;
}
EXTERN!{extern "system" {
    fn RtlInitUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCWSTR,
    );
    fn RtlInitUnicodeStringEx(
        DestinationString: PUNICODE_STRING,
        SourceString: PCWSTR,
    ) -> NTSTATUS;
    fn RtlCreateUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCWSTR,
    ) -> BOOLEAN;
    fn RtlCreateUnicodeStringFromAsciiz(
        DestinationString: PUNICODE_STRING,
        SourceString: PSTR,
    ) -> BOOLEAN;
    fn RtlFreeUnicodeString(
        UnicodeString: PUNICODE_STRING,
    );
}}
pub const RTL_DUPLICATE_UNICODE_STRING_NULL_TERMINATE: ULONG = 0x00000001;
pub const RTL_DUPLICATE_UNICODE_STRING_ALLOCATE_NULL_STRING: ULONG = 0x00000002;
EXTERN!{extern "system" {
    fn RtlDuplicateUnicodeString(
        Flags: ULONG,
        StringIn: PCUNICODE_STRING,
        StringOut: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlCopyUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCUNICODE_STRING,
    );
    fn RtlUpcaseUnicodeChar(
        SourceCharacter: WCHAR,
    ) -> WCHAR;
    fn RtlDowncaseUnicodeChar(
        SourceCharacter: WCHAR,
    ) -> WCHAR;
    fn RtlCompareUnicodeString(
        String1: PCUNICODE_STRING,
        String2: PCUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
    ) -> LONG;
    fn RtlCompareUnicodeStrings(
        String1: PCWCH,
        String1Length: SIZE_T,
        String2: PCWCH,
        String2Length: SIZE_T,
        CaseInSensitive: BOOLEAN,
    ) -> LONG;
    fn RtlEqualUnicodeString(
        String1: PCUNICODE_STRING,
        String2: PCUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
    ) -> BOOLEAN;
}}
pub const HASH_STRING_ALGORITHM_DEFAULT: ULONG = 0;
pub const HASH_STRING_ALGORITHM_X65599: ULONG = 1;
pub const HASH_STRING_ALGORITHM_INVALID: ULONG = 0xffffffff;
EXTERN!{extern "system" {
    fn RtlHashUnicodeString(
        String: PCUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
        HashAlgorithm: ULONG,
        HashValue: PULONG,
    ) -> NTSTATUS;
    fn RtlValidateUnicodeString(
        Flags: ULONG,
        String: PCUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlPrefixUnicodeString(
        String1: PCUNICODE_STRING,
        String2: PCUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
    ) -> BOOLEAN;
    fn RtlSuffixUnicodeString(
        String1: PUNICODE_STRING,
        String2: PUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
    ) -> BOOLEAN;
    fn RtlFindUnicodeSubstring(
        FullString: PUNICODE_STRING,
        SearchString: PUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
    ) -> PWCHAR;
}}
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_START_AT_END: ULONG = 0x00000001;
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_COMPLEMENT_CHAR_SET: ULONG = 0x00000002;
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_CASE_INSENSITIVE: ULONG = 0x00000004;
EXTERN!{extern "system" {
    fn RtlFindCharInUnicodeString(
        Flags: ULONG,
        StringToSearch: PUNICODE_STRING,
        CharSet: PUNICODE_STRING,
        NonInclusivePrefixLength: PUSHORT,
    ) -> NTSTATUS;
    fn RtlAppendUnicodeStringToString(
        Destination: PUNICODE_STRING,
        Source: PCUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlAppendUnicodeToString(
        Destination: PUNICODE_STRING,
        Source: PCWSTR,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlDowncaseUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlEraseUnicodeString(
        String: PUNICODE_STRING,
    );
    fn RtlAnsiStringToUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCANSI_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlUnicodeStringToAnsiString(
        DestinationString: PANSI_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlAnsiCharToUnicodeChar(
        SourceCharacter: *mut PUCHAR,
    ) -> WCHAR;
    fn RtlUpcaseUnicodeStringToAnsiString(
        DestinationString: PANSI_STRING,
        SourceString: PUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlOemStringToUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCOEM_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlUnicodeStringToOemString(
        DestinationString: POEM_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeStringToOemString(
        DestinationString: POEM_STRING,
        SourceString: PUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlUnicodeStringToCountedOemString(
        DestinationString: POEM_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeStringToCountedOemString(
        DestinationString: POEM_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlMultiByteToUnicodeN(
        UnicodeString: PWCH,
        MaxBytesInUnicodeString: ULONG,
        BytesInUnicodeString: PULONG,
        MultiByteString: *const CHAR,
        BytesInMultiByteString: ULONG,
    ) -> NTSTATUS;
    fn RtlMultiByteToUnicodeSize(
        BytesInUnicodeString: PULONG,
        MultiByteString: *const CHAR,
        BytesInMultiByteString: ULONG,
    ) -> NTSTATUS;
    fn RtlUnicodeToMultiByteN(
        MultiByteString: PCHAR,
        MaxBytesInMultiByteString: ULONG,
        BytesInMultiByteString: PULONG,
        UnicodeString: PCWCH,
        BytesInUnicodeString: ULONG,
    ) -> NTSTATUS;
    fn RtlUnicodeToMultiByteSize(
        BytesInMultiByteString: PULONG,
        UnicodeString: PCWCH,
        BytesInUnicodeString: ULONG,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeToMultiByteN(
        MultiByteString: PCHAR,
        MaxBytesInMultiByteString: ULONG,
        BytesInMultiByteString: PULONG,
        UnicodeString: PCWCH,
        BytesInUnicodeString: ULONG,
    ) -> NTSTATUS;
    fn RtlOemToUnicodeN(
        UnicodeString: PWCH,
        MaxBytesInUnicodeString: ULONG,
        BytesInUnicodeString: PULONG,
        OemString: PCCH,
        BytesInOemString: ULONG,
    ) -> NTSTATUS;
    fn RtlUnicodeToOemN(
        OemString: PCHAR,
        MaxBytesInOemString: ULONG,
        BytesInOemString: PULONG,
        UnicodeString: PCWCH,
        BytesInUnicodeString: ULONG,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeToOemN(
        OemString: PCHAR,
        MaxBytesInOemString: ULONG,
        BytesInOemString: PULONG,
        UnicodeString: PCWCH,
        BytesInUnicodeString: ULONG,
    ) -> NTSTATUS;
    fn RtlConsoleMultiByteToUnicodeN(
        UnicodeString: PWCH,
        MaxBytesInUnicodeString: ULONG,
        BytesInUnicodeString: PULONG,
        MultiByteString: PCH,
        BytesInMultiByteString: ULONG,
        pdwSpecialChar: PULONG,
    ) -> NTSTATUS;
    fn RtlUTF8ToUnicodeN(
        UnicodeStringDestination: PWSTR,
        UnicodeStringMaxByteCount: ULONG,
        UnicodeStringActualByteCount: PULONG,
        UTF8StringSource: PCCH,
        UTF8StringByteCount: ULONG,
    ) -> NTSTATUS;
    fn RtlUnicodeToUTF8N(
        UTF8StringDestination: PCHAR,
        UTF8StringMaxByteCount: ULONG,
        UTF8StringActualByteCount: PULONG,
        UnicodeStringSource: PCWCH,
        UnicodeStringByteCount: ULONG,
    ) -> NTSTATUS;
    fn RtlCustomCPToUnicodeN(
        CustomCP: PCPTABLEINFO,
        UnicodeString: PWCH,
        MaxBytesInUnicodeString: ULONG,
        BytesInUnicodeString: PULONG,
        CustomCPString: PCH,
        BytesInCustomCPString: ULONG,
    ) -> NTSTATUS;
    fn RtlUnicodeToCustomCPN(
        CustomCP: PCPTABLEINFO,
        CustomCPString: PCH,
        MaxBytesInCustomCPString: ULONG,
        BytesInCustomCPString: PULONG,
        UnicodeString: PWCH,
        BytesInUnicodeString: ULONG,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeToCustomCPN(
        CustomCP: PCPTABLEINFO,
        CustomCPString: PCH,
        MaxBytesInCustomCPString: ULONG,
        BytesInCustomCPString: PULONG,
        UnicodeString: PWCH,
        BytesInUnicodeString: ULONG,
    ) -> NTSTATUS;
    fn RtlInitCodePageTable(
        TableBase: PUSHORT,
        CodePageTable: PCPTABLEINFO,
    );
    fn RtlInitNlsTables(
        AnsiNlsBase: PUSHORT,
        OemNlsBase: PUSHORT,
        LanguageNlsBase: PUSHORT,
        TableInfo: PNLSTABLEINFO,
    );
    fn RtlResetRtlTranslations(
        TableInfo: PNLSTABLEINFO,
    );
    fn RtlIsTextUnicode(
        Buffer: PVOID,
        Size: ULONG,
        Result: PULONG,
    ) -> BOOLEAN;
}}
ENUM!{enum RTL_NORM_FORM {
    NormOther = 0x0,
    NormC = 0x1,
    NormD = 0x2,
    NormKC = 0x5,
    NormKD = 0x6,
    NormIdna = 0xd,
    DisallowUnassigned = 0x100,
    NormCDisallowUnassigned = 0x101,
    NormDDisallowUnassigned = 0x102,
    NormKCDisallowUnassigned = 0x105,
    NormKDDisallowUnassigned = 0x106,
    NormIdnaDisallowUnassigned = 0x10d,
}}
EXTERN!{extern "system" {
    fn RtlNormalizeString(
        NormForm: ULONG,
        SourceString: PCWSTR,
        SourceStringLength: LONG,
        DestinationString: PWSTR,
        DestinationStringLength: PLONG,
    ) -> NTSTATUS;
    fn RtlIsNormalizedString(
        NormForm: ULONG,
        SourceString: PCWSTR,
        SourceStringLength: LONG,
        Normalized: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlIsNameInExpression(
        Expression: PUNICODE_STRING,
        Name: PUNICODE_STRING,
        IgnoreCase: BOOLEAN,
        UpcaseTable: PWCH,
    ) -> BOOLEAN;
    fn RtlIsNameInUnUpcasedExpression(
        Expression: PUNICODE_STRING,
        Name: PUNICODE_STRING,
        IgnoreCase: BOOLEAN,
        UpcaseTable: PWCH,
    ) -> BOOLEAN;
    fn RtlEqualDomainName(
        String1: PUNICODE_STRING,
        String2: PUNICODE_STRING,
    ) -> BOOLEAN;
    fn RtlEqualComputerName(
        String1: PUNICODE_STRING,
        String2: PUNICODE_STRING,
    ) -> BOOLEAN;
    fn RtlDnsHostNameToComputerName(
        ComputerNameString: PUNICODE_STRING,
        DnsHostNameString: PUNICODE_STRING,
        AllocateComputerNameString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlStringFromGUID(
        Guid: *const GUID,
        GuidString: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlStringFromGUIDEx(
        Guid: *mut GUID,
        GuidString: PUNICODE_STRING,
        AllocateGuidString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlGUIDFromString(
        GuidString: PCUNICODE_STRING,
        Guid: *mut GUID,
    ) -> NTSTATUS;
    fn RtlCompareAltitudes(
        Altitude1: PCUNICODE_STRING,
        Altitude2: PCUNICODE_STRING,
    ) -> LONG;
    fn RtlIdnToAscii(
        Flags: ULONG,
        SourceString: PCWSTR,
        SourceStringLength: LONG,
        DestinationString: PWSTR,
        DestinationStringLength: PLONG,
    ) -> NTSTATUS;
    fn RtlIdnToUnicode(
        Flags: ULONG,
        SourceString: PCWSTR,
        SourceStringLength: LONG,
        DestinationString: PWSTR,
        DestinationStringLength: PLONG,
    ) -> NTSTATUS;
    fn RtlIdnToNameprepUnicode(
        Flags: ULONG,
        SourceString: PCWSTR,
        SourceStringLength: LONG,
        DestinationString: PWSTR,
        DestinationStringLength: PLONG,
    ) -> NTSTATUS;
}}
STRUCT!{struct PREFIX_TABLE_ENTRY {
    NodeTypeCode: CSHORT,
    NameLength: CSHORT,
    NextPrefixTree: *mut PREFIX_TABLE_ENTRY,
    Links: RTL_SPLAY_LINKS,
    Prefix: PSTRING,
}}
pub type PPREFIX_TABLE_ENTRY = *mut PREFIX_TABLE_ENTRY;
STRUCT!{struct PREFIX_TABLE {
    NodeTypeCode: CSHORT,
    NameLength: CSHORT,
    NextPrefixTree: PPREFIX_TABLE_ENTRY,
}}
pub type PPREFIX_TABLE = *mut PREFIX_TABLE;
EXTERN!{extern "system" {
    fn PfxInitialize(
        PrefixTable: PPREFIX_TABLE,
    );
    fn PfxInsertPrefix(
        PrefixTable: PPREFIX_TABLE,
        Prefix: PSTRING,
        PrefixTableEntry: PPREFIX_TABLE_ENTRY,
    ) -> BOOLEAN;
    fn PfxRemovePrefix(
        PrefixTable: PPREFIX_TABLE,
        PrefixTableEntry: PPREFIX_TABLE_ENTRY,
    );
    fn PfxFindPrefix(
        PrefixTable: PPREFIX_TABLE,
        FullName: PSTRING,
    ) -> PPREFIX_TABLE_ENTRY;
}}
STRUCT!{struct UNICODE_PREFIX_TABLE_ENTRY {
    NodeTypeCode: CSHORT,
    NameLength: CSHORT,
    NextPrefixTree: *mut UNICODE_PREFIX_TABLE_ENTRY,
    CaseMatch: *mut UNICODE_PREFIX_TABLE_ENTRY,
    Links: RTL_SPLAY_LINKS,
    Prefix: PUNICODE_STRING,
}}
pub type PUNICODE_PREFIX_TABLE_ENTRY = *mut UNICODE_PREFIX_TABLE_ENTRY;
STRUCT!{struct UNICODE_PREFIX_TABLE {
    NodeTypeCode: CSHORT,
    NameLength: CSHORT,
    NextPrefixTree: PUNICODE_PREFIX_TABLE_ENTRY,
    LastNextEntry: PUNICODE_PREFIX_TABLE_ENTRY,
}}
pub type PUNICODE_PREFIX_TABLE = *mut UNICODE_PREFIX_TABLE;
EXTERN!{extern "system" {
    fn RtlInitializeUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
    );
    fn RtlInsertUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
        Prefix: PUNICODE_STRING,
        PrefixTableEntry: PUNICODE_PREFIX_TABLE_ENTRY,
    ) -> BOOLEAN;
    fn RtlRemoveUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
        PrefixTableEntry: PUNICODE_PREFIX_TABLE_ENTRY,
    );
    fn RtlFindUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
        FullName: PCUNICODE_STRING,
        CaseInsensitiveIndex: ULONG,
    ) -> PUNICODE_PREFIX_TABLE_ENTRY;
    fn RtlNextUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
        Restart: BOOLEAN,
    ) -> PUNICODE_PREFIX_TABLE_ENTRY;
}}
STRUCT!{struct COMPRESSED_DATA_INFO {
    CompressionFormatAndEngine: USHORT,
    CompressionUnitShift: UCHAR,
    ChunkShift: UCHAR,
    ClusterShift: UCHAR,
    Reserved: UCHAR,
    NumberOfChunks: USHORT,
    CompressedChunkSizes: [ULONG; 1],
}}
pub type PCOMPRESSED_DATA_INFO = *mut COMPRESSED_DATA_INFO;
EXTERN!{extern "system" {
    fn RtlGetCompressionWorkSpaceSize(
        CompressionFormatAndEngine: USHORT,
        CompressBufferWorkSpaceSize: PULONG,
        CompressFragmentWorkSpaceSize: PULONG,
    ) -> NTSTATUS;
    fn RtlCompressBuffer(
        CompressionFormatAndEngine: USHORT,
        UncompressedBuffer: PUCHAR,
        UncompressedBufferSize: ULONG,
        CompressedBuffer: PUCHAR,
        CompressedBufferSize: ULONG,
        UncompressedChunkSize: ULONG,
        FinalCompressedSize: PULONG,
        WorkSpace: PVOID,
    ) -> NTSTATUS;
    fn RtlDecompressBuffer(
        CompressionFormat: USHORT,
        UncompressedBuffer: PUCHAR,
        UncompressedBufferSize: ULONG,
        CompressedBuffer: PUCHAR,
        CompressedBufferSize: ULONG,
        FinalUncompressedSize: PULONG,
    ) -> NTSTATUS;
    fn RtlDecompressBufferEx(
        CompressionFormat: USHORT,
        UncompressedBuffer: PUCHAR,
        UncompressedBufferSize: ULONG,
        CompressedBuffer: PUCHAR,
        CompressedBufferSize: ULONG,
        FinalUncompressedSize: PULONG,
        WorkSpace: PVOID,
    ) -> NTSTATUS;
    fn RtlDecompressFragment(
        CompressionFormat: USHORT,
        UncompressedFragment: PUCHAR,
        UncompressedFragmentSize: ULONG,
        CompressedBuffer: PUCHAR,
        CompressedBufferSize: ULONG,
        FragmentOffset: ULONG,
        FinalUncompressedSize: PULONG,
        WorkSpace: PVOID,
    ) -> NTSTATUS;
    fn RtlDescribeChunk(
        CompressionFormat: USHORT,
        CompressedBuffer: *mut PUCHAR,
        EndOfCompressedBufferPlus1: PUCHAR,
        ChunkBuffer: *mut PUCHAR,
        ChunkSize: PULONG,
    ) -> NTSTATUS;
    fn RtlReserveChunk(
        CompressionFormat: USHORT,
        CompressedBuffer: *mut PUCHAR,
        EndOfCompressedBufferPlus1: PUCHAR,
        ChunkBuffer: *mut PUCHAR,
        ChunkSize: ULONG,
    ) -> NTSTATUS;
    fn RtlDecompressChunks(
        UncompressedBuffer: PUCHAR,
        UncompressedBufferSize: ULONG,
        CompressedBuffer: PUCHAR,
        CompressedBufferSize: ULONG,
        CompressedTail: PUCHAR,
        CompressedTailSize: ULONG,
        CompressedDataInfo: PCOMPRESSED_DATA_INFO,
    ) -> NTSTATUS;
    fn RtlCompressChunks(
        UncompressedBuffer: PUCHAR,
        UncompressedBufferSize: ULONG,
        CompressedBuffer: PUCHAR,
        CompressedBufferSize: ULONG,
        CompressedDataInfo: PCOMPRESSED_DATA_INFO,
        CompressedDataInfoLength: ULONG,
        WorkSpace: PVOID,
    ) -> NTSTATUS;
    fn RtlConvertLCIDToString(
        LcidValue: LCID,
        Base: ULONG,
        Padding: ULONG,
        pResultBuf: PWSTR,
        Size: ULONG,
    ) -> NTSTATUS;
    fn RtlIsValidLocaleName(
        LocaleName: PWSTR,
        Flags: ULONG,
    ) -> BOOLEAN;
    fn RtlGetParentLocaleName(
        LocaleName: PWSTR,
        ParentLocaleName: PUNICODE_STRING,
        Flags: ULONG,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlLcidToLocaleName(
        lcid: LCID,
        LocaleName: PUNICODE_STRING,
        Flags: ULONG,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlLocaleNameToLcid(
        LocaleName: PWSTR,
        lcid: PLCID,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlLCIDToCultureName(
        Lcid: LCID,
        String: PUNICODE_STRING,
    ) -> BOOLEAN;
    fn RtlCultureNameToLCID(
        String: PUNICODE_STRING,
        Lcid: PLCID,
    ) -> BOOLEAN;
    fn RtlCleanUpTEBLangLists();
    fn RtlGetLocaleFileMappingAddress(
        BaseAddress: *mut PVOID,
        DefaultLocaleId: PLCID,
        DefaultCasingTableSize: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlGetCurrentPeb() -> PPEB;
    fn RtlAcquirePebLock();
    fn RtlReleasePebLock();
    fn RtlTryAcquirePebLock() -> LOGICAL;
    fn RtlAllocateFromPeb(
        Size: ULONG,
        Block: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlFreeToPeb(
        Block: PVOID,
        Size: ULONG,
    ) -> NTSTATUS;
}}
pub const DOS_MAX_COMPONENT_LENGTH: u32 = 255;
pub const DOS_MAX_PATH_LENGTH: u32 = DOS_MAX_COMPONENT_LENGTH + 5;
STRUCT!{struct CURDIR {
    DosPath: UNICODE_STRING,
    Handle: HANDLE,
}}
pub type PCURDIR = *mut CURDIR;
pub const RTL_USER_PROC_CURDIR_CLOSE: u32 = 0x00000002;
pub const RTL_USER_PROC_CURDIR_INHERIT: u32 = 0x00000003;
STRUCT!{struct RTL_DRIVE_LETTER_CURDIR {
    Flags: USHORT,
    Length: USHORT,
    TimeStamp: ULONG,
    DosPath: STRING,
}}
pub type PRTL_DRIVE_LETTER_CURDIR = *mut RTL_DRIVE_LETTER_CURDIR;
pub const RTL_MAX_DRIVE_LETTERS: usize = 32;
pub const RTL_DRIVE_LETTER_VALID: USHORT = 0x0001;
STRUCT!{struct RTL_USER_PROCESS_PARAMETERS {
    MaximumLength: ULONG,
    Length: ULONG,
    Flags: ULONG,
    DebugFlags: ULONG,
    ConsoleHandle: HANDLE,
    ConsoleFlags: ULONG,
    StandardInput: HANDLE,
    StandardOutput: HANDLE,
    StandardError: HANDLE,
    CurrentDirectory: CURDIR,
    DllPath: UNICODE_STRING,
    ImagePathName: UNICODE_STRING,
    CommandLine: UNICODE_STRING,
    Environment: PVOID,
    StartingX: ULONG,
    StartingY: ULONG,
    CountX: ULONG,
    CountY: ULONG,
    CountCharsX: ULONG,
    CountCharsY: ULONG,
    FillAttribute: ULONG,
    WindowFlags: ULONG,
    ShowWindowFlags: ULONG,
    WindowTitle: UNICODE_STRING,
    DesktopInfo: UNICODE_STRING,
    ShellInfo: UNICODE_STRING,
    RuntimeData: UNICODE_STRING,
    CurrentDirectories: [RTL_DRIVE_LETTER_CURDIR; RTL_MAX_DRIVE_LETTERS],
    EnvironmentSize: ULONG_PTR,
    EnvironmentVersion: ULONG_PTR,
    PackageDependencyData: PVOID,
    ProcessGroupId: ULONG,
    LoaderThreads: ULONG,
}}
pub type PRTL_USER_PROCESS_PARAMETERS = *mut RTL_USER_PROCESS_PARAMETERS;
pub const RTL_USER_PROC_PARAMS_NORMALIZED: ULONG = 0x00000001;
pub const RTL_USER_PROC_PROFILE_USER: ULONG = 0x00000002;
pub const RTL_USER_PROC_PROFILE_KERNEL: ULONG = 0x00000004;
pub const RTL_USER_PROC_PROFILE_SERVER: ULONG = 0x00000008;
pub const RTL_USER_PROC_RESERVE_1MB: ULONG = 0x00000020;
pub const RTL_USER_PROC_RESERVE_16MB: ULONG = 0x00000040;
pub const RTL_USER_PROC_CASE_SENSITIVE: ULONG = 0x00000080;
pub const RTL_USER_PROC_DISABLE_HEAP_DECOMMIT: ULONG = 0x00000100;
pub const RTL_USER_PROC_DLL_REDIRECTION_LOCAL: ULONG = 0x00001000;
pub const RTL_USER_PROC_APP_MANIFEST_PRESENT: ULONG = 0x00002000;
pub const RTL_USER_PROC_IMAGE_KEY_MISSING: ULONG = 0x00004000;
pub const RTL_USER_PROC_OPTIN_PROCESS: ULONG = 0x00020000;
EXTERN!{extern "system" {
    fn RtlCreateProcessParameters(
        pProcessParameters: *mut PRTL_USER_PROCESS_PARAMETERS,
        ImagePathName: PUNICODE_STRING,
        DllPath: PUNICODE_STRING,
        CurrentDirectory: PUNICODE_STRING,
        CommandLine: PUNICODE_STRING,
        Environment: PVOID,
        WindowTitle: PUNICODE_STRING,
        DesktopInfo: PUNICODE_STRING,
        ShellInfo: PUNICODE_STRING,
        RuntimeData: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlCreateProcessParametersEx(
        pProcessParameters: *mut PRTL_USER_PROCESS_PARAMETERS,
        ImagePathName: PUNICODE_STRING,
        DllPath: PUNICODE_STRING,
        CurrentDirectory: PUNICODE_STRING,
        CommandLine: PUNICODE_STRING,
        Environment: PVOID,
        WindowTitle: PUNICODE_STRING,
        DesktopInfo: PUNICODE_STRING,
        ShellInfo: PUNICODE_STRING,
        RuntimeData: PUNICODE_STRING,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlDestroyProcessParameters(
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    ) -> NTSTATUS;
    fn RtlNormalizeProcessParams(
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    ) -> PRTL_USER_PROCESS_PARAMETERS;
    fn RtlDeNormalizeProcessParams(
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    ) -> PRTL_USER_PROCESS_PARAMETERS;
}}
STRUCT!{struct RTL_USER_PROCESS_INFORMATION {
    Length: ULONG,
    Process: HANDLE,
    Thread: HANDLE,
    ClientId: CLIENT_ID,
    ImageInformation: SECTION_IMAGE_INFORMATION,
}}
pub type PRTL_USER_PROCESS_INFORMATION = *mut RTL_USER_PROCESS_INFORMATION;
EXTERN!{extern "system" {
    fn RtlCreateUserProcess(
        NtImagePathName: PUNICODE_STRING,
        AttributesDeprecated: ULONG,
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
        ProcessSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ParentProcess: HANDLE,
        InheritHandles: BOOLEAN,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
        ProcessInformation: PRTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
    fn RtlCreateUserProcessEx(
        NtImagePathName: PUNICODE_STRING,
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
        InheritHandles: BOOLEAN,
        Flags: ULONG,
        ProcessInformation: PRTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
    fn RtlExitUserProcess(
        ExitStatus: NTSTATUS,
    );
}}
pub const RTL_CLONE_PROCESS_FLAGS_CREATE_SUSPENDED: ULONG = 0x00000001;
pub const RTL_CLONE_PROCESS_FLAGS_INHERIT_HANDLES: ULONG = 0x00000002;
pub const RTL_CLONE_PROCESS_FLAGS_NO_SYNCHRONIZE: ULONG = 0x00000004;
EXTERN!{extern "system" {
    fn RtlCloneUserProcess(
        ProcessFlags: ULONG,
        ProcessSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        DebugPort: HANDLE,
        ProcessInformation: PRTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
    fn RtlUpdateClonedCriticalSection(
        CriticalSection: PRTL_CRITICAL_SECTION,
    );
    fn RtlUpdateClonedSRWLock(
        SRWLock: PRTL_SRWLOCK,
        Shared: LOGICAL,
    );
}}
STRUCT!{struct RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
    ReflectionProcessHandle: HANDLE,
    ReflectionThreadHandle: HANDLE,
    ReflectionClientId: CLIENT_ID,
}}
pub type PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION =
    *mut RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION;
EXTERN!{extern "system" {
    fn RtlCreateProcessReflection(
        ProcessHandle: HANDLE,
        Flags: ULONG,
        StartRoutine: PVOID,
        StartContext: PVOID,
        EventHandle: HANDLE,
        ReflectionInformation: PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION,
    ) -> NTSTATUS;
}}
EXTERN!{extern "C" {
    fn RtlSetProcessIsCritical(
        NewValue: BOOLEAN,
        OldValue: PBOOLEAN,
        CheckFlag: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlSetThreadIsCritical(
        NewValue: BOOLEAN,
        OldValue: PBOOLEAN,
        CheckFlag: BOOLEAN,
    ) -> NTSTATUS;
}}
EXTERN!{extern "system" {
    fn RtlValidProcessProtection(
        ProcessProtection: PS_PROTECTION,
    ) -> BOOLEAN;
    fn RtlTestProtectedAccess(
        Source: PS_PROTECTION,
        Target: PS_PROTECTION,
    ) -> BOOLEAN;
    fn RtlIsCurrentProcess(
        ProcessHandle: HANDLE,
    ) -> BOOLEAN;
    fn RtlIsCurrentThread(
        ThreadHandle: HANDLE,
    ) -> BOOLEAN;
}}
FN!{stdcall PUSER_THREAD_START_ROUTINE(
    ThreadParameter: PVOID,
) -> NTSTATUS}
EXTERN!{extern "system" {
    fn RtlCreateUserThread(
        Process: HANDLE,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        CreateSuspended: BOOLEAN,
        ZeroBits: ULONG,
        MaximumStackSize: SIZE_T,
        CommittedStackSize: SIZE_T,
        StartAddress: PUSER_THREAD_START_ROUTINE,
        Parameter: PVOID,
        Thread: PHANDLE,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    fn RtlExitUserThread(
        ExitStatus: NTSTATUS,
    );
    fn RtlIsCurrentThreadAttachExempt() -> BOOLEAN;
    fn RtlCreateUserStack(
        CommittedStackSize: SIZE_T,
        MaximumStackSize: SIZE_T,
        ZeroBits: ULONG_PTR,
        PageSize: SIZE_T,
        ReserveAlignment: ULONG_PTR,
        InitialTeb: PINITIAL_TEB,
    ) -> NTSTATUS;
    fn RtlFreeUserStack(
        AllocationBase: PVOID,
    ) -> NTSTATUS;
}}
STRUCT!{struct CONTEXT_CHUNK {
    Offset: LONG,
    Length: ULONG,
}}
pub type PCONTEXT_CHUNK = *mut CONTEXT_CHUNK;
STRUCT!{struct CONTEXT_EX {
    All: CONTEXT_CHUNK,
    Legacy: CONTEXT_CHUNK,
    XState: CONTEXT_CHUNK,
}}
pub type PCONTEXT_EX = *mut CONTEXT_EX;
pub const CONTEXT_EX_LENGTH: usize = 4096;
#[macro_export]
macro_rules! RTL_CONTEXT_EX_OFFSET {
    ($ContextEx:expr, $Chunk:ident) => {
        (*$ContextEx).$Chunk.Offset
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_EX_LENGTH {
    ($ContextEx:expr, $Chunk:ident) => {
        (*$ContextEx).$Chunk.Length
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_EX_CHUNK {
    ($Base:expr, $Layout:expr, $Chunk:ident) => {
        ($Base as usize + RTL_CONTEXT_EX_OFFSET!($Layout, $Chunk) as usize) as *mut c_void
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_OFFSET {
    ($Context:expr, $Chunk:ident) => {
        RTL_CONTEXT_EX_OFFSET!(($Context as *const $crate::winapi::um::winnt::CONTEXT).offset(1)
            as *const $crate::ntrtl::CONTEXT_EX, $Chunk)
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_LENGTH {
    ($Context:expr, $Chunk:ident) => {
        RTL_CONTEXT_EX_LENGTH!(($Context as *const $crate::winapi::um::winnt::CONTEXT).offset(1)
            as *const $crate::ntrtl::CONTEXT_EX, $Chunk)
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_CHUNK {
    ($Context:expr, $Chunk:ident) => {
        RTL_CONTEXT_EX_CHUNK!(
            ($Context as *const $crate::winapi::um::winnt::CONTEXT).offset(1)
                as *const $crate::ntrtl::CONTEXT_EX,
            ($Context as *const $crate::winapi::um::winnt::CONTEXT).offset(1)
                as *const $crate::ntrtl::CONTEXT_EX,
            $Chunk
        )
    };
}
EXTERN!{extern "system" {
    fn RtlInitializeContext(
        Process: HANDLE,
        Context: PCONTEXT,
        Parameter: PVOID,
        InitialPc: PVOID,
        InitialSp: PVOID,
    );
    fn RtlInitializeExtendedContext(
        Context: PCONTEXT,
        ContextFlags: ULONG,
        ContextEx: *mut PCONTEXT_EX,
    ) -> ULONG;
    fn RtlCopyExtendedContext(
        Destination: PCONTEXT_EX,
        ContextFlags: ULONG,
        Source: PCONTEXT_EX,
    ) -> ULONG;
    fn RtlGetExtendedContextLength(
        ContextFlags: ULONG,
        ContextLength: PULONG,
    ) -> ULONG;
    fn RtlGetExtendedFeaturesMask(
        ContextEx: PCONTEXT_EX,
    ) -> ULONG64;
    fn RtlLocateExtendedFeature(
        ContextEx: PCONTEXT_EX,
        FeatureId: ULONG,
        Length: PULONG,
    ) -> PVOID;
    fn RtlLocateLegacyContext(
        ContextEx: PCONTEXT_EX,
        Length: PULONG,
    ) -> PCONTEXT;
    fn RtlSetExtendedFeaturesMask(
        ContextEx: PCONTEXT_EX,
        FeatureMask: ULONG64,
    );
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
EXTERN!{extern "system" {
    fn RtlWow64GetThreadContext(
        ThreadHandle: HANDLE,
        ThreadContext: PWOW64_CONTEXT,
    ) -> NTSTATUS;
    fn RtlWow64SetThreadContext(
        ThreadHandle: HANDLE,
        ThreadContext: PWOW64_CONTEXT,
    ) -> NTSTATUS;
}}
EXTERN!{extern "system" {
    fn RtlRemoteCall(
        Process: HANDLE,
        Thread: HANDLE,
        CallSite: PVOID,
        ArgumentCount: ULONG,
        Arguments: PULONG_PTR,
        PassContext: BOOLEAN,
        AlreadySuspended: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlAddVectoredExceptionHandler(
        First: ULONG,
        Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> PVOID;
    fn RtlRemoveVectoredExceptionHandler(
        Handle: PVOID,
    ) -> ULONG;
    fn RtlAddVectoredContinueHandler(
        First: ULONG,
        Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> PVOID;
    fn RtlRemoveVectoredContinueHandler(
        Handle: PVOID,
    ) -> ULONG;
}}
FN!{stdcall PRTLP_UNHANDLED_EXCEPTION_FILTER(
    ExceptionInfo: PEXCEPTION_POINTERS,
) -> ULONG}
EXTERN!{extern "system" {
    fn RtlSetUnhandledExceptionFilter(
        UnhandledExceptionFilter: PRTLP_UNHANDLED_EXCEPTION_FILTER,
    );
    fn RtlUnhandledExceptionFilter(
        ExceptionPointers: PEXCEPTION_POINTERS,
    ) -> LONG;
    fn RtlUnhandledExceptionFilter2(
        ExceptionPointers: PEXCEPTION_POINTERS,
        Flags: ULONG,
    ) -> LONG;
    fn RtlKnownExceptionFilter(
        ExceptionPointers: PEXCEPTION_POINTERS,
    ) -> LONG;
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
IFDEF!{
ENUM!{enum FUNCTION_TABLE_TYPE {
    RF_SORTED = 0,
    RF_UNSORTED = 1,
    RF_CALLBACK = 2,
    RF_KERNEL_DYNAMIC = 3,
}}
STRUCT!{struct DYNAMIC_FUNCTION_TABLE {
    ListEntry: LIST_ENTRY,
    FunctionTable: PRUNTIME_FUNCTION,
    TimeStamp: LARGE_INTEGER,
    MinimumAddress: ULONG64,
    MaximumAddress: ULONG64,
    BaseAddress: ULONG64,
    Callback: PGET_RUNTIME_FUNCTION_CALLBACK,
    Context: PVOID,
    OutOfProcessCallbackDll: PWSTR,
    Type: FUNCTION_TABLE_TYPE,
    EntryCount: ULONG,
    TreeNode: RTL_BALANCED_NODE,
}}
pub type PDYNAMIC_FUNCTION_TABLE = *mut DYNAMIC_FUNCTION_TABLE;
EXTERN!{extern "system" {
    fn RtlGetFunctionTableListHead() -> PLIST_ENTRY;
}}
}
EXTERN!{extern "system" {
    fn RtlImageNtHeader(
        BaseOfImage: PVOID,
    ) -> PIMAGE_NT_HEADERS;
}}
pub const RTL_IMAGE_NT_HEADER_EX_FLAG_NO_RANGE_CHECK: ULONG = 0x00000001;
EXTERN!{extern "system" {
    fn RtlImageNtHeaderEx(
        Flags: ULONG,
        BaseOfImage: PVOID,
        Size: ULONG64,
        OutHeaders: *mut PIMAGE_NT_HEADERS,
    ) -> NTSTATUS;
    fn RtlAddressInSectionTable(
        NtHeaders: PIMAGE_NT_HEADERS,
        BaseOfImage: PVOID,
        VirtualAddress: ULONG,
    ) -> PVOID;
    fn RtlSectionTableFromVirtualAddress(
        NtHeaders: PIMAGE_NT_HEADERS,
        BaseOfImage: PVOID,
        VirtualAddress: ULONG,
    ) -> PIMAGE_SECTION_HEADER;
    fn RtlImageDirectoryEntryToData(
        BaseOfImage: PVOID,
        MappedAsImage: BOOLEAN,
        DirectoryEntry: USHORT,
        Size: PULONG,
    ) -> PVOID;
    fn RtlImageRvaToSection(
        NtHeaders: PIMAGE_NT_HEADERS,
        BaseOfImage: PVOID,
        Rva: ULONG,
    ) -> PIMAGE_SECTION_HEADER;
    fn RtlImageRvaToVa(
        NtHeaders: PIMAGE_NT_HEADERS,
        BaseOfImage: PVOID,
        Rva: ULONG,
        LastRvaSection: *mut PIMAGE_SECTION_HEADER,
    ) -> PVOID;
    fn RtlFindExportedRoutineByName(
        BaseOfImage: PVOID,
        RoutineName: PSTR,
    ) -> PVOID;
    fn RtlGuardCheckLongJumpTarget(
        PcValue: PVOID,
        IsFastFail: BOOL,
        IsLongJumpTarget: PBOOL,
    ) -> NTSTATUS;
    fn RtlCompareMemoryUlong(
        Source: PVOID,
        Length: SIZE_T,
        Pattern: ULONG,
    ) -> SIZE_T;
    fn RtlFillMemoryUlong(
        Destination: PVOID,
        Length: SIZE_T,
        Pattern: ULONG,
    );
    fn RtlFillMemoryUlonglong(
        Destination: PVOID,
        Length: SIZE_T,
        Pattern: ULONGLONG,
    );
    fn RtlCreateEnvironment(
        CloneCurrentEnvironment: BOOLEAN,
        Environment: *mut PVOID,
    ) -> NTSTATUS;
}}
pub const RTL_CREATE_ENVIRONMENT_TRANSLATE: ULONG = 0x1;
pub const RTL_CREATE_ENVIRONMENT_TRANSLATE_FROM_OEM: ULONG = 0x2;
pub const RTL_CREATE_ENVIRONMENT_EMPTY: ULONG = 0x4;
EXTERN!{extern "system" {
    fn RtlCreateEnvironmentEx(
        SourceEnv: PVOID,
        Environment: *mut PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlDestroyEnvironment(
        Environment: PVOID,
    ) -> NTSTATUS;
    fn RtlSetCurrentEnvironment(
        Environment: PVOID,
        PreviousEnvironment: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlSetEnvironmentVar(
        Environment: *mut PWSTR,
        Name: PWSTR,
        NameLength: SIZE_T,
        Value: PWSTR,
        ValueLength: SIZE_T,
    ) -> NTSTATUS;
    fn RtlSetEnvironmentVariable(
        Environment: *mut PVOID,
        Name: PUNICODE_STRING,
        Value: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlQueryEnvironmentVariable(
        Environment: PVOID,
        Name: PWSTR,
        NameLength: SIZE_T,
        Value: PWSTR,
        ValueLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;
    fn RtlQueryEnvironmentVariable_U(
        Environment: PVOID,
        Name: PUNICODE_STRING,
        Value: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlExpandEnvironmentStrings(
        Environment: PVOID,
        Src: PWSTR,
        SrcLength: SIZE_T,
        Dst: PWSTR,
        DstLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;
    fn RtlExpandEnvironmentStrings_U(
        Environment: PVOID,
        Source: PUNICODE_STRING,
        Destination: PUNICODE_STRING,
        ReturnedLength: PULONG,
    ) -> NTSTATUS;
    fn RtlSetEnvironmentStrings(
        NewEnvironment: PWCHAR,
        NewEnvironmentSize: SIZE_T,
    ) -> NTSTATUS;
}}
STRUCT!{struct RTLP_CURDIR_REF {
    ReferenceCount: LONG,
    DirectoryHandle: HANDLE,
}}
pub type PRTLP_CURDIR_REF = *mut RTLP_CURDIR_REF;
STRUCT!{struct RTL_RELATIVE_NAME_U {
    RelativeName: UNICODE_STRING,
    ContainingDirectory: HANDLE,
    CurDirRef: PRTLP_CURDIR_REF,
}}
pub type PRTL_RELATIVE_NAME_U = *mut RTL_RELATIVE_NAME_U;
ENUM!{enum RTL_PATH_TYPE {
    RtlPathTypeUnknown = 0,
    RtlPathTypeUncAbsolute = 1,
    RtlPathTypeDriveAbsolute = 2,
    RtlPathTypeDriveRelative = 3,
    RtlPathTypeRooted = 4,
    RtlPathTypeRelative = 5,
    RtlPathTypeLocalDevice = 6,
    RtlPathTypeRootLocalDevice = 7,
}}
EXTERN!{extern "C" {
    static mut RtlDosPathSeperatorsString: UNICODE_STRING;
    static mut RtlAlternateDosPathSeperatorString: UNICODE_STRING;
    static mut RtlNtPathSeperatorString: UNICODE_STRING;
}}
/// "ntdll.dll"
pub const RtlNtdllName: UTF16Const = UTF16Const(&[
    0x006E, 0x0074, 0x0064, 0x006C, 0x006C, 0x002E, 0x0064, 0x006C, 0x006C, 0u16,
]);
EXTERN!{extern "system" {
    fn RtlDetermineDosPathNameType_U(
        DosFileName: PWSTR,
    ) -> RTL_PATH_TYPE;
    fn RtlDetermineDosPathNameType_Ustr(
        DosFileName: PCUNICODE_STRING,
    ) -> RTL_PATH_TYPE;
    fn RtlIsDosDeviceName_U(
        DosFileName: PWSTR,
    ) -> ULONG;
    fn RtlIsDosDeviceName_Ustr(
        DosFileName: PUNICODE_STRING,
    ) -> ULONG;
    fn RtlGetFullPathName_U(
        FileName: PWSTR,
        BufferLength: ULONG,
        Buffer: PWSTR,
        FilePart: *mut PWSTR,
    ) -> ULONG;
    fn RtlGetFullPathName_UEx(
        FileName: PWSTR,
        BufferLength: ULONG,
        Buffer: PWSTR,
        FilePart: *mut PWSTR,
        BytesRequired: *mut ULONG,
    ) -> NTSTATUS;
    fn RtlGetFullPathName_UstrEx(
        FileName: PUNICODE_STRING,
        StaticString: PUNICODE_STRING,
        DynamicString: PUNICODE_STRING,
        StringUsed: *mut PUNICODE_STRING,
        FilePartPrefixCch: *mut SIZE_T,
        NameInvalid: PBOOLEAN,
        InputPathType: *mut RTL_PATH_TYPE,
        BytesRequired: *mut SIZE_T,
    ) -> NTSTATUS;
    fn RtlGetCurrentDirectory_U(
        BufferLength: ULONG,
        Buffer: PWSTR,
    ) -> ULONG;
    fn RtlSetCurrentDirectory_U(
        PathName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlGetLongestNtPathLength() -> ULONG;
    fn RtlDosPathNameToNtPathName_U(
        DosFileName: PWSTR,
        NtFileName: PUNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> BOOLEAN;
    fn RtlDosPathNameToNtPathName_U_WithStatus(
        DosFileName: PWSTR,
        NtFileName: PUNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
    fn RtlDosLongPathNameToNtPathName_U_WithStatus(
        DosFileName: PWSTR,
        NtFileName: PUNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
    fn RtlDosPathNameToRelativeNtPathName_U(
        DosFileName: PWSTR,
        NtFileName: PUNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> BOOLEAN;
    fn RtlDosPathNameToRelativeNtPathName_U_WithStatus(
        DosFileName: PWSTR,
        NtFileName: PUNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
    fn RtlDosLongPathNameToRelativeNtPathName_U_WithStatus(
        DosFileName: PWSTR,
        NtFileName: PUNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
    fn RtlReleaseRelativeName(
        RelativeName: PRTL_RELATIVE_NAME_U,
    );
    fn RtlDosSearchPath_U(
        Path: PWSTR,
        FileName: PWSTR,
        Extension: PWSTR,
        BufferLength: ULONG,
        Buffer: PWSTR,
        FilePart: *mut PWSTR,
    ) -> ULONG;
}}
pub const RTL_DOS_SEARCH_PATH_FLAG_APPLY_ISOLATION_REDIRECTION: ULONG = 0x00000001;
pub const RTL_DOS_SEARCH_PATH_FLAG_DISALLOW_DOT_RELATIVE_PATH_SEARCH: ULONG = 0x00000002;
pub const RTL_DOS_SEARCH_PATH_FLAG_APPLY_DEFAULT_EXTENSION_WHEN_NOT_RELATIVE_PATH_EVEN_IF_FILE_HAS_EXTENSION: ULONG = 0x00000004;
EXTERN!{extern "system" {
    fn RtlDosSearchPath_Ustr(
        Flags: ULONG,
        Path: PUNICODE_STRING,
        FileName: PUNICODE_STRING,
        DefaultExtension: PUNICODE_STRING,
        StaticString: PUNICODE_STRING,
        DynamicString: PUNICODE_STRING,
        FullFileNameOut: *mut PCUNICODE_STRING,
        FilePartPrefixCch: *mut SIZE_T,
        BytesRequired: *mut SIZE_T,
    ) -> NTSTATUS;
    fn RtlDoesFileExists_U(
        FileName: PWSTR,
    ) -> BOOLEAN;
    fn RtlGetLengthWithoutLastFullDosOrNtPathElement(
        Flags: ULONG,
        PathString: PUNICODE_STRING,
        Length: PULONG,
    ) -> NTSTATUS;
    fn RtlGetLengthWithoutTrailingPathSeperators(
        Flags: ULONG,
        PathString: PUNICODE_STRING,
        Length: PULONG,
    ) -> NTSTATUS;
}}
STRUCT!{struct GENERATE_NAME_CONTEXT {
    Checksum: USHORT,
    CheckSumInserted: BOOLEAN,
    NameLength: UCHAR,
    NameBuffer: [WCHAR; 8],
    ExtensionLength: ULONG,
    ExtensionBuffer: [WCHAR; 4],
    LastIndexValue: ULONG,
}}
pub type PGENERATE_NAME_CONTEXT = *mut GENERATE_NAME_CONTEXT;
EXTERN!{extern "system" {
    fn RtlGenerate8dot3Name(
        Name: PCUNICODE_STRING,
        AllowExtendedCharacters: BOOLEAN,
        Context: PGENERATE_NAME_CONTEXT,
        Name8dot3: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlComputePrivatizedDllName_U(
        DllName: PUNICODE_STRING,
        RealName: PUNICODE_STRING,
        LocalName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlGetSearchPath(
        SearchPathA: *mut PWSTR,
    ) -> BOOLEAN;
    fn RtlSetSearchPathMode(
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlGetExePath() -> PWSTR;
    fn RtlGetNtSystemRoot() -> PWSTR;
    fn RtlAreLongPathsEnabled() -> BOOLEAN;
    fn RtlIsThreadWithinLoaderCallout() -> BOOLEAN;
    fn RtlDllShutdownInProgress() -> BOOLEAN;
}}
STRUCT!{struct RTL_HEAP_ENTRY_u_s1 {
    Settable: SIZE_T,
    Tag: ULONG,
}}
STRUCT!{struct RTL_HEAP_ENTRY_u_s2 {
    CommittedSize: SIZE_T,
    FirstBlock: PVOID,
}}
UNION!{union RTL_HEAP_ENTRY_u {
    s1: RTL_HEAP_ENTRY_u_s1,
    s2: RTL_HEAP_ENTRY_u_s2,
}}
STRUCT!{struct RTL_HEAP_ENTRY {
    Size: SIZE_T,
    Flags: USHORT,
    AllocatorBackTraceIndex: USHORT,
    u: RTL_HEAP_ENTRY_u,
}}
pub type PRTL_HEAP_ENTRY = *mut RTL_HEAP_ENTRY;
pub const RTL_HEAP_BUSY: USHORT = 0x0001;
pub const RTL_HEAP_SEGMENT: USHORT = 0x0002;
pub const RTL_HEAP_SETTABLE_VALUE: USHORT = 0x0010;
pub const RTL_HEAP_SETTABLE_FLAG1: USHORT = 0x0020;
pub const RTL_HEAP_SETTABLE_FLAG2: USHORT = 0x0040;
pub const RTL_HEAP_SETTABLE_FLAG3: USHORT = 0x0080;
pub const RTL_HEAP_SETTABLE_FLAGS: USHORT = 0x00e0;
pub const RTL_HEAP_UNCOMMITTED_RANGE: USHORT = 0x0100;
pub const RTL_HEAP_PROTECTED_ENTRY: USHORT = 0x0200;
STRUCT!{struct RTL_HEAP_TAG {
    NumberOfAllocations: ULONG,
    NumberOfFrees: ULONG,
    BytesAllocated: SIZE_T,
    TagIndex: USHORT,
    CreatorBackTraceIndex: USHORT,
    TagName: [WCHAR; 24],
}}
pub type PRTL_HEAP_TAG = *mut RTL_HEAP_TAG;
STRUCT!{struct RTL_HEAP_INFORMATION {
    BaseAddress: PVOID,
    Flags: ULONG,
    EntryOverhead: USHORT,
    CreatorBackTraceIndex: USHORT,
    BytesAllocated: SIZE_T,
    BytesCommitted: SIZE_T,
    NumberOfTags: ULONG,
    NumberOfEntries: ULONG,
    NumberOfPseudoTags: ULONG,
    PseudoTagGranularity: ULONG,
    Reserved: [ULONG; 5],
    Tags: PRTL_HEAP_TAG,
    Entries: PRTL_HEAP_ENTRY,
}}
pub type PRTL_HEAP_INFORMATION = *mut RTL_HEAP_INFORMATION;
STRUCT!{struct RTL_PROCESS_HEAPS {
    NumberOfHeaps: ULONG,
    Heaps: [RTL_HEAP_INFORMATION; 1],
}}
pub type PRTL_PROCESS_HEAPS = *mut RTL_PROCESS_HEAPS;
FN!{stdcall PRTL_HEAP_COMMIT_ROUTINE(
    Base: PVOID,
    CommitAddress: *mut PVOID,
    CommitSize: PSIZE_T,
) -> NTSTATUS}
STRUCT!{struct RTL_HEAP_PARAMETERS {
    Length: ULONG,
    SegmentReserve: SIZE_T,
    SegmentCommit: SIZE_T,
    DeCommitFreeBlockThreshold: SIZE_T,
    DeCommitTotalFreeThreshold: SIZE_T,
    MaximumAllocationSize: SIZE_T,
    VirtualMemoryThreshold: SIZE_T,
    InitialCommit: SIZE_T,
    InitialReserve: SIZE_T,
    CommitRoutine: PRTL_HEAP_COMMIT_ROUTINE,
    Reserved: [SIZE_T; 2],
}}
pub type PRTL_HEAP_PARAMETERS = *mut RTL_HEAP_PARAMETERS;
pub const HEAP_SETTABLE_USER_VALUE: ULONG = 0x00000100;
pub const HEAP_SETTABLE_USER_FLAG1: ULONG = 0x00000200;
pub const HEAP_SETTABLE_USER_FLAG2: ULONG = 0x00000400;
pub const HEAP_SETTABLE_USER_FLAG3: ULONG = 0x00000800;
pub const HEAP_SETTABLE_USER_FLAGS: ULONG = 0x00000e00;
pub const HEAP_CLASS_0: ULONG = 0x00000000;
pub const HEAP_CLASS_1: ULONG = 0x00001000;
pub const HEAP_CLASS_2: ULONG = 0x00002000;
pub const HEAP_CLASS_3: ULONG = 0x00003000;
pub const HEAP_CLASS_4: ULONG = 0x00004000;
pub const HEAP_CLASS_5: ULONG = 0x00005000;
pub const HEAP_CLASS_6: ULONG = 0x00006000;
pub const HEAP_CLASS_7: ULONG = 0x00007000;
pub const HEAP_CLASS_8: ULONG = 0x00008000;
pub const HEAP_CLASS_MASK: ULONG = 0x0000f000;
EXTERN!{extern "system" {
    fn RtlCreateHeap(
        Flags: ULONG,
        HeapBase: PVOID,
        ReserveSize: SIZE_T,
        CommitSize: SIZE_T,
        Lock: PVOID,
        Parameters: PRTL_HEAP_PARAMETERS,
    ) -> PVOID;
    fn RtlDestroyHeap(
        HeapHandle: PVOID,
    ) -> PVOID;
    fn RtlAllocateHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        Size: SIZE_T,
    ) -> PVOID;
    fn RtlFreeHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        BaseAddress: PVOID,
    ) -> BOOLEAN;
    fn RtlSizeHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        BaseAddress: PVOID,
    ) -> SIZE_T;
    fn RtlZeroHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlProtectHeap(
        HeapHandle: PVOID,
        MakeReadOnly: BOOLEAN,
    );
}}
#[inline] #[cfg(not(target_arch = "aarch64"))]
pub unsafe fn RtlProcessHeap() -> PVOID {
    use crate::ntpsapi::NtCurrentPeb;
    (*NtCurrentPeb()).ProcessHeap
}
EXTERN!{extern "system" {
    fn RtlLockHeap(
        HeapHandle: PVOID,
    ) -> BOOLEAN;
    fn RtlUnlockHeap(
        HeapHandle: PVOID,
    ) -> BOOLEAN;
    fn RtlReAllocateHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        BaseAddress: PVOID,
        Size: SIZE_T,
    ) -> PVOID;
    fn RtlGetUserInfoHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        BaseAddress: PVOID,
        UserValue: *mut PVOID,
        UserFlags: PULONG,
    ) -> BOOLEAN;
    fn RtlSetUserValueHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        BaseAddress: PVOID,
        UserValue: PVOID,
    ) -> BOOLEAN;
    fn RtlSetUserFlagsHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        BaseAddress: PVOID,
        UserFlagsReset: ULONG,
        UserFlagsSet: ULONG,
    ) -> BOOLEAN;
}}
STRUCT!{struct RTL_HEAP_TAG_INFO {
    NumberOfAllocations: ULONG,
    NumberOfFrees: ULONG,
    BytesAllocated: SIZE_T,
}}
pub type PRTL_HEAP_TAG_INFO = *mut RTL_HEAP_TAG_INFO;
EXTERN!{extern "system" {
    fn RtlCreateTagHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        TagPrefix: PWSTR,
        TagNames: PWSTR,
    ) -> ULONG;
    fn RtlQueryTagHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        TagIndex: USHORT,
        ResetCounters: BOOLEAN,
        TagInfo: PRTL_HEAP_TAG_INFO,
    ) -> PWSTR;
    fn RtlExtendHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        Base: PVOID,
        Size: SIZE_T,
    ) -> NTSTATUS;
    fn RtlCompactHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
    ) -> SIZE_T;
    fn RtlValidateHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        BaseAddress: PVOID,
    ) -> BOOLEAN;
    fn RtlValidateProcessHeaps() -> BOOLEAN;
    fn RtlGetProcessHeaps(
        NumberOfHeaps: ULONG,
        ProcessHeaps: *mut PVOID,
    ) -> ULONG;
}}
FN!{stdcall PRTL_ENUM_HEAPS_ROUTINE(
    HeapHandle: PVOID,
    Parameter: PVOID,
) -> NTSTATUS}
EXTERN!{extern "system" {
    fn RtlEnumProcessHeaps(
        EnumRoutine: PRTL_ENUM_HEAPS_ROUTINE,
        Parameter: PVOID,
    ) -> NTSTATUS;
}}
STRUCT!{struct RTL_HEAP_USAGE_ENTRY {
    Next: *mut RTL_HEAP_USAGE_ENTRY,
    Address: PVOID,
    Size: SIZE_T,
    AllocatorBackTraceIndex: USHORT,
    TagIndex: USHORT,
}}
pub type PRTL_HEAP_USAGE_ENTRY = *mut RTL_HEAP_USAGE_ENTRY;
STRUCT!{struct RTL_HEAP_USAGE {
    Length: ULONG,
    BytesAllocated: SIZE_T,
    BytesCommitted: SIZE_T,
    BytesReserved: SIZE_T,
    BytesReservedMaximum: SIZE_T,
    Entries: PRTL_HEAP_USAGE_ENTRY,
    AddedEntries: PRTL_HEAP_USAGE_ENTRY,
    RemovedEntries: PRTL_HEAP_USAGE_ENTRY,
    Reserved: [ULONG_PTR; 8],
}}
pub type PRTL_HEAP_USAGE = *mut RTL_HEAP_USAGE;
pub const HEAP_USAGE_ALLOCATED_BLOCKS: ULONG = HEAP_REALLOC_IN_PLACE_ONLY;
pub const HEAP_USAGE_FREE_BUFFER: ULONG = HEAP_ZERO_MEMORY;
EXTERN!{extern "system" {
    fn RtlUsageHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        Usage: PRTL_HEAP_USAGE,
    ) -> NTSTATUS;
}}
STRUCT!{struct RTL_HEAP_WALK_ENTRY_u_Block {
    Settable: SIZE_T,
    TagIndex: USHORT,
    AllocatorBackTraceIndex: USHORT,
    Reserved: [ULONG; 2],
}}
STRUCT!{struct RTL_HEAP_WALK_ENTRY_u_Segment {
    CommittedSize: ULONG,
    UnCommittedSize: ULONG,
    FirstEntry: PVOID,
    LastEntry: PVOID,
}}
UNION!{union RTL_HEAP_WALK_ENTRY_u {
    Block: RTL_HEAP_WALK_ENTRY_u_Block,
    Segment: RTL_HEAP_WALK_ENTRY_u_Segment,
}}
STRUCT!{struct RTL_HEAP_WALK_ENTRY {
    DataAddress: PVOID,
    DataSize: SIZE_T,
    OverheadBytes: UCHAR,
    SegmentIndex: UCHAR,
    Flags: USHORT,
    u: RTL_HEAP_WALK_ENTRY_u,
}}
pub type PRTL_HEAP_WALK_ENTRY = *mut RTL_HEAP_WALK_ENTRY;
EXTERN!{extern "system" {
    fn RtlWalkHeap(
        HeapHandle: PVOID,
        Entry: PRTL_HEAP_WALK_ENTRY,
    ) -> NTSTATUS;
}}
pub const HeapDetailedFailureInformation: u32 = 0x80000001;
pub const HeapSetDebuggingInformation: u32 = 0x80000002;
ENUM!{enum HEAP_COMPATIBILITY_MODE {
    HEAP_COMPATIBILITY_STANDARD = 0,
    HEAP_COMPATIBILITY_LAL = 1,
    HEAP_COMPATIBILITY_LFH = 2,
}}
STRUCT!{struct PROCESS_HEAP_INFORMATION {
    ReserveSize: ULONG_PTR,
    CommitSize: ULONG_PTR,
    NumberOfHeaps: ULONG,
    FirstHeapInformationOffset: ULONG_PTR,
}}
pub type PPROCESS_HEAP_INFORMATION = *mut PROCESS_HEAP_INFORMATION;
STRUCT!{struct HEAP_INFORMATION {
    Address: ULONG_PTR,
    Mode: ULONG,
    ReserveSize: ULONG_PTR,
    CommitSize: ULONG_PTR,
    FirstRegionInformationOffset: ULONG_PTR,
    NextHeapInformationOffset: ULONG_PTR,
}}
pub type PHEAP_INFORMATION = *mut HEAP_INFORMATION;
UNION!{union HEAP_EXTENDED_INFORMATION_u {
    ProcessHeapInformation: PROCESS_HEAP_INFORMATION,
    HeapInformation: HEAP_INFORMATION,
}}
STRUCT!{struct HEAP_EXTENDED_INFORMATION {
    Process: HANDLE,
    Heap: ULONG_PTR,
    Level: ULONG,
    CallbackRoutine: PVOID,
    CallbackContext: PVOID,
    u: HEAP_EXTENDED_INFORMATION_u,
}}
pub type PHEAP_EXTENDED_INFORMATION = *mut HEAP_EXTENDED_INFORMATION;
FN!{stdcall PRTL_HEAP_LEAK_ENUMERATION_ROUTINE(
    Reserved: LONG,
    HeapHandle: PVOID,
    BaseAddress: PVOID,
    BlockSize: SIZE_T,
    StackTraceDepth: ULONG,
    StackTrace: *mut PVOID,
) -> NTSTATUS}
STRUCT!{struct HEAP_DEBUGGING_INFORMATION {
    InterceptorFunction: PVOID,
    InterceptorValue: USHORT,
    ExtendedOptions: ULONG,
    StackTraceDepth: ULONG,
    MinTotalBlockSize: SIZE_T,
    MaxTotalBlockSize: SIZE_T,
    HeapLeakEnumerationRoutine: PRTL_HEAP_LEAK_ENUMERATION_ROUTINE,
}}
pub type PHEAP_DEBUGGING_INFORMATION = *mut HEAP_DEBUGGING_INFORMATION;
EXTERN!{extern "system" {
    fn RtlQueryHeapInformation(
        HeapHandle: PVOID,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: PVOID,
        HeapInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;
    fn RtlSetHeapInformation(
        HeapHandle: PVOID,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: PVOID,
        HeapInformationLength: SIZE_T,
    ) -> NTSTATUS;
    fn RtlMultipleAllocateHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        Size: SIZE_T,
        Count: ULONG,
        Array: *mut PVOID,
    ) -> ULONG;
    fn RtlMultipleFreeHeap(
        HeapHandle: PVOID,
        Flags: ULONG,
        Count: ULONG,
        Array: *mut PVOID,
    ) -> ULONG;
    fn RtlDetectHeapLeaks();
    fn RtlFlushHeaps();
}}
STRUCT!{struct RTL_MEMORY_ZONE_SEGMENT {
    NextSegment: *mut RTL_MEMORY_ZONE_SEGMENT,
    Size: SIZE_T,
    Next: PVOID,
    Limit: PVOID,
}}
pub type PRTL_MEMORY_ZONE_SEGMENT = *mut RTL_MEMORY_ZONE_SEGMENT;
STRUCT!{struct RTL_MEMORY_ZONE {
    Segment: RTL_MEMORY_ZONE_SEGMENT,
    Lock: RTL_SRWLOCK,
    LockCount: ULONG,
    FirstSegment: PRTL_MEMORY_ZONE_SEGMENT,
}}
pub type PRTL_MEMORY_ZONE = *mut RTL_MEMORY_ZONE;
EXTERN!{extern "system" {
    fn RtlCreateMemoryZone(
        MemoryZone: *mut PVOID,
        InitialSize: SIZE_T,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlDestroyMemoryZone(
        MemoryZone: PVOID,
    ) -> NTSTATUS;
    fn RtlAllocateMemoryZone(
        MemoryZone: PVOID,
        BlockSize: SIZE_T,
        Block: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlResetMemoryZone(
        MemoryZone: PVOID,
    ) -> NTSTATUS;
    fn RtlLockMemoryZone(
        MemoryZone: PVOID,
    ) -> NTSTATUS;
    fn RtlUnlockMemoryZone(
        MemoryZone: PVOID,
    ) -> NTSTATUS;
    fn RtlCreateMemoryBlockLookaside(
        MemoryBlockLookaside: *mut PVOID,
        Flags: ULONG,
        InitialSize: ULONG,
        MinimumBlockSize: ULONG,
        MaximumBlockSize: ULONG,
    ) -> NTSTATUS;
    fn RtlDestroyMemoryBlockLookaside(
        MemoryBlockLookaside: PVOID,
    ) -> NTSTATUS;
    fn RtlAllocateMemoryBlockLookaside(
        MemoryBlockLookaside: PVOID,
        BlockSize: ULONG,
        Block: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlFreeMemoryBlockLookaside(
        MemoryBlockLookaside: PVOID,
        Block: PVOID,
    ) -> NTSTATUS;
    fn RtlExtendMemoryBlockLookaside(
        MemoryBlockLookaside: PVOID,
        Increment: ULONG,
    ) -> NTSTATUS;
    fn RtlResetMemoryBlockLookaside(
        MemoryBlockLookaside: PVOID,
    ) -> NTSTATUS;
    fn RtlLockMemoryBlockLookaside(
        MemoryBlockLookaside: PVOID,
    ) -> NTSTATUS;
    fn RtlUnlockMemoryBlockLookaside(
        MemoryBlockLookaside: PVOID,
    ) -> NTSTATUS;
    fn RtlGetCurrentTransaction() -> HANDLE;
    fn RtlSetCurrentTransaction(
        TransactionHandle: HANDLE,
    ) -> LOGICAL;
}}
#[inline]
pub const fn RtlIsEqualLuid(L1: &LUID, L2: &LUID) -> bool {
    (L1.LowPart == L2.LowPart) && (L1.HighPart == L2.HighPart)
}
#[inline]
pub const fn RtlIsZeroLuid(L1: &LUID) -> bool {
    (L1.LowPart | L1.HighPart as u32) == 0
}
#[inline]
pub const fn RtlConvertLongToLuid(Long: LONG) -> LUID {
    LUID { LowPart: Long as u32, HighPart: ((Long as i64) >> 32) as i32 }
}
#[inline]
pub const fn RtlConvertUlongToLuid(Ulong: ULONG) -> LUID {
    LUID { LowPart: Ulong, HighPart: 0 }
}
EXTERN!{extern "system" {
    fn RtlCopyLuid(
        DestinationLuid: PLUID,
        SourceLuid: PLUID,
    );
    fn RtlCopyLuidAndAttributesArray(
        Count: ULONG,
        Src: PLUID_AND_ATTRIBUTES,
        Dest: PLUID_AND_ATTRIBUTES,
    );
}}
STRUCT!{struct RTL_PROCESS_VERIFIER_OPTIONS {
    SizeStruct: ULONG,
    Option: ULONG,
    OptionData: [UCHAR; 1],
}}
pub type PRTL_PROCESS_VERIFIER_OPTIONS = *mut RTL_PROCESS_VERIFIER_OPTIONS;
UNION!{union RTL_DEBUG_INFORMATION_u {
    Modules: *mut RTL_PROCESS_MODULES,
    ModulesEx: *mut RTL_PROCESS_MODULE_INFORMATION_EX,
}}
STRUCT!{struct RTL_DEBUG_INFORMATION {
    SectionHandleClient: HANDLE,
    ViewBaseClient: PVOID,
    ViewBaseTarget: PVOID,
    ViewBaseDelta: ULONG_PTR,
    EventPairClient: HANDLE,
    EventPairTarget: HANDLE,
    TargetProcessId: HANDLE,
    TargetThreadHandle: HANDLE,
    Flags: ULONG,
    OffsetFree: SIZE_T,
    CommitSize: SIZE_T,
    ViewSize: SIZE_T,
    u: RTL_DEBUG_INFORMATION_u,
    BackTraces: *mut RTL_PROCESS_BACKTRACES,
    Heaps: *mut RTL_PROCESS_HEAPS,
    Locks: *mut RTL_PROCESS_LOCKS,
    SpecificHeap: PVOID,
    TargetProcessHandle: HANDLE,
    VerifierOptions: PRTL_PROCESS_VERIFIER_OPTIONS,
    ProcessHeap: PVOID,
    CriticalSectionHandle: HANDLE,
    CriticalSectionOwnerThread: HANDLE,
    Reserved: [PVOID; 4],
}}
pub type PRTL_DEBUG_INFORMATION = *mut RTL_DEBUG_INFORMATION;
EXTERN!{extern "system" {
    fn RtlCreateQueryDebugBuffer(
        MaximumCommit: ULONG,
        UseEventPair: BOOLEAN,
    ) -> PRTL_DEBUG_INFORMATION;
    fn RtlDestroyQueryDebugBuffer(
        Buffer: PRTL_DEBUG_INFORMATION,
    ) -> NTSTATUS;
    fn RtlCommitDebugInfo(
        Buffer: PRTL_DEBUG_INFORMATION,
        Size: SIZE_T,
    ) -> PVOID;
    fn RtlDeCommitDebugInfo(
        Buffer: PRTL_DEBUG_INFORMATION,
        p: PVOID,
        Size: SIZE_T,
    );
}}
pub const RTL_QUERY_PROCESS_MODULES: ULONG = 0x00000001;
pub const RTL_QUERY_PROCESS_BACKTRACES: ULONG = 0x00000002;
pub const RTL_QUERY_PROCESS_HEAP_SUMMARY: ULONG = 0x00000004;
pub const RTL_QUERY_PROCESS_HEAP_TAGS: ULONG = 0x00000008;
pub const RTL_QUERY_PROCESS_HEAP_ENTRIES: ULONG = 0x00000010;
pub const RTL_QUERY_PROCESS_LOCKS: ULONG = 0x00000020;
pub const RTL_QUERY_PROCESS_MODULES32: ULONG = 0x00000040;
pub const RTL_QUERY_PROCESS_VERIFIER_OPTIONS: ULONG = 0x00000080;
pub const RTL_QUERY_PROCESS_MODULESEX: ULONG = 0x00000100;
pub const RTL_QUERY_PROCESS_HEAP_ENTRIES_EX: ULONG = 0x00000200;
pub const RTL_QUERY_PROCESS_CS_OWNER: ULONG = 0x00000400;
pub const RTL_QUERY_PROCESS_NONINVASIVE: ULONG = 0x80000000;
EXTERN!{extern "system" {
    fn RtlQueryProcessDebugInformation(
        UniqueProcessId: HANDLE,
        Flags: ULONG,
        Buffer: PRTL_DEBUG_INFORMATION,
    ) -> NTSTATUS;
    fn RtlFindMessage(
        DllHandle: PVOID,
        MessageTableId: ULONG,
        MessageLanguageId: ULONG,
        MessageId: ULONG,
        MessageEntry: *mut PMESSAGE_RESOURCE_ENTRY,
    ) -> NTSTATUS;
    fn RtlFormatMessage(
        MessageFormat: PWSTR,
        MaximumWidth: ULONG,
        IgnoreInserts: BOOLEAN,
        ArgumentsAreAnsi: BOOLEAN,
        ArgumentsAreAnArray: BOOLEAN,
        Arguments: *mut va_list,
        Buffer: PWSTR,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
STRUCT!{struct PARSE_MESSAGE_CONTEXT {
    fFlags: ULONG,
    cwSavColumn: ULONG,
    iwSrc: SIZE_T,
    iwDst: SIZE_T,
    iwDstSpace: SIZE_T,
    lpvArgStart: va_list,
}}
pub type PPARSE_MESSAGE_CONTEXT = *mut PARSE_MESSAGE_CONTEXT;
#[inline]
pub fn INIT_PARSE_MESSAGE_CONTEXT(ctx: &mut PARSE_MESSAGE_CONTEXT) {
    ctx.fFlags = 0;
}
#[inline]
pub fn TEST_PARSE_MESSAGE_CONTEXT_FLAG(ctx: &mut PARSE_MESSAGE_CONTEXT, flag: ULONG) -> ULONG {
    ctx.fFlags & flag
}
#[inline]
pub fn SET_PARSE_MESSAGE_CONTEXT_FLAG(ctx: &mut PARSE_MESSAGE_CONTEXT, flag: ULONG) -> ULONG {
    ctx.fFlags |= flag;
    ctx.fFlags
}
#[inline]
pub fn CLEAR_PARSE_MESSAGE_CONTEXT_FLAG(ctx: &mut PARSE_MESSAGE_CONTEXT, flag: ULONG) -> ULONG {
    ctx.fFlags &= !flag;
    ctx.fFlags
}
EXTERN!{extern "system" {
    fn RtlFormatMessageEx(
        MessageFormat: PWSTR,
        MaximumWidth: ULONG,
        IgnoreInserts: BOOLEAN,
        ArgumentsAreAnsi: BOOLEAN,
        ArgumentsAreAnArray: BOOLEAN,
        Arguments: *mut va_list,
        Buffer: PWSTR,
        Length: ULONG,
        ReturnLength: PULONG,
        ParseContext: PPARSE_MESSAGE_CONTEXT,
    ) -> NTSTATUS;
    fn RtlNtStatusToDosError(
        Status: NTSTATUS,
    ) -> ULONG;
    fn RtlNtStatusToDosErrorNoTeb(
        Status: NTSTATUS,
    ) -> ULONG;
    fn RtlGetLastNtStatus() -> NTSTATUS;
    fn RtlGetLastWin32Error() -> LONG;
    fn RtlSetLastWin32ErrorAndNtStatusFromNtStatus(
        Status: NTSTATUS,
    );
    fn RtlSetLastWin32Error(
        Win32Error: LONG,
    );
    fn RtlRestoreLastWin32Error(
        Win32Error: LONG,
    );
}}
pub const RTL_ERRORMODE_FAILCRITICALERRORS: ULONG = 0x0010;
pub const RTL_ERRORMODE_NOGPFAULTERRORBOX: ULONG = 0x0020;
pub const RTL_ERRORMODE_NOOPENFILEERRORBOX: ULONG = 0x0040;
EXTERN!{extern "system" {
    fn RtlGetThreadErrorMode() -> ULONG;
    fn RtlSetThreadErrorMode(
        NewMode: ULONG,
        OldMode: PULONG,
    ) -> NTSTATUS;
    fn RtlReportException(
        ExceptionRecord: PEXCEPTION_RECORD,
        ContextRecord: PCONTEXT,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlReportExceptionEx(
        ExceptionRecord: PEXCEPTION_RECORD,
        ContextRecord: PCONTEXT,
        Flags: ULONG,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlWerpReportException(
        ProcessId: ULONG,
        CrashReportSharedMem: HANDLE,
        Flags: ULONG,
        CrashVerticalProcessHandle: PHANDLE,
    ) -> NTSTATUS;
    fn RtlReportSilentProcessExit(
        ProcessHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn RtlUniform(
        Seed: PULONG,
    ) -> ULONG;
    fn RtlRandom(
        Seed: PULONG,
    ) -> ULONG;
    fn RtlRandomEx(
        Seed: PULONG,
    ) -> ULONG;
    fn RtlComputeImportTableHash(
        FileHandle: HANDLE,
        Hash: PCHAR,
        ImportTableHashRevision: ULONG,
    ) -> NTSTATUS;
    fn RtlIntegerToChar(
        Value: ULONG,
        Base: ULONG,
        OutputLength: LONG,
        String: PSTR,
    ) -> NTSTATUS;
    fn RtlCharToInteger(
        String: PCSZ,
        Base: ULONG,
        Value: PULONG,
    ) -> NTSTATUS;
    fn RtlLargeIntegerToChar(
        Value: PLARGE_INTEGER,
        Base: ULONG,
        OutputLength: LONG,
        String: PSTR,
    ) -> NTSTATUS;
    fn RtlIntegerToUnicodeString(
        Value: ULONG,
        Base: ULONG,
        String: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlInt64ToUnicodeString(
        Value: ULONGLONG,
        Base: ULONG,
        String: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlUnicodeStringToInteger(
        String: PCUNICODE_STRING,
        Base: ULONG,
        Value: PULONG,
    ) -> NTSTATUS;
    fn RtlIpv4AddressToStringExW(
        Address: *const in_addr,
        Port: USHORT,
        AddressString: PWSTR,
        AddressStringLength: PULONG,
    ) -> NTSTATUS;
    fn RtlIpv6AddressToStringExW(
        Address: *const in6_addr,
        ScopeId: ULONG,
        Port: USHORT,
        AddressString: PWSTR,
        AddressStringLength: PULONG,
    ) -> NTSTATUS;
    fn RtlIpv4StringToAddressExW(
        AddressString: PCWSTR,
        Strict: BOOLEAN,
        Address: *mut in_addr,
        Port: PUSHORT,
    ) -> NTSTATUS;
    fn RtlIpv6StringToAddressExW(
        AddressString: PCWSTR,
        Address: *mut in6_addr,
        ScopeId: PULONG,
        Port: PUSHORT,
    ) -> NTSTATUS;
}}
STRUCT!{struct TIME_FIELDS {
    Year: CSHORT,
    Month: CSHORT,
    Day: CSHORT,
    Hour: CSHORT,
    Minute: CSHORT,
    Second: CSHORT,
    Milliseconds: CSHORT,
    Weekday: CSHORT,
}}
pub type PTIME_FIELDS = *mut TIME_FIELDS;
EXTERN!{extern "system" {
    fn RtlCutoverTimeToSystemTime(
        CutoverTime: PTIME_FIELDS,
        SystemTime: PLARGE_INTEGER,
        CurrentSystemTime: PLARGE_INTEGER,
        ThisYear: BOOLEAN,
    ) -> BOOLEAN;
    fn RtlSystemTimeToLocalTime(
        SystemTime: PLARGE_INTEGER,
        LocalTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlLocalTimeToSystemTime(
        LocalTime: PLARGE_INTEGER,
        SystemTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlTimeToElapsedTimeFields(
        Time: PLARGE_INTEGER,
        TimeFields: PTIME_FIELDS,
    );
    fn RtlTimeToTimeFields(
        Time: PLARGE_INTEGER,
        TimeFields: PTIME_FIELDS,
    );
    fn RtlTimeFieldsToTime(
        TimeFields: PTIME_FIELDS,
        Time: PLARGE_INTEGER,
    ) -> BOOLEAN;
    fn RtlTimeToSecondsSince1980(
        Time: PLARGE_INTEGER,
        ElapsedSeconds: PULONG,
    ) -> BOOLEAN;
    fn RtlSecondsSince1980ToTime(
        ElapsedSeconds: ULONG,
        Time: PLARGE_INTEGER,
    );
    fn RtlTimeToSecondsSince1970(
        Time: PLARGE_INTEGER,
        ElapsedSeconds: PULONG,
    ) -> BOOLEAN;
    fn RtlSecondsSince1970ToTime(
        ElapsedSeconds: ULONG,
        Time: PLARGE_INTEGER,
    );
}}
STRUCT!{struct RTL_TIME_ZONE_INFORMATION {
    Bias: LONG,
    StandardName: [WCHAR; 32],
    StandardStart: TIME_FIELDS,
    StandardBias: LONG,
    DaylightName: [WCHAR; 32],
    DaylightStart: TIME_FIELDS,
    DaylightBias: LONG,
}}
pub type PRTL_TIME_ZONE_INFORMATION = *mut RTL_TIME_ZONE_INFORMATION;
EXTERN!{extern "system" {
    fn RtlQueryTimeZoneInformation(
        TimeZoneInformation: PRTL_TIME_ZONE_INFORMATION,
    ) -> NTSTATUS;
    fn RtlSetTimeZoneInformation(
        TimeZoneInformation: PRTL_TIME_ZONE_INFORMATION,
    ) -> NTSTATUS;
}}
STRUCT!{struct RTL_BITMAP {
    SizeOfBitMap: ULONG,
    Buffer: PULONG,
}}
pub type PRTL_BITMAP = *mut RTL_BITMAP;
EXTERN!{extern "system" {
    fn RtlInitializeBitMap(
        BitMapHeader: PRTL_BITMAP,
        BitMapBuffer: PULONG,
        SizeOfBitMap: ULONG,
    );
    fn RtlClearBit(
        BitMapHeader: PRTL_BITMAP,
        BitNumber: ULONG,
    );
    fn RtlSetBit(
        BitMapHeader: PRTL_BITMAP,
        BitNumber: ULONG,
    );
    fn RtlTestBit(
        BitMapHeader: PRTL_BITMAP,
        BitNumber: ULONG,
    ) -> BOOLEAN;
    fn RtlClearAllBits(
        BitMapHeader: PRTL_BITMAP,
    );
    fn RtlSetAllBits(
        BitMapHeader: PRTL_BITMAP,
    );
    fn RtlFindClearBits(
        BitMapHeader: PRTL_BITMAP,
        NumberToFind: ULONG,
        HintIndex: ULONG,
    ) -> ULONG;
    fn RtlFindSetBits(
        BitMapHeader: PRTL_BITMAP,
        NumberToFind: ULONG,
        HintIndex: ULONG,
    ) -> ULONG;
    fn RtlFindClearBitsAndSet(
        BitMapHeader: PRTL_BITMAP,
        NumberToFind: ULONG,
        HintIndex: ULONG,
    ) -> ULONG;
    fn RtlFindSetBitsAndClear(
        BitMapHeader: PRTL_BITMAP,
        NumberToFind: ULONG,
        HintIndex: ULONG,
    ) -> ULONG;
    fn RtlClearBits(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: ULONG,
        NumberToClear: ULONG,
    );
    fn RtlSetBits(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: ULONG,
        NumberToSet: ULONG,
    );
    fn RtlFindMostSignificantBit(
        Set: ULONGLONG,
    ) -> CCHAR;
    fn RtlFindLeastSignificantBit(
        Set: ULONGLONG,
    ) -> CCHAR;
}}
STRUCT!{struct RTL_BITMAP_RUN {
    StartingIndex: ULONG,
    NumberOfBits: ULONG,
}}
pub type PRTL_BITMAP_RUN = *mut RTL_BITMAP_RUN;
EXTERN!{extern "system" {
    fn RtlFindClearRuns(
        BitMapHeader: PRTL_BITMAP,
        RunArray: PRTL_BITMAP_RUN,
        SizeOfRunArray: ULONG,
        LocateLongestRuns: BOOLEAN,
    ) -> ULONG;
    fn RtlFindLongestRunClear(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: PULONG,
    ) -> ULONG;
    fn RtlFindFirstRunClear(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: PULONG,
    ) -> ULONG;
}}
#[inline]
pub unsafe fn RtlCheckBit(BitMapHeader: &RTL_BITMAP, BitPosition: ULONG) -> u8 {
    #[cfg(target_arch = "x86_64")] {
        core::arch::x86_64::_bittest64(BitMapHeader.Buffer as *const i64, BitPosition as i64)
    }
    #[cfg(any(target_arch = "x86", target_arch = "aarch64"))] {
        (*BitMapHeader.Buffer.offset(BitPosition as isize / 32) >> (BitPosition % 32) & 1) as u8
    }
}
EXTERN!{extern "system" {
    fn RtlNumberOfClearBits(
        BitMapHeader: PRTL_BITMAP,
    ) -> ULONG;
    fn RtlNumberOfSetBits(
        BitMapHeader: PRTL_BITMAP,
    ) -> ULONG;
    fn RtlAreBitsClear(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: ULONG,
        Length: ULONG,
    ) -> BOOLEAN;
    fn RtlAreBitsSet(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: ULONG,
        Length: ULONG,
    ) -> BOOLEAN;
    fn RtlFindNextForwardRunClear(
        BitMapHeader: PRTL_BITMAP,
        FromIndex: ULONG,
        StartingRunIndex: PULONG,
    ) -> ULONG;
    fn RtlFindLastBackwardRunClear(
        BitMapHeader: PRTL_BITMAP,
        FromIndex: ULONG,
        StartingRunIndex: PULONG,
    ) -> ULONG;
    fn RtlNumberOfSetBitsUlongPtr(
        Target: ULONG_PTR,
    ) -> ULONG;
    fn RtlInterlockedClearBitRun(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: ULONG,
        NumberToClear: ULONG,
    );
    fn RtlInterlockedSetBitRun(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: ULONG,
        NumberToSet: ULONG,
    );
    fn RtlCopyBitMap(
        Source: PRTL_BITMAP,
        Destination: PRTL_BITMAP,
        TargetBit: ULONG,
    );
    fn RtlExtractBitMap(
        Source: PRTL_BITMAP,
        Destination: PRTL_BITMAP,
        TargetBit: ULONG,
        NumberOfBits: ULONG,
    );
    fn RtlNumberOfClearBitsInRange(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: ULONG,
        Length: ULONG,
    ) -> ULONG;
    fn RtlNumberOfSetBitsInRange(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: ULONG,
        Length: ULONG,
    ) -> ULONG;
}}
STRUCT!{struct RTL_BITMAP_EX {
    SizeOfBitMap: ULONG64,
    Buffer: PULONG64,
}}
pub type PRTL_BITMAP_EX = *mut RTL_BITMAP_EX;
EXTERN!{extern "system" {
    fn RtlInitializeBitMapEx(
        BitMapHeader: PRTL_BITMAP_EX,
        BitMapBuffer: PULONG64,
        SizeOfBitMap: ULONG64,
    );
    fn RtlTestBitEx(
        BitMapHeader: PRTL_BITMAP_EX,
        BitNumber: ULONG64,
    ) -> BOOLEAN;
    fn RtlClearAllBitsEx(
        BitMapHeader: PRTL_BITMAP_EX,
    );
    fn RtlClearBitEx(
        BitMapHeader: PRTL_BITMAP_EX,
        BitNumber: ULONG64,
    );
    fn RtlSetBitEx(
        BitMapHeader: PRTL_BITMAP_EX,
        BitNumber: ULONG64,
    );
    fn RtlFindSetBitsEx(
        BitMapHeader: PRTL_BITMAP_EX,
        NumberToFind: ULONG64,
        HintIndex: ULONG64,
    ) -> ULONG64;
    fn RtlFindSetBitsAndClearEx(
        BitMapHeader: PRTL_BITMAP_EX,
        NumberToFind: ULONG64,
        HintIndex: ULONG64,
    ) -> ULONG64;
}}
UNION!{union RTL_HANDLE_TABLE_ENTRY {
    Flags: ULONG,
    NextFree: *mut RTL_HANDLE_TABLE_ENTRY,
}}
pub type PRTL_HANDLE_TABLE_ENTRY = *mut RTL_HANDLE_TABLE_ENTRY;
pub const RTL_HANDLE_ALLOCATED: USHORT = 0x0001;
STRUCT!{struct RTL_HANDLE_TABLE {
    MaximumNumberOfHandles: ULONG,
    SizeOfHandleTableEntry: ULONG,
    Reserved: [ULONG; 2],
    FreeHandles: PRTL_HANDLE_TABLE_ENTRY,
    CommittedHandles: PRTL_HANDLE_TABLE_ENTRY,
    UnCommittedHandles: PRTL_HANDLE_TABLE_ENTRY,
    MaxReservedHandles: PRTL_HANDLE_TABLE_ENTRY,
}}
pub type PRTL_HANDLE_TABLE = *mut RTL_HANDLE_TABLE;
EXTERN!{extern "system" {
    fn RtlInitializeHandleTable(
        MaximumNumberOfHandles: ULONG,
        SizeOfHandleTableEntry: ULONG,
        HandleTable: PRTL_HANDLE_TABLE,
    );
    fn RtlDestroyHandleTable(
        HandleTable: PRTL_HANDLE_TABLE,
    ) -> NTSTATUS;
    fn RtlAllocateHandle(
        HandleTable: PRTL_HANDLE_TABLE,
        HandleIndex: PULONG,
    ) -> PRTL_HANDLE_TABLE_ENTRY;
    fn RtlFreeHandle(
        HandleTable: PRTL_HANDLE_TABLE,
        Handle: PRTL_HANDLE_TABLE_ENTRY,
    ) -> BOOLEAN;
    fn RtlIsValidHandle(
        HandleTable: PRTL_HANDLE_TABLE,
        Handle: PRTL_HANDLE_TABLE_ENTRY,
    ) -> BOOLEAN;
    fn RtlIsValidIndexHandle(
        HandleTable: PRTL_HANDLE_TABLE,
        HandleIndex: ULONG,
        Handle: *mut PRTL_HANDLE_TABLE_ENTRY,
    ) -> BOOLEAN;
}}
pub const RTL_ATOM_MAXIMUM_INTEGER_ATOM: RTL_ATOM = 0xc000;
pub const RTL_ATOM_INVALID_ATOM: RTL_ATOM = 0x0000;
pub const RTL_ATOM_TABLE_DEFAULT_NUMBER_OF_BUCKETS: u32 = 37;
pub const RTL_ATOM_MAXIMUM_NAME_LENGTH: u32 = 255;
pub const RTL_ATOM_PINNED: u32 = 0x01;
EXTERN!{extern "system" {
    fn RtlCreateAtomTable(
        NumberOfBuckets: ULONG,
        AtomTableHandle: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlDestroyAtomTable(
        AtomTableHandle: PVOID,
    ) -> NTSTATUS;
    fn RtlEmptyAtomTable(
        AtomTableHandle: PVOID,
        IncludePinnedAtoms: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlAddAtomToAtomTable(
        AtomTableHandle: PVOID,
        AtomName: PWSTR,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn RtlLookupAtomInAtomTable(
        AtomTableHandle: PVOID,
        AtomName: PWSTR,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn RtlDeleteAtomFromAtomTable(
        AtomTableHandle: PVOID,
        Atom: RTL_ATOM,
    ) -> NTSTATUS;
    fn RtlPinAtomInAtomTable(
        AtomTableHandle: PVOID,
        Atom: RTL_ATOM,
    ) -> NTSTATUS;
    fn RtlQueryAtomInAtomTable(
        AtomTableHandle: PVOID,
        Atom: RTL_ATOM,
        AtomUsage: PULONG,
        AtomFlags: PULONG,
        AtomName: PWSTR,
        AtomNameLength: PULONG,
    ) -> NTSTATUS;
    fn RtlGetIntegerAtom(
        AtomName: PWSTR,
        IntegerAtom: PUSHORT,
    ) -> BOOLEAN;
    fn RtlValidSid(
        Sid: PSID,
    ) -> BOOLEAN;
    fn RtlEqualSid(
        Sid1: PSID,
        Sid2: PSID,
    ) -> BOOLEAN;
    fn RtlEqualPrefixSid(
        Sid1: PSID,
        Sid2: PSID,
    ) -> BOOLEAN;
    fn RtlLengthRequiredSid(
        SubAuthorityCount: ULONG,
    ) -> ULONG;
    fn RtlFreeSid(
        Sid: PSID,
    ) -> PVOID;
    fn RtlAllocateAndInitializeSid(
        IdentifierAuthority: PSID_IDENTIFIER_AUTHORITY,
        SubAuthorityCount: UCHAR,
        SubAuthority0: ULONG,
        SubAuthority1: ULONG,
        SubAuthority2: ULONG,
        SubAuthority3: ULONG,
        SubAuthority4: ULONG,
        SubAuthority5: ULONG,
        SubAuthority6: ULONG,
        SubAuthority7: ULONG,
        Sid: *mut PSID,
    ) -> NTSTATUS;
    fn RtlInitializeSid(
        Sid: PSID,
        IdentifierAuthority: PSID_IDENTIFIER_AUTHORITY,
        SubAuthorityCount: UCHAR,
    ) -> NTSTATUS;
}}
EXTERN!{extern "C" {
    fn RtlInitializeSidEx(
        Sid: PSID,
        IdentifierAuthority: PSID_IDENTIFIER_AUTHORITY,
        SubAuthorityCount: UCHAR,
        ...
    ) -> NTSTATUS;
}}
EXTERN!{extern "system" {
    fn RtlIdentifierAuthoritySid(
        Sid: PSID,
    ) -> PSID_IDENTIFIER_AUTHORITY;
    fn RtlSubAuthoritySid(
        Sid: PSID,
        SubAuthority: ULONG,
    ) -> PULONG;
    fn RtlSubAuthorityCountSid(
        Sid: PSID,
    ) -> PUCHAR;
    fn RtlLengthSid(
        Sid: PSID,
    ) -> ULONG;
    fn RtlCopySid(
        DestinationSidLength: ULONG,
        DestinationSid: PSID,
        SourceSid: PSID,
    ) -> NTSTATUS;
    fn RtlCopySidAndAttributesArray(
        Count: ULONG,
        Src: PSID_AND_ATTRIBUTES,
        SidAreaSize: ULONG,
        Dest: PSID_AND_ATTRIBUTES,
        SidArea: PSID,
        RemainingSidArea: *mut PSID,
        RemainingSidAreaSize: PULONG,
    ) -> NTSTATUS;
    fn RtlCreateServiceSid(
        ServiceName: PUNICODE_STRING,
        ServiceSid: PSID,
        ServiceSidLength: PULONG,
    ) -> NTSTATUS;
    fn RtlSidDominates(
        Sid1: PSID,
        Sid2: PSID,
        Dominates: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlSidDominatesForTrust(
        Sid1: PSID,
        Sid2: PSID,
        DominatesTrust: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlSidEqualLevel(
        Sid1: PSID,
        Sid2: PSID,
        EqualLevel: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlSidIsHigherLevel(
        Sid1: PSID,
        Sid2: PSID,
        HigherLevel: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlCreateVirtualAccountSid(
        Name: PCUNICODE_STRING,
        BaseSubAuthority: ULONG,
        Sid: PSID,
        SidLength: PULONG,
    ) -> NTSTATUS;
    fn RtlReplaceSidInSd(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        OldSid: PSID,
        NewSid: PSID,
        NumChanges: *mut ULONG,
    ) -> NTSTATUS;
}}
pub const MAX_UNICODE_STACK_BUFFER_LENGTH: usize = 256;
EXTERN!{extern "system" {
    fn RtlConvertSidToUnicodeString(
        UnicodeString: PUNICODE_STRING,
        Sid: PSID,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlSidHashInitialize(
        SidAttr: PSID_AND_ATTRIBUTES,
        SidCount: ULONG,
        SidAttrHash: PSID_AND_ATTRIBUTES_HASH,
    ) -> NTSTATUS;
    fn RtlSidHashLookup(
        SidAttrHash: PSID_AND_ATTRIBUTES_HASH,
        Sid: PSID,
    ) -> PSID_AND_ATTRIBUTES;
    fn RtlIsElevatedRid(
        SidAttr: PSID_AND_ATTRIBUTES,
    ) -> BOOLEAN;
    fn RtlDeriveCapabilitySidsFromName(
        UnicodeString: PUNICODE_STRING,
        CapabilityGroupSid: PSID,
        CapabilitySid: PSID,
    ) -> NTSTATUS;
    fn RtlCreateSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Revision: ULONG,
    ) -> NTSTATUS;
    fn RtlValidSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> BOOLEAN;
    fn RtlLengthSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> ULONG;
    fn RtlValidRelativeSecurityDescriptor(
        SecurityDescriptorInput: PSECURITY_DESCRIPTOR,
        SecurityDescriptorLength: ULONG,
        RequiredInformation: SECURITY_INFORMATION,
    ) -> BOOLEAN;
    fn RtlGetControlSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Control: PSECURITY_DESCRIPTOR_CONTROL,
        Revision: PULONG,
    ) -> NTSTATUS;
    fn RtlSetControlSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ControlBitsOfInterest: SECURITY_DESCRIPTOR_CONTROL,
        ControlBitsToSet: SECURITY_DESCRIPTOR_CONTROL,
    ) -> NTSTATUS;
    fn RtlSetAttributesSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Control: SECURITY_DESCRIPTOR_CONTROL,
        Revision: PULONG,
    ) -> NTSTATUS;
    fn RtlGetSecurityDescriptorRMControl(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        RMControl: PUCHAR,
    ) -> BOOLEAN;
    fn RtlSetSecurityDescriptorRMControl(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        RMControl: PUCHAR,
    );
    fn RtlSetDaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DaclPresent: BOOLEAN,
        Dacl: PACL,
        DaclDefaulted: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlGetDaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DaclPresent: PBOOLEAN,
        Dacl: *mut PACL,
        DaclDefaulted: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlSetSaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        SaclPresent: BOOLEAN,
        Sacl: PACL,
        SaclDefaulted: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlGetSaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        SaclPresent: PBOOLEAN,
        Sacl: *mut PACL,
        SaclDefaulted: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlSetOwnerSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Owner: PSID,
        OwnerDefaulted: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlGetOwnerSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Owner: *mut PSID,
        OwnerDefaulted: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlSetGroupSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Group: PSID,
        GroupDefaulted: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlGetGroupSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Group: *mut PSID,
        GroupDefaulted: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlMakeSelfRelativeSD(
        AbsoluteSecurityDescriptor: PSECURITY_DESCRIPTOR,
        SelfRelativeSecurityDescriptor: PSECURITY_DESCRIPTOR,
        BufferLength: PULONG,
    ) -> NTSTATUS;
    fn RtlAbsoluteToSelfRelativeSD(
        AbsoluteSecurityDescriptor: PSECURITY_DESCRIPTOR,
        SelfRelativeSecurityDescriptor: PSECURITY_DESCRIPTOR,
        BufferLength: PULONG,
    ) -> NTSTATUS;
    fn RtlSelfRelativeToAbsoluteSD(
        SelfRelativeSecurityDescriptor: PSECURITY_DESCRIPTOR,
        AbsoluteSecurityDescriptor: PSECURITY_DESCRIPTOR,
        AbsoluteSecurityDescriptorSize: PULONG,
        Dacl: PACL,
        DaclSize: PULONG,
        Sacl: PACL,
        SaclSize: PULONG,
        Owner: PSID,
        OwnerSize: PULONG,
        PrimaryGroup: PSID,
        PrimaryGroupSize: PULONG,
    ) -> NTSTATUS;
    fn RtlSelfRelativeToAbsoluteSD2(
        pSelfRelativeSecurityDescriptor: PSECURITY_DESCRIPTOR,
        pBufferSize: PULONG,
    ) -> NTSTATUS;
    fn RtlAreAllAccessesGranted(
        GrantedAccess: ACCESS_MASK,
        DesiredAccess: ACCESS_MASK,
    ) -> BOOLEAN;
    fn RtlAreAnyAccessesGranted(
        GrantedAccess: ACCESS_MASK,
        DesiredAccess: ACCESS_MASK,
    ) -> BOOLEAN;
    fn RtlMapGenericMask(
        AccessMask: PACCESS_MASK,
        GenericMapping: PGENERIC_MAPPING,
    );
    fn RtlCreateAcl(
        Acl: PACL,
        AclLength: ULONG,
        AclRevision: ULONG,
    ) -> NTSTATUS;
    fn RtlValidAcl(
        Acl: PACL,
    ) -> BOOLEAN;
    fn RtlQueryInformationAcl(
        Acl: PACL,
        AclInformation: PVOID,
        AclInformationLength: ULONG,
        AclInformationClass: ACL_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn RtlSetInformationAcl(
        Acl: PACL,
        AclInformation: PVOID,
        AclInformationLength: ULONG,
        AclInformationClass: ACL_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn RtlAddAce(
        Acl: PACL,
        AceRevision: ULONG,
        StartingAceIndex: ULONG,
        AceList: PVOID,
        AceListLength: ULONG,
    ) -> NTSTATUS;
    fn RtlDeleteAce(
        Acl: PACL,
        AceIndex: ULONG,
    ) -> NTSTATUS;
    fn RtlGetAce(
        Acl: PACL,
        AceIndex: ULONG,
        Ace: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlFirstFreeAce(
        Acl: PACL,
        FirstFree: *mut PVOID,
    ) -> BOOLEAN;
    fn RtlFindAceByType(
        pAcl: PACL,
        AceType: UCHAR,
        pIndex: PULONG,
    ) -> PVOID;
    fn RtlOwnerAcesPresent(
        pAcl: PACL,
    ) -> BOOLEAN;
    fn RtlAddAccessAllowedAce(
        Acl: PACL,
        AceRevision: ULONG,
        AccessMask: ACCESS_MASK,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAccessAllowedAceEx(
        Acl: PACL,
        AceRevision: ULONG,
        AceFlags: ULONG,
        AccessMask: ACCESS_MASK,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAccessDeniedAce(
        Acl: PACL,
        AceRevision: ULONG,
        AccessMask: ACCESS_MASK,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAccessDeniedAceEx(
        Acl: PACL,
        AceRevision: ULONG,
        AceFlags: ULONG,
        AccessMask: ACCESS_MASK,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAuditAccessAce(
        Acl: PACL,
        AceRevision: ULONG,
        AccessMask: ACCESS_MASK,
        Sid: PSID,
        AuditSuccess: BOOLEAN,
        AuditFailure: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlAddAuditAccessAceEx(
        Acl: PACL,
        AceRevision: ULONG,
        AceFlags: ULONG,
        AccessMask: ACCESS_MASK,
        Sid: PSID,
        AuditSuccess: BOOLEAN,
        AuditFailure: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlAddAccessAllowedObjectAce(
        Acl: PACL,
        AceRevision: ULONG,
        AceFlags: ULONG,
        AccessMask: ACCESS_MASK,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAccessDeniedObjectAce(
        Acl: PACL,
        AceRevision: ULONG,
        AceFlags: ULONG,
        AccessMask: ACCESS_MASK,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAuditAccessObjectAce(
        Acl: PACL,
        AceRevision: ULONG,
        AceFlags: ULONG,
        AccessMask: ACCESS_MASK,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
        AuditSuccess: BOOLEAN,
        AuditFailure: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlAddCompoundAce(
        Acl: PACL,
        AceRevision: ULONG,
        AceType: UCHAR,
        AccessMask: ACCESS_MASK,
        ServerSid: PSID,
        ClientSid: PSID,
    ) -> NTSTATUS;
    fn RtlAddMandatoryAce(
        Acl: PACL,
        AceRevision: ULONG,
        AceFlags: ULONG,
        Sid: PSID,
        AceType: UCHAR,
        AccessMask: ACCESS_MASK,
    ) -> NTSTATUS;
    fn RtlDefaultNpAcl(
        Acl: *mut PACL,
    ) -> NTSTATUS;
    fn RtlNewSecurityObject(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        IsDirectoryObject: BOOLEAN,
        Token: HANDLE,
        GenericMapping: PGENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlNewSecurityObjectEx(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectType: *mut GUID,
        IsDirectoryObject: BOOLEAN,
        AutoInheritFlags: ULONG,
        Token: HANDLE,
        GenericMapping: PGENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlNewSecurityObjectWithMultipleInheritance(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectType: *mut *mut GUID,
        GuidCount: ULONG,
        IsDirectoryObject: BOOLEAN,
        AutoInheritFlags: ULONG,
        Token: HANDLE,
        GenericMapping: PGENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlDeleteSecurityObject(
        ObjectDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn RtlQuerySecurityObject(
        ObjectDescriptor: PSECURITY_DESCRIPTOR,
        SecurityInformation: SECURITY_INFORMATION,
        ResultantDescriptor: PSECURITY_DESCRIPTOR,
        DescriptorLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn RtlSetSecurityObject(
        SecurityInformation: SECURITY_INFORMATION,
        ModificationDescriptor: PSECURITY_DESCRIPTOR,
        ObjectsSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        GenericMapping: PGENERIC_MAPPING,
        Token: HANDLE,
    ) -> NTSTATUS;
    fn RtlSetSecurityObjectEx(
        SecurityInformation: SECURITY_INFORMATION,
        ModificationDescriptor: PSECURITY_DESCRIPTOR,
        ObjectsSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        AutoInheritFlags: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        Token: HANDLE,
    ) -> NTSTATUS;
    fn RtlConvertToAutoInheritSecurityObject(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CurrentSecurityDescriptor: PSECURITY_DESCRIPTOR,
        NewSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectType: *mut GUID,
        IsDirectoryObject: BOOLEAN,
        GenericMapping: PGENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlNewInstanceSecurityObject(
        ParentDescriptorChanged: BOOLEAN,
        CreatorDescriptorChanged: BOOLEAN,
        OldClientTokenModifiedId: PLUID,
        NewClientTokenModifiedId: PLUID,
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        IsDirectoryObject: BOOLEAN,
        Token: HANDLE,
        GenericMapping: PGENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlCopySecurityDescriptor(
        InputSecurityDescriptor: PSECURITY_DESCRIPTOR,
        OutputSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn RtlRunEncodeUnicodeString(
        Seed: PUCHAR,
        String: PUNICODE_STRING,
    );
    fn RtlRunDecodeUnicodeString(
        Seed: UCHAR,
        String: PUNICODE_STRING,
    );
    fn RtlImpersonateSelf(
        ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    ) -> NTSTATUS;
    fn RtlImpersonateSelfEx(
        ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
        AdditionalAccess: ACCESS_MASK,
        ThreadToken: PHANDLE,
    ) -> NTSTATUS;
    fn RtlAdjustPrivilege(
        Privilege: ULONG,
        Enable: BOOLEAN,
        Client: BOOLEAN,
        WasEnabled: PBOOLEAN,
    ) -> NTSTATUS;
}}
pub const RTL_ACQUIRE_PRIVILEGE_REVERT: ULONG = 0x00000001;
pub const RTL_ACQUIRE_PRIVILEGE_PROCESS: ULONG = 0x00000002;
EXTERN!{extern "system" {
    fn RtlAcquirePrivilege(
        Privilege: PULONG,
        NumPriv: ULONG,
        Flags: ULONG,
        ReturnedState: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlReleasePrivilege(
        StatePointer: PVOID,
    );
    fn RtlRemovePrivileges(
        TokenHandle: HANDLE,
        PrivilegesToKeep: PULONG,
        PrivilegeCount: ULONG,
    ) -> NTSTATUS;
    fn RtlIsUntrustedObject(
        Handle: HANDLE,
        Object: PVOID,
        IsUntrustedObject: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlQueryValidationRunlevel(
        ComponentName: PUNICODE_STRING,
    ) -> ULONG;
    fn RtlCreateBoundaryDescriptor(
        Name: PUNICODE_STRING,
        Flags: ULONG,
    ) -> PVOID;
    fn RtlDeleteBoundaryDescriptor(
        BoundaryDescriptor: PVOID,
    );
    fn RtlAddSIDToBoundaryDescriptor(
        BoundaryDescriptor: *mut PVOID,
        RequiredSid: PSID,
    ) -> NTSTATUS;
    fn RtlAddIntegrityLabelToBoundaryDescriptor(
        BoundaryDescriptor: *mut PVOID,
        IntegrityLabel: PSID,
    ) -> NTSTATUS;
    fn RtlGetVersion(
        lpVersionInformation: PRTL_OSVERSIONINFOW,
    ) -> NTSTATUS;
    fn RtlVerifyVersionInfo(
        VersionInfo: PRTL_OSVERSIONINFOEXW,
        TypeMask: ULONG,
        ConditionMask: ULONGLONG,
    ) -> NTSTATUS;
    fn RtlGetNtVersionNumbers(
        NtMajorVersion: PULONG,
        NtMinorVersion: PULONG,
        NtBuildNumber: PULONG,
    );
    fn RtlGetNtGlobalFlags() -> ULONG;
    fn RtlGetNtProductType(
        NtProductType: PNT_PRODUCT_TYPE,
    ) -> BOOLEAN;
    fn RtlGetSuiteMask() -> ULONG;
    fn RtlRegisterWait(
        WaitHandle: PHANDLE,
        Handle: HANDLE,
        Function: WAITORTIMERCALLBACKFUNC,
        Context: PVOID,
        Milliseconds: ULONG,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlDeregisterWait(
        WaitHandle: HANDLE,
    ) -> NTSTATUS;
    fn RtlDeregisterWaitEx(
        WaitHandle: HANDLE,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn RtlQueueWorkItem(
        Function: WORKERCALLBACKFUNC,
        Context: PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlSetIoCompletionCallback(
        FileHandle: HANDLE,
        CompletionProc: APC_CALLBACK_FUNCTION,
        Flags: ULONG,
    ) -> NTSTATUS;
}}
FN!{stdcall PRTL_START_POOL_THREAD(
    Function: PTHREAD_START_ROUTINE,
    Parameter: PVOID,
    ThreadHandle: PHANDLE,
) -> NTSTATUS}
FN!{stdcall PRTL_EXIT_POOL_THREAD(
    ExitStatus: NTSTATUS,
) -> NTSTATUS}
EXTERN!{extern "system" {
    fn RtlSetThreadPoolStartFunc(
        StartPoolThread: PRTL_START_POOL_THREAD,
        ExitPoolThread: PRTL_EXIT_POOL_THREAD,
    ) -> NTSTATUS;
    fn RtlUserThreadStart(
        Function: PTHREAD_START_ROUTINE,
        Parameter: PVOID,
    );
    fn LdrInitializeThunk(
        ContextRecord: PCONTEXT,
        Parameter: PVOID,
    );
    fn RtlCreateTimerQueue(
        TimerQueueHandle: PHANDLE,
    ) -> NTSTATUS;
    fn RtlCreateTimer(
        TimerQueueHandle: HANDLE,
        Handle: PHANDLE,
        Function: WAITORTIMERCALLBACKFUNC,
        Context: PVOID,
        DueTime: ULONG,
        Period: ULONG,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn RtlUpdateTimer(
        TimerQueueHandle: HANDLE,
        TimerHandle: HANDLE,
        DueTime: ULONG,
        Period: ULONG,
    ) -> NTSTATUS;
    fn RtlDeleteTimer(
        TimerQueueHandle: HANDLE,
        TimerToCancel: HANDLE,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn RtlDeleteTimerQueue(
        TimerQueueHandle: HANDLE,
    ) -> NTSTATUS;
    fn RtlDeleteTimerQueueEx(
        TimerQueueHandle: HANDLE,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn RtlFormatCurrentUserKeyPath(
        CurrentUserKeyPath: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlOpenCurrentUser(
        DesiredAccess: ACCESS_MASK,
        CurrentUserKey: PHANDLE,
    ) -> NTSTATUS;
}}
pub const RTL_REGISTRY_ABSOLUTE: ULONG = 0;
pub const RTL_REGISTRY_SERVICES: ULONG = 1;
pub const RTL_REGISTRY_CONTROL: ULONG = 2;
pub const RTL_REGISTRY_WINDOWS_NT: ULONG = 3;
pub const RTL_REGISTRY_DEVICEMAP: ULONG = 4;
pub const RTL_REGISTRY_USER: ULONG = 5;
pub const RTL_REGISTRY_MAXIMUM: ULONG = 6;
pub const RTL_REGISTRY_HANDLE: ULONG = 0x40000000;
pub const RTL_REGISTRY_OPTIONAL: ULONG = 0x80000000;
EXTERN!{extern "system" {
    fn RtlCreateRegistryKey(
        RelativeTo: ULONG,
        Path: PWSTR,
    ) -> NTSTATUS;
    fn RtlCheckRegistryKey(
        RelativeTo: ULONG,
        Path: PWSTR,
    ) -> NTSTATUS;
}}
FN!{stdcall PRTL_QUERY_REGISTRY_ROUTINE(
    ValueName: PWSTR,
    ValueType: ULONG,
    ValueData: PVOID,
    ValueLength: ULONG,
    Context: PVOID,
    EntryContext: PVOID,
) -> NTSTATUS}
STRUCT!{struct RTL_QUERY_REGISTRY_TABLE {
    QueryRoutine: PRTL_QUERY_REGISTRY_ROUTINE,
    Flags: ULONG,
    Name: PWSTR,
    EntryContext: PVOID,
    DefaultType: ULONG,
    DefaultData: PVOID,
    DefaultLength: ULONG,
}}
pub type PRTL_QUERY_REGISTRY_TABLE = *mut RTL_QUERY_REGISTRY_TABLE;
pub const RTL_QUERY_REGISTRY_SUBKEY: ULONG = 0x00000001;
pub const RTL_QUERY_REGISTRY_TOPKEY: ULONG = 0x00000002;
pub const RTL_QUERY_REGISTRY_REQUIRED: ULONG = 0x00000004;
pub const RTL_QUERY_REGISTRY_NOVALUE: ULONG = 0x00000008;
pub const RTL_QUERY_REGISTRY_NOEXPAND: ULONG = 0x00000010;
pub const RTL_QUERY_REGISTRY_DIRECT: ULONG = 0x00000020;
pub const RTL_QUERY_REGISTRY_DELETE: ULONG = 0x00000040;
EXTERN!{extern "system" {
    fn RtlQueryRegistryValues(
        RelativeTo: ULONG,
        Path: PCWSTR,
        QueryTable: PRTL_QUERY_REGISTRY_TABLE,
        Context: PVOID,
        Environment: PVOID,
    ) -> NTSTATUS;
    fn RtlQueryRegistryValuesEx(
        RelativeTo: ULONG,
        Path: PWSTR,
        QueryTable: PRTL_QUERY_REGISTRY_TABLE,
        Context: PVOID,
        Environment: PVOID,
    ) -> NTSTATUS;
    fn RtlWriteRegistryValue(
        RelativeTo: ULONG,
        Path: PCWSTR,
        ValueName: PCWSTR,
        ValueType: ULONG,
        ValueData: PVOID,
        ValueLength: ULONG,
    ) -> NTSTATUS;
    fn RtlDeleteRegistryValue(
        RelativeTo: ULONG,
        Path: PCWSTR,
        ValueName: PCWSTR,
    ) -> NTSTATUS;
    fn RtlEnableThreadProfiling(
        ThreadHandle: HANDLE,
        Flags: ULONG,
        HardwareCounters: ULONG64,
        PerformanceDataHandle: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlDisableThreadProfiling(
        PerformanceDataHandle: PVOID,
    ) -> NTSTATUS;
    fn RtlQueryThreadProfiling(
        ThreadHandle: HANDLE,
        Enabled: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlReadThreadProfilingData(
        PerformanceDataHandle: HANDLE,
        Flags: ULONG,
        PerformanceData: PPERFORMANCE_DATA,
    ) -> NTSTATUS;
    fn RtlGetNativeSystemInformation(
        SystemInformationClass: ULONG,
        NativeSystemInformation: PVOID,
        InformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn RtlQueueApcWow64Thread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;
    fn RtlWow64EnableFsRedirection(
        Wow64FsEnableRedirection: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlWow64EnableFsRedirectionEx(
        Wow64FsEnableRedirection: PVOID,
        OldFsRedirectionLevel: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlComputeCrc32(
        PartialCrc: ULONG32,
        Buffer: PVOID,
        Length: ULONG,
    ) -> ULONG32;
    fn RtlEncodePointer(
        Ptr: PVOID,
    ) -> PVOID;
    fn RtlDecodePointer(
        Ptr: PVOID,
    ) -> PVOID;
    fn RtlEncodeSystemPointer(
        Ptr: PVOID,
    ) -> PVOID;
    fn RtlDecodeSystemPointer(
        Ptr: PVOID,
    ) -> PVOID;
    fn RtlEncodeRemotePointer(
        ProcessHandle: HANDLE,
        Pointer: PVOID,
        EncodedPointer: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlDecodeRemotePointer(
        ProcessHandle: HANDLE,
        Pointer: PVOID,
        DecodedPointer: *mut PVOID,
    ) -> NTSTATUS;
    fn RtlIsProcessorFeaturePresent(
        ProcessorFeature: ULONG,
    ) -> BOOLEAN;
    fn RtlGetCurrentProcessorNumber() -> ULONG;
    fn RtlGetCurrentProcessorNumberEx(
        ProcessorNumber: PPROCESSOR_NUMBER,
    );
    fn RtlPushFrame(
        Frame: PTEB_ACTIVE_FRAME,
    );
    fn RtlPopFrame(
        Frame: PTEB_ACTIVE_FRAME,
    );
    fn RtlGetFrame() -> PTEB_ACTIVE_FRAME;
}}
pub const RTL_WALK_USER_MODE_STACK: ULONG = 0x00000001;
pub const RTL_WALK_VALID_FLAGS: ULONG = 0x00000001;
pub const RTL_STACK_WALKING_MODE_FRAMES_TO_SKIP_SHIFT: ULONG = 0x00000008;
EXTERN!{extern "system" {
    fn RtlWalkFrameChain(
        Callers: *mut PVOID,
        Count: ULONG,
        Flags: ULONG,
    ) -> ULONG;
    fn RtlGetCallersAddress(
        CallersAddress: *mut PVOID,
        CallersCaller: *mut PVOID,
    );
    fn RtlGetEnabledExtendedFeatures(
        FeatureMask: ULONG64,
    ) -> ULONG64;
    fn RtlGetEnabledExtendedAndSupervisorFeatures(
        FeatureMask: ULONG64,
    ) -> ULONG64;
    fn RtlLocateSupervisorFeature(
        XStateHeader: PXSAVE_AREA_HEADER,
        FeatureId: ULONG,
        Length: PULONG,
    ) -> PVOID;
}}
STRUCT!{struct RTL_ELEVATION_FLAGS {
    Flags: ULONG,
}}
BITFIELD!{RTL_ELEVATION_FLAGS Flags: ULONG [
    ElevationEnabled set_ElevationEnabled[0..1],
    VirtualizationEnabled set_VirtualizationEnabled[1..2],
    InstallerDetectEnabled set_InstallerDetectEnabled[2..3],
    ReservedBits set_ReservedBits[3..32],
]}
pub type PRTL_ELEVATION_FLAGS = *mut RTL_ELEVATION_FLAGS;
EXTERN!{extern "system" {
    fn RtlQueryElevationFlags(
        Flags: PRTL_ELEVATION_FLAGS,
    ) -> NTSTATUS;
    fn RtlRegisterThreadWithCsrss() -> NTSTATUS;
    fn RtlLockCurrentThread() -> NTSTATUS;
    fn RtlUnlockCurrentThread() -> NTSTATUS;
    fn RtlLockModuleSection(
        Address: PVOID,
    ) -> NTSTATUS;
    fn RtlUnlockModuleSection(
        Address: PVOID,
    ) -> NTSTATUS;
}}
pub const RTL_UNLOAD_EVENT_TRACE_NUMBER: u32 = 64;
STRUCT!{struct RTL_UNLOAD_EVENT_TRACE {
    BaseAddress: PVOID,
    SizeOfImage: SIZE_T,
    Sequence: ULONG,
    TimeDateStamp: ULONG,
    CheckSum: ULONG,
    ImageName: [WCHAR; 32],
    Version: [ULONG; 2],
}}
pub type PRTL_UNLOAD_EVENT_TRACE = *mut RTL_UNLOAD_EVENT_TRACE;
STRUCT!{struct RTL_UNLOAD_EVENT_TRACE32 {
    BaseAddress: ULONG,
    SizeOfImage: ULONG,
    Sequence: ULONG,
    TimeDateStamp: ULONG,
    CheckSum: ULONG,
    ImageName: [WCHAR; 32],
    Version: [ULONG; 2],
}}
pub type PRTL_UNLOAD_EVENT_TRACE32 = *mut RTL_UNLOAD_EVENT_TRACE32;
EXTERN!{extern "system" {
    fn RtlGetUnloadEventTrace() -> PRTL_UNLOAD_EVENT_TRACE;
    fn RtlGetUnloadEventTraceEx(
        ElementSize: *mut PULONG,
        ElementCount: *mut PULONG,
        EventTrace: *mut PVOID,
    );
    fn RtlQueryPerformanceCounter(
        PerformanceCounter: PLARGE_INTEGER,
    ) -> LOGICAL;
    fn RtlQueryPerformanceFrequency(
        PerformanceFrequency: PLARGE_INTEGER,
    ) -> LOGICAL;
}}
ENUM!{enum IMAGE_MITIGATION_POLICY {
    ImageDepPolicy = 0,
    ImageAslrPolicy = 1,
    ImageDynamicCodePolicy = 2,
    ImageStrictHandleCheckPolicy = 3,
    ImageSystemCallDisablePolicy = 4,
    ImageMitigationOptionsMask = 5,
    ImageExtensionPointDisablePolicy = 6,
    ImageControlFlowGuardPolicy = 7,
    ImageSignaturePolicy = 8,
    ImageFontDisablePolicy = 9,
    ImageImageLoadPolicy = 10,
    ImagePayloadRestrictionPolicy = 11,
    ImageChildProcessPolicy = 12,
    ImageSehopPolicy = 13,
    ImageHeapPolicy = 14,
    MaxImageMitigationPolicy = 15,
}}
UNION!{union RTL_IMAGE_MITIGATION_POLICY {
    Bitfields1: ULONG64,
    Bitfields2: ULONG64,
}}
BITFIELD!{unsafe RTL_IMAGE_MITIGATION_POLICY Bitfields1: ULONG64 [
    AuditState set_AuditState[0..2],
    AuditFlag set_AuditFlag[2..3],
    EnableAdditionalAuditingOption set_EnableAdditionalAuditingOption[3..4],
    Reserved set_Reserved[4..64],
]}
BITFIELD!{unsafe RTL_IMAGE_MITIGATION_POLICY Bitfields2: ULONG64 [
    PolicyState set_PolicyState[0..2],
    AlwaysInherit set_AlwaysInherit[2..3],
    EnableAdditionalPolicyOption set_EnableAdditionalPolicyOption[3..4],
    AuditReserved set_AuditReserved[4..64],
]}
pub type PRTL_IMAGE_MITIGATION_POLICY = *mut RTL_IMAGE_MITIGATION_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_DEP_POLICY {
    Dep: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_DEP_POLICY = *mut RTL_IMAGE_MITIGATION_DEP_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_ASLR_POLICY {
    ForceRelocateImages: RTL_IMAGE_MITIGATION_POLICY,
    BottomUpRandomization: RTL_IMAGE_MITIGATION_POLICY,
    HighEntropyRandomization: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_ASLR_POLICY = *mut RTL_IMAGE_MITIGATION_ASLR_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY {
    BlockDynamicCode: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY = *mut RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    StrictHandleChecks: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY =
    *mut RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    BlockWin32kSystemCalls: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY =
    *mut RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    DisableExtensionPoints: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY =
    *mut RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    ControlFlowGuard: RTL_IMAGE_MITIGATION_POLICY,
    StrictControlFlowGuard: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY =
    *mut RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY {
    BlockNonMicrosoftSignedBinaries: RTL_IMAGE_MITIGATION_POLICY,
    EnforceSigningOnModuleDependencies: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY =
    *mut RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY {
    DisableNonSystemFonts: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY = *mut RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY {
    BlockRemoteImageLoads: RTL_IMAGE_MITIGATION_POLICY,
    BlockLowLabelImageLoads: RTL_IMAGE_MITIGATION_POLICY,
    PreferSystem32: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY = *mut RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    EnableExportAddressFilter: RTL_IMAGE_MITIGATION_POLICY,
    EnableExportAddressFilterPlus: RTL_IMAGE_MITIGATION_POLICY,
    EnableImportAddressFilter: RTL_IMAGE_MITIGATION_POLICY,
    EnableRopStackPivot: RTL_IMAGE_MITIGATION_POLICY,
    EnableRopCallerCheck: RTL_IMAGE_MITIGATION_POLICY,
    EnableRopSimExec: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY =
    *mut RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY {
    DisallowChildProcessCreation: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY =
    *mut RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_SEHOP_POLICY {
    Sehop: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_SEHOP_POLICY = *mut RTL_IMAGE_MITIGATION_SEHOP_POLICY;
STRUCT!{struct RTL_IMAGE_MITIGATION_HEAP_POLICY {
    TerminateOnHeapErrors: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_HEAP_POLICY = *mut RTL_IMAGE_MITIGATION_HEAP_POLICY;
ENUM!{enum RTL_IMAGE_MITIGATION_OPTION_STATE {
    RtlMitigationOptionStateNotConfigured = 0,
    RtlMitigationOptionStateOn = 1,
    RtlMitigationOptionStateOff = 2,
}}
pub const RTL_IMAGE_MITIGATION_FLAG_RESET: ULONG = 0x1;
pub const RTL_IMAGE_MITIGATION_FLAG_REMOVE: ULONG = 0x2;
pub const RTL_IMAGE_MITIGATION_FLAG_OSDEFAULT: ULONG = 0x4;
pub const RTL_IMAGE_MITIGATION_FLAG_AUDIT: ULONG = 0x8;
EXTERN!{extern "system" {
    fn RtlQueryImageMitigationPolicy(
        ImagePath: PWSTR,
        Policy: IMAGE_MITIGATION_POLICY,
        Flags: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
    ) -> NTSTATUS;
    fn RtlSetImageMitigationPolicy(
        ImagePath: PWSTR,
        Policy: IMAGE_MITIGATION_POLICY,
        Flags: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
    ) -> NTSTATUS;
    fn RtlGetCurrentServiceSessionId() -> ULONG;
    fn RtlGetActiveConsoleId() -> ULONG;
    fn RtlGetConsoleSessionForegroundProcessId() -> ULONGLONG;
    fn RtlGetTokenNamedObjectPath(
        Token: HANDLE,
        Sid: PSID,
        ObjectPath: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlGetAppContainerNamedObjectPath(
        Token: HANDLE,
        AppContainerSid: PSID,
        RelativePath: BOOLEAN,
        ObjectPath: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlGetAppContainerParent(
        AppContainerSid: PSID,
        AppContainerSidParent: *mut PSID,
    ) -> NTSTATUS;
    fn RtlCheckSandboxedToken(
        TokenHandle: HANDLE,
        IsSandboxed: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlCheckTokenCapability(
        TokenHandle: HANDLE,
        CapabilitySidToCheck: PSID,
        HasCapability: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlCapabilityCheck(
        TokenHandle: HANDLE,
        CapabilityName: PUNICODE_STRING,
        HasCapability: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlCheckTokenMembership(
        TokenHandle: HANDLE,
        SidToCheck: PSID,
        IsMember: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlCheckTokenMembershipEx(
        TokenHandle: HANDLE,
        SidToCheck: PSID,
        Flags: ULONG,
        IsMember: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlIsParentOfChildAppContainer(
        ParentAppContainerSid: PSID,
        ChildAppContainerSid: PSID,
    ) -> NTSTATUS;
    fn RtlIsCapabilitySid(
        Sid: PSID,
    ) -> BOOLEAN;
    fn RtlIsPackageSid(
        Sid: PSID,
    ) -> BOOLEAN;
    fn RtlIsValidProcessTrustLabelSid(
        Sid: PSID,
    ) -> BOOLEAN;
    fn RtlIsStateSeparationEnabled() -> BOOLEAN;
}}
ENUM!{enum APPCONTAINER_SID_TYPE {
    NotAppContainerSidType = 0,
    ChildAppContainerSidType = 1,
    ParentAppContainerSidType = 2,
    InvalidAppContainerSidType = 3,
    MaxAppContainerSidType = 4,
}}
pub type PAPPCONTAINER_SID_TYPE = *mut APPCONTAINER_SID_TYPE;
EXTERN!{extern "system" {
    fn RtlGetAppContainerSidType(
        AppContainerSid: PSID,
        AppContainerSidType: PAPPCONTAINER_SID_TYPE,
    ) -> NTSTATUS;
    fn RtlFlsAlloc(
        Callback: PFLS_CALLBACK_FUNCTION,
        FlsIndex: PULONG,
    ) -> NTSTATUS;
    fn RtlFlsFree(
        FlsIndex: ULONG,
    ) -> NTSTATUS;
}}
ENUM!{enum STATE_LOCATION_TYPE {
    LocationTypeRegistry = 0,
    LocationTypeFileSystem = 1,
    LocationTypeMaximum = 2,
}}
EXTERN!{extern "system" {
    fn RtlGetPersistedStateLocation(
        SourceID: PCWSTR,
        CustomValue: PCWSTR,
        DefaultPath: PCWSTR,
        StateLocationType: STATE_LOCATION_TYPE,
        TargetPath: PWCHAR,
        BufferLengthIn: ULONG,
        BufferLengthOut: PULONG,
    ) -> NTSTATUS;
    fn RtlIsCloudFilesPlaceholder(
        FileAttributes: ULONG,
        ReparseTag: ULONG,
    ) -> BOOLEAN;
    fn RtlIsPartialPlaceholder(
        FileAttributes: ULONG,
        ReparseTag: ULONG,
    ) -> BOOLEAN;
    fn RtlIsPartialPlaceholderFileHandle(
        FileHandle: HANDLE,
        IsPartialPlaceholder: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlIsPartialPlaceholderFileInfo(
        InfoBuffer: *const c_void,
        InfoClass: FILE_INFORMATION_CLASS,
        IsPartialPlaceholder: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlIsNonEmptyDirectoryReparsePointAllowed(
        ReparseTag: ULONG,
    ) -> BOOLEAN;
    fn RtlAppxIsFileOwnedByTrustedInstaller(
        FileHandle: HANDLE,
        IsFileOwnedByTrustedInstaller: PBOOLEAN,
    ) -> NTSTATUS;
}}
STRUCT!{struct PS_PKG_CLAIM {
    Flags: ULONGLONG,
    Origin: ULONGLONG,
}}
pub type PPS_PKG_CLAIM = *mut PS_PKG_CLAIM;
EXTERN!{extern "system" {
    fn RtlQueryPackageClaims(
        TokenHandle: HANDLE,
        PackageFullName: PWSTR,
        PackageSize: PSIZE_T,
        AppId: PWSTR,
        AppIdSize: PSIZE_T,
        DynamicId: *mut GUID,
        PkgClaim: PPS_PKG_CLAIM,
        AttributesPresent: PULONG64,
    ) -> NTSTATUS;
    fn RtlQueryProtectedPolicy(
        PolicyGuid: *mut GUID,
        PolicyValue: PULONG_PTR,
    ) -> NTSTATUS;
    fn RtlSetProtectedPolicy(
        PolicyGuid: *mut GUID,
        PolicyValue: ULONG_PTR,
        OldPolicyValue: PULONG_PTR,
    ) -> NTSTATUS;
    fn RtlIsMultiSessionSku() -> BOOLEAN;
    fn RtlIsMultiUsersInSessionSku() -> BOOLEAN;
}}
ENUM!{enum RTL_BSD_ITEM_TYPE {
    RtlBsdItemVersionNumber = 0,
    RtlBsdItemProductType = 1,
    RtlBsdItemAabEnabled = 2,
    RtlBsdItemAabTimeout = 3,
    RtlBsdItemBootGood = 4,
    RtlBsdItemBootShutdown = 5,
    RtlBsdSleepInProgress = 6,
    RtlBsdPowerTransition = 7,
    RtlBsdItemBootAttemptCount = 8,
    RtlBsdItemBootCheckpoint = 9,
    RtlBsdItemBootId = 10,
    RtlBsdItemShutdownBootId = 11,
    RtlBsdItemReportedAbnormalShutdownBootId = 12,
    RtlBsdItemErrorInfo = 13,
    RtlBsdItemPowerButtonPressInfo = 14,
    RtlBsdItemChecksum = 15,
    RtlBsdItemMax = 16,
}}
STRUCT!{struct RTL_BSD_ITEM {
    Type: RTL_BSD_ITEM_TYPE,
    DataBuffer: PVOID,
    DataLength: ULONG,
}}
pub type PRTL_BSD_ITEM = *mut RTL_BSD_ITEM;
EXTERN!{extern "system" {
    fn RtlCreateBootStatusDataFile() -> NTSTATUS;
    fn RtlLockBootStatusData(
        FileHandle: PHANDLE,
    ) -> NTSTATUS;
    fn RtlUnlockBootStatusData(
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn RtlGetSetBootStatusData(
        FileHandle: HANDLE,
        Read: BOOLEAN,
        DataClass: RTL_BSD_ITEM_TYPE,
        Buffer: PVOID,
        BufferSize: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn RtlCheckBootStatusIntegrity(
        FileHandle: HANDLE,
        Verified: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlCheckPortableOperatingSystem(
        IsPortable: PBOOLEAN,
    ) -> NTSTATUS;
    fn RtlSetPortableOperatingSystem(
        IsPortable: BOOLEAN,
    ) -> NTSTATUS;
}}
EXTERN!{extern "system" {
    fn RtlOsDeploymentState(
        Flags: DWORD,
    ) -> OS_DEPLOYEMENT_STATE_VALUES;
    fn RtlFindClosestEncodableLength(
        SourceLength: ULONGLONG,
        TargetLength: PULONGLONG,
    ) -> NTSTATUS;
}}
FN!{stdcall PRTL_SECURE_MEMORY_CACHE_CALLBACK(
    Address: PVOID,
    Length: SIZE_T,
) -> NTSTATUS}
EXTERN!{extern "system" {
    fn RtlRegisterSecureMemoryCacheCallback(
        Callback: PRTL_SECURE_MEMORY_CACHE_CALLBACK,
    ) -> NTSTATUS;
    fn RtlDeregisterSecureMemoryCacheCallback(
        Callback: PRTL_SECURE_MEMORY_CACHE_CALLBACK,
    ) -> NTSTATUS;
    fn RtlFlushSecureMemoryCache(
        MemoryCache: PVOID,
        MemoryLength: SIZE_T,
    ) -> BOOLEAN;
}}
