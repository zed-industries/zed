// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_float, c_int, c_void};
use shared::guiddef::CLSID;
use shared::minwindef::{BOOL, DWORD, ULONG};
use um::sapi::*;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, ULONGLONG, WCHAR};
pub use um::sapiddk51::{
    SPRECOEXTENSION,
    SPALTERNATESCLSID,
};
pub const SR_LOCALIZED_DESCRIPTION: &'static str = "Description";
pub use um::sapiddk51::{
    ISpTokenUI, ISpTokenUIVtbl,
    ISpObjectTokenEnumBuilder, ISpObjectTokenEnumBuilderVtbl,
    SPWORDHANDLE,
    SPRULEHANDLE,
    SPGRAMMARHANDLE,
    SPRECOCONTEXTHANDLE,
    SPPHRASERULEHANDLE,
    SPPHRASEPROPERTYHANDLE,
    SPTRANSITIONID,
    ISpErrorLog, ISpErrorLogVtbl,
    ISpGrammarCompiler, ISpGrammarCompilerVtbl,
    ISpGramCompBackend, ISpGramCompBackendVtbl,
    ISpITNProcessor, ISpITNProcessorVtbl,
    ISpPhraseBuilder, ISpPhraseBuilderVtbl,
    ISpTask,
    ISpThreadTask,
    ISpThreadControl, ISpThreadControlVtbl,
    SPTMTHREADINFO,
    ISpTaskManager, ISpTaskManagerVtbl,
    SPVSKIPTYPE,
    SPVST_SENTENCE,
    SPVESACTIONS,
    SPVES_CONTINUE,
    SPVES_ABORT,
    SPVES_SKIP,
    SPVES_RATE,
    SPVES_VOLUME,
    ISpTTSEngineSite, ISpTTSEngineSiteVtbl,
    SPVTEXTFRAG,
    ISpTTSEngine, ISpTTSEngineVtbl,
    SPWORDENTRY,
    SPRULEENTRY,
    SPTRANSITIONTYPE,
    SPTRANSEPSILON,
    SPTRANSWORD,
    SPTRANSRULE,
    SPTRANSTEXTBUF,
    SPTRANSWILDCARD,
    SPTRANSDICTATION,
    SPTRANSITIONENTRY,
    SPTRANSITIONPROPERTY,
    SPSTATEINFO,
    SPPATHENTRY,
    ISpCFGInterpreterSite, ISpCFGInterpreterSiteVtbl,
    ISpCFGInterpreter, ISpCFGInterpreterVtbl,
    SPCFGNOTIFY,
    SPCFGN_ADD,
    SPCFGN_REMOVE,
    SPCFGN_INVALIDATE,
    SPCFGN_ACTIVATE,
    SPCFGN_DEACTIVATE,
    SPRESULTTYPE,
    SPRT_CFG,
    SPRT_SLM,
    SPRT_PROPRIETARY,
    SPRT_FALSE_RECOGNITION,
};
pub const SPRT_TYPE_MASK: SPRESULTTYPE = 3;
pub const SPRT_EMULATED: SPRESULTTYPE = 1 << 3;
pub const SPRT_EXTENDABLE_PARSE: SPRESULTTYPE = 1 << 4;
pub use um::sapiddk51::{
    SPPHRASEALT,
    SPRECORESULTINFO,
};
STRUCT!{struct SPRECORESULTINFOEX {
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
    ullStreamTimeStart: ULONGLONG,
    ullStreamTimeEnd: ULONGLONG,
}}
pub use um::sapiddk51::{
    SPWORDINFOOPT,
    SPWIO_NONE,
    SPWIO_WANT_TEXT,
    SPRULEINFOOPT,
    SPRIO_NONE,
    SPPARSEINFO,
    ISpSREngineSite, ISpSREngineSiteVtbl,
};
RIDL!{#[uuid(0x7bc6e012, 0x684a, 0x493e, 0xbd, 0xd4, 0x2b, 0xf5, 0xfb, 0xf4, 0x8c, 0xfe)]
interface ISpSREngineSite2(ISpSREngineSite2Vtbl): ISpSREngineSite(ISpSREngineSiteVtbl) {
    fn AddEventEx(
        pEvent: *const SPEVENTEX,
        hSAPIRecoContext: SPRECOCONTEXTHANDLE,
    ) -> HRESULT,
    fn UpdateRecoPosEx(
        ullCurrentRecoPos: ULONGLONG,
        ullCurrentRecoTime: ULONGLONG,
    ) -> HRESULT,
    fn GetRuleTransition(
        ulGrammarID: ULONG,
        RuleIndex: ULONG,
        pTrans: *mut SPTRANSITIONENTRY,
    ) -> HRESULT,
    fn RecognitionEx(
        pResultInfo: *const SPRECORESULTINFOEX,
    ) -> HRESULT,
}}
pub use um::sapiddk51::{
    SPPROPSRC,
    SPPROPSRC_RECO_INST,
    SPPROPSRC_RECO_CTX,
    SPPROPSRC_RECO_GRAMMAR,
    ISpSREngine, ISpSREngineVtbl,
};
RIDL!{#[uuid(0x7ba627d8, 0x33f9, 0x4375, 0x90, 0xc5, 0x99, 0x85, 0xae, 0xe5, 0xed, 0xe5)]
interface ISpSREngine2(ISpSREngine2Vtbl): ISpSREngine(ISpSREngineVtbl) {
    fn PrivateCallImmediate(
        pvEngineContext: *mut c_void,
        pInCallFrame: *const c_void,
        ulInCallFrameSize: ULONG,
        ppvCoMemResponse: *mut *mut c_void,
        pulResponseSize: *mut ULONG,
    ) -> HRESULT,
    fn SetAdaptationData2(
        pvEngineContext: *mut c_void,
        pAdaptationData: *const WCHAR,
        cch: ULONG,
        pTopicName: LPCWSTR,
        eSettings: SPADAPTATIONSETTINGS,
        eRelevance: SPADAPTATIONRELEVANCE,
    ) -> HRESULT,
    fn SetGrammarPrefix(
        pvEngineGrammar: *mut c_void,
        pszPrefix: LPCWSTR,
        fIsPrefixRequired: BOOL,
    ) -> HRESULT,
    fn SetRulePriority(
        hRule: SPRULEHANDLE,
        pvClientRuleContext: *mut c_void,
        nRulePriority: c_int,
    ) -> HRESULT,
    fn EmulateRecognition(
        pPhrase: *mut ISpPhrase,
        dwCompareFlags: DWORD,
    ) -> HRESULT,
    fn SetSLMWeight(
        pvEngineGrammar: *mut c_void,
        flWeight: c_float,
    ) -> HRESULT,
    fn SetRuleWeight(
        hRule: SPRULEHANDLE,
        pvClientRuleContext: *mut c_void,
        flWeight: c_float,
    ) -> HRESULT,
    fn SetTrainingState(
        fDoingTraining: BOOL,
        fAdaptFromTrainingData: BOOL,
    ) -> HRESULT,
    fn ResetAcousticModelAdaptation() -> HRESULT,
    fn OnLoadCFG(
        pvEngineGrammar: *mut c_void,
        pvGrammarData: *const SPBINARYGRAMMAR,
        ulGrammarID: ULONG,
    ) -> HRESULT,
    fn OnUnloadCFG(
        pvEngineGrammar: *mut c_void,
        ulGrammarID: ULONG,
    ) -> HRESULT,
}}
pub use um::sapiddk51::SPPHRASEALTREQUEST;
RIDL!{#[uuid(0xfece8294, 0x2be1, 0x408f, 0x8e, 0x68, 0x2d, 0xe3, 0x77, 0x09, 0x2f, 0x0e)]
interface ISpSRAlternates(ISpSRAlternatesVtbl): IUnknown(IUnknownVtbl) {
    fn GetAlternates(
        pAltRequest: *mut SPPHRASEALTREQUEST,
        ppAlts: *mut *mut SPPHRASEALT,
        pcAlts: *mut ULONG,
    ) -> HRESULT,
    fn Commit(
        pAltRequest: *mut SPPHRASEALTREQUEST,
        pAlt: *mut SPPHRASEALT,
        ppvResultExtra: *mut c_void,
        pcbResultExtra: *mut ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf338f437, 0xcb33, 0x4020, 0x9c, 0xab, 0xc7, 0x1f, 0xf9, 0xce, 0x12, 0xd3)]
interface ISpSRAlternates2(ISpSRAlternates2Vtbl): ISpSRAlternates(ISpSRAlternatesVtbl) {
    fn CommitText(
        pAltRequest: *mut SPPHRASEALTREQUEST,
        pcszNewText: LPCWSTR,
        commitFlags: SPCOMMITFLAGS,
    ) -> HRESULT,
}}
pub use um::sapiddk51::{_ISpPrivateEngineCall, _ISpPrivateEngineCallVtbl};
RIDL!{#[uuid(0xdefd682a, 0xfe0a, 0x42b9, 0xbf, 0xa1, 0x56, 0xd3, 0xd6, 0xce, 0xcf, 0xaf)]
interface ISpPrivateEngineCallEx(ISpPrivateEngineCallExVtbl): IUnknown(IUnknownVtbl) {
    fn CallEngineSynchronize(
        pInFrame: *const c_void,
        ulInFrameSize: ULONG,
        ppCoMemOutFrame: *mut *mut c_void,
        pulOutFrameSize: *mut ULONG,
    ) -> HRESULT,
    fn CallEngineImmediate(
        pInFrame: *const c_void,
        ulInFrameSize: ULONG,
        ppCoMemOutFrame: *mut *mut c_void,
        pulOutFrameSize: *mut ULONG,
    ) -> HRESULT,
}}
pub use um::sapiddk51::{
    LIBID_SpeechDDKLib,
    CLSID_SpDataKey,
    CLSID_SpObjectTokenEnum,
    CLSID_SpPhraseBuilder,
    CLSID_SpITNProcessor,
    CLSID_SpGrammarCompiler,
};
extern {
    pub static CLSID_SpW3CGrammarCompiler: CLSID;
}
pub use um::sapiddk51::CLSID_SpGramCompBackend;
