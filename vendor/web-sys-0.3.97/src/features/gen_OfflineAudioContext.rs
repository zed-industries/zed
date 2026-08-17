#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        vendor_prefix = "webkit",
        extends = "BaseAudioContext",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "OfflineAudioContext",
        typescript_type = "OfflineAudioContext"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineAudioContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub type OfflineAudioContext;
    #[wasm_bindgen(method, getter, js_class = "OfflineAudioContext", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn length(this: &OfflineAudioContext) -> u32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineAudioContext",
        js_name = "oncomplete"
    )]
    #[doc = "Getter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn oncomplete(this: &OfflineAudioContext) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OfflineAudioContext",
        js_name = "oncomplete"
    )]
    #[doc = "Setter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn set_oncomplete(this: &OfflineAudioContext, value: Option<&::js_sys::Function>);
    #[cfg(feature = "AudioDestinationNode")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineAudioContext",
        js_name = "destination"
    )]
    #[doc = "Getter for the `destination` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/destination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDestinationNode`, `OfflineAudioContext`*"]
    pub fn destination(this: &OfflineAudioContext) -> AudioDestinationNode;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineAudioContext",
        js_name = "sampleRate"
    )]
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn sample_rate(this: &OfflineAudioContext) -> f32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineAudioContext",
        js_name = "currentTime"
    )]
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn current_time(this: &OfflineAudioContext) -> f64;
    #[cfg(feature = "AudioListener")]
    #[wasm_bindgen(method, getter, js_class = "OfflineAudioContext", js_name = "listener")]
    #[doc = "Getter for the `listener` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/listener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`, `OfflineAudioContext`*"]
    pub fn listener(this: &OfflineAudioContext) -> AudioListener;
    #[cfg(feature = "AudioContextState")]
    #[wasm_bindgen(method, getter, js_class = "OfflineAudioContext", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextState`, `OfflineAudioContext`*"]
    pub fn state(this: &OfflineAudioContext) -> AudioContextState;
    #[cfg(feature = "AudioWorklet")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "OfflineAudioContext",
        js_name = "audioWorklet"
    )]
    #[doc = "Getter for the `audioWorklet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/audioWorklet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorklet`, `OfflineAudioContext`*"]
    pub fn audio_worklet(this: &OfflineAudioContext) -> Result<AudioWorklet, JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineAudioContext",
        js_name = "onstatechange"
    )]
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn onstatechange(this: &OfflineAudioContext) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OfflineAudioContext",
        js_name = "onstatechange"
    )]
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn set_onstatechange(this: &OfflineAudioContext, value: Option<&::js_sys::Function>);
    #[cfg(feature = "OfflineAudioContextOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "OfflineAudioContext")]
    #[doc = "The `new OfflineAudioContext(..)` constructor, creating a new instance of `OfflineAudioContext`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/OfflineAudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `OfflineAudioContextOptions`*"]
    pub fn new_with_context_options(
        context_options: &OfflineAudioContextOptions,
    ) -> Result<OfflineAudioContext, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "OfflineAudioContext")]
    #[doc = "The `new OfflineAudioContext(..)` constructor, creating a new instance of `OfflineAudioContext`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/OfflineAudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn new_with_number_of_channels_and_length_and_sample_rate(
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<OfflineAudioContext, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "startRendering"
    )]
    #[doc = "The `startRendering()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/startRendering)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn start_rendering(this: &OfflineAudioContext) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "AnalyserNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createAnalyser"
    )]
    #[doc = "The `createAnalyser()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createAnalyser)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`, `OfflineAudioContext`*"]
    pub fn create_analyser(this: &OfflineAudioContext) -> Result<AnalyserNode, JsValue>;
    #[cfg(feature = "BiquadFilterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createBiquadFilter"
    )]
    #[doc = "The `createBiquadFilter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBiquadFilter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`, `OfflineAudioContext`*"]
    pub fn create_biquad_filter(this: &OfflineAudioContext) -> Result<BiquadFilterNode, JsValue>;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createBuffer"
    )]
    #[doc = "The `createBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioContext`*"]
    pub fn create_buffer(
        this: &OfflineAudioContext,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, JsValue>;
    #[cfg(feature = "AudioBufferSourceNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createBufferSource"
    )]
    #[doc = "The `createBufferSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBufferSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `OfflineAudioContext`*"]
    pub fn create_buffer_source(
        this: &OfflineAudioContext,
    ) -> Result<AudioBufferSourceNode, JsValue>;
    #[cfg(feature = "ChannelMergerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createChannelMerger"
    )]
    #[doc = "The `createChannelMerger()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelMerger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerNode`, `OfflineAudioContext`*"]
    pub fn create_channel_merger(this: &OfflineAudioContext) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(feature = "ChannelMergerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createChannelMerger"
    )]
    #[doc = "The `createChannelMerger()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelMerger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerNode`, `OfflineAudioContext`*"]
    pub fn create_channel_merger_with_number_of_inputs(
        this: &OfflineAudioContext,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(feature = "ChannelSplitterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createChannelSplitter"
    )]
    #[doc = "The `createChannelSplitter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelSplitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterNode`, `OfflineAudioContext`*"]
    pub fn create_channel_splitter(
        this: &OfflineAudioContext,
    ) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(feature = "ChannelSplitterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createChannelSplitter"
    )]
    #[doc = "The `createChannelSplitter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelSplitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterNode`, `OfflineAudioContext`*"]
    pub fn create_channel_splitter_with_number_of_outputs(
        this: &OfflineAudioContext,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(feature = "ConstantSourceNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createConstantSource"
    )]
    #[doc = "The `createConstantSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createConstantSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`, `OfflineAudioContext`*"]
    pub fn create_constant_source(
        this: &OfflineAudioContext,
    ) -> Result<ConstantSourceNode, JsValue>;
    #[cfg(feature = "ConvolverNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createConvolver"
    )]
    #[doc = "The `createConvolver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createConvolver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverNode`, `OfflineAudioContext`*"]
    pub fn create_convolver(this: &OfflineAudioContext) -> Result<ConvolverNode, JsValue>;
    #[cfg(feature = "DelayNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createDelay"
    )]
    #[doc = "The `createDelay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDelay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayNode`, `OfflineAudioContext`*"]
    pub fn create_delay(this: &OfflineAudioContext) -> Result<DelayNode, JsValue>;
    #[cfg(feature = "DelayNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createDelay"
    )]
    #[doc = "The `createDelay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDelay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayNode`, `OfflineAudioContext`*"]
    pub fn create_delay_with_max_delay_time(
        this: &OfflineAudioContext,
        max_delay_time: f64,
    ) -> Result<DelayNode, JsValue>;
    #[cfg(feature = "DynamicsCompressorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createDynamicsCompressor"
    )]
    #[doc = "The `createDynamicsCompressor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDynamicsCompressor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorNode`, `OfflineAudioContext`*"]
    pub fn create_dynamics_compressor(
        this: &OfflineAudioContext,
    ) -> Result<DynamicsCompressorNode, JsValue>;
    #[cfg(feature = "GainNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createGain"
    )]
    #[doc = "The `createGain()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createGain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainNode`, `OfflineAudioContext`*"]
    pub fn create_gain(this: &OfflineAudioContext) -> Result<GainNode, JsValue>;
    #[cfg(feature = "IirFilterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createIIRFilter"
    )]
    #[doc = "The `createIIRFilter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createIIRFilter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`, `OfflineAudioContext`*"]
    pub fn create_iir_filter(
        this: &OfflineAudioContext,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, JsValue>;
    #[cfg(feature = "OscillatorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createOscillator"
    )]
    #[doc = "The `createOscillator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createOscillator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `OscillatorNode`*"]
    pub fn create_oscillator(this: &OfflineAudioContext) -> Result<OscillatorNode, JsValue>;
    #[cfg(feature = "PannerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPanner"
    )]
    #[doc = "The `createPanner()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPanner)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PannerNode`*"]
    pub fn create_panner(this: &OfflineAudioContext) -> Result<PannerNode, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave(
        this: &OfflineAudioContext,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_slice(
        this: &OfflineAudioContext,
        real: &::js_sys::Float32Array,
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_slice_and_f32_array(
        this: &OfflineAudioContext,
        real: &mut [f32],
        imag: &::js_sys::Float32Array,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_array(
        this: &OfflineAudioContext,
        real: &::js_sys::Float32Array,
        imag: &::js_sys::Float32Array,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_constraints(
        this: &OfflineAudioContext,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_slice_and_constraints(
        this: &OfflineAudioContext,
        real: &::js_sys::Float32Array,
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_slice_and_f32_array_and_constraints(
        this: &OfflineAudioContext,
        real: &mut [f32],
        imag: &::js_sys::Float32Array,
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_array_and_constraints(
        this: &OfflineAudioContext,
        real: &::js_sys::Float32Array,
        imag: &::js_sys::Float32Array,
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor(
        this: &OfflineAudioContext,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size(
        this: &OfflineAudioContext,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        this: &OfflineAudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        this: &OfflineAudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "StereoPannerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createStereoPanner"
    )]
    #[doc = "The `createStereoPanner()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createStereoPanner)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `StereoPannerNode`*"]
    pub fn create_stereo_panner(this: &OfflineAudioContext) -> Result<StereoPannerNode, JsValue>;
    #[cfg(feature = "WaveShaperNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "createWaveShaper"
    )]
    #[doc = "The `createWaveShaper()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createWaveShaper)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `WaveShaperNode`*"]
    pub fn create_wave_shaper(this: &OfflineAudioContext) -> Result<WaveShaperNode, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "decodeAudioData"
    )]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn decode_audio_data(
        this: &OfflineAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "decodeAudioData"
    )]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn decode_audio_data_with_success_callback(
        this: &OfflineAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OfflineAudioContext",
        js_name = "decodeAudioData"
    )]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        this: &OfflineAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OfflineAudioContext")]
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    pub fn resume(this: &OfflineAudioContext) -> Result<::js_sys::Promise, JsValue>;
}
