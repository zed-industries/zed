// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREAD_RESTRICTIONS_H_
#define BASE_THREADING_THREAD_RESTRICTIONS_H_

#include <optional>

#include "base/auto_reset.h"
#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/debug/stack_trace.h"
#include "base/gtest_prod_util.h"
#include "base/location.h"
#include "build/build_config.h"

// -----------------------------------------------------------------------------
// Usage documentation
// -----------------------------------------------------------------------------
//
// Overview:
// This file exposes functions to ban and allow certain slow operations
// on a per-thread basis. To annotate *usage* of such slow operations, refer to
// scoped_blocking_call.h instead.
//
// Specific allowances that can be controlled in this file are:
//
// - Blocking call: Refers to any call that causes the calling thread to wait
//   off-CPU. It includes but is not limited to calls that wait on synchronous
//   file I/O operations: read or write a file from disk, interact with a pipe
//   or a socket, rename or delete a file, enumerate files in a directory, etc.
//   Acquiring a low contention lock is not considered a blocking call.
//
//   Prefer to allow a blocking call by posting a task to
//   base::ThreadPoolInstance with base::MayBlock().
//
// - Waiting on a //base sync primitive: Refers to calling one of these methods:
//   - base::WaitableEvent::*Wait*
//   - base::ConditionVariable::*Wait*
//   - base::Process::WaitForExit*
//
//   Prefer not to wait on //base sync primitives (see below for alternatives).
//   When it is unavoidable, use ScopedAllowBaseSyncPrimitives in a task posted
//   to base::ThreadPoolInstance with base::MayBlock().
//
// - Accessing singletons: Accessing global state (Singleton / LazyInstance) is
//   problematic on threads whom aren't joined on shutdown as they can be using
//   the state as it becomes invalid during tear down. base::NoDestructor is the
//   preferred alternative for global state and doesn't have this restriction.
//
// - Long CPU work: Refers to any code that takes more than 100 ms to
//   run when there is no CPU contention and no hard page faults and therefore,
//   is not suitable to run on a thread required to keep the browser responsive
//   (where jank could be visible to the user).
//
// The following disallowance functions are offered:
//  - DisallowBlocking(): Disallows blocking calls on the current thread.
//  - DisallowBaseSyncPrimitives(): Disallows waiting on a //base sync primitive
//    on the current thread.
//  - DisallowSingleton(): Disallows using singletons on the current thread.
//  - DisallowUnresponsiveTasks(): Disallows blocking calls, waiting on a //base
//    sync primitive, and long CPU work on the current thread.
//
// In addition, scoped-allowance mechanisms are offered to make an exception
// within a scope for a behavior that is normally disallowed.
//  - ScopedAllowBlocking: Allows blocking calls. Prefer to use base::MayBlock()
//    instead.
//  - ScopedAllowBaseSyncPrimitives: Allows waiting on a //base sync primitive.
//    Must also be in a scope where blocking calls are allowed.
//  - ScopedAllowBaseSyncPrimitivesOutsideBlockingScope: Allow waiting on a
//    //base sync primitive, even in a scope where blocking calls are
//    disallowed. Prefer to use a combination of base::MayBlock() and
//    ScopedAllowBaseSyncPrimitives.
//
// Avoid using allowances outside of unit tests. In unit tests, use allowances
// with the suffix "ForTesting":
//  - ScopedAllowBlockingForTesting: Allows blocking calls in unit tests.
//  - ScopedAllowBaseSyncPrimitivesForTesting: Allows waiting on a //base sync
//    primitive in unit tests. For convenience this can be used in a scope
//    where blocking calls are disallowed. Note that base::TestWaitableEvent can
//    be used without this, also for convenience.
//
// Prefer making blocking calls from tasks posted to base::ThreadPoolInstance
// with base::MayBlock().
//
// Instead of waiting on a WaitableEvent or a ConditionVariable, prefer putting
// the work that should happen after the wait in a continuation callback and
// post it from where the WaitableEvent or ConditionVariable would have been
// signaled. If something needs to be scheduled after many tasks have executed,
// use base::BarrierClosure.
//
// On Windows, join processes asynchronously using base::win::ObjectWatcher.
//
// Where unavoidable, put ScopedAllow* instances in the narrowest scope possible
// in the caller making the blocking call but no further down. For example: if a
// Cleanup() method needs to do a blocking call, document Cleanup() as blocking
// and add a ScopedAllowBlocking instance in callers that can't avoid making
// this call from a context where blocking is banned, as such:
//
//   void Client::MyMethod() {
//     (...)
//     {
//       // Blocking is okay here because XYZ.
//       ScopedAllowBlocking allow_blocking;
//       my_foo_->Cleanup();
//     }
//     (...)
//   }
//
//   // This method can block.
//   void Foo::Cleanup() {
//     // Do NOT add the ScopedAllowBlocking in Cleanup() directly as that hides
//     // its blocking nature from unknowing callers and defeats the purpose of
//     // these checks.
//     FlushStateToDisk();
//   }
//
// Note: In rare situations where the blocking call is an implementation detail
// (i.e. the impl makes a call that invokes AssertBlockingAllowed() but it
// somehow knows that in practice this will not block), it might be okay to hide
// the ScopedAllowBlocking instance in the impl with a comment explaining why
// that's okay.

class BrowserProcessImpl;
class BrowserThemePack;
class ChromeNSSCryptoModuleDelegate;
class DesktopNotificationBalloon;
class FirefoxProfileLock;
class GaiaConfig;
class KeyStorageLinux;
class NativeBackendKWallet;
class NativeDesktopMediaList;
class PartnerBookmarksReader;
class Profile;
class ProfileImpl;
class ScopedAllowBlockingForProfile;
class StartupTabProviderImpl;
class WebEngineBrowserMainParts;
struct StartupProfilePathInfo;

namespace base {
class Environment;
class File;
class FilePath;
class CommandLine;
namespace sequence_manager::internal {
class WorkTracker;
}  // namespace sequence_manager::internal
}  // namespace base

StartupProfilePathInfo GetStartupProfilePath(
    const base::FilePath& cur_dir,
    const base::CommandLine& command_line,
    bool ignore_profile_picker);

#if BUILDFLAG(IS_IOS)
class BrowserStateDirectoryBuilder;
#endif

Profile* GetLastProfileMac();
bool HasWaylandDisplay(base::Environment* env);

namespace android_webview {
class AwBrowserContext;
class AwFormDatabaseService;
class CookieManager;
class JsSandboxIsolate;
class OverlayProcessorWebView;
class ScopedAllowInitGLBindings;
class VizCompositorThreadRunnerWebView;
}  // namespace android_webview
namespace ash {
class LoginEventRecorder;
class StartupCustomizationDocument;
class StartupUtils;
bool CameraAppUIShouldEnableLocalOverride(const std::string&);
namespace converters::diagnostics {
class MojoUtils;
}
namespace system {
class StatisticsProviderImpl;
class ProcStatFile;
}  // namespace system
}  // namespace ash
namespace audio {
class OutputDevice;
}
namespace blink {
class AudioDestination;
class DiskDataAllocator;
class RTCVideoDecoderAdapter;
class RTCVideoEncoder;
class SourceStream;
class VideoFrameResourceProvider;
class WebRtcVideoFrameAdapter;
class VideoTrackRecorderImplContextProvider;
class WorkerThread;
namespace scheduler {
class NonMainThreadImpl;
}
}  // namespace blink
namespace cc {
class CategorizedWorkerPoolJob;
class CategorizedWorkerPool;
class CompletionEvent;
class TileTaskManagerImpl;
}  // namespace cc
namespace chrome {
bool PathProvider(int, base::FilePath*);
void SessionEnding();
}  // namespace chrome
namespace chromecast {
class CrashUtil;
}
namespace chromeos {
class BlockingMethodCaller;
namespace system {
bool IsCoreSchedulingAvailable();
int NumberOfPhysicalCores();
}  // namespace system
}  // namespace chromeos
namespace content {
class BrowserGpuChannelHostFactory;
class BrowserMainLoop;
class BrowserProcessIOThread;
class BrowserTestBase;
#if BUILDFLAG(IS_IOS)
class ContentMainRunnerImpl;
#endif  // BUILDFLAG(IS_IOS)
class DesktopCaptureDevice;
class DWriteFontCollectionProxy;
class DWriteFontProxyImpl;
class EmergencyTraceFinalisationCoordinator;
class InProcessUtilityThread;
class NestedMessagePumpAndroid;
class NetworkServiceInstancePrivate;
class PepperPrintSettingsManagerImpl;
class RenderProcessHostImpl;
class RenderProcessHost;
class RenderWidgetHostViewMac;
class RendererBlinkPlatformImpl;
class SandboxHostLinux;
class ScopedAllowWaitForDebugURL;
class ServiceWorkerContextClient;
class ShellPathProvider;
class SlowWebPreferenceCache;
class SynchronousCompositor;
class SynchronousCompositorHost;
class SynchronousCompositorSyncCallBridge;
class ScopedAllowBlockingForViewAura;
class TextInputClientMac;
class WebContentsViewMac;
base::File CreateFileForDrop(base::FilePath*);
}  // namespace content
namespace cronet {
class CronetPrefsManager;
class CronetContext;
}  // namespace cronet
namespace crypto {
class ScopedAllowBlockingForNSS;
}
namespace dbus {
class Bus;
}
namespace drive {
class FakeDriveService;
}
namespace device {
class UsbContext;
}
namespace discardable_memory {
class ClientDiscardableSharedMemoryManager;
}
namespace disk_cache {
class BackendImpl;
class InFlightIO;
bool CleanupDirectorySync(const base::FilePath&);
}  // namespace disk_cache
namespace enterprise_connectors {
class LinuxKeyRotationCommand;
}  // namespace enterprise_connectors
namespace extensions {
class InstalledLoader;
class UnpackedInstaller;
}  // namespace extensions
namespace font_service::internal {
class MappedFontFile;
}
namespace gl {
struct GLImplementationParts;
namespace init {
bool InitializeStaticGLBindings(GLImplementationParts);
}
}  // namespace gl
namespace gpu {
class GpuMemoryBufferImplDXGI;
}
namespace history_report {
class HistoryReportJniBridge;
}
namespace ios_web_view {
class WebViewBrowserState;
}
namespace io_thread {
class IOSIOThread;
}
namespace leveldb::port {
class CondVar;
}  // namespace leveldb::port
namespace nearby::chrome {
class BleV2GattClient;
class BleV2Medium;
class ScheduledExecutor;
class SubmittableExecutor;
class WifiDirectSocket;
}  // namespace nearby::chrome
namespace media {
class AudioInputDevice;
class AudioOutputDevice;
class BlockingUrlProtocol;
template <class WorkerInterface,
          class WorkerImpl,
          class Worker,
          class WorkerStatus,
          WorkerStatus StatusNotOk,
          WorkerStatus StatusOk,
          WorkerStatus StatusWork>
class CodecWorkerImpl;
class FileVideoCaptureDeviceFactory;
class GpuMojoMediaClientWin;
class MailboxVideoFrameConverter;
class MojoVideoEncodeAccelerator;
class PaintCanvasVideoRenderer;
class V4L2DevicePoller;  // TODO(crbug.com/41486289): remove this.
}  // namespace media
namespace memory_instrumentation {
class OSMetrics;
}
namespace memory_pressure {
class UserLevelMemoryPressureSignalGenerator;
}
namespace metrics {
class AndroidMetricsServiceClient;
class CleanExitBeacon;
}  // namespace metrics
namespace midi {
class TaskService;  // https://crbug.com/796830
}
namespace module_installer {
class ScopedAllowModulePakLoad;
}
namespace mojo {
class SyncCallRestrictions;
namespace core {
class ScopedIPCSupport;
namespace ipcz_driver {
class MojoTrap;
}
}  // namespace core
}  // namespace mojo
namespace net {
class GSSAPISharedLibrary;
class MultiThreadedCertVerifierScopedAllowBaseSyncPrimitives;
class MultiThreadedProxyResolverScopedAllowJoinOnIO;
class NetworkChangeNotifierApple;
class NetworkConfigWatcherAppleThread;
class ProxyConfigServiceWin;
class ScopedAllowBlockingForSettingGetter;
namespace internal {
class AddressTrackerLinux;
class PemFileCertStore;
}  // namespace internal
}  // namespace net
namespace printing {
class LocalPrinterHandlerDefault;
#if BUILDFLAG(IS_MAC)
class PrintBackendServiceImpl;
#endif
class PrintBackendServiceManager;
class PrintPreviewUIUntrusted;
class PrinterQuery;
}  // namespace printing
namespace proxy_resolver {
class ScopedAllowThreadJoinForProxyResolverV8Tracing;
}
namespace remote_cocoa {
class DroppedScreenShotCopierMac;
class SelectFileDialogBridge;
}  // namespace remote_cocoa
namespace remoting {
class AutoThread;
class ScopedAllowBlockingForCrashReporting;
class ScopedBypassIOThreadRestrictions;
namespace protocol {
class ScopedAllowSyncPrimitivesForWebRtcDataStreamAdapter;
class ScopedAllowSyncPrimitivesForWebRtcTransport;
class ScopedAllowSyncPrimitivesForWebRtcVideoStream;
class ScopedAllowThreadJoinForWebRtcTransport;
}  // namespace protocol
}  // namespace remoting
namespace rlz_lib {
class FinancialPing;
}
namespace service_manager {
class ServiceProcessLauncher;
}
namespace shell_integration_linux {
class LaunchXdgUtilityScopedAllowBaseSyncPrimitives;
}
namespace storage {
class ObfuscatedFileUtil;
}
namespace syncer {
class GetLocalChangesRequest;
class HttpBridge;
}  // namespace syncer
namespace tracing {
class FuchsiaPerfettoProducerConnector;
}
namespace ui {
class DrmThreadProxy;
class DrmDisplayHostManager;
class ScopedAllowBlockingForGbmSurface;
class SelectFileDialogLinux;
class WindowResizeHelperMac;
}  // namespace ui
namespace updater {
class SystemctlLauncherScopedAllowBaseSyncPrimitives;
}
namespace viz {
class HostGpuMemoryBufferManager;
class ClientGpuMemoryBufferManager;
class DisplayCompositorMemoryAndTaskController;
class SkiaOutputSurfaceImpl;
class SharedImageInterfaceProvider;
}  // namespace viz
namespace vr {
class VrShell;
}
namespace web {
class WebMainLoop;
}  // namespace web
namespace weblayer {
class BrowserContextImpl;
class ContentBrowserClientImpl;
class ProfileImpl;
class WebLayerPathProvider;
}  // namespace weblayer
// NOTE: Please do not append entries here. Put them in the list above and keep
// the list sorted.

namespace base {

namespace android {
class JavaHandlerThread;
class PmfUtils;
class ScopedAllowBlockingForImportantFileWriter;
}  // namespace android

namespace apple::internal {
base::FilePath GetExecutablePath();
}

namespace debug {
class StackTrace;
}

namespace internal {
class GetAppOutputScopedAllowBaseSyncPrimitives;
class JobTaskSource;
class TaskTracker;
bool ReadProcFile(const FilePath& file, std::string* buffer);
}  // namespace internal

namespace sequence_manager::internal {
class TaskQueueImpl;
}  // namespace sequence_manager::internal

namespace subtle {
class PlatformSharedMemoryRegion;
}

namespace win {
class OSInfo;
class ObjectWatcher;
class ScopedAllowBlockingForUserAccountControl;
}  // namespace win

class AdjustOOMScoreHelper;
class ChromeOSVersionInfo;
class FileDescriptorWatcher;
class FilePath;
class Process;
class ScopedAllowBlockingForProc;
class ScopedAllowBlockingForProcessMetrics;
class ScopedAllowThreadRecallForStackSamplingProfiler;
class SimpleThread;
class StackSamplingProfiler;
class TestCustomDisallow;
class Thread;

// NaCL doesn't support stack capture.
// Android can hang in stack capture (crbug.com/959139).
#if BUILDFLAG(IS_NACL) || BUILDFLAG(IS_ANDROID)
#define CAPTURE_THREAD_RESTRICTIONS_STACK_TRACES() false
#else
// Stack capture is slow. Only enable it in developer builds, to avoid user
// visible jank when thread restrictions are set.
#define CAPTURE_THREAD_RESTRICTIONS_STACK_TRACES() EXPENSIVE_DCHECKS_ARE_ON()
#endif

// A boolean and the stack from which it was set. Note: The stack is not
// captured in all builds, see `CAPTURE_THREAD_RESTRICTIONS_STACK_TRACES()`.
class BooleanWithOptionalStack {
 public:
  // Default value.
  BooleanWithOptionalStack() = default;

  // Value when explicitly set.
  explicit BooleanWithOptionalStack(bool value);

  explicit operator bool() const { return value_; }

  friend std::ostream& operator<<(std::ostream& out,
                                  const BooleanWithOptionalStack& bws);

 private:
  bool value_ = false;
#if CAPTURE_THREAD_RESTRICTIONS_STACK_TRACES()
  std::optional<debug::StackTrace> stack_;
#endif
};

// Asserts that blocking calls are allowed in the current scope.
NOT_TAIL_CALLED BASE_EXPORT void AssertBlockingAllowed();
NOT_TAIL_CALLED BASE_EXPORT void AssertBlockingDisallowedForTesting();

// Disallows blocking on the current thread.
NOT_TAIL_CALLED BASE_EXPORT void DisallowBlocking();

// Disallows blocking calls within its scope.
class BASE_EXPORT ScopedDisallowBlocking {
 public:
  ScopedDisallowBlocking();

  ScopedDisallowBlocking(const ScopedDisallowBlocking&) = delete;
  ScopedDisallowBlocking& operator=(const ScopedDisallowBlocking&) = delete;

  ~ScopedDisallowBlocking();

 private:
  const AutoReset<BooleanWithOptionalStack> resetter_;
};

class BASE_EXPORT ScopedAllowBlocking {
 public:
  ScopedAllowBlocking(const ScopedAllowBlocking&) = delete;
  ScopedAllowBlocking& operator=(const ScopedAllowBlocking&) = delete;

 private:
  FRIEND_TEST_ALL_PREFIXES(ThreadRestrictionsTest,
                           NestedAllowRestoresPreviousStack);
  FRIEND_TEST_ALL_PREFIXES(ThreadRestrictionsTest, ScopedAllowBlocking);
  friend class ScopedAllowBlockingForTesting;

  // This can only be instantiated by friends. Use ScopedAllowBlockingForTesting
  // in unit tests to avoid the friend requirement.
  // Sorted by class name (with namespace), #if blocks at the bottom.
  friend class ::BrowserProcessImpl;
  friend class ::BrowserThemePack;  // http://crbug.com/80206
  friend class ::DesktopNotificationBalloon;
  friend class ::FirefoxProfileLock;
  friend class ::GaiaConfig;
  friend class ::ProfileImpl;
  friend class ::ScopedAllowBlockingForProfile;
  friend class ::StartupTabProviderImpl;
  friend class ::WebEngineBrowserMainParts;
  friend class android_webview::AwBrowserContext;
  friend class android_webview::ScopedAllowInitGLBindings;
  friend class ash::LoginEventRecorder;
  friend class ash::StartupCustomizationDocument;  // http://crosbug.com/11103
  friend class ash::StartupUtils;
  friend class ash::converters::diagnostics::MojoUtils;  // http://b/322741627
  friend class ash::system::ProcStatFile;
  friend class base::AdjustOOMScoreHelper;
  friend class base::ChromeOSVersionInfo;
  friend class base::Process;
  friend class base::ScopedAllowBlockingForProc;
  friend class base::ScopedAllowBlockingForProcessMetrics;
  friend class base::StackSamplingProfiler;
  friend class base::android::ScopedAllowBlockingForImportantFileWriter;
  friend class base::android::PmfUtils;
  friend class base::debug::StackTrace;
  friend class base::subtle::PlatformSharedMemoryRegion;
  friend class base::win::ScopedAllowBlockingForUserAccountControl;
  friend class blink::DiskDataAllocator;
  friend class chromecast::CrashUtil;
  friend class content::BrowserProcessIOThread;
  friend class content::DWriteFontProxyImpl;
  friend class content::NetworkServiceInstancePrivate;
  friend class content::PepperPrintSettingsManagerImpl;
  friend class content::RenderProcessHostImpl;
  friend class content::RenderWidgetHostViewMac;  // http://crbug.com/121917
  friend class content::
      ScopedAllowBlockingForViewAura;  // http://crbug.com/332579
  friend class content::ShellPathProvider;
  friend class content::WebContentsViewMac;
  friend class cronet::CronetContext;
  friend class cronet::CronetPrefsManager;
  friend class crypto::ScopedAllowBlockingForNSS;  // http://crbug.com/59847
  friend class drive::FakeDriveService;
  friend class extensions::InstalledLoader;
  friend class extensions::UnpackedInstaller;
  friend class font_service::internal::MappedFontFile;
  friend class ios_web_view::WebViewBrowserState;
  friend class io_thread::IOSIOThread;
  friend class media::FileVideoCaptureDeviceFactory;
  friend class memory_instrumentation::OSMetrics;
  friend class memory_pressure::UserLevelMemoryPressureSignalGenerator;
  friend class metrics::AndroidMetricsServiceClient;
  friend class metrics::CleanExitBeacon;
  friend class module_installer::ScopedAllowModulePakLoad;
  friend class net::GSSAPISharedLibrary;    // http://crbug.com/66702
  friend class net::ProxyConfigServiceWin;  // http://crbug.com/61453
  friend class net::
      ScopedAllowBlockingForSettingGetter;  // http://crbug.com/69057
  friend class net::internal::PemFileCertStore;
  friend class printing::LocalPrinterHandlerDefault;
  friend class printing::PrintBackendServiceManager;
  friend class printing::PrintPreviewUIUntrusted;
  friend class printing::PrinterQuery;
  friend class remote_cocoa::
      DroppedScreenShotCopierMac;  // https://crbug.com/1148078
  friend class remote_cocoa::SelectFileDialogBridge;
  friend class remoting::
      ScopedBypassIOThreadRestrictions;  // http://crbug.com/1144161
  friend class remoting::ScopedAllowBlockingForCrashReporting;
  friend class ui::DrmDisplayHostManager;
  friend class ui::ScopedAllowBlockingForGbmSurface;
  friend class ui::SelectFileDialogLinux;
  friend class weblayer::BrowserContextImpl;
  friend class weblayer::ContentBrowserClientImpl;
  friend class weblayer::ProfileImpl;
  friend class weblayer::WebLayerPathProvider;
#if BUILDFLAG(IS_MAC)
  friend class printing::PrintBackendServiceImpl;
#endif
#if BUILDFLAG(IS_WIN)
  friend class base::win::OSInfo;
  friend class content::SlowWebPreferenceCache;  // http://crbug.com/1262162
  friend class media::GpuMojoMediaClientWin;     // https://crbug.com/360642944
#endif
#if BUILDFLAG(IS_IOS)
  friend class ::BrowserStateDirectoryBuilder;
#endif

  // Sorted by function name (with namespace), ignoring the return type.
  friend Profile* ::GetLastProfileMac();  // http://crbug.com/1176734
  // Note: This function return syntax is required so the "::" doesn't get
  // mis-parsed. See https://godbolt.org/z/KGhnPxfc8 for the issue.
  friend auto ::GetStartupProfilePath(const base::FilePath& cur_dir,
                                      const base::CommandLine& command_line,
                                      bool ignore_profile_picker)
      -> StartupProfilePathInfo;
  friend bool ::HasWaylandDisplay(
      base::Environment* env);  // http://crbug.com/1246928
  friend bool ash::CameraAppUIShouldEnableLocalOverride(const std::string&);
  friend base::FilePath base::apple::internal::GetExecutablePath();
  friend bool base::internal::ReadProcFile(const FilePath& file,
                                           std::string* buffer);
  friend bool chrome::PathProvider(int,
                                   base::FilePath*);  // http://crbug.com/259796
  friend void chrome::SessionEnding();
  friend bool chromeos::system::IsCoreSchedulingAvailable();
  friend int chromeos::system::NumberOfPhysicalCores();
  friend base::File content::CreateFileForDrop(
      base::FilePath* file_path);  // http://crbug.com/110709
  friend bool disk_cache::CleanupDirectorySync(const base::FilePath&);
  friend bool gl::init::InitializeStaticGLBindings(gl::GLImplementationParts);

  ScopedAllowBlocking(const Location& from_here = Location::Current());
  ~ScopedAllowBlocking();

  const AutoReset<BooleanWithOptionalStack> resetter_;
};

class ScopedAllowBlockingForTesting {
 public:
  ScopedAllowBlockingForTesting() = default;

  ScopedAllowBlockingForTesting(const ScopedAllowBlockingForTesting&) = delete;
  ScopedAllowBlockingForTesting& operator=(
      const ScopedAllowBlockingForTesting&) = delete;

  ~ScopedAllowBlockingForTesting() = default;

 private:
  ScopedAllowBlocking scoped_allow_blocking_;
};

NOT_TAIL_CALLED BASE_EXPORT void DisallowBaseSyncPrimitives();

// Disallows singletons within its scope.
class BASE_EXPORT ScopedDisallowBaseSyncPrimitives {
 public:
  ScopedDisallowBaseSyncPrimitives();

  ScopedDisallowBaseSyncPrimitives(const ScopedDisallowBaseSyncPrimitives&) =
      delete;
  ScopedDisallowBaseSyncPrimitives& operator=(
      const ScopedDisallowBaseSyncPrimitives&) = delete;

  ~ScopedDisallowBaseSyncPrimitives();

 private:
  const AutoReset<BooleanWithOptionalStack> resetter_;
};

class BASE_EXPORT ScopedAllowBaseSyncPrimitives {
 public:
  ScopedAllowBaseSyncPrimitives(const ScopedAllowBaseSyncPrimitives&) = delete;
  ScopedAllowBaseSyncPrimitives& operator=(
      const ScopedAllowBaseSyncPrimitives&) = delete;

 private:
  // This can only be instantiated by friends. Use
  // ScopedAllowBaseSyncPrimitivesForTesting in unit tests to avoid the friend
  // requirement.
  FRIEND_TEST_ALL_PREFIXES(ThreadRestrictionsTest,
                           ScopedAllowBaseSyncPrimitives);
  FRIEND_TEST_ALL_PREFIXES(ThreadRestrictionsTest,
                           ScopedAllowBaseSyncPrimitivesResetsState);
  FRIEND_TEST_ALL_PREFIXES(ThreadRestrictionsTest,
                           ScopedAllowBaseSyncPrimitivesWithBlockingDisallowed);

  // Allowed usage:
  // Sorted by class name (with namespace).
  friend class ::ChromeNSSCryptoModuleDelegate;
  friend class ::PartnerBookmarksReader;
  friend class ::tracing::FuchsiaPerfettoProducerConnector;
  friend class android_webview::JsSandboxIsolate;
  friend class base::SimpleThread;
  friend class base::internal::GetAppOutputScopedAllowBaseSyncPrimitives;
  friend class blink::SourceStream;
  friend class blink::VideoTrackRecorderImplContextProvider;
  friend class blink::WorkerThread;
  friend class blink::scheduler::NonMainThreadImpl;
  friend class cc::CategorizedWorkerPoolJob;
  friend class content::BrowserMainLoop;
  friend class content::BrowserProcessIOThread;
  friend class content::DWriteFontCollectionProxy;
  friend class content::RendererBlinkPlatformImpl;
  friend class content::ServiceWorkerContextClient;
  friend class device::UsbContext;
  friend class enterprise_connectors::LinuxKeyRotationCommand;
  friend class history_report::HistoryReportJniBridge;
  friend class internal::TaskTracker;
  friend class leveldb::port::CondVar;
  friend class nearby::chrome::ScheduledExecutor;
  friend class nearby::chrome::SubmittableExecutor;
  friend class nearby::chrome::BleV2GattClient;
  friend class nearby::chrome::BleV2Medium;
  friend class nearby::chrome::WifiDirectSocket;
  friend class media::AudioOutputDevice;
  friend class media::BlockingUrlProtocol;
  template <class WorkerInterface,
            class WorkerImpl,
            class Worker,
            class WorkerStatus,
            WorkerStatus StatusNotOk,
            WorkerStatus StatusOk,
            WorkerStatus StatusWork>
  friend class media::CodecWorkerImpl;
  friend class media::MojoVideoEncodeAccelerator;
  friend class mojo::core::ScopedIPCSupport;
  friend class net::MultiThreadedCertVerifierScopedAllowBaseSyncPrimitives;
  friend class rlz_lib::FinancialPing;
  friend class shell_integration_linux::
      LaunchXdgUtilityScopedAllowBaseSyncPrimitives;
  friend class storage::ObfuscatedFileUtil;
  friend class syncer::HttpBridge;
  friend class syncer::GetLocalChangesRequest;
  friend class updater::SystemctlLauncherScopedAllowBaseSyncPrimitives;

  // Usage that should be fixed:
  // Sorted by class name (with namespace).
  friend class ::NativeBackendKWallet;  // http://crbug.com/125331
  friend class android_webview::
      OverlayProcessorWebView;                     // http://crbug.com/341151462
  friend class blink::VideoFrameResourceProvider;  // http://crbug.com/878070
  friend class viz::
      DisplayCompositorMemoryAndTaskController;    // http://crbug.com/341151462
  friend class viz::SkiaOutputSurfaceImpl;         // http://crbug.com/341151462
  friend class viz::SharedImageInterfaceProvider;  // http://crbug.com/341151462

  ScopedAllowBaseSyncPrimitives();
  ~ScopedAllowBaseSyncPrimitives();

  const AutoReset<BooleanWithOptionalStack> resetter_;
};

class BASE_EXPORT
    [[maybe_unused,
      nodiscard]] ScopedAllowBaseSyncPrimitivesOutsideBlockingScope {
 public:
  ScopedAllowBaseSyncPrimitivesOutsideBlockingScope(
      const ScopedAllowBaseSyncPrimitivesOutsideBlockingScope&) = delete;
  ScopedAllowBaseSyncPrimitivesOutsideBlockingScope& operator=(
      const ScopedAllowBaseSyncPrimitivesOutsideBlockingScope&) = delete;

 private:
  // This can only be instantiated by friends. Use
  // ScopedAllowBaseSyncPrimitivesForTesting in unit tests to avoid the friend
  // requirement.
  FRIEND_TEST_ALL_PREFIXES(ThreadRestrictionsTest,
                           ScopedAllowBaseSyncPrimitivesOutsideBlockingScope);
  FRIEND_TEST_ALL_PREFIXES(
      ThreadRestrictionsTest,
      ScopedAllowBaseSyncPrimitivesOutsideBlockingScopeResetsState);

  // Allowed usage:
  // Sorted by class name (with namespace).
  friend class ::BrowserProcessImpl;  // http://crbug.com/125207
  friend class ::KeyStorageLinux;
  friend class ::NativeDesktopMediaList;
  friend class android::JavaHandlerThread;
  friend class android_webview::
      AwFormDatabaseService;  // http://crbug.com/904431
  friend class android_webview::CookieManager;
  friend class android_webview::VizCompositorThreadRunnerWebView;
  friend class audio::OutputDevice;
  friend class base::FileDescriptorWatcher;
  friend class base::ScopedAllowThreadRecallForStackSamplingProfiler;
  friend class base::StackSamplingProfiler;
  friend class base::internal::JobTaskSource;
  friend class base::sequence_manager::internal::TaskQueueImpl;
  friend class base::sequence_manager::internal::WorkTracker;
  friend class base::win::ObjectWatcher;
  friend class blink::AudioDestination;
  friend class blink::RTCVideoDecoderAdapter;
  friend class blink::RTCVideoEncoder;
  friend class blink::WebRtcVideoFrameAdapter;
  friend class cc::CategorizedWorkerPoolJob;
  friend class cc::CategorizedWorkerPool;
  friend class cc::TileTaskManagerImpl;
  friend class content::DesktopCaptureDevice;
  friend class content::EmergencyTraceFinalisationCoordinator;
  friend class content::InProcessUtilityThread;
  friend class content::RenderProcessHost;
  friend class content::SandboxHostLinux;
  friend class content::ScopedAllowWaitForDebugURL;
  friend class content::SynchronousCompositor;
  friend class content::SynchronousCompositorHost;
  friend class content::SynchronousCompositorSyncCallBridge;
  friend class gpu::GpuMemoryBufferImplDXGI;
  friend class media::AudioInputDevice;
  friend class media::AudioOutputDevice;
  friend class media::MailboxVideoFrameConverter;
  friend class media::PaintCanvasVideoRenderer;
  friend class media::V4L2DevicePoller;  // TODO(crbug.com/41486289): remove
                                         // this.
  friend class mojo::SyncCallRestrictions;
  friend class mojo::core::ipcz_driver::MojoTrap;
  friend class net::NetworkConfigWatcherAppleThread;
  friend class ui::DrmThreadProxy;
  friend class viz::ClientGpuMemoryBufferManager;
  friend class viz::HostGpuMemoryBufferManager;
  friend class vr::VrShell;

  // Usage that should be fixed:
  friend class ::ash::system::StatisticsProviderImpl;  // http://b/261818124
  friend class ::chromeos::BlockingMethodCaller;  // http://crbug.com/125360
  friend class base::Thread;                      // http://crbug.com/918039
  friend class cc::CompletionEvent;               // http://crbug.com/902653
  friend class content::
      BrowserGpuChannelHostFactory;          // http://crbug.com/125248
  friend class content::TextInputClientMac;  // http://crbug.com/121917
  friend class dbus::Bus;                    // http://crbug.com/125222
  friend class discardable_memory::
      ClientDiscardableSharedMemoryManager;  // http://crbug.com/1396355
  friend class disk_cache::BackendImpl;      // http://crbug.com/74623
  friend class disk_cache::InFlightIO;       // http://crbug.com/74623
  friend class midi::TaskService;            // https://crbug.com/796830
  friend class net::
      MultiThreadedProxyResolverScopedAllowJoinOnIO;  // http://crbug.com/69710
  friend class net::NetworkChangeNotifierApple;       // http://crbug.com/125097
  friend class net::internal::AddressTrackerLinux;    // http://crbug.com/125097
  friend class proxy_resolver::
      ScopedAllowThreadJoinForProxyResolverV8Tracing;  // http://crbug.com/69710
  friend class remoting::AutoThread;  // https://crbug.com/944316
  friend class remoting::protocol::
      ScopedAllowSyncPrimitivesForWebRtcDataStreamAdapter;  // http://b/233844893
  friend class remoting::protocol::
      ScopedAllowSyncPrimitivesForWebRtcTransport;  // http://crbug.com/1198501
  friend class remoting::protocol::
      ScopedAllowSyncPrimitivesForWebRtcVideoStream;  // http://b/304681143
  friend class remoting::protocol::
      ScopedAllowThreadJoinForWebRtcTransport;  // http://crbug.com/660081
  // Not used in production yet, https://crbug.com/844078.
  friend class service_manager::ServiceProcessLauncher;
  friend class ui::WindowResizeHelperMac;  // http://crbug.com/902829

  ScopedAllowBaseSyncPrimitivesOutsideBlockingScope(
      const Location& from_here = Location::Current());

  ~ScopedAllowBaseSyncPrimitivesOutsideBlockingScope();

  const AutoReset<BooleanWithOptionalStack> resetter_;
};

// Allow base-sync-primitives in tests, doesn't require explicit friend'ing like
// ScopedAllowBaseSyncPrimitives-types aimed at production do.
// Note: For WaitableEvents in the test logic, base::TestWaitableEvent is
// exposed as a convenience to avoid the need for
// ScopedAllowBaseSyncPrimitivesForTesting.
class BASE_EXPORT ScopedAllowBaseSyncPrimitivesForTesting {
 public:
  ScopedAllowBaseSyncPrimitivesForTesting();

  ScopedAllowBaseSyncPrimitivesForTesting(
      const ScopedAllowBaseSyncPrimitivesForTesting&) = delete;
  ScopedAllowBaseSyncPrimitivesForTesting& operator=(
      const ScopedAllowBaseSyncPrimitivesForTesting&) = delete;

  ~ScopedAllowBaseSyncPrimitivesForTesting();

 private:
  const AutoReset<BooleanWithOptionalStack> resetter_;
};

// Counterpart to base::DisallowUnresponsiveTasks() for tests to allow them to
// block their thread after it was banned.
class BASE_EXPORT ScopedAllowUnresponsiveTasksForTesting {
 public:
  ScopedAllowUnresponsiveTasksForTesting();

  ScopedAllowUnresponsiveTasksForTesting(
      const ScopedAllowUnresponsiveTasksForTesting&) = delete;
  ScopedAllowUnresponsiveTasksForTesting& operator=(
      const ScopedAllowUnresponsiveTasksForTesting&) = delete;

  ~ScopedAllowUnresponsiveTasksForTesting();

 private:
  const AutoReset<BooleanWithOptionalStack> base_sync_resetter_;
  const AutoReset<BooleanWithOptionalStack> blocking_resetter_;
  const AutoReset<BooleanWithOptionalStack> cpu_resetter_;
};

namespace internal {

// Asserts that waiting on a //base sync primitive is allowed in the current
// scope.
NOT_TAIL_CALLED BASE_EXPORT void AssertBaseSyncPrimitivesAllowed();

// Resets all thread restrictions on the current thread.
BASE_EXPORT void ResetThreadRestrictionsForTesting();

// Check whether the current thread is allowed to use singletons (Singleton /
// LazyInstance).  DCHECKs if not.
NOT_TAIL_CALLED BASE_EXPORT void AssertSingletonAllowed();

}  // namespace internal

// Disallow using singleton on the current thread.
NOT_TAIL_CALLED BASE_EXPORT void DisallowSingleton();

// Disallows singletons within its scope.
class BASE_EXPORT ScopedDisallowSingleton {
 public:
  ScopedDisallowSingleton();

  ScopedDisallowSingleton(const ScopedDisallowSingleton&) = delete;
  ScopedDisallowSingleton& operator=(const ScopedDisallowSingleton&) = delete;

  ~ScopedDisallowSingleton();

 private:
  const AutoReset<BooleanWithOptionalStack> resetter_;
};

// Asserts that running long CPU work is allowed in the current scope.
NOT_TAIL_CALLED BASE_EXPORT void AssertLongCPUWorkAllowed();

NOT_TAIL_CALLED BASE_EXPORT void DisallowUnresponsiveTasks();

// Friend-only methods to permanently allow the current thread to use
// blocking/sync-primitives calls. Threads start out in the *allowed* state but
// are typically *disallowed* via the above base::Disallow*() methods after
// being initialized.
//
// Only use these to permanently set the allowance on a thread, e.g. on
// shutdown. For temporary allowances, use scopers above.
class BASE_EXPORT PermanentThreadAllowance {
 public:
  // Class is merely a namespace-with-friends.
  PermanentThreadAllowance() = delete;

 private:
  // Sorted by class name (with namespace)
  friend class base::TestCustomDisallow;
  friend class content::BrowserMainLoop;
  friend class content::BrowserTestBase;
#if BUILDFLAG(IS_IOS)
  friend class content::ContentMainRunnerImpl;
#endif  // BUILDFLAG(IS_IOS)
  friend class web::WebMainLoop;

  static void AllowBlocking();
  static void AllowBaseSyncPrimitives();
};

}  // namespace base

#endif  // BASE_THREADING_THREAD_RESTRICTIONS_H_
