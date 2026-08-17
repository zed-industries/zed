// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRMANUALPATHSTOIGNORE_H_
#define TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRMANUALPATHSTOIGNORE_H_

// Array listing regular expressions of paths that should be ignored when
// running the rewrite_raw_ptr_fields tool on Chromium sources.
//
// If a source file path contains any of the lines in the filter file below,
// then such source file will not be rewritten.
//
// Lines prefixed with "!" can be used to force include files that matched a
// file path to be ignored.
//
// Note that the rewriter has a hardcoded logic for a handful of path-based
// exclusions that cannot be expressed as substring matches:
// - Excluding paths containing "third_party/", but still covering
//   "third_party/blink/"
//   (see the isInThirdPartyLocation AST matcher in RewriteRawPtrFields.cpp).
// - Excluding paths _starting_ with "gen/" or containing "/gen/"
//   (i.e. hopefully just the paths under out/.../gen/... directory)
//   via the isInGeneratedLocation AST matcher in RewriteRawPtrFields.cpp.
constexpr const char* const kRawPtrManualPathsToIgnore[] = {
    // Exclude to prevent PartitionAlloc<->raw_ptr<T> cyclical dependency.
    "base/allocator/",

    // win:pe_image target that uses this file does not depend on base/.
    "base/no_destructor.h",

    // Can't depend on //base, pointers/references under this directory can't be
    // rewritten.
    "testing/rust_gtest_interop/",

    // Exclude - deprecated and contains legacy C++ and pre-C++11 code.
    "ppapi/",

    // Exclude tools that do not ship in the Chrome binary. Can't depend on
    // //base.
    "base/android/linker/",
    "/tools/",  // catches subdirs e.g. /net/tools, but not devtools/ etc.
    "chrome/chrome_elf/",
    "chrome/installer/mini_installer/",
    "testing/platform_test.h",

    // DEPS prohibits includes from base/
    "chrome/install_static",
    "sandbox/mac/",

    // Exclude pocdll.dll as it doesn't depend on //base and only used for
    // testing.
    "sandbox/win/sandbox_poc/pocdll",

    // Exclude internal definitions of undocumented Windows structures.
    "sandbox/win/src/nt_internals.h",

    // Cannot dep on //base as files are also used by chrome_elf.
    "sandbox/policy/win/hook_util/",

    // Exclude directories that don't depend on //base, because nothing there
    // uses
    // anything from /base.
    "sandbox/linux/system_headers/",
    "components/history_clusters/core/",
    "ui/qt/",

    // The folder holds headers that are duplicated in the Android source and
    // need to
    // provide a stable C ABI. Can't depend on //base.
    "android_webview/public/",

    // Exclude dir that should hold C headers.
    "mojo/public/c/",

    // Renderer-only code is generally allowed to use MiraclePtr. These
    // directories, however, are specifically disallowed, for perf reasons.
    //
    // Note that some renderer-only directories are already excluded
    // elsewhere - for example "v8/" is excluded, because it's in another
    // repository.
    //
    // Also, note that isInThirdPartyLocation AST matcher in
    // RewriteRawPtrFields.cpp explicitly allows third_party/blink
    "third_party/blink/renderer/core/",
    "third_party/blink/renderer/platform/heap/",
    "third_party/blink/renderer/platform/wtf/",
    "third_party/blink/renderer/platform/fonts/",

    // The below paths are an explicitly listed subset of Renderer-only code,
    // because the plan is to Oilpanize it.
    // TODO(crbug.com/330759291): Remove once Oilpanization is completed or
    // abandoned.
    "third_party/blink/renderer/core/paint/",
    "third_party/blink/renderer/platform/graphics/compositing/",
    "third_party/blink/renderer/platform/graphics/paint/",

    // Contains sysroot dirs like debian_bullseye_amd64-sysroot/ that are not
    // part of the repository.
    "build/linux/",

    // glslang_tab.cpp.h uses #line directive and modifies the file path to
    // "MachineIndependent/glslang.y" so the isInThirdPartyLocation() filter
    // cannot
    // catch it even though glslang_tab.cpp.h is in third_party/
    "MachineIndependent/",
};

#endif  // TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRMANUALPATHSTOIGNORE_H_
