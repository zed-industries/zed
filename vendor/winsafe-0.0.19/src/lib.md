Windows API and GUI in safe, idiomatic Rust.

[Crate](https://crates.io/crates/winsafe) •
[GitHub](https://github.com/rodrigocfd/winsafe) •
[Docs (stable)](https://docs.rs/winsafe/) •
[Docs (master branch)](https://rodrigocfd.github.io/winsafe/winsafe/) •
[Examples](https://github.com/rodrigocfd/winsafe-examples)

WinSafe has:

* low-level Win32 API constants, functions and structs;
* high-level structs to build native Win32 GUI applications.

# Usage

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
winsafe = { version = "0.0.19", features = [] }
```

Then you must enable the [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) you want to be included – these modules are named after native Windows DLL and library names, mostly.

The following Cargo features are available so far:

| Feature | Description |
| - | - |
| `comctl` | ComCtl32.dll, for [Common Controls](https://learn.microsoft.com/en-us/windows/win32/api/_controls/) |
| `dshow` | [DirectShow](https://learn.microsoft.com/en-us/windows/win32/directshow/directshow) |
| `dwm` | [Desktop Window Manager](https://learn.microsoft.com/en-us/windows/win32/dwm/dwm-overview) |
| `dxgi` | [DirectX Graphics Infrastructure](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dx-graphics-dxgi) |
| `gdi` | Gdi32.dll, the [Windows GDI](https://learn.microsoft.com/en-us/windows/win32/gdi/windows-gdi) |
| **`gui`** | **The WinSafe high-level GUI abstractions** |
| `kernel` | Kernel32.dll, Advapi32.dll and Ktmw32.dll – all others will include it |
| `mf` | [Media Foundation](https://learn.microsoft.com/en-us/windows/win32/medfound/microsoft-media-foundation-sdk) |
| `ole` | OLE and basic COM support |
| `oleaut` | [OLE Automation](https://learn.microsoft.com/en-us/windows/win32/api/_automat/) |
| `shell` | Shell32.dll and Shlwapi.dll, the COM-based [Windows Shell](https://learn.microsoft.com/en-us/windows/win32/shell/shell-entry) |
| `taskschd` | [Task Scheduler](https://learn.microsoft.com/en-us/windows/win32/taskschd/task-scheduler-start-page) |
| `user` | User32.dll and ComDlg32.dll, the basic Windows GUI support |
| `uxtheme` | UxTheme.dll, extended window theming |
| `version` | Version.dll, to manipulate *.exe version info |

If you're looking for a comprehensive Win32 coverage, take a look at [winapi](https://crates.io/crates/winapi) or [windows](https://crates.io/crates/windows) crates, which are *unsafe*, but have everything.

# The GUI API

WinSafe features idiomatic bindings for the Win32 API, but on top of that, it features a set of high-level GUI structs, which scaffolds the boilerplate needed to build native Win32 GUI applications, event-oriented. Unless you're doing something really specific, these high-level wrappers are highly recommended – you'll usually start with the [`WindowMain`](crate::gui::WindowMain).

One of the greatest strenghts of the GUI API is supporting the use of resource files, which can be created with a WYSIWYG [resource editor](https://en.wikipedia.org/wiki/Resource_(Windows)#Resource_software).

GUI structs can be found in module [`gui`](crate::gui).

# Native function calls

The best way to understand the idea behind WinSafe bindings is comparing them to the correspondent C code.

For example, take the following C code:

```c
HWND hwnd = GetDesktopWindow();
SetFocus(hwnd);
```

This is equivalent to:

```rust,ignore
use winsafe::{prelude::*, HWND};

let hwnd = HWND::GetDesktopWindow();
hwnd.SetFocus();
```

Note how [`GetDesktopWindow`](crate::prelude::user_Hwnd::GetDesktopWindow) is a static method of [`HWND`](crate::HWND), and [`SetFocus`](crate::prelude::user_Hwnd::SetFocus) is an instance method called directly upon `hwnd`. All native handles (`HWND`, [`HDC`](crate::HDC), [`HINSTANCE`](crate::HINSTANCE), etc.) are structs, thus:

* native Win32 functions that return a handle are *static methods* in WinSafe;
* native Win32 functions whose *first parameter* is a handle are *instance methods*.

Now this C code:

```c
PostQuitMessage(0);
```

Is equivalent to:

```rust,ignore
use winsafe::PostQuitMessage;

PostQuitMessage(0);
```

Since [`PostQuitMessage`](crate::PostQuitMessage) is a free function, it's simply at the root of the crate.

Also note that some functions which require a cleanup routine – like [`BeginPaint`](crate::prelude::user_Hwnd::BeginPaint), for example – will return the resource wrapped in a [guard](crate::guard), which will perform the cleanup automatically. You'll never have to manually call [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint).

Sending messages are a special case, see the [`msg`](crate::msg) module.

# Native constants

All native Win32 constants can be found in the [`co`](crate::co) module. They're all *typed*, what means that different constant types cannot be mixed (unless you explicitly say so).

Technically, each constant type is simply a [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) with a couple implementations, including those allowing bitflag operations. Also, all constant values can be converted to its underlying [integer type](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types).

The name of the constant type is often its prefix. For example, constants of [`MessageBox`](crate::prelude::user_Hwnd::MessageBox) function, like `MB_OKCANCEL`, belong to a type called [`MB`](crate::co::MB).

For example, take the following C code:

```c
let hwnd = GetDesktopWindow();
MessageBox(hwnd, "Hello, world", "My hello", MB_OKCANCEL | MB_ICONINFORMATION);
```

This is equivalent to:

```rust,ignore
use winsafe::{prelude::*, co::MB, HWND};

let hwnd = HWND::GetDesktopWindow();
hwnd.MessageBox("Hello, world", "Title", MB::OKCANCEL | MB::ICONINFORMATION)?;
# Ok::<_, winsafe::co::ERROR>(())
```

The method [`MessageBox`](crate::prelude::user_Hwnd::MessageBox), like most functions that can return errors, will return [`SysResult`](crate::SysResult), which can contain an [`ERROR`](crate::co::ERROR) constant.

# Native structs

WinSafe implements native Win32 structs in a very restricted way. First off, fields which control the size of the struct – often named `cbSize` – are *private* and automatically set when the struct is instantiated.

Pointer fields are also private, and they can be set and retrieved *only* through getter and setter methods. In particular, when setting a string pointer field, you need to pass a reference to a [`WString`](crate::WString) buffer, which will keep the actual string contents.

For example, the following C code:

```c
WNDCLASSEX wcx = {0};
wcx.cbSize = sizeof(WNDCLASSEX);
wcx.lpszClassName = "MY_WINDOW";

if (RegisterClassEx(&wcx) == 0) {
    DWORD err = GetLastError();
    // handle error...
}
```

Is equivalent to:

```rust,ignore
use winsafe::{RegisterClassEx, WNDCLASSEX, WString};

let mut wcx = WNDCLASSEX::default();

let mut buf = WString::from_str("MY_WINDOW");
wcx.set_lpszClassName(Some(&mut buf));

if let Err(err) = RegisterClassEx(&wcx) {
    // handle error...
}
```

Note how you *don't need* to call [`GetLastError`](crate::GetLastError) to retrieve the error code: it's returned by the method itself in the [`SysResult`](crate::SysResult).

# Text encoding

Windows natively uses [Unicode UTF-16](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings).

WinSafe uses Unicode UTF-16 internally but exposes idiomatic UTF-8, performing conversions automatically when needed, so you don't have to worry about [`OsString`](std::ffi::OsString) or any low-level conversion.

However, there are cases where a string conversion is still needed, like when dealing with native Win32 structs. In such cases, you can use the [`WString`](crate::WString) struct, which is also capable of working as a buffer to receive text from Win32 calls.

# Errors and result aliases

WinSafe declares a few [`Result` aliases](https://doc.rust-lang.org/rust-by-example/error/result/result_alias.html) which are returned by its functions and methods:

| Alias | Error | Used for |
| - | - | - |
| [`SysResult`](crate::SysResult) | [`ERROR`](crate::co::ERROR) | Standard [system errors](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes). |
| [`HrResult`](crate::HrResult) | [`HRESULT`](crate::co::HRESULT) | [COM errors](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a).
| [`AnyResult`](crate::AnyResult) | `Box<dyn Error + Send + Sync>` | Holding different error types. All other `Result` aliases can be converted into it. |

# Utilities

Beyond the [GUI](crate::gui) API, WinSafe features a few high-level abstractions to deal with some particularly complex Win32 topics. Unless you need something specific, prefer using these over the raw, native calls:

| Utility | Used for |
| - | - |
| [`Encoding`](crate::Encoding) | String encodings. |
| [`File`](crate::File) | File read/write and other operations. |
| [`FileMapped`](crate::FileMapped) | Memory-mapped file operations. |
| [`path`](crate::path) | File path operations. |
| [`WString`](crate::WString) | Managing native wide strings. |
