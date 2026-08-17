/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_PLATFORM_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_PLATFORM_H_

#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <tuple>
#include <vector>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/platform_thread.h"
#include "base/time/time.h"
#include "build/build_config.h"
#include "cc/trees/raster_context_provider_wrapper.h"
#include "components/viz/common/surfaces/frame_sink_id.h"
#include "media/base/audio_capturer_source.h"
#include "media/base/audio_latency.h"
#include "media/base/audio_renderer_sink.h"
#include "third_party/blink/public/common/security/protocol_handler_security_level.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/peerconnection/webrtc_ip_handling_policy.mojom-forward.h"
#include "third_party/blink/public/platform/audio/web_audio_device_source_type.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/url_loader_throttle_provider.h"
#include "third_party/blink/public/platform/web_audio_device.h"
#include "third_party/blink/public/platform/web_data.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_v8_value_converter.h"
#include "third_party/blink/public/platform/websocket_handshake_throttle_provider.h"
#include "ui/base/resource/resource_scale_factor.h"
#include "ui/gl/angle_implementation.h"
#include "v8/include/v8-local-handle.h"

class SkCanvas;
class SkBitmap;

namespace base {
class SingleThreadTaskRunner;
class RefCountedMemory;
}  // namespace base

namespace cc {
class RasterDarkModeFilter;
}

namespace gfx {
class ColorSpace;
}

namespace gpu {
class GpuChannelHost;
}

namespace media {
struct AudioSinkParameters;
struct AudioSourceParameters;
class DecoderFactory;
class MediaLog;
class MediaPermission;
class GpuVideoAcceleratorFactories;
}  // namespace media

namespace net {
class SchemefulSite;
}

namespace network {
namespace mojom {
class URLLoaderFactory;
}
class PendingSharedURLLoaderFactory;
}

namespace url {
class Origin;
}

namespace v8 {
class Context;
}  // namespace v8

namespace viz {
class RasterContextProvider;
}

namespace blink {

class BrowserInterfaceBrokerProxy;
class MediaInspectorContext;
class MainThread;
class ThreadSafeBrowserInterfaceBrokerProxy;
class URLLoaderThrottle;
class UserMetricsAction;
class WebAudioBus;
class WebAudioLatencyHint;
class WebAudioSinkDescriptor;
class WebCrypto;
class WebDedicatedWorker;
class WebDedicatedWorkerHostFactoryClient;
class WebGraphicsContext3DProvider;
class WebLocalFrame;
class WebSandboxSupport;
class WebSecurityOrigin;
class WebThemeEngine;
class WebVideoCaptureImplManager;
struct WebContentSecurityPolicyHeader;

namespace scheduler {
class WebThreadScheduler;
}

namespace mojom {
class ServiceWorkerContainerHostInterfaceBase;
}

class BLINK_PLATFORM_EXPORT Platform {
 public:
  // Initialize platform and wtf. If you need to initialize the entire Blink,
  // you should use blink::Initialize. WebThreadScheduler must be owned by
  // the embedder. InitializeBlink must be called before WebThreadScheduler is
  // created and passed to InitializeMainThread.
  static void InitializeBlink();
  static void InitializeMainThread(
      Platform*,
      scheduler::WebThreadScheduler* main_thread_scheduler);
  static Platform* Current();

  // This is another entry point for embedders that only require simple
  // execution environment of Blink. This version automatically sets up Blink
  // with a minimally viable implementation of WebThreadScheduler and
  // blink::Thread for the main thread.
  //
  // TODO(yutak): Fix function name as it seems obsolete at this point.
  static void CreateMainThreadAndInitialize(Platform*);

  // Used to switch the current platform only for testing.
  // You should not pass in a Platform object that is not fully instantiated.
  //
  // NOTE: Instead of calling this directly, us a ScopedTestingPlatformSupport
  // which will restore the previous platform on exit, preventing tests from
  // clobbering each other.
  static void SetCurrentPlatformForTesting(Platform*);

  // This sets up a minimally viable implementation of blink::Thread without
  // changing the current Platform. This is essentially a workaround for the
  // initialization order in ScopedUnittestsEnvironmentSetup, and nobody else
  // should use this.
  static void CreateMainThreadForTesting();

  // These are dirty workaround for tests requiring the main thread task runner
  // from a non-main thread. If your test needs base::TaskEnvironment
  // and a non-main thread may call MainThread()->GetTaskRunner(), call
  // SetMainThreadTaskRunnerForTesting() in your test fixture's SetUp(), and
  // call UnsetMainThreadTaskRunnerForTesting() in TearDown().
  //
  // TODO(yutak): Ideally, these should be packed in a custom test fixture
  // along with TaskEnvironment for reusability.
  static void SetMainThreadTaskRunnerForTesting();
  static void UnsetMainThreadTaskRunnerForTesting();

  Platform();
  virtual ~Platform();

  // May return null if sandbox support is not necessary
  virtual WebSandboxSupport* GetSandboxSupport() { return nullptr; }

  // Returns a theme engine. Should be non-null.
  WebThemeEngine* ThemeEngine();

  // Audio --------------------------------------------------------------

  virtual double AudioHardwareSampleRate() { return 0; }
  virtual size_t AudioHardwareBufferSize() { return 0; }
  virtual unsigned AudioHardwareOutputChannels() { return 0; }
  virtual base::TimeDelta GetHungRendererDelay() { return base::TimeDelta(); }

  // Creates an audio output device platform interface for Web Audio API.
  virtual std::unique_ptr<WebAudioDevice> CreateAudioDevice(
      const WebAudioSinkDescriptor& sink_descriptor,
      unsigned number_of_output_channels,
      const WebAudioLatencyHint& latency_hint,
      std::optional<float> context_sample_rate,
      media::AudioRendererSink::RenderCallback*) {
    return nullptr;
  }

  // IDN conversion ------------------------------------------------------

  virtual WebString ConvertIDNToUnicode(const WebString& host) { return host; }

  // History -------------------------------------------------------------

  // Returns the hash for the given canonicalized URL for use in visited
  // link coloring.
  virtual uint64_t VisitedLinkHash(std::string_view canonical_url) { return 0; }

  // Returns the hash for the given triple-partition key for use in partitioned
  // visited link coloring.
  virtual uint64_t PartitionedVisitedLinkFingerprint(
      std::string_view canonical_link_url,
      const net::SchemefulSite& top_level_site,
      const WebSecurityOrigin& frame_origin) {
    // Return the null-fingerprint value.
    return 0;
  }

  // Returns whether the given link hash is in the user's history. The
  // hash must have been generated by calling VisitedLinkHash().
  virtual bool IsLinkVisited(uint64_t link_hash) { return false; }

  // Each render process has an associated VisitedLinkReader, where all
  // per-origin salts for the process are stored. Documents that receive a salt
  // via the VisitedLinkNavigationThrottle should notify the VisitedLinkReader
  // so its value can be stored in its canonical `salts_` map.
  virtual void AddOrUpdateVisitedLinkSalt(const url::Origin& origin,
                                          uint64_t salt) {}

  // Keep it the same as ImageDecoder::kNoDecodedImageByteLimit
  static const size_t kNoDecodedImageByteLimit = static_cast<size_t>(-1);

  // Returns the maximum amount of memory a decoded image should be allowed.
  // See comments on ImageDecoder::max_decoded_bytes_.
  virtual size_t MaxDecodedImageBytes() { return kNoDecodedImageByteLimit; }

  // Returns MaxDecodedImageBytes() from the current platform. If the current
  // platform is not available, it returns |kNoDecodedImageByteLimit| as
  // default instead;
  static size_t GetMaxDecodedImageBytes();

  // See: SysUtils::IsLowEndDevice for the full details of what "low-end" means.
  // This returns true for devices that can use more extreme tradeoffs for
  // performance. Many low memory devices (<=1GB) are not considered low-end.
  virtual bool IsLowEndDevice() { return false; }

  // Process -------------------------------------------------------------

  // Returns a unique FrameSinkID for the current renderer process
  virtual viz::FrameSinkId GenerateFrameSinkId() { return viz::FrameSinkId(); }

  // Returns whether this process is locked to a single site (i.e. a scheme
  // plus eTLD+1, such as https://google.com), or to a more specific origin.
  // This means the process will not be used to load documents or workers from
  // URLs outside that site.
  virtual bool IsLockedToSite() const { return false; }

  // Network -------------------------------------------------------------

  // Returns the default User-Agent string, it can either full User-Agent string
  // or reduced User-Agent string based on policy setting.
  virtual WebString UserAgent() { return WebString(); }

  // Returns the User Agent metadata. This will replace `UserAgent()` if we
  // end up shipping https://github.com/WICG/ua-client-hints.
  virtual blink::UserAgentMetadata UserAgentMetadata() {
    return blink::UserAgentMetadata();
  }

  // Determines whether it is safe to redirect from |from_url| to |to_url|.
  virtual bool IsRedirectSafe(const GURL& from_url, const GURL& to_url) {
    return false;
  }

  // Appends throttles if the browser has sent a variations header to the
  // renderer.
  virtual void AppendVariationsThrottles(
      const url::Origin& top_origin,
      std::vector<std::unique_ptr<blink::URLLoaderThrottle>>* throttles) {}

  // Allows the embedder to return a (possibly null)
  // blink::URLLoaderThrottleProvider for a worker.
  virtual std::unique_ptr<URLLoaderThrottleProvider>
  CreateURLLoaderThrottleProviderForWorker(
      URLLoaderThrottleProviderType provider_type) {
    return nullptr;
  }

  // Allows the embedder to provide a WebSocketHandshakeThrottleProvider. If it
  // returns nullptr then none will be used.
  virtual std::unique_ptr<WebSocketHandshakeThrottleProvider>
  CreateWebSocketHandshakeThrottleProvider() {
    return nullptr;
  }

  // Whether or not newly created Isolates are indicated to be in the background
  // or not.
  virtual bool IsolateStartsInBackground() { return false; }

  // Allows the embedder to control whether the renderer should leverage the
  // compiled code cache with hashing for a given `request_url`.
  virtual bool ShouldUseCodeCacheWithHashing(const WebURL& request_url) const {
    return true;
  }

  // Resources -----------------------------------------------------------

  // Returns a localized string resource (with substitution parameters).
  virtual WebString QueryLocalizedString(int resource_id) {
    return WebString();
  }
  virtual WebString QueryLocalizedString(int resource_id,
                                         const WebString& parameter) {
    return WebString();
  }
  virtual WebString QueryLocalizedString(int resource_id,
                                         const WebString& parameter1,
                                         const WebString& parameter2) {
    return WebString();
  }

  // SavableResource ----------------------------------------------------

  virtual bool IsURLSavableForSavableResource(const WebURL& url) {
    return false;
  }

  // Threads -------------------------------------------------------

  // The two compositor-related functions below are called by the embedder.
  // TODO(yutak): Perhaps we should move these to somewhere else?

  // Create and initialize the compositor thread. After this function
  // completes, you can access CompositorThreadTaskRunner().
  void CreateAndSetCompositorThread();

  // Returns the task runner of the compositor thread. This is available
  // once CreateAndSetCompositorThread() is called.
  scoped_refptr<base::SingleThreadTaskRunner> CompositorThreadTaskRunner();

  // Returns the video frame compositor thread task runner. This may
  // conditionally be the same as the compositor thread task runner.
  virtual scoped_refptr<base::SingleThreadTaskRunner>
  VideoFrameCompositorTaskRunner() {
    return CompositorThreadTaskRunner();
  }

  // Returns the task runner of the media thread.
  // This method should only be called on the main thread, or it crashes.
  virtual scoped_refptr<base::SequencedTaskRunner> MediaThreadTaskRunner() {
    return nullptr;
  }

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
  // This is called after the thread is created, so the embedder
  // can initiate an IPC to change its thread type (on Linux we can't
  // increase the nice value, so we need to ask the browser process). This
  // function is only called from the main thread.
  virtual void SetThreadType(base::PlatformThreadId, base::ThreadType) {}
#endif

  // Resources -----------------------------------------------------------

  // Returns true if GetDataResource would return non-null data for the
  // specified |resource_id|.
  virtual bool HasDataResource(int resource_id) const { return false; }

  // Returns a blob of data corresponding to |resource_id|. This should not be
  // used for resources which have compress="gzip" in *.grd.
  virtual WebData GetDataResource(
      int resource_id,
      ui::ResourceScaleFactor scale_factor = ui::kScaleFactorNone) {
    return WebData();
  }

  // Returns string data from a data resource. compress="gzip" and "brotli" are
  // detected automatically.
  virtual std::string GetDataResourceString(int resource_id) {
    return std::string();
  }

  // Returns the raw bytes of a data resource for the specified `resource_id`.
  // Can be called from any thread.
  virtual base::RefCountedMemory* GetDataResourceBytes(int resource_id) {
    return nullptr;
  }

  // Returns the resource ID for the webui bundled code cache corresponding to
  // the `webui_resource_url`, if it exists.
  virtual std::optional<int> GetWebUIBundledCodeCacheResourceId(
      const GURL& webui_resource_url) {
    return std::nullopt;
  }

  // Decodes the in-memory audio file data and returns the linear PCM audio data
  // in the |destination_bus|.
  // Returns true on success.
  virtual bool DecodeAudioFileData(WebAudioBus* destination_bus,
                                   base::span<const char> audio_file_data) {
    return false;
  }

  // Process lifetime management -----------------------------------------

  // Disable/Enable sudden termination on a process level. When possible, it
  // is preferable to disable sudden termination on a per-frame level via
  // mojom::LocalFrameHost::SuddenTerminationDisablerChanged.
  // This method should only be called on the main thread.
  virtual void SuddenTerminationChanged(bool enabled) {}

  // System --------------------------------------------------------------

  // Returns a value such as "en-US".
  virtual WebString DefaultLocale() { return WebString(); }

  // Returns an interface to the IO task runner.
  virtual scoped_refptr<base::SingleThreadTaskRunner> GetIOTaskRunner() const {
    return nullptr;
  }

  virtual base::PlatformThreadId GetIOThreadId() const {
    return base::kInvalidThreadId;
  }

  // Returns the sequenced task runner used to funnel video frames from
  // MediaStreamVideoSource. It may conditionally be the same as the result
  // from GetIOTaskRunner().
  virtual scoped_refptr<base::SequencedTaskRunner>
  GetMediaStreamVideoSourceVideoTaskRunner() const {
    return GetIOTaskRunner();
  }

  // Returns an interface to run nested message loop. Used for debugging.
  class NestedMessageLoopRunner {
   public:
    virtual ~NestedMessageLoopRunner() = default;
    virtual void Run() = 0;
    virtual void QuitNow() = 0;
  };
  virtual std::unique_ptr<NestedMessageLoopRunner>
  CreateNestedMessageLoopRunner() const {
    return nullptr;
  }
  // Testing -------------------------------------------------------------

  // Record a UMA sequence action.  The UserMetricsAction construction must
  // be on a single line for extract_actions.py to find it.  Please see
  // that script for more details.  Intended use is:
  // RecordAction(UserMetricsAction("MyAction"))
  virtual void RecordAction(const UserMetricsAction&) {}

  typedef uint64_t WebMemoryAllocatorDumpGuid;

  // GPU ----------------------------------------------------------------
  //
  enum ContextType {
    kWebGL1ContextType,  // WebGL 1.0 context, use only for WebGL canvases
    kWebGL2ContextType,  // WebGL 2.0 context, use only for WebGL canvases
    kGLES2ContextType,   // GLES 2.0 context, default, good for using skia
    kGLES3ContextType,   // GLES 3.0 context
    kWebGPUContextType,  // WebGPU context
  };
  struct ContextAttributes {
    bool prefer_low_power_gpu = false;
    bool fail_if_major_performance_caveat = false;
    ContextType context_type = kGLES2ContextType;

    // Offscreen contexts created for WebGL should not need the RasterInterface
    // or GrContext. If either of these are set to false, it will not be
    // possible to use the corresponding interface for the lifetime of the
    // context.
    bool enable_raster_interface = false;
    bool support_grcontext = false;
  };
  struct GraphicsInfo {
    unsigned vendor_id = 0;
    unsigned device_id = 0;
    unsigned reset_notification_strategy = 0;
    bool sandboxed = false;
    bool amd_switchable = false;
    bool optimus = false;
    bool using_gpu_compositing = false;
    bool using_passthrough_command_decoder = false;
    gl::ANGLEImplementation angle_implementation =
        gl::ANGLEImplementation::kNone;
    WebString vendor_info;
    WebString renderer_info;
    WebString driver_version;
    WebString error_message;
  };
  // Returns a newly allocated and initialized offscreen context provider,
  // backed by an independent context. Returns null if the context cannot be
  // created or initialized.
  virtual std::unique_ptr<WebGraphicsContext3DProvider>
  CreateOffscreenGraphicsContext3DProvider(const ContextAttributes&,
                                           const WebURL& document_url,
                                           GraphicsInfo*);

  // Returns a newly allocated and initialized offscreen context provider,
  // backed by the process-wide shared main thread context. Returns null if
  // the context cannot be created or initialized.
  virtual std::unique_ptr<WebGraphicsContext3DProvider>
  CreateSharedOffscreenGraphicsContext3DProvider();

  // Returns a newly allocated and initialized WebGPU context provider,
  // backed by an independent context. Returns null if the context cannot be
  // created or initialized.
  virtual std::unique_ptr<WebGraphicsContext3DProvider>
  CreateWebGPUGraphicsContext3DProvider(const WebURL& document_url);

  virtual void CreateWebGPUGraphicsContext3DProviderAsync(
      const blink::WebURL& document_url,
      base::OnceCallback<
          void(std::unique_ptr<blink::WebGraphicsContext3DProvider>)> callback);

  // When true, animations will run on a compositor thread independently from
  // the blink main thread.
  // This is true when there exists a renderer compositor in this process. But
  // for unit tests, a single-threaded compositor may be used so it may remain
  // false.
  virtual bool IsThreadedAnimationEnabled() { return false; }

  // Whether the compositor is using gpu and expects gpu resources as inputs,
  // or software based resources.
  // NOTE: This function should not be called from core/ and modules/, but
  // called by platform/graphics/ is fine.
  virtual bool IsGpuCompositingDisabled() const { return true; }

#if BUILDFLAG(IS_ANDROID)
  // Returns if synchronous compositing is enabled. Only used for Android
  // webview.
  virtual bool IsSynchronousCompositingEnabledForAndroidWebView() {
    return false;
  }

  // Returns if zero copy synchronouse software draw is enabled. Only used
  // when SynchronousCompositing is enabled and only when in single process
  // mode.
  virtual bool IsZeroCopySynchronousSwDrawEnabledForAndroidWebView() {
    return false;
  }

  // Return the SkCanvas that is to be used if
  // ZeroCopySynchronousSwDrawEnabled returns true.
  virtual SkCanvas* SynchronousCompositorGetSkCanvasForAndroidWebView() {
    return nullptr;
  }
#endif

  // Whether LCD text is enabled.
  virtual bool IsLcdTextEnabled() { return false; }

  // Whether rubberbanding/elatic on overscrolling is enabled. This usually
  // varies between each OS and can be configured via user settings in the OS.
  virtual bool IsElasticOverscrollEnabled() { return false; }

  // Whether the scroll animator that produces smooth scrolling is enabled.
  virtual bool IsScrollAnimatorEnabled() { return true; }

  // Returns a context provider that will be bound on the main thread
  // thread.
  virtual scoped_refptr<viz::RasterContextProvider>
  SharedMainThreadContextProvider();

  // Returns a worker context provider that will be bound on the compositor
  // thread.
  virtual scoped_refptr<cc::RasterContextProviderWrapper>
  SharedCompositorWorkerContextProvider(
      cc::RasterDarkModeFilter* dark_mode_filter);

  // Synchronously establish a channel to the GPU plugin if not previously
  // established or if it has been lost (for example if the GPU plugin crashed).
  // If there is a pending asynchronous request, it will be completed by the
  // time this routine returns.
  virtual scoped_refptr<gpu::GpuChannelHost> EstablishGpuChannelSync();

  // Is mojo::Remote<mojom::Gpu> disconnected?
  virtual bool IsGpuRemoteDisconnected();

  // Same as above, but asynchronous.
  using EstablishGpuChannelCallback =
      base::OnceCallback<void(scoped_refptr<gpu::GpuChannelHost>)>;
  virtual void EstablishGpuChannel(EstablishGpuChannelCallback callback);

  // Media stream ----------------------------------------------------
  virtual scoped_refptr<media::AudioCapturerSource> NewAudioCapturerSource(
      blink::WebLocalFrame* web_frame,
      const media::AudioSourceParameters& params) {
    return nullptr;
  }

  virtual bool RTCSmoothnessAlgorithmEnabled() { return true; }

  // WebRTC ----------------------------------------------------------

  virtual std::optional<double> GetWebRtcMaxCaptureFrameRate() {
    return std::nullopt;
  }

  virtual scoped_refptr<media::AudioRendererSink> NewAudioRendererSink(
      blink::WebAudioDeviceSourceType source_type,
      blink::WebLocalFrame* web_frame,
      const media::AudioSinkParameters& params) {
    return nullptr;
  }
  virtual media::AudioLatency::Type GetAudioSourceLatencyType(
      blink::WebAudioDeviceSourceType source_type) {
    return media::AudioLatency::Type::kPlayback;
  }

  virtual bool ShouldEnforceWebRTCRoutingPreferences() { return true; }

  virtual media::MediaPermission* GetWebRTCMediaPermission(
      WebLocalFrame* web_frame) {
    return nullptr;
  }

  virtual bool UsesFakeCodecForPeerConnection() { return false; }

  virtual bool IsWebRtcEncryptionEnabled() { return true; }

  // TODO(qingsi): Consolidate the legacy |ip_handling_policy| with
  // |allow_mdns_obfuscation| following the latest spec on IP handling modes
  // with mDNS introduced
  // (https://tools.ietf.org/html/draft-ietf-rtcweb-ip-handling-12);
  virtual void GetWebRTCRendererPreferences(
      WebLocalFrame* web_frame,
      mojom::WebRtcIpHandlingPolicy* ip_handling_policy,
      uint16_t* udp_min_port,
      uint16_t* udp_max_port,
      bool* allow_mdns_obfuscation) {}

  virtual bool IsWebRtcHWEncodingEnabled() { return true; }

  virtual bool IsWebRtcHWDecodingEnabled() { return true; }

  virtual bool AllowsLoopbackInPeerConnection() { return false; }

  // VideoCapture -------------------------------------------------------

  virtual WebVideoCaptureImplManager* GetVideoCaptureImplManager() {
    return nullptr;
  }

  // WebWorker ----------------------------------------------------------

  virtual std::unique_ptr<WebDedicatedWorkerHostFactoryClient>
  CreateDedicatedWorkerHostFactoryClient(WebDedicatedWorker*,
                                         const BrowserInterfaceBrokerProxy&);
  virtual void DidStartWorkerThread() {}
  virtual void WillStopWorkerThread() {}
  virtual void WorkerContextCreated(const v8::Local<v8::Context>& worker) {}
  virtual bool AllowScriptExtensionForServiceWorker(
      const WebSecurityOrigin& script_origin) {
    return false;
  }
  virtual ProtocolHandlerSecurityLevel GetProtocolHandlerSecurityLevel(
      const WebSecurityOrigin& origin) {
    return ProtocolHandlerSecurityLevel::kStrict;
  }

  // Returns true if the origin can register a service worker. Scheme must be
  // http (localhost only), https, or a custom-set secure scheme.
  virtual bool OriginCanAccessServiceWorkers(const GURL& url) { return false; }

  // Clones the current `service_worker_container_host` and returns the original
  // host and the cloned one together.
  //
  // TODO(https://crbug.com/1110176): Remove this method once the mojom
  // interface ServiceWorkerContainerHost is moved out of mojom_core, which is
  // not available from renderer/platform.
  virtual std::tuple<
      CrossVariantMojoRemote<mojom::ServiceWorkerContainerHostInterfaceBase>,
      CrossVariantMojoRemote<mojom::ServiceWorkerContainerHostInterfaceBase>>
  CloneServiceWorkerContainerHost(
      CrossVariantMojoRemote<mojom::ServiceWorkerContainerHostInterfaceBase>
          service_worker_container_host) {
    return std::make_tuple(
        /*original_service_worker_container_host=*/CrossVariantMojoRemote<
            mojom::ServiceWorkerContainerHostInterfaceBase>(),
        /*cloned_service_worker_container_host=*/CrossVariantMojoRemote<
            mojom::ServiceWorkerContainerHostInterfaceBase>());
  }

  // Creates a ServiceWorkerSubresourceLoaderFactory.
  virtual void CreateServiceWorkerSubresourceLoaderFactory(
      CrossVariantMojoRemote<mojom::ServiceWorkerContainerHostInterfaceBase>
          service_worker_container_host,
      const WebString& client_id,
      std::unique_ptr<network::PendingSharedURLLoaderFactory> fallback_factory,
      mojo::PendingReceiver<network::mojom::URLLoaderFactory> receiver,
      scoped_refptr<base::SequencedTaskRunner> task_runner);

  // WebCrypto ----------------------------------------------------------

  virtual WebCrypto* Crypto() { return nullptr; }

  // Mojo ---------------------------------------------------------------

  // Callable from any thread. Asks the browser to bind an interface receiver on
  // behalf of this renderer.
  //
  // Note that all GetInterface requests made on this object will hop to the IO
  // thread before being passed to the browser process.
  //
  // Callers should consider scoping their interfaces to a more specific context
  // before resorting to use of process-scoped interface bindings. Frames and
  // workers have their own contexts, and their BrowserInterfaceBrokerProxy
  // instances have less overhead since they don't need to be thread-safe.
  // Using a more narrowly defined scope when possible is also generally better
  // for security.
  virtual ThreadSafeBrowserInterfaceBrokerProxy* GetBrowserInterfaceBroker();

  // Media Log -----------------------------------------------------------

  // MediaLog is used by WebCodecs to report events and errors up to the
  // chrome://media-internals page and the DevTools media tab.
  // |owner_task_runner| must be bound to the main thead or the worker thread
  // on which WebCodecs will using the MediaLog. It is safe to add logs to
  // MediaLog from any thread, but it must be destroyed on |owner_task_runner|.
  // MediaLog owners should destroy the MediaLog if the ExecutionContext is
  // destroyed, since |inspector_context| may no longer be valid at that point.
  // |is_on_worker| is used to avoid logging to the chrome://media-internal
  // page, which can only be logged to from the window main thread.
  // Note: |inspector_context| is only used on |owner_task_runner|, so
  // destroying the MediaLog on |owner_task_runner| should avoid races.
  virtual std::unique_ptr<media::MediaLog> GetMediaLog(
      MediaInspectorContext* inspector_context,
      scoped_refptr<base::SingleThreadTaskRunner> owner_task_runner,
      bool is_on_worker);

  // GpuVideoAcceleratorFactories --------------------------------------

  virtual media::GpuVideoAcceleratorFactories* GetGpuFactories() {
    return nullptr;
  }

  virtual base::WeakPtr<media::DecoderFactory> GetMediaDecoderFactory() {
    return nullptr;
  }

  virtual void SetRenderingColorSpace(const gfx::ColorSpace& color_space) {}

  virtual gfx::ColorSpace GetRenderingColorSpace() const;

  // V8 Metrics -----------------------------------------------------------

  // Called when adding a histogram entry. Allows customizing the name the
  // histogram is logged as.
  virtual std::string GetNameForHistogram(const char* name) {
    return std::string{name};
  }

  // V8 Context Snapshot --------------------------------------------------

  // This method returns true only when
  // tools/v8_context_snapshot/v8_context_snapshot_generator is running (which
  // runs during Chromium's build step).
  virtual bool IsTakingV8ContextSnapshot() { return false; }

  // Crash Reporting -----------------------------------------------------

  // Set the active URL for crash reporting. The active URL is stored as crash
  // keys and is usually set for the duration of processing an IPC message. To
  // unset pass an empty WebURL and WebString.
  virtual void SetActiveURL(const WebURL& url, const WebString& top_url) {}

  // Sad Page -----------------------------------------------------

  // Returns a sad page bitmap used when the child frame has crashed.
  virtual SkBitmap* GetSadPageBitmap() { return nullptr; }

  // V8 Converter -------------------------------------------------

  // Returns WebV8ValueConverter that converts between v8::Value and
  // base::Value.
  virtual std::unique_ptr<WebV8ValueConverter> CreateWebV8ValueConverter() {
    return nullptr;
  }

  // V8 Configuration ---------------------------------------------

  // Returns whether V8 feature flag overrides were disallowed by the embedder.
  virtual bool DisallowV8FeatureFlagOverrides() const { return false; }

  // Content Security Policy --------------------------------------

  // Appends to `csp`, the default CSP which should be applied to the given
  // `url`. This allows the embedder to customize the applied CSP.
  virtual void AppendContentSecurityPolicy(
      const WebURL& url,
      std::vector<WebContentSecurityPolicyHeader>* csp) {}

  // Cross-origin subframes are generally not allowed to display a file picker
  // for security reasons. This method allows content embedders to specify
  // whether a cross-origin subframe of a particular origin should be allowed to
  // display the file picker.
  //
  // For example, Chrome's built-in PDF viewer may be hosted in a cross-origin
  // subframe. To allow this viewer to function correctly, Chrome uses this
  // method to grant it access to the file picker.
  virtual bool IsFilePickerAllowedForCrossOriginSubframe(
      const WebSecurityOrigin& origin) {
    return false;
  }

#if BUILDFLAG(IS_ANDROID)
  // User Level Memory Pressure Signal Generator ------------------
  virtual void SetPrivateMemoryFootprint(
      uint64_t private_memory_footprint_bytes) {}

  virtual bool IsUserLevelMemoryPressureSignalEnabled() { return false; }
  virtual std::pair<base::TimeDelta, base::TimeDelta>
  InertAndMinimumIntervalOfUserLevelMemoryPressureSignal() {
    return std::make_pair(base::TimeDelta(), base::TimeDelta());
  }
#endif

 private:
  static void InitializeMainThreadCommon(
      std::unique_ptr<MainThread> main_thread);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_PLATFORM_H_
