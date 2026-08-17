| Framework | Why is this unsupported? |
| --- | --- |
| `AGL` | OpenGL-specific, use Metal instead. |
| `Accelerate` | Very C-centric, hard for us to map. |
| `ActivityKit` | Swift-only. |
| `AdAttributionKit` | Swift-only. |
| `AddressBook` | Deprecated, use Contacts instead. |
| `AddressBookUI` | Deprecated, use Contacts instead. |
| `AppIntents` | Swift-only. |
| `AppleScriptKit` | Basically empty nowadays. |
| `AppleScriptObjC` | Basically empty nowadays. |
| `AssetsLibrary` | Deprecated, use PhotoKit instead. |
| `Assignables` | Swift-only. |
| `AudioUnit` | Deprecated, use AudioToolbox instead. |
| `AudioVideoBridging` | Deprecated, use AVKit/AVFoundation instead (maybe?). |
| `AutomatedDeviceEnrollment` | Swift-only. |
| `BrowserKit` | TODO. |
| `CalendarStore` | Very deprecated. |
| `CarKey` | Swift-only. |
| `Charts` | Swift-only. |
| `Cocoa` | Basically empty nowadays, use AppKit. |
| `Combine` | Swift-only. |
| `ContactProvider` | Mostly Swift-only. |
| `CoreDisplay` | Basically empty. |
| `CoreHID` | Swift-only. |
| `CoreMIDIServer` | Very deprecated. |
| `CoreTransferable` | Swift-only. |
| `CreateML` | Swift-only. |
| `CreateMLComponents` | Swift-only. |
| `CryptoKit` | Swift-only. |
| `DVDPlayback` | Deprecated, use AVKit/AVFoundation instead. |
| `DeveloperToolsSupport` | Swift-only. |
| `DeviceActivity` | Swift-only. |
| `DeviceDiscoveryUI` | Needs Network first. |
| `DirectoryService` | Deprecated, use OpenDirectory instead. |
| `DiscRecording` | Deprecated, use AVKit/AVFoundation instead. |
| `DiscRecordingUI` | Deprecated, use AVKit/AVFoundation instead. |
| `DockKit` | Swift-only. |
| `DriverKit` | Uses C++ classes. |
| `ExtensionFoundation` | Swift-only. |
| `FamilyControls` | Swift-only. |
| `FinanceKit` | Swift-only. |
| `FinanceKitUI` | Swift-only. |
| `ForceFeedback` | Very C-centric and old. |
| `GLKit` | OpenGL-specific, use Metal instead. |
| `GLUT` | OpenGL-specific, use Metal instead. |
| `GSS` | Very C-centric and old. |
| `GroupActivities` | Swift-only. |
| `Hypervisor` | Very low-level, consider crates like `applevisor` instead. |
| `ICADevices` | Deprecated, use ImageCaptureCore instead. |
| `ImagePlayground` | Swift-only. |
| `InstallerPlugins` | Deprecated. |
| `InstantMessage` | Deprecated in macOS 10.9. |
| `JavaNativeFoundation` | Basically empty. |
| `JavaRuntimeSupport` | Probably not really interesting.. |
| `JournalingSuggestions` | Swift-only. |
| `Kerberos` | Too low-level. |
| `Kernel` | Too low-level. |
| `KernelManagement` | Basically empty. |
| `LDAP` | Basically empty. |
| `LightweightCodeRequirements` | Swift-only. |
| `LiveCommunicationKit` | Swift-only. |
| `LockedCameraCapture` | Swift-only. |
| `ManagedApp` | Swift-only. |
| `ManagedAppDistribution` | Swift-only. |
| `ManagedSettings` | Swift-only. |
| `ManagedSettingsUI` | Swift-only. |
| `MarketplaceKit` | Swift-only. |
| `Matter` | Mostly available [here](https://github.com/project-chip/connectedhomeip). |
| `MatterSupport` | Swift-only. |
| `MediaLibrary` | Deprecated, use PhotoKit instead. |
| `Message` | Basically empty. |
| `MobileCoreServices` | Deprecated, use CoreServices + UniformTypeIdentifiers instead. |
| `MusicKit` | Swift-only. |
| `NetFS` | Deprecated, use macFUSE or FSKit instead (probably). |
| `Network` | TODO, see [#646](https://github.com/madsmtm/objc2/issues/646). |
| `OpenAL` | Very C-centric, use newer Audio frameworks instead. |
| `OpenCL` | Very C-centric and old. |
| `OpenGL` | OpenGL-specific, use Metal instead. |
| `OpenGLES` | OpenGL-specific, use Metal instead. |
| `PCSC` | Too low-level, consider crates like `pcsc` instead. |
| `ProximityReader` | Swift-only. |
| `ProximityReaderStub` | Basically empty. |
| `QTKit` | No headers present in Xcode's SDK. |
| `RealityFoundation` | Swift-only. |
| `RealityKit` | Swift-only. |
| `RoomPlan` | Swift-only. |
| `Ruby` | Very C-centric and old. |
| `SecureElementCredential` | Swift-only. |
| `SecurityUI` | TODO. |
| `StickerFoundation` | Basically empty. |
| `StickerKit` | Basically empty. |
| `SwiftData` | Swift-only. |
| `SwiftUI` | Swift-only. |
| `SwiftUICore` | Swift-only. |
| `SyncServices` | Deprecated. |
| `System` | Deprecated wrapper over libSystem.dylib. |
| `TWAIN` | Very C-centric and old. |
| `TabletopKit` | Swift-only. |
| `TabularData` | Swift-only. |
| `Tcl` | Very C-centric and old. |
| `TipKit` | Swift-only. |
| `Tk` | Very C-centric and old. |
| `Translation` | Swift-only. |
| `TranslationUIProvider` | Swift-only. |
| `Twitter` | Deprecated, use Social instead. |
| `VideoDecodeAcceleration` | Very C-centric and old. |
| `VisionKit` | Swift-only. |
| `WeatherKit` | Swift-only. |
| `WidgetKit` | Mostly Swift-only. |
| `WorkoutKit` | Swift-only. |
| `iAd` | Disabled on server side, use AdServices instead. |
| `vecLib` | Very C-centric and old. |
| `vmnet` | Very C-centric and old. |
