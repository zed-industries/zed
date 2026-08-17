windows_core::imp::define_interface!(ISpeechContinuousRecognitionCompletedEventArgs, ISpeechContinuousRecognitionCompletedEventArgs_Vtbl, 0xe3d069bb_e30c_5e18_424b_7fbe81f8fbd0);
impl windows_core::RuntimeType for ISpeechContinuousRecognitionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognitionResultStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechContinuousRecognitionResultGeneratedEventArgs, ISpeechContinuousRecognitionResultGeneratedEventArgs_Vtbl, 0x19091e1e_6e7e_5a46_40fb_76594f786504);
impl windows_core::RuntimeType for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechContinuousRecognitionSession, ISpeechContinuousRecognitionSession_Vtbl, 0x6a213c04_6614_49f8_99a2_b5e9b3a085c8);
impl windows_core::RuntimeType for ISpeechContinuousRecognitionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoStopSilenceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetAutoStopSilenceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartWithModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechContinuousRecognitionMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PauseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ResultGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveResultGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionCompilationResult, ISpeechRecognitionCompilationResult_Vtbl, 0x407e6c5d_6ac7_4da4_9cc1_2fce32cf7489);
impl windows_core::RuntimeType for ISpeechRecognitionCompilationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionCompilationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognitionResultStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionConstraint, ISpeechRecognitionConstraint_Vtbl, 0x79ac1628_4d68_43c4_8911_40dc4101b55b);
impl windows_core::RuntimeType for ISpeechRecognitionConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISpeechRecognitionConstraint, windows_core::IUnknown, windows_core::IInspectable);
impl ISpeechRecognitionConstraint {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<SpeechRecognitionConstraintType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Probability(&self) -> windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Probability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProbability)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeName for ISpeechRecognitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint";
}
pub trait ISpeechRecognitionConstraint_Impl: windows_core::IUnknownImpl {
    fn IsEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn Tag(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<SpeechRecognitionConstraintType>;
    fn Probability(&self) -> windows_core::Result<SpeechRecognitionConstraintProbability>;
    fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> windows_core::Result<()>;
}
impl ISpeechRecognitionConstraint_Vtbl {
    pub const fn new<Identity: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsEnabled<Identity: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpeechRecognitionConstraint_Impl::IsEnabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpeechRecognitionConstraint_Impl::SetIsEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn Tag<Identity: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpeechRecognitionConstraint_Impl::Tag(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTag<Identity: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpeechRecognitionConstraint_Impl::SetTag(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Type<Identity: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpeechRecognitionConstraint_Impl::Type(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Probability<Identity: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpeechRecognitionConstraint_Impl::Probability(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProbability<Identity: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpeechRecognitionConstraint_Impl::SetProbability(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpeechRecognitionConstraint, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, OFFSET>,
            Tag: Tag::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Probability: Probability::<Identity, OFFSET>,
            SetProbability: SetProbability::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecognitionConstraint as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionConstraint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognitionConstraintType) -> windows_core::HRESULT,
    pub Probability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognitionConstraintProbability) -> windows_core::HRESULT,
    pub SetProbability: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRecognitionConstraintProbability) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionGrammarFileConstraint, ISpeechRecognitionGrammarFileConstraint_Vtbl, 0xb5031a8f_85ca_4fa4_b11a_474fc41b3835);
impl windows_core::RuntimeType for ISpeechRecognitionGrammarFileConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GrammarFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GrammarFile: usize,
}
windows_core::imp::define_interface!(ISpeechRecognitionGrammarFileConstraintFactory, ISpeechRecognitionGrammarFileConstraintFactory_Vtbl, 0x3da770eb_c479_4c27_9f19_89974ef392d1);
impl windows_core::RuntimeType for ISpeechRecognitionGrammarFileConstraintFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateWithTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateWithTag: usize,
}
windows_core::imp::define_interface!(ISpeechRecognitionHypothesis, ISpeechRecognitionHypothesis_Vtbl, 0x7a7b25b0_99c5_4f7d_bf84_10aa1302b634);
impl windows_core::RuntimeType for ISpeechRecognitionHypothesis {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesis_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionHypothesisGeneratedEventArgs, ISpeechRecognitionHypothesisGeneratedEventArgs_Vtbl, 0x55161a7a_8023_5866_411d_1213bb271476);
impl windows_core::RuntimeType for ISpeechRecognitionHypothesisGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Hypothesis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionListConstraint, ISpeechRecognitionListConstraint_Vtbl, 0x09c487e9_e4ad_4526_81f2_4946fb481d98);
impl windows_core::RuntimeType for ISpeechRecognitionListConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Commands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionListConstraintFactory, ISpeechRecognitionListConstraintFactory_Vtbl, 0x40f3cdc7_562a_426a_9f3b_3b4e282be1d5);
impl windows_core::RuntimeType for ISpeechRecognitionListConstraintFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraintFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionQualityDegradingEventArgs, ISpeechRecognitionQualityDegradingEventArgs_Vtbl, 0x4fe24105_8c3a_4c7e_8d0a_5bd4f5b14ad8);
impl windows_core::RuntimeType for ISpeechRecognitionQualityDegradingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionQualityDegradingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Problem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognitionAudioProblem) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionResult, ISpeechRecognitionResult_Vtbl, 0x4e303157_034e_4652_857e_d0454cc4beec);
impl windows_core::RuntimeType for ISpeechRecognitionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognitionResultStatus) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Confidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognitionConfidence) -> windows_core::HRESULT,
    pub SemanticInterpretation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Constraint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RulePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RawConfidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionResult2, ISpeechRecognitionResult2_Vtbl, 0xaf7ed1ba_451b_4166_a0c1_1ffe84032d03);
impl windows_core::RuntimeType for ISpeechRecognitionResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PhraseStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PhraseDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionSemanticInterpretation, ISpeechRecognitionSemanticInterpretation_Vtbl, 0xaae1da9b_7e32_4c1f_89fe_0c65f486f52e);
impl windows_core::RuntimeType for ISpeechRecognitionSemanticInterpretation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionSemanticInterpretation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionTopicConstraint, ISpeechRecognitionTopicConstraint_Vtbl, 0xbf6fdf19_825d_4e69_a681_36e48cf1c93e);
impl windows_core::RuntimeType for ISpeechRecognitionTopicConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Scenario: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognitionScenario) -> windows_core::HRESULT,
    pub TopicHint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionTopicConstraintFactory, ISpeechRecognitionTopicConstraintFactory_Vtbl, 0x6e6863df_ec05_47d7_a5df_56a3431e58d2);
impl windows_core::RuntimeType for ISpeechRecognitionTopicConstraintFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraintFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRecognitionScenario, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithTag: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRecognitionScenario, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognitionVoiceCommandDefinitionConstraint, ISpeechRecognitionVoiceCommandDefinitionConstraint_Vtbl, 0xf2791c2b_1ef4_4ae7_9d77_b6ff10b8a3c2);
impl windows_core::RuntimeType for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISpeechRecognizer, ISpeechRecognizer_Vtbl, 0x0bc3c9cb_c26a_40f2_aeb5_8096b2e48073);
impl windows_core::RuntimeType for ISpeechRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub CurrentLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    CurrentLanguage: usize,
    pub Constraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UIOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompileConstraintsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecognizeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecognizeWithUIAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecognitionQualityDegrading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveRecognitionQualityDegrading: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognizer2, ISpeechRecognizer2_Vtbl, 0x63c9baf1_91e3_4ea4_86a1_7c3867d084a6);
impl windows_core::RuntimeType for ISpeechRecognizer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContinuousRecognitionSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognizerState) -> windows_core::HRESULT,
    pub StopRecognitionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HypothesisGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveHypothesisGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognizerFactory, ISpeechRecognizerFactory_Vtbl, 0x60c488dd_7fb8_4033_ac70_d046f64818e1);
impl windows_core::RuntimeType for ISpeechRecognizerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Create: usize,
}
windows_core::imp::define_interface!(ISpeechRecognizerStateChangedEventArgs, ISpeechRecognizerStateChangedEventArgs_Vtbl, 0x563d4f09_ba03_4bad_ad81_ddc6c4dab0c3);
impl windows_core::RuntimeType for ISpeechRecognizerStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognizerState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognizerStatics, ISpeechRecognizerStatics_Vtbl, 0x87a35eac_a7dc_4b0b_bcc9_24f47c0b7ebf);
impl windows_core::RuntimeType for ISpeechRecognizerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub SystemSpeechLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    SystemSpeechLanguage: usize,
    #[cfg(feature = "Globalization")]
    pub SupportedTopicLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    SupportedTopicLanguages: usize,
    #[cfg(feature = "Globalization")]
    pub SupportedGrammarLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    SupportedGrammarLanguages: usize,
}
windows_core::imp::define_interface!(ISpeechRecognizerStatics2, ISpeechRecognizerStatics2_Vtbl, 0x1d1b0d95_7565_4ef9_a2f3_ba15162a96cf);
impl windows_core::RuntimeType for ISpeechRecognizerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub TrySetSystemSpeechLanguageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    TrySetSystemSpeechLanguageAsync: usize,
}
windows_core::imp::define_interface!(ISpeechRecognizerTimeouts, ISpeechRecognizerTimeouts_Vtbl, 0x2ef76fca_6a3c_4dca_a153_df1bc88a79af);
impl windows_core::RuntimeType for ISpeechRecognizerTimeouts {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerTimeouts_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InitialSilenceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetInitialSilenceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub EndSilenceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetEndSilenceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub BabbleTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetBabbleTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechRecognizerUIOptions, ISpeechRecognizerUIOptions_Vtbl, 0x7888d641_b92b_44ba_a25f_d1864630641f);
impl windows_core::RuntimeType for ISpeechRecognizerUIOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerUIOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExampleText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExampleText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AudiblePrompt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAudiblePrompt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsReadBackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsReadBackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShowConfirmation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowConfirmation: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechContinuousRecognitionCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechContinuousRecognitionCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechContinuousRecognitionCompletedEventArgs {
    pub fn Status(&self) -> windows_core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpeechContinuousRecognitionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechContinuousRecognitionCompletedEventArgs>();
}
unsafe impl windows_core::Interface for SpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = <ISpeechContinuousRecognitionCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechContinuousRecognitionCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechContinuousRecognitionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs";
}
unsafe impl Send for SpeechContinuousRecognitionCompletedEventArgs {}
unsafe impl Sync for SpeechContinuousRecognitionCompletedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechContinuousRecognitionMode(pub i32);
impl SpeechContinuousRecognitionMode {
    pub const Default: Self = Self(0i32);
    pub const PauseOnRecognition: Self = Self(1i32);
}
impl windows_core::TypeKind for SpeechContinuousRecognitionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechContinuousRecognitionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechContinuousRecognitionResultGeneratedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechContinuousRecognitionResultGeneratedEventArgs {
    pub fn Result(&self) -> windows_core::Result<SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechContinuousRecognitionResultGeneratedEventArgs>();
}
unsafe impl windows_core::Interface for SpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = <ISpeechContinuousRecognitionResultGeneratedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechContinuousRecognitionResultGeneratedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs";
}
unsafe impl Send for SpeechContinuousRecognitionResultGeneratedEventArgs {}
unsafe impl Sync for SpeechContinuousRecognitionResultGeneratedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechContinuousRecognitionSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechContinuousRecognitionSession, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechContinuousRecognitionSession {
    pub fn AutoStopSilenceTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoStopSilenceTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoStopSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoStopSilenceTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartWithModeAsync(&self, mode: SpeechContinuousRecognitionMode) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartWithModeAsync)(windows_core::Interface::as_raw(this), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CancelAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PauseAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Resume(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Resume)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Completed<P0>(&self, value: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, value: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResultGenerated<P0>(&self, value: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultGenerated)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveResultGenerated(&self, value: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveResultGenerated)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SpeechContinuousRecognitionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechContinuousRecognitionSession>();
}
unsafe impl windows_core::Interface for SpeechContinuousRecognitionSession {
    type Vtable = <ISpeechContinuousRecognitionSession as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechContinuousRecognitionSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechContinuousRecognitionSession {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession";
}
unsafe impl Send for SpeechContinuousRecognitionSession {}
unsafe impl Sync for SpeechContinuousRecognitionSession {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechRecognitionAudioProblem(pub i32);
impl SpeechRecognitionAudioProblem {
    pub const None: Self = Self(0i32);
    pub const TooNoisy: Self = Self(1i32);
    pub const NoSignal: Self = Self(2i32);
    pub const TooLoud: Self = Self(3i32);
    pub const TooQuiet: Self = Self(4i32);
    pub const TooFast: Self = Self(5i32);
    pub const TooSlow: Self = Self(6i32);
}
impl windows_core::TypeKind for SpeechRecognitionAudioProblem {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechRecognitionAudioProblem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionAudioProblem;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionCompilationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionCompilationResult, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognitionCompilationResult {
    pub fn Status(&self) -> windows_core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpeechRecognitionCompilationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionCompilationResult>();
}
unsafe impl windows_core::Interface for SpeechRecognitionCompilationResult {
    type Vtable = <ISpeechRecognitionCompilationResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionCompilationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionCompilationResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult";
}
unsafe impl Send for SpeechRecognitionCompilationResult {}
unsafe impl Sync for SpeechRecognitionCompilationResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechRecognitionConfidence(pub i32);
impl SpeechRecognitionConfidence {
    pub const High: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
    pub const Rejected: Self = Self(3i32);
}
impl windows_core::TypeKind for SpeechRecognitionConfidence {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechRecognitionConfidence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConfidence;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechRecognitionConstraintProbability(pub i32);
impl SpeechRecognitionConstraintProbability {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
    pub const Max: Self = Self(2i32);
}
impl windows_core::TypeKind for SpeechRecognitionConstraintProbability {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechRecognitionConstraintProbability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintProbability;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechRecognitionConstraintType(pub i32);
impl SpeechRecognitionConstraintType {
    pub const Topic: Self = Self(0i32);
    pub const List: Self = Self(1i32);
    pub const Grammar: Self = Self(2i32);
    pub const VoiceCommandDefinition: Self = Self(3i32);
}
impl windows_core::TypeKind for SpeechRecognitionConstraintType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechRecognitionConstraintType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionGrammarFileConstraint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionGrammarFileConstraint, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SpeechRecognitionGrammarFileConstraint, ISpeechRecognitionConstraint);
impl SpeechRecognitionGrammarFileConstraint {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<SpeechRecognitionConstraintType> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Probability(&self) -> windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Probability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProbability)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GrammarFile(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GrammarFile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0>(file: P0) -> windows_core::Result<SpeechRecognitionGrammarFileConstraint>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateWithTag<P0>(file: P0, tag: &windows_core::HSTRING) -> windows_core::Result<SpeechRecognitionGrammarFileConstraint>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithTag)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(tag), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISpeechRecognitionGrammarFileConstraintFactory<R, F: FnOnce(&ISpeechRecognitionGrammarFileConstraintFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechRecognitionGrammarFileConstraint, ISpeechRecognitionGrammarFileConstraintFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpeechRecognitionGrammarFileConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionGrammarFileConstraint>();
}
unsafe impl windows_core::Interface for SpeechRecognitionGrammarFileConstraint {
    type Vtable = <ISpeechRecognitionGrammarFileConstraint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionGrammarFileConstraint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionGrammarFileConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint";
}
unsafe impl Send for SpeechRecognitionGrammarFileConstraint {}
unsafe impl Sync for SpeechRecognitionGrammarFileConstraint {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionHypothesis(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionHypothesis, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognitionHypothesis {
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for SpeechRecognitionHypothesis {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionHypothesis>();
}
unsafe impl windows_core::Interface for SpeechRecognitionHypothesis {
    type Vtable = <ISpeechRecognitionHypothesis as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionHypothesis as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionHypothesis {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis";
}
unsafe impl Send for SpeechRecognitionHypothesis {}
unsafe impl Sync for SpeechRecognitionHypothesis {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionHypothesisGeneratedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionHypothesisGeneratedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognitionHypothesisGeneratedEventArgs {
    pub fn Hypothesis(&self) -> windows_core::Result<SpeechRecognitionHypothesis> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hypothesis)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpeechRecognitionHypothesisGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionHypothesisGeneratedEventArgs>();
}
unsafe impl windows_core::Interface for SpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = <ISpeechRecognitionHypothesisGeneratedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionHypothesisGeneratedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionHypothesisGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs";
}
unsafe impl Send for SpeechRecognitionHypothesisGeneratedEventArgs {}
unsafe impl Sync for SpeechRecognitionHypothesisGeneratedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionListConstraint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionListConstraint, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SpeechRecognitionListConstraint, ISpeechRecognitionConstraint);
impl SpeechRecognitionListConstraint {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<SpeechRecognitionConstraintType> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Probability(&self) -> windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Probability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProbability)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Commands(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Commands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(commands: P0) -> windows_core::Result<SpeechRecognitionListConstraint>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), commands.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithTag<P0>(commands: P0, tag: &windows_core::HSTRING) -> windows_core::Result<SpeechRecognitionListConstraint>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithTag)(windows_core::Interface::as_raw(this), commands.param().abi(), core::mem::transmute_copy(tag), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISpeechRecognitionListConstraintFactory<R, F: FnOnce(&ISpeechRecognitionListConstraintFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechRecognitionListConstraint, ISpeechRecognitionListConstraintFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpeechRecognitionListConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionListConstraint>();
}
unsafe impl windows_core::Interface for SpeechRecognitionListConstraint {
    type Vtable = <ISpeechRecognitionListConstraint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionListConstraint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionListConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint";
}
unsafe impl Send for SpeechRecognitionListConstraint {}
unsafe impl Sync for SpeechRecognitionListConstraint {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionQualityDegradingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionQualityDegradingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognitionQualityDegradingEventArgs {
    pub fn Problem(&self) -> windows_core::Result<SpeechRecognitionAudioProblem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Problem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpeechRecognitionQualityDegradingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionQualityDegradingEventArgs>();
}
unsafe impl windows_core::Interface for SpeechRecognitionQualityDegradingEventArgs {
    type Vtable = <ISpeechRecognitionQualityDegradingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionQualityDegradingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionQualityDegradingEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs";
}
unsafe impl Send for SpeechRecognitionQualityDegradingEventArgs {}
unsafe impl Sync for SpeechRecognitionQualityDegradingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionResult, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognitionResult {
    pub fn Status(&self) -> windows_core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Confidence(&self) -> windows_core::Result<SpeechRecognitionConfidence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Confidence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SemanticInterpretation(&self) -> windows_core::Result<SpeechRecognitionSemanticInterpretation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SemanticInterpretation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAlternates(&self, maxalternates: u32) -> windows_core::Result<windows_collections::IVectorView<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAlternates)(windows_core::Interface::as_raw(this), maxalternates, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Constraint(&self) -> windows_core::Result<ISpeechRecognitionConstraint> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Constraint)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RulePath(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RulePath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawConfidence(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawConfidence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhraseStartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhraseStartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhraseDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhraseDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpeechRecognitionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionResult>();
}
unsafe impl windows_core::Interface for SpeechRecognitionResult {
    type Vtable = <ISpeechRecognitionResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionResult";
}
unsafe impl Send for SpeechRecognitionResult {}
unsafe impl Sync for SpeechRecognitionResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechRecognitionResultStatus(pub i32);
impl SpeechRecognitionResultStatus {
    pub const Success: Self = Self(0i32);
    pub const TopicLanguageNotSupported: Self = Self(1i32);
    pub const GrammarLanguageMismatch: Self = Self(2i32);
    pub const GrammarCompilationFailure: Self = Self(3i32);
    pub const AudioQualityFailure: Self = Self(4i32);
    pub const UserCanceled: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
    pub const TimeoutExceeded: Self = Self(7i32);
    pub const PauseLimitExceeded: Self = Self(8i32);
    pub const NetworkFailure: Self = Self(9i32);
    pub const MicrophoneUnavailable: Self = Self(10i32);
}
impl windows_core::TypeKind for SpeechRecognitionResultStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechRecognitionResultStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionResultStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechRecognitionScenario(pub i32);
impl SpeechRecognitionScenario {
    pub const WebSearch: Self = Self(0i32);
    pub const Dictation: Self = Self(1i32);
    pub const FormFilling: Self = Self(2i32);
}
impl windows_core::TypeKind for SpeechRecognitionScenario {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechRecognitionScenario {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionScenario;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionSemanticInterpretation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionSemanticInterpretation, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognitionSemanticInterpretation {
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpeechRecognitionSemanticInterpretation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionSemanticInterpretation>();
}
unsafe impl windows_core::Interface for SpeechRecognitionSemanticInterpretation {
    type Vtable = <ISpeechRecognitionSemanticInterpretation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionSemanticInterpretation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionSemanticInterpretation {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation";
}
unsafe impl Send for SpeechRecognitionSemanticInterpretation {}
unsafe impl Sync for SpeechRecognitionSemanticInterpretation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionTopicConstraint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionTopicConstraint, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SpeechRecognitionTopicConstraint, ISpeechRecognitionConstraint);
impl SpeechRecognitionTopicConstraint {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<SpeechRecognitionConstraintType> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Probability(&self) -> windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Probability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProbability)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scenario(&self) -> windows_core::Result<SpeechRecognitionScenario> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scenario)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TopicHint(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TopicHint)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Create(scenario: SpeechRecognitionScenario, topichint: &windows_core::HSTRING) -> windows_core::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), scenario, core::mem::transmute_copy(topichint), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithTag(scenario: SpeechRecognitionScenario, topichint: &windows_core::HSTRING, tag: &windows_core::HSTRING) -> windows_core::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithTag)(windows_core::Interface::as_raw(this), scenario, core::mem::transmute_copy(topichint), core::mem::transmute_copy(tag), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISpeechRecognitionTopicConstraintFactory<R, F: FnOnce(&ISpeechRecognitionTopicConstraintFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechRecognitionTopicConstraint, ISpeechRecognitionTopicConstraintFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpeechRecognitionTopicConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionTopicConstraint>();
}
unsafe impl windows_core::Interface for SpeechRecognitionTopicConstraint {
    type Vtable = <ISpeechRecognitionTopicConstraint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionTopicConstraint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionTopicConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint";
}
unsafe impl Send for SpeechRecognitionTopicConstraint {}
unsafe impl Sync for SpeechRecognitionTopicConstraint {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognitionVoiceCommandDefinitionConstraint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognitionVoiceCommandDefinitionConstraint, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SpeechRecognitionVoiceCommandDefinitionConstraint, ISpeechRecognitionConstraint);
impl SpeechRecognitionVoiceCommandDefinitionConstraint {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<SpeechRecognitionConstraintType> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Probability(&self) -> windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Probability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProbability)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognitionVoiceCommandDefinitionConstraint>();
}
unsafe impl windows_core::Interface for SpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = <ISpeechRecognitionVoiceCommandDefinitionConstraint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognitionVoiceCommandDefinitionConstraint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint";
}
unsafe impl Send for SpeechRecognitionVoiceCommandDefinitionConstraint {}
unsafe impl Sync for SpeechRecognitionVoiceCommandDefinitionConstraint {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognizer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SpeechRecognizer, super::super::Foundation::IClosable);
impl SpeechRecognizer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechRecognizer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Globalization")]
    pub fn CurrentLanguage(&self) -> windows_core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Constraints(&self) -> windows_core::Result<windows_collections::IVector<ISpeechRecognitionConstraint>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Constraints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timeouts(&self) -> windows_core::Result<SpeechRecognizerTimeouts> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timeouts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UIOptions(&self) -> windows_core::Result<SpeechRecognizerUIOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CompileConstraintsAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<SpeechRecognitionCompilationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompileConstraintsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecognizeAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecognizeWithUIAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizeWithUIAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecognitionQualityDegrading<P0>(&self, speechrecognitionqualitydegradinghandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognitionQualityDegrading)(windows_core::Interface::as_raw(this), speechrecognitionqualitydegradinghandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRecognitionQualityDegrading(&self, cookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRecognitionQualityDegrading)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn StateChanged<P0>(&self, statechangedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), statechangedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, cookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ContinuousRecognitionSession(&self) -> windows_core::Result<SpeechContinuousRecognitionSession> {
        let this = &windows_core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuousRecognitionSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<SpeechRecognizerState> {
        let this = &windows_core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StopRecognitionAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopRecognitionAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HypothesisGenerated<P0>(&self, value: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HypothesisGenerated)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHypothesisGenerated(&self, value: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHypothesisGenerated)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Globalization")]
    pub fn Create<P0>(language: P0) -> windows_core::Result<SpeechRecognizer>
    where
        P0: windows_core::Param<super::super::Globalization::Language>,
    {
        Self::ISpeechRecognizerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), language.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn SystemSpeechLanguage() -> windows_core::Result<super::super::Globalization::Language> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemSpeechLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn SupportedTopicLanguages() -> windows_core::Result<windows_collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedTopicLanguages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn SupportedGrammarLanguages() -> windows_core::Result<windows_collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedGrammarLanguages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn TrySetSystemSpeechLanguageAsync<P0>(speechlanguage: P0) -> windows_core::Result<windows_future::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Globalization::Language>,
    {
        Self::ISpeechRecognizerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetSystemSpeechLanguageAsync)(windows_core::Interface::as_raw(this), speechlanguage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISpeechRecognizerFactory<R, F: FnOnce(&ISpeechRecognizerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISpeechRecognizerStatics<R, F: FnOnce(&ISpeechRecognizerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISpeechRecognizerStatics2<R, F: FnOnce(&ISpeechRecognizerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpeechRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognizer>();
}
unsafe impl windows_core::Interface for SpeechRecognizer {
    type Vtable = <ISpeechRecognizer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognizer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognizer {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizer";
}
unsafe impl Send for SpeechRecognizer {}
unsafe impl Sync for SpeechRecognizer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechRecognizerState(pub i32);
impl SpeechRecognizerState {
    pub const Idle: Self = Self(0i32);
    pub const Capturing: Self = Self(1i32);
    pub const Processing: Self = Self(2i32);
    pub const SoundStarted: Self = Self(3i32);
    pub const SoundEnded: Self = Self(4i32);
    pub const SpeechDetected: Self = Self(5i32);
    pub const Paused: Self = Self(6i32);
}
impl windows_core::TypeKind for SpeechRecognizerState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechRecognizerState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognizerState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognizerStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognizerStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognizerStateChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<SpeechRecognizerState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpeechRecognizerStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognizerStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for SpeechRecognizerStateChangedEventArgs {
    type Vtable = <ISpeechRecognizerStateChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognizerStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognizerStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs";
}
unsafe impl Send for SpeechRecognizerStateChangedEventArgs {}
unsafe impl Sync for SpeechRecognizerStateChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognizerTimeouts(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognizerTimeouts, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognizerTimeouts {
    pub fn InitialSilenceTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitialSilenceTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInitialSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInitialSilenceTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EndSilenceTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndSilenceTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEndSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEndSilenceTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BabbleTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BabbleTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBabbleTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBabbleTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SpeechRecognizerTimeouts {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognizerTimeouts>();
}
unsafe impl windows_core::Interface for SpeechRecognizerTimeouts {
    type Vtable = <ISpeechRecognizerTimeouts as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognizerTimeouts as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognizerTimeouts {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts";
}
unsafe impl Send for SpeechRecognizerTimeouts {}
unsafe impl Sync for SpeechRecognizerTimeouts {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechRecognizerUIOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechRecognizerUIOptions, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechRecognizerUIOptions {
    pub fn ExampleText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExampleText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetExampleText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExampleText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AudiblePrompt(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudiblePrompt)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAudiblePrompt(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAudiblePrompt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsReadBackEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadBackEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsReadBackEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsReadBackEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowConfirmation(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowConfirmation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShowConfirmation(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShowConfirmation)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SpeechRecognizerUIOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechRecognizerUIOptions>();
}
unsafe impl windows_core::Interface for SpeechRecognizerUIOptions {
    type Vtable = <ISpeechRecognizerUIOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechRecognizerUIOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechRecognizerUIOptions {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions";
}
unsafe impl Send for SpeechRecognizerUIOptions {}
unsafe impl Sync for SpeechRecognizerUIOptions {}
