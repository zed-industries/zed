#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "RTCPeerConnection",
        typescript_type = "RTCPeerConnection"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcPeerConnection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub type RtcPeerConnection;
    #[cfg(feature = "RtcSessionDescription")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "localDescription"
    )]
    #[doc = "Getter for the `localDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/localDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn local_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    #[cfg(feature = "RtcSessionDescription")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "currentLocalDescription"
    )]
    #[doc = "Getter for the `currentLocalDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentLocalDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn current_local_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    #[cfg(feature = "RtcSessionDescription")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "pendingLocalDescription"
    )]
    #[doc = "Getter for the `pendingLocalDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingLocalDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn pending_local_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    #[cfg(feature = "RtcSessionDescription")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "remoteDescription"
    )]
    #[doc = "Getter for the `remoteDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/remoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn remote_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    #[cfg(feature = "RtcSessionDescription")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "currentRemoteDescription"
    )]
    #[doc = "Getter for the `currentRemoteDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentRemoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn current_remote_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    #[cfg(feature = "RtcSessionDescription")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "pendingRemoteDescription"
    )]
    #[doc = "Getter for the `pendingRemoteDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingRemoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn pending_remote_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    #[cfg(feature = "RtcSignalingState")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "signalingState"
    )]
    #[doc = "Getter for the `signalingState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/signalingState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSignalingState`*"]
    pub fn signaling_state(this: &RtcPeerConnection) -> RtcSignalingState;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "canTrickleIceCandidates"
    )]
    #[doc = "Getter for the `canTrickleIceCandidates` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/canTrickleIceCandidates)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn can_trickle_ice_candidates(this: &RtcPeerConnection) -> Option<bool>;
    #[cfg(feature = "RtcIceGatheringState")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "iceGatheringState"
    )]
    #[doc = "Getter for the `iceGatheringState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceGatheringState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceGatheringState`, `RtcPeerConnection`*"]
    pub fn ice_gathering_state(this: &RtcPeerConnection) -> RtcIceGatheringState;
    #[cfg(feature = "RtcIceConnectionState")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "iceConnectionState"
    )]
    #[doc = "Getter for the `iceConnectionState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceConnectionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceConnectionState`, `RtcPeerConnection`*"]
    pub fn ice_connection_state(this: &RtcPeerConnection) -> RtcIceConnectionState;
    #[cfg(feature = "RtcPeerConnectionState")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "connectionState"
    )]
    #[doc = "Getter for the `connectionState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/connectionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcPeerConnectionState`*"]
    pub fn connection_state(this: &RtcPeerConnection) -> RtcPeerConnectionState;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "peerIdentity"
    )]
    #[doc = "Getter for the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/peerIdentity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn peer_identity(this: &RtcPeerConnection) -> ::js_sys::Promise;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "idpLoginUrl"
    )]
    #[doc = "Getter for the `idpLoginUrl` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/idpLoginUrl)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn idp_login_url(this: &RtcPeerConnection) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "onnegotiationneeded"
    )]
    #[doc = "Getter for the `onnegotiationneeded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onnegotiationneeded(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "onnegotiationneeded"
    )]
    #[doc = "Setter for the `onnegotiationneeded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onnegotiationneeded(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "onicecandidate"
    )]
    #[doc = "Getter for the `onicecandidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onicecandidate(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "onicecandidate"
    )]
    #[doc = "Setter for the `onicecandidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onicecandidate(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "onsignalingstatechange"
    )]
    #[doc = "Getter for the `onsignalingstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onsignalingstatechange(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "onsignalingstatechange"
    )]
    #[doc = "Setter for the `onsignalingstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onsignalingstatechange(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "onaddstream"
    )]
    #[doc = "Getter for the `onaddstream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddstream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onaddstream(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "onaddstream"
    )]
    #[doc = "Setter for the `onaddstream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddstream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onaddstream(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "RTCPeerConnection", js_name = "onaddtrack")]
    #[doc = "Getter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onaddtrack(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "RTCPeerConnection", js_name = "onaddtrack")]
    #[doc = "Setter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onaddtrack(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "RTCPeerConnection", js_name = "ontrack")]
    #[doc = "Getter for the `ontrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn ontrack(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "RTCPeerConnection", js_name = "ontrack")]
    #[doc = "Setter for the `ontrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_ontrack(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "onremovestream"
    )]
    #[doc = "Getter for the `onremovestream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onremovestream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onremovestream(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "onremovestream"
    )]
    #[doc = "Setter for the `onremovestream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onremovestream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onremovestream(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "oniceconnectionstatechange"
    )]
    #[doc = "Getter for the `oniceconnectionstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn oniceconnectionstatechange(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "oniceconnectionstatechange"
    )]
    #[doc = "Setter for the `oniceconnectionstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_oniceconnectionstatechange(
        this: &RtcPeerConnection,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "onicegatheringstatechange"
    )]
    #[doc = "Getter for the `onicegatheringstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onicegatheringstatechange(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "onicegatheringstatechange"
    )]
    #[doc = "Setter for the `onicegatheringstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onicegatheringstatechange(
        this: &RtcPeerConnection,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "onconnectionstatechange"
    )]
    #[doc = "Getter for the `onconnectionstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onconnectionstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onconnectionstatechange(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "onconnectionstatechange"
    )]
    #[doc = "Setter for the `onconnectionstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onconnectionstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onconnectionstatechange(
        this: &RtcPeerConnection,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnection",
        js_name = "ondatachannel"
    )]
    #[doc = "Getter for the `ondatachannel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn ondatachannel(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCPeerConnection",
        js_name = "ondatachannel"
    )]
    #[doc = "Setter for the `ondatachannel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_ondatachannel(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "RTCPeerConnection")]
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn new() -> Result<RtcPeerConnection, JsValue>;
    #[cfg(feature = "RtcConfiguration")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCPeerConnection")]
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    pub fn new_with_configuration(
        configuration: &RtcConfiguration,
    ) -> Result<RtcPeerConnection, JsValue>;
    #[cfg(feature = "RtcConfiguration")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCPeerConnection")]
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    pub fn new_with_configuration_and_constraints(
        configuration: &RtcConfiguration,
        constraints: Option<&::js_sys::Object>,
    ) -> Result<RtcPeerConnection, JsValue>;
    #[cfg(feature = "RtcIceCandidateInit")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addIceCandidate")]
    #[doc = "The `addIceCandidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`, `RtcPeerConnection`*"]
    pub fn add_ice_candidate_with_opt_rtc_ice_candidate_init(
        this: &RtcPeerConnection,
        candidate: Option<&RtcIceCandidateInit>,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcIceCandidate")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addIceCandidate")]
    #[doc = "The `addIceCandidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnection`*"]
    pub fn add_ice_candidate_with_opt_rtc_ice_candidate(
        this: &RtcPeerConnection,
        candidate: Option<&RtcIceCandidate>,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcIceCandidate")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addIceCandidate")]
    #[doc = "The `addIceCandidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnection`*"]
    pub fn add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        candidate: &RtcIceCandidate,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addStream")]
    #[doc = "The `addStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `RtcPeerConnection`*"]
    pub fn add_stream(this: &RtcPeerConnection, stream: &MediaStream);
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, variadic, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams: &::js_sys::Array,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_0(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_1(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_2(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_3(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_4(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_5(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_6(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
        more_streams_6: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_7(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
        more_streams_6: &MediaStream,
        more_streams_7: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpTransceiver",))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTransceiver")]
    #[doc = "The `addTransceiver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpTransceiver`*"]
    pub fn add_transceiver_with_media_stream_track(
        this: &RtcPeerConnection,
        track_or_kind: &MediaStreamTrack,
    ) -> RtcRtpTransceiver;
    #[cfg(feature = "RtcRtpTransceiver")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTransceiver")]
    #[doc = "The `addTransceiver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpTransceiver`*"]
    pub fn add_transceiver_with_str(
        this: &RtcPeerConnection,
        track_or_kind: &str,
    ) -> RtcRtpTransceiver;
    #[cfg(all(
        feature = "MediaStreamTrack",
        feature = "RtcRtpTransceiver",
        feature = "RtcRtpTransceiverInit",
    ))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTransceiver")]
    #[doc = "The `addTransceiver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpTransceiver`, `RtcRtpTransceiverInit`*"]
    pub fn add_transceiver_with_media_stream_track_and_init(
        this: &RtcPeerConnection,
        track_or_kind: &MediaStreamTrack,
        init: &RtcRtpTransceiverInit,
    ) -> RtcRtpTransceiver;
    #[cfg(all(feature = "RtcRtpTransceiver", feature = "RtcRtpTransceiverInit",))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "addTransceiver")]
    #[doc = "The `addTransceiver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpTransceiver`, `RtcRtpTransceiverInit`*"]
    pub fn add_transceiver_with_str_and_init(
        this: &RtcPeerConnection,
        track_or_kind: &str,
        init: &RtcRtpTransceiverInit,
    ) -> RtcRtpTransceiver;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn close(this: &RtcPeerConnection);
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createAnswer")]
    #[doc = "The `createAnswer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn create_answer(this: &RtcPeerConnection) -> ::js_sys::Promise;
    #[cfg(feature = "RtcAnswerOptions")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createAnswer")]
    #[doc = "The `createAnswer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcAnswerOptions`, `RtcPeerConnection`*"]
    pub fn create_answer_with_rtc_answer_options(
        this: &RtcPeerConnection,
        options: &RtcAnswerOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createAnswer")]
    #[doc = "The `createAnswer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn create_answer_with_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcDataChannel")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createDataChannel")]
    #[doc = "The `createDataChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcPeerConnection`*"]
    pub fn create_data_channel(this: &RtcPeerConnection, label: &str) -> RtcDataChannel;
    #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelInit",))]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createDataChannel")]
    #[doc = "The `createDataChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelInit`, `RtcPeerConnection`*"]
    pub fn create_data_channel_with_data_channel_dict(
        this: &RtcPeerConnection,
        label: &str,
        data_channel_dict: &RtcDataChannelInit,
    ) -> RtcDataChannel;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createOffer")]
    #[doc = "The `createOffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn create_offer(this: &RtcPeerConnection) -> ::js_sys::Promise;
    #[cfg(feature = "RtcOfferOptions")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createOffer")]
    #[doc = "The `createOffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`, `RtcPeerConnection`*"]
    pub fn create_offer_with_rtc_offer_options(
        this: &RtcPeerConnection,
        options: &RtcOfferOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createOffer")]
    #[doc = "The `createOffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn create_offer_with_callback_and_failure_callback(
        this: &RtcPeerConnection,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcOfferOptions")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "createOffer")]
    #[doc = "The `createOffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`, `RtcPeerConnection`*"]
    pub fn create_offer_with_callback_and_failure_callback_and_options(
        this: &RtcPeerConnection,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
        options: &RtcOfferOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(
        catch,
        static_method_of = "RtcPeerConnection",
        js_class = "RTCPeerConnection",
        js_name = "generateCertificate"
    )]
    #[doc = "The `generateCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn generate_certificate_with_object(
        keygen_algorithm: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "RtcPeerConnection",
        js_class = "RTCPeerConnection",
        js_name = "generateCertificate"
    )]
    #[doc = "The `generateCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn generate_certificate_with_str(
        keygen_algorithm: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "RtcConfiguration")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getConfiguration")]
    #[doc = "The `getConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    pub fn get_configuration(this: &RtcPeerConnection) -> RtcConfiguration;
    #[wasm_bindgen(
        method,
        js_class = "RTCPeerConnection",
        js_name = "getIdentityAssertion"
    )]
    #[doc = "The `getIdentityAssertion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getIdentityAssertion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_identity_assertion(this: &RtcPeerConnection) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getLocalStreams")]
    #[doc = "The `getLocalStreams()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getLocalStreams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_local_streams(this: &RtcPeerConnection) -> ::js_sys::Array;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getReceivers")]
    #[doc = "The `getReceivers()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getReceivers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_receivers(this: &RtcPeerConnection) -> ::js_sys::Array;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getRemoteStreams")]
    #[doc = "The `getRemoteStreams()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getRemoteStreams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_remote_streams(this: &RtcPeerConnection) -> ::js_sys::Array;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getSenders")]
    #[doc = "The `getSenders()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getSenders)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_senders(this: &RtcPeerConnection) -> ::js_sys::Array;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getStats")]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_stats(this: &RtcPeerConnection) -> ::js_sys::Promise;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getStats")]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`*"]
    pub fn get_stats_with_selector(
        this: &RtcPeerConnection,
        selector: Option<&MediaStreamTrack>,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getStats")]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`*"]
    pub fn get_stats_with_selector_and_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        selector: Option<&MediaStreamTrack>,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "getTransceivers")]
    #[doc = "The `getTransceivers()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getTransceivers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_transceivers(this: &RtcPeerConnection) -> ::js_sys::Array;
    #[cfg(feature = "RtcRtpSender")]
    #[wasm_bindgen(method, js_class = "RTCPeerConnection", js_name = "removeTrack")]
    #[doc = "The `removeTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/removeTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn remove_track(this: &RtcPeerConnection, sender: &RtcRtpSender);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "RTCPeerConnection",
        js_name = "setConfiguration"
    )]
    #[doc = "The `setConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_configuration(this: &RtcPeerConnection) -> Result<(), JsValue>;
    #[cfg(feature = "RtcConfiguration")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "RTCPeerConnection",
        js_name = "setConfiguration"
    )]
    #[doc = "The `setConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    pub fn set_configuration_with_configuration(
        this: &RtcPeerConnection,
        configuration: &RtcConfiguration,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "RTCPeerConnection",
        js_name = "setIdentityProvider"
    )]
    #[doc = "The `setIdentityProvider()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_identity_provider(this: &RtcPeerConnection, provider: &str);
    #[cfg(feature = "RtcIdentityProviderOptions")]
    #[wasm_bindgen(
        method,
        js_class = "RTCPeerConnection",
        js_name = "setIdentityProvider"
    )]
    #[doc = "The `setIdentityProvider()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`, `RtcPeerConnection`*"]
    pub fn set_identity_provider_with_options(
        this: &RtcPeerConnection,
        provider: &str,
        options: &RtcIdentityProviderOptions,
    );
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[wasm_bindgen(
        method,
        js_class = "RTCPeerConnection",
        js_name = "setLocalDescription"
    )]
    #[doc = "The `setLocalDescription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    pub fn set_local_description(
        this: &RtcPeerConnection,
        description: &RtcSessionDescriptionInit,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[wasm_bindgen(
        method,
        js_class = "RTCPeerConnection",
        js_name = "setLocalDescription"
    )]
    #[doc = "The `setLocalDescription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    pub fn set_local_description_with_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        description: &RtcSessionDescriptionInit,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[wasm_bindgen(
        method,
        js_class = "RTCPeerConnection",
        js_name = "setRemoteDescription"
    )]
    #[doc = "The `setRemoteDescription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    pub fn set_remote_description(
        this: &RtcPeerConnection,
        description: &RtcSessionDescriptionInit,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[wasm_bindgen(
        method,
        js_class = "RTCPeerConnection",
        js_name = "setRemoteDescription"
    )]
    #[doc = "The `setRemoteDescription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    pub fn set_remote_description_with_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        description: &RtcSessionDescriptionInit,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
}
