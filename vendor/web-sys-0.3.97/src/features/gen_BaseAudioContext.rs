#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "BaseAudioContext",
        typescript_type = "BaseAudioContext"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseAudioContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub type BaseAudioContext;
    #[cfg(feature = "AudioDestinationNode")]
    #[wasm_bindgen(method, getter, js_class = "BaseAudioContext", js_name = "destination")]
    #[doc = "Getter for the `destination` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/destination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDestinationNode`, `BaseAudioContext`*"]
    pub fn destination(this: &BaseAudioContext) -> AudioDestinationNode;
    #[wasm_bindgen(method, getter, js_class = "BaseAudioContext", js_name = "sampleRate")]
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn sample_rate(this: &BaseAudioContext) -> f32;
    #[wasm_bindgen(method, getter, js_class = "BaseAudioContext", js_name = "currentTime")]
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn current_time(this: &BaseAudioContext) -> f64;
    #[cfg(feature = "AudioListener")]
    #[wasm_bindgen(method, getter, js_class = "BaseAudioContext", js_name = "listener")]
    #[doc = "Getter for the `listener` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/listener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`, `BaseAudioContext`*"]
    pub fn listener(this: &BaseAudioContext) -> AudioListener;
    #[cfg(feature = "AudioContextState")]
    #[wasm_bindgen(method, getter, js_class = "BaseAudioContext", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextState`, `BaseAudioContext`*"]
    pub fn state(this: &BaseAudioContext) -> AudioContextState;
    #[cfg(feature = "AudioWorklet")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "BaseAudioContext",
        js_name = "audioWorklet"
    )]
    #[doc = "Getter for the `audioWorklet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/audioWorklet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorklet`, `BaseAudioContext`*"]
    pub fn audio_worklet(this: &BaseAudioContext) -> Result<AudioWorklet, JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BaseAudioContext",
        js_name = "onstatechange"
    )]
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn onstatechange(this: &BaseAudioContext) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BaseAudioContext",
        js_name = "onstatechange"
    )]
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn set_onstatechange(this: &BaseAudioContext, value: Option<&::js_sys::Function>);
    #[cfg(feature = "AnalyserNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createAnalyser"
    )]
    #[doc = "The `createAnalyser()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createAnalyser)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`, `BaseAudioContext`*"]
    pub fn create_analyser(this: &BaseAudioContext) -> Result<AnalyserNode, JsValue>;
    #[cfg(feature = "BiquadFilterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createBiquadFilter"
    )]
    #[doc = "The `createBiquadFilter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBiquadFilter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`*"]
    pub fn create_biquad_filter(this: &BaseAudioContext) -> Result<BiquadFilterNode, JsValue>;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(catch, method, js_class = "BaseAudioContext", js_name = "createBuffer")]
    #[doc = "The `createBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `BaseAudioContext`*"]
    pub fn create_buffer(
        this: &BaseAudioContext,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, JsValue>;
    #[cfg(feature = "AudioBufferSourceNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createBufferSource"
    )]
    #[doc = "The `createBufferSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBufferSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `BaseAudioContext`*"]
    pub fn create_buffer_source(this: &BaseAudioContext) -> Result<AudioBufferSourceNode, JsValue>;
    #[cfg(feature = "ChannelMergerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createChannelMerger"
    )]
    #[doc = "The `createChannelMerger()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelMerger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelMergerNode`*"]
    pub fn create_channel_merger(this: &BaseAudioContext) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(feature = "ChannelMergerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createChannelMerger"
    )]
    #[doc = "The `createChannelMerger()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelMerger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelMergerNode`*"]
    pub fn create_channel_merger_with_number_of_inputs(
        this: &BaseAudioContext,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(feature = "ChannelSplitterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createChannelSplitter"
    )]
    #[doc = "The `createChannelSplitter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelSplitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`*"]
    pub fn create_channel_splitter(this: &BaseAudioContext)
        -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(feature = "ChannelSplitterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createChannelSplitter"
    )]
    #[doc = "The `createChannelSplitter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelSplitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`*"]
    pub fn create_channel_splitter_with_number_of_outputs(
        this: &BaseAudioContext,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(feature = "ConstantSourceNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createConstantSource"
    )]
    #[doc = "The `createConstantSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createConstantSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`*"]
    pub fn create_constant_source(this: &BaseAudioContext) -> Result<ConstantSourceNode, JsValue>;
    #[cfg(feature = "ConvolverNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createConvolver"
    )]
    #[doc = "The `createConvolver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createConvolver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`*"]
    pub fn create_convolver(this: &BaseAudioContext) -> Result<ConvolverNode, JsValue>;
    #[cfg(feature = "DelayNode")]
    #[wasm_bindgen(catch, method, js_class = "BaseAudioContext", js_name = "createDelay")]
    #[doc = "The `createDelay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDelay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`*"]
    pub fn create_delay(this: &BaseAudioContext) -> Result<DelayNode, JsValue>;
    #[cfg(feature = "DelayNode")]
    #[wasm_bindgen(catch, method, js_class = "BaseAudioContext", js_name = "createDelay")]
    #[doc = "The `createDelay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDelay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`*"]
    pub fn create_delay_with_max_delay_time(
        this: &BaseAudioContext,
        max_delay_time: f64,
    ) -> Result<DelayNode, JsValue>;
    #[cfg(feature = "DynamicsCompressorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createDynamicsCompressor"
    )]
    #[doc = "The `createDynamicsCompressor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDynamicsCompressor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`*"]
    pub fn create_dynamics_compressor(
        this: &BaseAudioContext,
    ) -> Result<DynamicsCompressorNode, JsValue>;
    #[cfg(feature = "GainNode")]
    #[wasm_bindgen(catch, method, js_class = "BaseAudioContext", js_name = "createGain")]
    #[doc = "The `createGain()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createGain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`*"]
    pub fn create_gain(this: &BaseAudioContext) -> Result<GainNode, JsValue>;
    #[cfg(feature = "IirFilterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createIIRFilter"
    )]
    #[doc = "The `createIIRFilter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createIIRFilter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `IirFilterNode`*"]
    pub fn create_iir_filter(
        this: &BaseAudioContext,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, JsValue>;
    #[cfg(feature = "OscillatorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createOscillator"
    )]
    #[doc = "The `createOscillator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createOscillator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`*"]
    pub fn create_oscillator(this: &BaseAudioContext) -> Result<OscillatorNode, JsValue>;
    #[cfg(feature = "PannerNode")]
    #[wasm_bindgen(catch, method, js_class = "BaseAudioContext", js_name = "createPanner")]
    #[doc = "The `createPanner()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPanner)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`*"]
    pub fn create_panner(this: &BaseAudioContext) -> Result<PannerNode, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave(
        this: &BaseAudioContext,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_slice(
        this: &BaseAudioContext,
        real: &::js_sys::Float32Array,
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_slice_and_f32_array(
        this: &BaseAudioContext,
        real: &mut [f32],
        imag: &::js_sys::Float32Array,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_array(
        this: &BaseAudioContext,
        real: &::js_sys::Float32Array,
        imag: &::js_sys::Float32Array,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_constraints(
        this: &BaseAudioContext,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_slice_and_constraints(
        this: &BaseAudioContext,
        real: &::js_sys::Float32Array,
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_slice_and_f32_array_and_constraints(
        this: &BaseAudioContext,
        real: &mut [f32],
        imag: &::js_sys::Float32Array,
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_array_and_constraints(
        this: &BaseAudioContext,
        real: &::js_sys::Float32Array,
        imag: &::js_sys::Float32Array,
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor(this: &BaseAudioContext)
        -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size(
        this: &BaseAudioContext,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        this: &BaseAudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        this: &BaseAudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "StereoPannerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createStereoPanner"
    )]
    #[doc = "The `createStereoPanner()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createStereoPanner)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`*"]
    pub fn create_stereo_panner(this: &BaseAudioContext) -> Result<StereoPannerNode, JsValue>;
    #[cfg(feature = "WaveShaperNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "createWaveShaper"
    )]
    #[doc = "The `createWaveShaper()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createWaveShaper)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`*"]
    pub fn create_wave_shaper(this: &BaseAudioContext) -> Result<WaveShaperNode, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "decodeAudioData"
    )]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn decode_audio_data(
        this: &BaseAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "decodeAudioData"
    )]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn decode_audio_data_with_success_callback(
        this: &BaseAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BaseAudioContext",
        js_name = "decodeAudioData"
    )]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        this: &BaseAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "BaseAudioContext")]
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn resume(this: &BaseAudioContext) -> Result<::js_sys::Promise, JsValue>;
}
