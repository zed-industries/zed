# Test binary generator for Portable Executable

## Preprocessor definitions

- `ENABLE_TLS`: if defined, compile the binary with Thread Local Storage (TLS) enabled.

## How to build

This project is designed to be compiled by Clang (`clang-cl`) _not_ MSVC toolchain, primarily because 1) MSVC linker supports more features and may unexpectedly generate larger binaries than LLD linker under Clang, 2) compiler and linker flags are designed for specifically Clang for best efforts in reducing size of the resulting binary, 3) sometimes LLD links much smarter e.g., metadatas such as unnecessary rich headers (can be disabled by `/EMITTOOLVERSIONINFO:NO`).

While MSVC toolchains are theoretically possible; but not recommended.

### CMake

**Prerequisites**
- 64-bit Windows host
- CMake 3.12 or later
- [Ninja build system](https://ninja-build.org) (any version)
- Clang (ideally 17 or later)

Firstly run the following commands on terminal:

```bash
mkdir build
```

```bash
cd build
```

```bash
cmake .. -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=clang-cl
```

Then build the binary:

```bash
cmake --build . --config release
```

### Visual Studio

**Prerequisites**
- 64-bit Windows host
- Visual Studio 2022 (editions do not matter)
  - C++ Clang Compiler for Windows [or this method if you have manual installation](#optional-referencing-manually-installed-clang-toolchain)
  - MSBuild support for LLVM (clang-cl) toolset

Open `bingen.sln` under [`etc/projects/bingen`](etc/projects/bingen/) and compile as Release (Debug configuration is redacted).

#### Optional: Referencing manually-installed Clang toolchain

If you do not have or not willing to install Clang under Visual Studio individual components, [customize the build by folder for MSBuild](https://learn.microsoft.com/en-us/visualstudio/msbuild/customize-by-directory?view=vs-2022) by deploying following `Directory.build.props` right next to the `.sln`:

```xml
<!-- Directory.build.props -->
<Project>
  <PropertyGroup>
    <LLVMInstallDir>C:/path/to/llvm/bin</LLVMInstallDir>
    <LLVMToolsVersion>xx.xxxx.x</LLVMToolsVersion>
  </PropertyGroup>
</Project>
```
