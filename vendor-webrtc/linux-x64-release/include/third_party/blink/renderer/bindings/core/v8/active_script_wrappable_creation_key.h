// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ACTIVE_SCRIPT_WRAPPABLE_CREATION_KEY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ACTIVE_SCRIPT_WRAPPABLE_CREATION_KEY_H_

namespace blink {

// Creation key needed to instantiate ActiveScriptWrappable objects.
//
// By adding your class as friend below you acknowledge that you have checked
// alternatives for keeping the object alive, see class comment of
// `ActiveScriptWrappable`.
class ActiveScriptWrappableCreationKey final {
 private:
  // NOLINTNEXTLINE: No =default to disallow aggregate initialization.
  ActiveScriptWrappableCreationKey() {}

  friend class AbortSignal;
  friend class Animation;
  friend class AudioScheduledSourceNode;
  friend class AudioWorkletNode;
  friend class BackgroundFetchRegistration;
  friend class BaseAudioContext;
  friend class BatteryManager;
  friend class BeforeInstallPromptEvent;
  friend class BluetoothDevice;
  friend class BluetoothRemoteGATTCharacteristic;
  friend class BroadcastChannel;
  friend class CacheStorage;
  friend class CanvasRenderingContext;
  template <typename Traits>
  friend class DecoderTemplate;
  friend class DedicatedWorker;
  friend class DocumentTransition;
  friend class DOMFileSystem;
  friend class DOMWebSocket;
  friend class EditContext;
  template <typename Traits>
  friend class EncoderBase;
  friend class EventSource;
  friend class FetchEvent;
  friend class FileReader;
  friend class FileWriter;
  friend class FontFace;
  template <typename NativeFrameType>
  friend class FrameQueueUnderlyingSource;
  friend class Geolocation;
  friend class HIDDevice;
  friend class HTMLImageElement;
  friend class HTMLInputElement;
  friend class HTMLMediaElement;
  friend class HTMLPlugInElement;
  friend class IDBDatabase;
  friend class IDBRequest;
  friend class IDBTransaction;
  friend class IdleDetector;
  friend class ImageDecoderExternal;
  friend class IntersectionObserver;
  friend class MediaElementAudioSourceNode;
  friend class MediaDevices;
  friend class MediaKeys;
  friend class MediaKeySession;
  friend class MediaQueryList;
  friend class MediaSource;
  friend class MediaRecorder;
  friend class MediaStream;
  friend class MediaStreamAudioDestinationNode;
  friend class MediaStreamAudioSourceNode;
  friend class MediaStreamTrack;
  friend class MessagePort;
  friend class MIDIAccess;
  friend class MIDIPort;
  friend class MojoInterfaceInterceptor;
  friend class MojoWatcher;
  friend class MutationObserver;
  friend class NavigatorManagedData;
  friend class NDEFReader;
  friend class NetworkInformation;
  friend class Notification;
  friend class PaymentRequest;
  friend class PaymentResponse;
  friend class PerformanceObserver;
  friend class PermissionStatus;
  friend class PictureInPictureWindow;
  friend class PresentationAvailability;
  friend class PresentationRequest;
  friend class ReadableStreamDefaultReader;
  friend class RemotePlayback;
  friend class ReportingObserver;
  friend class ResizeObserver;
  friend class RTCDataChannel;
  friend class RTCIceTransport;
  friend class RTCPeerConnection;
  friend class ScriptProcessorNode;
  friend class Sensor;
  friend class SerialPort;
  friend class ServiceWorker;
  friend class ServiceWorkerRegistration;
  friend class SharedWorker;
  friend class SmartCardReader;
  friend class SmartCardReaderPresenceObserver;
  friend class SourceBuffer;
  friend class SpeechRecognition;
  friend class SVGImageElement;
  friend class TCPSocket;
  friend class UDPSocket;
  friend class WakeLockSentinel;
  friend class WebPrintJob;
  friend class WebSocketStream;
  friend class WebTransport;
  friend class WorkerGlobalScope;
  friend class WorkletGlobalScope;
  friend class XMLHttpRequest;
  friend class XRSession;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ACTIVE_SCRIPT_WRAPPABLE_CREATION_KEY_H_
