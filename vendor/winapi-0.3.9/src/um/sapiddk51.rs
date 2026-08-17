// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_char, c_float, c_long, c_void};
use shared::guiddef::{CLSID, GUID, IID, REFGUID};
use shared::minwindef::{BOOL, BYTE, DWORD, ULONG, USHORT, WORD};
use shared::mmreg::WAVEFORMATEX;
use shared::windef::HWND;
use um::oaidl::VARIANT;
use um::objidlbase::IStream;
use um::sapi::*;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LONG, LPCWSTR, LPWSTR, PVOID, ULONGLONG, WCHAR};
pub const SPRECOEXTENSION: &'static str = "RecoExtension";
pub const SPALTERNATESCLSID: &'static str = "AlternatesCLSID";
RIDL!{#[uuid(0xf8e690f0, 0x39cb, 0x4843, 0xb8, 0xd7, 0xc8, 0x46, 0x96, 0xe1, 0x11, 0x9d)]
interface ISpTokenUI(ISpTokenUIVtbl): IUnknown(IUnknownVtbl) {
    fn IsUISupported(
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        punkObject: *mut IUnknown,
        pfSupported: *mut BOOL,
    ) -> HRESULT,
    fn DisplayUI(
        hwndParent: HWND,
        pszTitle: LPCWSTR,
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        pToken: *mut ISpObjectToken,
        punkObject: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x06b64f9f, 0x7fda, 0x11d2, 0xb4, 0xf2, 0x00, 0xc0, 0x4f, 0x79, 0x73, 0x96)]
interface ISpObjectTokenEnumBuilder(ISpObjectTokenEnumBuilderVtbl):
    IEnumSpObjectTokens(IEnumSpObjectTokensVtbl) {
    fn SetAttribs(
        pszReqAttribs: LPCWSTR,
        pszOptAttribs: LPCWSTR,
    ) -> HRESULT,
    fn AddTokens(
        cTokens: ULONG,
        pToken: *mut *mut ISpObjectToken,
    ) -> HRESULT,
    fn AddTokensFromDataKey(
        pDataKey: *mut ISpDataKey,
        pszSubKey: LPCWSTR,
        pszCategoryId: LPCWSTR,
    ) -> HRESULT,
    fn AddTokensFromTokenEnum(
        pTokenEnum: *mut IEnumSpObjectTokens,
    ) -> HRESULT,
    fn Sort(
        pszTokenIdToListFirst: LPCWSTR,
    ) -> HRESULT,
}}
DECLARE_HANDLE!{SPWORDHANDLE, SPWORDHANDLE__}
DECLARE_HANDLE!{SPRULEHANDLE, SPRULEHANDLE__}
DECLARE_HANDLE!{SPGRAMMARHANDLE, SPGRAMMARHANDLE__}
DECLARE_HANDLE!{SPRECOCONTEXTHANDLE, SPRECOCONTEXTHANDLE__}
DECLARE_HANDLE!{SPPHRASERULEHANDLE, SPPHRASERULEHANDLE__}
DECLARE_HANDLE!{SPPHRASEPROPERTYHANDLE, SPPHRASEPROPERTYHANDLE__}
DECLARE_HANDLE!{SPTRANSITIONID, SPTRANSITIONID__}
RIDL!{#[uuid(0xf4711347, 0xe608, 0x11d2, 0xa0, 0x86, 0x00, 0xc0, 0x4f, 0x8e, 0xf9, 0xb5)]
interface ISpErrorLog(ISpErrorLogVtbl): IUnknown(IUnknownVtbl) {
    fn AddError(
        lLineNumber: c_long,
        hr: HRESULT,
        pszDescription: LPCWSTR,
        pszHelpFile: LPCWSTR,
        dwHelpContext: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb1e29d58, 0xa675, 0x11d2, 0x83, 0x02, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpGrammarCompiler(ISpGrammarCompilerVtbl): IUnknown(IUnknownVtbl) {
    fn CompileStream(
        pSource: *mut IStream,
        pDest: *mut IStream,
        pHeader: *mut IStream,
        pReserved: *mut IUnknown,
        pErrorLog: *mut ISpErrorLog,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3ddca27c, 0x665c, 0x4786, 0x9f, 0x97, 0x8c, 0x90, 0xc3, 0x48, 0x8b, 0x61)]
interface ISpGramCompBackend(ISpGramCompBackendVtbl): ISpGrammarBuilder(ISpGrammarBuilderVtbl) {
    fn SetSaveObjects(
        pStream: *mut IStream,
        pErrorLog: *mut ISpErrorLog,
    ) -> HRESULT,
    fn InitFromBinaryGrammar(
        pBinaryData: *const SPBINARYGRAMMAR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x12d7360f, 0xa1c9, 0x11d3, 0xbc, 0x90, 0x00, 0xc0, 0x4f, 0x72, 0xdf, 0x9f)]
interface ISpITNProcessor(ISpITNProcessorVtbl): IUnknown(IUnknownVtbl) {
    fn LoadITNGrammar(
        pszCLSID: LPWSTR,
    ) -> HRESULT,
    fn ITNPhrase(
        pPhrase: *mut ISpPhraseBuilder,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x88a3342a, 0x0bed, 0x4834, 0x92, 0x2b, 0x88, 0xd4, 0x31, 0x73, 0x16, 0x2f)]
interface ISpPhraseBuilder(ISpPhraseBuilderVtbl): ISpPhrase(ISpPhraseVtbl) {
    fn InitFromPhrase(
        pPhrase: *const SPPHRASE,
    ) -> HRESULT,
    fn InitFromSerializedPhrase(
        pPhrase: *const SPSERIALIZEDPHRASE,
    ) -> HRESULT,
    fn AddElements(
        cElements: ULONG,
        pElement: *const SPPHRASEELEMENT,
    ) -> HRESULT,
    fn AddRules(
        hParent: SPPHRASERULEHANDLE,
        pRule: *const SPPHRASERULE,
        phNewRule: *mut SPPHRASERULEHANDLE,
    ) -> HRESULT,
    fn AddProperties(
        hParent: SPPHRASEPROPERTYHANDLE,
        pProperty: *const SPPHRASEPROPERTY,
        phNewProperty: *mut SPPHRASEPROPERTYHANDLE,
    ) -> HRESULT,
    fn AddReplacements(
        cReplacements: ULONG,
        pReplacements: *const SPPHRASEREPLACEMENT,
    ) -> HRESULT,
}}
pub type ISpTask = *mut c_void;
pub type ISpThreadTask = *mut c_void;
RIDL!{#[uuid(0xa6be4d73, 0x4403, 0x4358, 0xb2, 0x2d, 0x03, 0x46, 0xe2, 0x3b, 0x17, 0x64)]
interface ISpThreadControl(ISpThreadControlVtbl): ISpNotifySink(ISpNotifySinkVtbl) {
    fn StartThread(
        dwFlags: DWORD,
        phwnd: *mut HWND,
    ) -> HRESULT,
    fn WaitForThreadDone(
        fForceStop: BOOL,
        phrThreadResult: *mut HRESULT,
        msTimeOut: ULONG,
    ) -> HRESULT,
    fn TerminateThread() -> HRESULT,
    fn ThreadHandle() -> HANDLE,
    fn ThreadId() -> DWORD,
    fn NotifyEvent() -> HANDLE,
    fn WindowHandle() -> HWND,
    fn ThreadCompleteEvent() -> HANDLE,
    fn ExitThreadEvent() -> HANDLE,
}}
STRUCT!{struct SPTMTHREADINFO {
    lPoolSize: c_long,
    lPriority: c_long,
    ulConcurrencyLimit: ULONG,
    ulMaxQuickAllocThreads: ULONG,
}}
RIDL!{#[uuid(0x2baeef81, 0x2ca3, 0x4331, 0x98, 0xf3, 0x26, 0xec, 0x5a, 0xbe, 0xfb, 0x03)]
interface ISpTaskManager(ISpTaskManagerVtbl): IUnknown(IUnknownVtbl) {
    fn SetThreadPoolInfo(
        pPoolInfo: *const SPTMTHREADINFO,
    ) -> HRESULT,
    fn GetThreadPoolInfo(
        pPoolInfo: *mut SPTMTHREADINFO,
    ) -> HRESULT,
    fn QueueTask(
        pTask: *mut ISpTask,
        pvTaskData: *mut c_void,
        hCompEvent: HANDLE,
        pdwGroupId: *mut DWORD,
        pTaskID: *mut DWORD,
    ) -> HRESULT,
    fn CreateReoccurringTask(
        pTask: *mut ISpTask,
        pvTaskData: *mut c_void,
        hCompEvent: HANDLE,
        ppTaskCtrl: *mut *mut ISpNotifySink,
    ) -> HRESULT,
    fn CreateThreadControl(
        pTask: *mut ISpThreadTask,
        pvTaskData: *mut c_void,
        nPriority: c_long,
        ppTaskCtrl: *mut *mut ISpThreadControl,
    ) -> HRESULT,
    fn TerminateTask(
        dwGroupId: DWORD,
        ulWaitPeriod: ULONG,
    ) -> HRESULT,
}}
ENUM!{enum SPVSKIPTYPE {
    SPVST_SENTENCE = 1 << 0,
}}
ENUM!{enum SPVESACTIONS {
    SPVES_CONTINUE = 0,
    SPVES_ABORT = 1 << 0,
    SPVES_SKIP = 1 << 1,
    SPVES_RATE = 1 << 2,
    SPVES_VOLUME = 1 << 3,
}}
RIDL!{#[uuid(0x9880499b, 0xcce9, 0x11d2, 0xb5, 0x03, 0x00, 0xc0, 0x4f, 0x79, 0x73, 0x96)]
interface ISpTTSEngineSite(ISpTTSEngineSiteVtbl): ISpEventSink(ISpEventSinkVtbl) {
    fn GetActions() -> DWORD,
    fn Write(
        pBuff: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn GetRate(
        pRateAdjust: *mut c_long,
    ) -> HRESULT,
    fn GetVolume(pusVolume: *mut USHORT,
    ) -> HRESULT,
    fn GetSkipInfo(
        peType: *mut SPVSKIPTYPE,
        plNumItems: *mut c_long,
    ) -> HRESULT,
    fn CompleteSkip(
        ulNumSkipped: c_long,
    ) -> HRESULT,
}}
STRUCT!{struct SPVTEXTFRAG {
    pNext: *mut SPVTEXTFRAG,
    State: SPVSTATE,
    pTextStart: LPCWSTR,
    ulTextLen: ULONG,
    ulTextSrcOffset: ULONG,
}}
RIDL!{#[uuid(0xa74d7c8e, 0x4cc5, 0x4f2f, 0xa6, 0xeb, 0x80, 0x4d, 0xee, 0x18, 0x50, 0x0e)]
interface ISpTTSEngine(ISpTTSEngineVtbl): IUnknown(IUnknownVtbl) {
    fn Speak(
        dwSpeakFlags: DWORD,
        rguidFormatId: REFGUID,
        pWaveFormatEx: *const WAVEFORMATEX,
        pTextFragList: *const SPVTEXTFRAG,
        pOutputSite: *mut ISpTTSEngineSite,
    ) -> HRESULT,
    fn GetOutputFormat(
        pTargetFmtId: *const GUID,
        pTargetWaveFormatEx: *const WAVEFORMATEX,
        pOutputFormatId: *mut GUID,
        ppCoMemOutputWaveFormatEx: *mut WAVEFORMATEX,
    ) -> HRESULT,
}}
STRUCT!{struct SPWORDENTRY {
    hWord: SPWORDHANDLE,
    LangID: WORD,
    pszDisplayText: *mut WCHAR,
    pszLexicalForm: *mut WCHAR,
    aPhoneId: *mut SPPHONEID,
    pvClientContext: *mut c_void,
}}
STRUCT!{struct SPRULEENTRY {
    hRule: SPRULEHANDLE,
    hInitialState: SPSTATEHANDLE,
    Attributes: DWORD,
    pvClientRuleContext: *mut c_void,
    pvClientGrammarContext: *mut c_void,
}}
ENUM!{enum SPTRANSITIONTYPE {
    SPTRANSEPSILON = 0,
    SPTRANSWORD,
    SPTRANSRULE,
    SPTRANSTEXTBUF,
    SPTRANSWILDCARD,
    SPTRANSDICTATION,
}}
STRUCT!{struct SPTRANSITIONENTRY_u_s1 {
    hRuleInitialState: SPSTATEHANDLE,
    hRule: SPRULEHANDLE,
    pvClientRuleContext: *mut c_void,
}}
STRUCT!{struct SPTRANSITIONENTRY_u_s2 {
    hWord: SPWORDHANDLE,
    pvClientWordContext: *mut c_void,
}}
UNION!{union SPTRANSITIONENTRY_u {
    [usize; 3],
    s1 s1_mut: SPTRANSITIONENTRY_u_s1,
    s2 s2_mut: SPTRANSITIONENTRY_u_s2,
    pvGrammarCookie pvGrammarCookie_mut: *mut c_void,
}}
STRUCT!{struct SPTRANSITIONENTRY {
    ID: SPTRANSITIONID,
    hNextState: SPSTATEHANDLE,
    Type: BYTE,
    RequiredConfidence: c_char,
    fHasProperty: DWORD,
    Weight: c_float,
    u: SPTRANSITIONENTRY_u,
}}
STRUCT!{struct SPTRANSITIONPROPERTY {
    pszName: LPCWSTR,
    ulId: ULONG,
    pszValue: LPCWSTR,
    vValue: VARIANT,
}}
STRUCT!{struct SPSTATEINFO {
    cAllocatedEntries: ULONG,
    pTransitions: *mut SPTRANSITIONENTRY,
    cEpsilons: ULONG,
    cRules: ULONG,
    cWords: ULONG,
    cSpecialTransitions: ULONG,
}}
STRUCT!{struct SPPATHENTRY {
    hTransition: SPTRANSITIONID,
    elem: SPPHRASEELEMENT,
}}
RIDL!{#[uuid(0x6a6ffad8, 0x78b6, 0x473d, 0xb8, 0x44, 0x98, 0x15, 0x2e, 0x4f, 0xb1, 0x6b)]
interface ISpCFGInterpreterSite(ISpCFGInterpreterSiteVtbl): IUnknown(IUnknownVtbl) {
    fn AddTextReplacement(
        pReplace: *mut SPPHRASEREPLACEMENT,
    ) -> HRESULT,
    fn AddProperty(
        pProperty: *const SPPHRASEPROPERTY,
    ) -> HRESULT,
    fn GetResourceValue(
        pszResourceName: LPCWSTR,
        ppCoMemResource: *mut LPWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf3d3f926, 0x11fc, 0x11d3, 0xbb, 0x97, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpCFGInterpreter(ISpCFGInterpreterVtbl): IUnknown(IUnknownVtbl) {
    fn InitGrammar(
        pszGrammarName: LPCWSTR,
        pvGrammarData: *mut *const c_void,
    ) -> HRESULT,
    fn Interpret(
        pPhrase: *mut ISpPhraseBuilder,
        ulFirstElement: *const ULONG,
        ulCountOfElements: *const ULONG,
        pSite: *mut ISpCFGInterpreterSite,
    ) -> HRESULT,
}}
ENUM!{enum SPCFGNOTIFY {
    SPCFGN_ADD,
    SPCFGN_REMOVE,
    SPCFGN_INVALIDATE,
    SPCFGN_ACTIVATE,
    SPCFGN_DEACTIVATE,
}}
ENUM!{enum SPRESULTTYPE {
    SPRT_CFG = 0,
    SPRT_SLM = 1,
    SPRT_PROPRIETARY = 2,
    SPRT_FALSE_RECOGNITION = 1 << 2,
}}
STRUCT!{struct SPPHRASEALT {
    pPhrase: *mut ISpPhraseBuilder,
    ulStartElementInParent: ULONG,
    cElementsInParent: ULONG,
    cElementsInAlternate: ULONG,
    pvAltExtra: *mut c_void,
    cbAltExtra: ULONG,
}}
STRUCT!{struct SPRECORESULTINFO {
    cbSize: ULONG,
    eResultType: SPRESULTTYPE,
    fHypothesis: BOOL,
    fProprietaryAutoPause: BOOL,
    ullStreamPosStart: ULONGLONG,
    ullStreamPosEnd: ULONGLONG,
    hGrammar: SPGRAMMARHANDLE,
    ulSizeEngineData: ULONG,
    pvEngineData: *mut c_void,
    pPhrase: *mut ISpPhraseBuilder,
    aPhraseAlts: *mut SPPHRASEALT,
    ulNumAlts: ULONG,
}}
ENUM!{enum SPWORDINFOOPT {
    SPWIO_NONE = 0,
    SPWIO_WANT_TEXT = 1,
}}
ENUM!{enum SPRULEINFOOPT {
    SPRIO_NONE = 0,
}}
STRUCT!{struct SPPARSEINFO {
    cbSize: ULONG,
    hRule: SPRULEHANDLE,
    ullAudioStreamPosition: ULONGLONG,
    ulAudioSize: ULONG,
    cTransitions: ULONG,
    pPath: *mut SPPATHENTRY,
    SREngineID: GUID,
    ulSREnginePrivateDataSize: ULONG,
    pSREnginePrivateData: *const BYTE,
    fHypothesis: BOOL,
}}
RIDL!{#[uuid(0x3b414aec, 0x720c, 0x4883, 0xb9, 0xef, 0x17, 0x8c, 0xd3, 0x94, 0xfb, 0x3a)]
interface ISpSREngineSite(ISpSREngineSiteVtbl): IUnknown(IUnknownVtbl) {
    fn Read(
        pv: *mut c_void,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn DataAvailable(
        pcb: *mut ULONG,
    ) -> HRESULT,
    fn SetBufferNotifySize(
        cbSize: ULONG,
    ) -> HRESULT,
    fn ParseFromTransitions(
        pParseInfo: *const SPPARSEINFO,
        ppNewPhrase: *mut *mut ISpPhraseBuilder,
    ) -> HRESULT,
    fn Recognition(
        pResultInfo: *const SPRECORESULTINFO,
    ) -> HRESULT,
    fn AddEvent(
        pEvent: *const SPEVENT,
        hSAPIRecoContext: SPRECOCONTEXTHANDLE,
    ) -> HRESULT,
    fn Synchronize(
        ullProcessedThruPos: ULONGLONG,
    ) -> HRESULT,
    fn GetWordInfo(
        pWordEntry: *mut SPWORDENTRY,
        Options: SPWORDINFOOPT,
    ) -> HRESULT,
    fn SetWordClientContext(
        hWord: SPWORDHANDLE,
        pvClientContext: *mut c_void,
    ) -> HRESULT,
    fn GetRuleInfo(
        pRuleEntry: *mut SPRULEENTRY,
        Options: SPRULEINFOOPT,
    ) -> HRESULT,
    fn SetRuleClientContext(
        hRule: SPRULEHANDLE,
        pvClientContext: *mut c_void,
    ) -> HRESULT,
    fn GetStateInfo(
        hState: SPSTATEHANDLE,
        pStateInfo: *mut SPSTATEINFO,
    ) -> HRESULT,
    fn GetResource(
        hRule: SPRULEHANDLE,
        pszResourceName: LPCWSTR,
        ppCoMemResource: *mut LPWSTR,
    ) -> HRESULT,
    fn GetTransitionProperty(
        ID: SPTRANSITIONID,
        ppCoMemProperty: *mut *mut SPTRANSITIONPROPERTY,
    ) -> HRESULT,
    fn IsAlternate(
        hRule: SPRULEHANDLE,
        hAltRule: SPRULEHANDLE,
    ) -> HRESULT,
    fn GetMaxAlternates(
        hRule: SPRULEHANDLE,
        pulNumAlts: *mut ULONG,
    ) -> HRESULT,
    fn GetContextMaxAlternates(
        hContext: SPRECOCONTEXTHANDLE,
        pulNumAlts: *mut ULONG,
    ) -> HRESULT,
    fn UpdateRecoPos(
        ullCurrentRecoPos: ULONGLONG,
    ) -> HRESULT,
}}
ENUM!{enum SPPROPSRC {
    SPPROPSRC_RECO_INST,
    SPPROPSRC_RECO_CTX,
    SPPROPSRC_RECO_GRAMMAR,
}}
RIDL!{#[uuid(0x2f472991, 0x854b, 0x4465, 0xb6, 0x13, 0xfb, 0xaf, 0xb3, 0xad, 0x8e, 0xd8)]
interface ISpSREngine(ISpSREngineVtbl): IUnknown(IUnknownVtbl) {
    fn SetSite(
        pSite: *mut ISpSREngineSite,
    ) -> HRESULT,
    fn GetInputAudioFormat(
        pguidSourceFormatId: *const GUID,
        pSourceWaveFormatEx: *const WAVEFORMATEX,
        pguidDesiredFormatId: *mut GUID,
        ppCoMemDesiredWaveFormatEx: *mut WAVEFORMATEX,
    ) -> HRESULT,
    fn RecognizeStream(
        rguidFmtId: REFGUID,
        pWaveFormatEx: *const WAVEFORMATEX,
        hRequestSync: HANDLE,
        hDataAvailable: HANDLE,
        hExit: HANDLE,
        fNewAudioStream: BOOL,
        fRealTimeAudio: BOOL,
        pAudioObjectToken: *mut ISpObjectToken,
    ) -> HRESULT,
    fn SetRecoProfile(
        pProfile: *mut ISpObjectToken,
    ) -> HRESULT,
    fn OnCreateGrammar(
        pvEngineRecoContext: *mut c_void,
        hSAPIGrammar: SPGRAMMARHANDLE,
        ppvEngineGrammarContext: *mut *mut c_void,
    ) -> HRESULT,
    fn OnDeleteGrammar(
        pvEngineGrammar: *mut c_void,
    ) -> HRESULT,
    fn LoadProprietaryGrammar(
        pvEngineGrammar: *mut c_void,
        rguidParam: REFGUID,
        pszStringParam: LPCWSTR,
        pvDataParam: *const c_void,
        ulDataSize: ULONG,
        Options: SPLOADOPTIONS,
    ) -> HRESULT,
    fn UnloadProprietaryGrammar(
        pvEngineGrammar: *mut c_void,
    ) -> HRESULT,
    fn SetProprietaryRuleState(
        pvEngineGrammar: *mut c_void,
        pszName: LPCWSTR,
        pReserved: *mut c_void,
        NewState: SPRULESTATE,
        pcRulesChanged: *mut ULONG,
    ) -> HRESULT,
    fn SetProprietaryRuleIdState(
        pvEngineGrammar: *mut c_void,
        dwRuleId: DWORD,
        NewState: SPRULESTATE,
    ) -> HRESULT,
    fn LoadSLM(
        pvEngineGrammar: *mut c_void,
        pszTopicName: LPCWSTR,
    ) -> HRESULT,
    fn UnloadSLM(
        pvEngineGrammar: *mut c_void,
    ) -> HRESULT,
    fn SetSLMState(
        pvEngineGrammar: *mut c_void,
        NewState: SPRULESTATE,
    ) -> HRESULT,
    fn SetWordSequenceData(
        pvEngineGrammar: *mut c_void,
        pText: *const WCHAR,
        cchText: ULONG,
        pInfo: *const SPTEXTSELECTIONINFO,
    ) -> HRESULT,
    fn SetTextSelection(
        pvEngineGrammar: *mut c_void,
        pInfo: *const SPTEXTSELECTIONINFO,
    ) -> HRESULT,
    fn IsPronounceable(
        pvEngineGrammar: *mut c_void,
        pszWord: LPCWSTR,
        pWordPronounceable: *mut SPWORDPRONOUNCEABLE,
    ) -> HRESULT,
    fn OnCreateRecoContext(
        hSAPIRecoContext: SPRECOCONTEXTHANDLE,
        ppvEngineContext: *mut *mut c_void,
    ) -> HRESULT,
    fn OnDeleteRecoContext(
        pvEngineContext: *mut c_void,
    ) -> HRESULT,
    fn OnPrivateCall(
        pvEngineContext: *mut c_void,
        pCallFrame: PVOID,
        ulCallFrameSize: ULONG,
    ) -> HRESULT,
    fn SetAdaptationData(
        pvEngineContext: *mut c_void,
        pAdaptationData: *const WCHAR,
        cch: ULONG,
    ) -> HRESULT,
    fn SetPropertyNum(
        eSrc: SPPROPSRC,
        pvSrcObj: *mut c_void,
        pName: *const WCHAR,
        lValue: LONG,
    ) -> HRESULT,
    fn GetPropertyNum(
        eSrc: SPPROPSRC,
        pvSrcObj: *mut c_void,
        pName: *const WCHAR,
        lValue: *mut LONG,
    ) -> HRESULT,
    fn SetPropertyString(
        eSrc: SPPROPSRC,
        pvSrcObj: *mut c_void,
        pName: LPCWSTR,
        pValue: LPCWSTR,
    ) -> HRESULT,
    fn GetPropertyString(
        eSrc: SPPROPSRC,
        pvSrcObj: *mut c_void,
        pName: LPCWSTR,
        ppCoMemValue: *mut LPWSTR,
    ) -> HRESULT,
    fn SetGrammarState(
        pvEngineGrammar: *mut c_void,
        eGrammarState: SPGRAMMARSTATE,
    ) -> HRESULT,
    fn WordNotify(
        Action: SPCFGNOTIFY,
        cWords: ULONG,
        pWords: *const SPWORDENTRY,
    ) -> HRESULT,
    fn RuleNotify(
        Action: SPCFGNOTIFY,
        cRules: ULONG,
        pRules: *const SPRULEENTRY,
    ) -> HRESULT,
    fn PrivateCallEx(
        pvEngineContext: *mut c_void,
        pInCallFrame: *const c_void,
        ulInCallFrameSize: ULONG,
        ppvCoMemResponse: *mut *mut c_void,
        pulResponseSize: *mut ULONG,
    ) -> HRESULT,
    fn SetContextState(
        pvEngineContext: *mut c_void,
        eContextState: SPCONTEXTSTATE,
    ) -> HRESULT,
}}
STRUCT!{struct SPPHRASEALTREQUEST {
    ulStartElement: ULONG,
    cElements: ULONG,
    ulRequestAltCount: ULONG,
    pvResultExtra: *mut c_void,
    cbResultExtra: ULONG,
    pPhrase: *mut ISpPhrase,
    pRecoContext: *mut ISpRecoContext,
}}
RIDL!{#[uuid(0x8e7c791e, 0x4467, 0x11d3, 0x97, 0x23, 0x00, 0xc0, 0x4f, 0x72, 0xdb, 0x08)]
interface _ISpPrivateEngineCall(_ISpPrivateEngineCallVtbl): IUnknown(IUnknownVtbl) {
    fn CallEngine(
        pCallFrame: *mut c_void,
        ulCallFrameSize: ULONG,
    ) -> HRESULT,
    fn CallEngineEx(
        pInFrame: *const c_void,
        ulInFrameSize: ULONG,
        ppCoMemOutFrame: *mut *mut c_void,
        pulOutFrameSize: *mut ULONG,
    ) -> HRESULT,
}}
extern {
    pub static LIBID_SpeechDDKLib: IID;
    pub static CLSID_SpDataKey: CLSID;
}
RIDL!{#[uuid(0xd9f6ee60, 0x58c9, 0x458b, 0x88, 0xe1, 0x2f, 0x90, 0x8f, 0xd7, 0xf8, 0x7c)]
class SpDataKey;}
extern {
    pub static CLSID_SpObjectTokenEnum: CLSID;
    pub static CLSID_SpPhraseBuilder: CLSID;
    pub static CLSID_SpITNProcessor: CLSID;
    pub static CLSID_SpGrammarCompiler: CLSID;
    pub static CLSID_SpGramCompBackend: CLSID;
}
