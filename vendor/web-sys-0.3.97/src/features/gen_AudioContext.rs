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
        js_name = "AudioContext",
        typescript_type = "AudioContext"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub type AudioContext;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "AudioContext", js_name = "sinkId")]
    #[doc = "Getter for the `sinkId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/sinkId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sink_id(this: &AudioContext) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "AudioContext", js_name = "onsinkchange")]
    #[doc = "Getter for the `onsinkchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onsinkchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onsinkchange(this: &AudioContext) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "AudioContext", js_name = "onsinkchange")]
    #[doc = "Setter for the `onsinkchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onsinkchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onsinkchange(this: &AudioContext, value: Option<&::js_sys::Function>);
    #[cfg(feature = "AudioDestinationNode")]
    #[wasm_bindgen(method, getter, js_class = "AudioContext", js_name = "destination")]
    #[doc = "Getter for the `destination` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/destination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `AudioDestinationNode`*"]
    pub fn destination(this: &AudioContext) -> AudioDestinationNode;
    #[wasm_bindgen(method, getter, js_class = "AudioContext", js_name = "sampleRate")]
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn sample_rate(this: &AudioContext) -> f32;
    #[wasm_bindgen(method, getter, js_class = "AudioContext", js_name = "currentTime")]
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn current_time(this: &AudioContext) -> f64;
    #[cfg(feature = "AudioListener")]
    #[wasm_bindgen(method, getter, js_class = "AudioContext", js_name = "listener")]
    #[doc = "Getter for the `listener` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/listener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `AudioListener`*"]
    pub fn listener(this: &AudioContext) -> AudioListener;
    #[cfg(feature = "AudioContextState")]
    #[wasm_bindgen(method, getter, js_class = "AudioContext", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `AudioContextState`*"]
    pub fn state(this: &AudioContext) -> AudioContextState;
    #[cfg(feature = "AudioWorklet")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "AudioContext",
        js_name = "audioWorklet"
    )]
    #[doc = "Getter for the `audioWorklet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/audioWorklet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `AudioWorklet`*"]
    pub fn audio_worklet(this: &AudioContext) -> Result<AudioWorklet, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "AudioContext", js_name = "onstatechange")]
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn onstatechange(this: &AudioContext) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "AudioContext", js_name = "onstatechange")]
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn set_onstatechange(this: &AudioContext, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "AudioContext")]
    #[doc = "The `new AudioContext(..)` constructor, creating a new instance of `AudioContext`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/AudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn new() -> Result<AudioContext, JsValue>;
    #[cfg(feature = "AudioContextOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioContext")]
    #[doc = "The `new AudioContext(..)` constructor, creating a new instance of `AudioContext`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/AudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `AudioContextOptions`*"]
    pub fn new_with_context_options(
        context_options: &AudioContextOptions,
    ) -> Result<AudioContext, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioContext")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn close(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "HtmlMediaElement", feature = "MediaElementAudioSourceNode",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createMediaElementSource"
    )]
    #[doc = "The `createMediaElementSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaElementSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `HtmlMediaElement`, `MediaElementAudioSourceNode`*"]
    pub fn create_media_element_source(
        this: &AudioContext,
        media_element: &HtmlMediaElement,
    ) -> Result<MediaElementAudioSourceNode, JsValue>;
    #[cfg(feature = "MediaStreamAudioDestinationNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createMediaStreamDestination"
    )]
    #[doc = "The `createMediaStreamDestination()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamDestination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioDestinationNode`*"]
    pub fn create_media_stream_destination(
        this: &AudioContext,
    ) -> Result<MediaStreamAudioDestinationNode, JsValue>;
    #[cfg(all(feature = "MediaStream", feature = "MediaStreamAudioSourceNode",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createMediaStreamSource"
    )]
    #[doc = "The `createMediaStreamSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `MediaStream`, `MediaStreamAudioSourceNode`*"]
    pub fn create_media_stream_source(
        this: &AudioContext,
        media_stream: &MediaStream,
    ) -> Result<MediaStreamAudioSourceNode, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "AudioContext", js_name = "setSinkId")]
    #[doc = "The `setSinkId()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/setSinkId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_sink_id_with_str(
        this: &AudioContext,
        sink_id: &str,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSinkOptions")]
    #[wasm_bindgen(method, js_class = "AudioContext", js_name = "setSinkId")]
    #[doc = "The `setSinkId()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/setSinkId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `AudioSinkOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_sink_id_with_audio_sink_options(
        this: &AudioContext,
        sink_id: &AudioSinkOptions,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[wasm_bindgen(catch, method, js_class = "AudioContext")]
    #[doc = "The `suspend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/suspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn suspend(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "AnalyserNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createAnalyser")]
    #[doc = "The `createAnalyser()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createAnalyser)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`, `AudioContext`*"]
    pub fn create_analyser(this: &AudioContext) -> Result<AnalyserNode, JsValue>;
    #[cfg(feature = "BiquadFilterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createBiquadFilter"
    )]
    #[doc = "The `createBiquadFilter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBiquadFilter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `BiquadFilterNode`*"]
    pub fn create_biquad_filter(this: &AudioContext) -> Result<BiquadFilterNode, JsValue>;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createBuffer")]
    #[doc = "The `createBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioContext`*"]
    pub fn create_buffer(
        this: &AudioContext,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, JsValue>;
    #[cfg(feature = "AudioBufferSourceNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createBufferSource"
    )]
    #[doc = "The `createBufferSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBufferSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioContext`*"]
    pub fn create_buffer_source(this: &AudioContext) -> Result<AudioBufferSourceNode, JsValue>;
    #[cfg(feature = "ChannelMergerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createChannelMerger"
    )]
    #[doc = "The `createChannelMerger()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelMerger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ChannelMergerNode`*"]
    pub fn create_channel_merger(this: &AudioContext) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(feature = "ChannelMergerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createChannelMerger"
    )]
    #[doc = "The `createChannelMerger()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelMerger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ChannelMergerNode`*"]
    pub fn create_channel_merger_with_number_of_inputs(
        this: &AudioContext,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(feature = "ChannelSplitterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createChannelSplitter"
    )]
    #[doc = "The `createChannelSplitter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelSplitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ChannelSplitterNode`*"]
    pub fn create_channel_splitter(this: &AudioContext) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(feature = "ChannelSplitterNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createChannelSplitter"
    )]
    #[doc = "The `createChannelSplitter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelSplitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ChannelSplitterNode`*"]
    pub fn create_channel_splitter_with_number_of_outputs(
        this: &AudioContext,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(feature = "ConstantSourceNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createConstantSource"
    )]
    #[doc = "The `createConstantSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createConstantSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ConstantSourceNode`*"]
    pub fn create_constant_source(this: &AudioContext) -> Result<ConstantSourceNode, JsValue>;
    #[cfg(feature = "ConvolverNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createConvolver")]
    #[doc = "The `createConvolver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createConvolver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ConvolverNode`*"]
    pub fn create_convolver(this: &AudioContext) -> Result<ConvolverNode, JsValue>;
    #[cfg(feature = "DelayNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createDelay")]
    #[doc = "The `createDelay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDelay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `DelayNode`*"]
    pub fn create_delay(this: &AudioContext) -> Result<DelayNode, JsValue>;
    #[cfg(feature = "DelayNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createDelay")]
    #[doc = "The `createDelay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDelay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `DelayNode`*"]
    pub fn create_delay_with_max_delay_time(
        this: &AudioContext,
        max_delay_time: f64,
    ) -> Result<DelayNode, JsValue>;
    #[cfg(feature = "DynamicsCompressorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createDynamicsCompressor"
    )]
    #[doc = "The `createDynamicsCompressor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDynamicsCompressor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `DynamicsCompressorNode`*"]
    pub fn create_dynamics_compressor(
        this: &AudioContext,
    ) -> Result<DynamicsCompressorNode, JsValue>;
    #[cfg(feature = "GainNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createGain")]
    #[doc = "The `createGain()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createGain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `GainNode`*"]
    pub fn create_gain(this: &AudioContext) -> Result<GainNode, JsValue>;
    #[cfg(feature = "IirFilterNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createIIRFilter")]
    #[doc = "The `createIIRFilter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createIIRFilter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `IirFilterNode`*"]
    pub fn create_iir_filter(
        this: &AudioContext,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, JsValue>;
    #[cfg(feature = "OscillatorNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createOscillator")]
    #[doc = "The `createOscillator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createOscillator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `OscillatorNode`*"]
    pub fn create_oscillator(this: &AudioContext) -> Result<OscillatorNode, JsValue>;
    #[cfg(feature = "PannerNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createPanner")]
    #[doc = "The `createPanner()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPanner)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PannerNode`*"]
    pub fn create_panner(this: &AudioContext) -> Result<PannerNode, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave(
        this: &AudioContext,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_slice(
        this: &AudioContext,
        real: &::js_sys::Float32Array,
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_slice_and_f32_array(
        this: &AudioContext,
        real: &mut [f32],
        imag: &::js_sys::Float32Array,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_array(
        this: &AudioContext,
        real: &::js_sys::Float32Array,
        imag: &::js_sys::Float32Array,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_constraints(
        this: &AudioContext,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_slice_and_constraints(
        this: &AudioContext,
        real: &::js_sys::Float32Array,
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_slice_and_f32_array_and_constraints(
        this: &AudioContext,
        real: &mut [f32],
        imag: &::js_sys::Float32Array,
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createPeriodicWave"
    )]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_f32_array_and_f32_array_and_constraints(
        this: &AudioContext,
        real: &::js_sys::Float32Array,
        imag: &::js_sys::Float32Array,
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor(this: &AudioContext) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size(
        this: &AudioContext,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        this: &AudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createScriptProcessor"
    )]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        this: &AudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "StereoPannerNode")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioContext",
        js_name = "createStereoPanner"
    )]
    #[doc = "The `createStereoPanner()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createStereoPanner)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `StereoPannerNode`*"]
    pub fn create_stereo_panner(this: &AudioContext) -> Result<StereoPannerNode, JsValue>;
    #[cfg(feature = "WaveShaperNode")]
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "createWaveShaper")]
    #[doc = "The `createWaveShaper()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createWaveShaper)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `WaveShaperNode`*"]
    pub fn create_wave_shaper(this: &AudioContext) -> Result<WaveShaperNode, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "decodeAudioData")]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn decode_audio_data(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "decodeAudioData")]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn decode_audio_data_with_success_callback(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioContext", js_name = "decodeAudioData")]
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioContext")]
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn resume(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;
}
