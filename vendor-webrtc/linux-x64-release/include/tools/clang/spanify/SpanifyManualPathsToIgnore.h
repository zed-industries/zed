// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_SPANIFY_SPANIFYMANUALPATHSTOIGNORE_H_
#define TOOLS_CLANG_SPANIFY_SPANIFYMANUALPATHSTOIGNORE_H_

#include <array>

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
inline constexpr std::array kSpanifyManualPathsToIgnore = {
    // DEPS basically prohibits includes from base/.
    "base/allocator/partition_alloc",

    // win:pe_image target that uses this file does not depend on base/.
    "base/no_destructor.h",

    // dwarf_helpers from //base/BUILD.gn is a dependency of base and can't
    // depend on it and thus can't use base::span.
    "base/debug/buffered_dwarf_reader.cc",
    "base/debug/buffered_dwarf_reader.h",
    "base/debug/dwarf_line_no.cc",
    "base/debug/dwarf_line_no.h",

    // span_unittests explicitly wants to test compatibility of certain types,
    // rewriting would break that.
    "base/containers/span_unittest.cc",

    // The comment atop this test suite explains that it "contains intentional
    // memory errors" to verify Chromium tooling.
    "base/tools_sanity_unittest.cc",

    // Can't depend on //base, pointers/references under this directory can't be
    // rewritten.
    "testing/rust_gtest_interop/",

    // Exclude - deprecated and contains legacy C++ and pre-C++11 code.
    "ppapi/",

    // Exclude tools that do not ship in the Chrome binary. Can't depend on
    // //base.
    "base/android/linker/",
    "chrome/chrome_elf/",
    "chrome/installer/mini_installer/",
    "testing/platform_test.h",
    "/tools/",  // catches subdirs e.g. /net/tools, but not devtools/ etc.
    "!tools/clang/spanify/tests/",  // Add back this dir so we can run tests for
                                    // spanify.

    // DEPS prohibits includes from base/
    "chrome/install_static",
    "sandbox/mac/",

    // Exclude pocdll.dll as it doesn't depend on //base and only used for
    // testing.
    "sandbox/win/sandbox_poc/pocdll",

    // Exclude internal definitions of undocumented Windows structures.
    "sandbox/win/src/nt_internals.h",

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

    // Contains sysroot dirs like debian_bullseye_amd64-sysroot/ that are not
    // part of the repository.
    "build/linux/",

    // glslang_tab.cpp.h uses #line directive and modifies the file path to
    // "MachineIndependent/glslang.y" so the isInThirdPartyLocation() filter
    // cannot
    // catch it even though glslang_tab.cpp.h is in third_party/
    "MachineIndependent/",

    // Exclusion for potential performance reasons with the std::array rewrite.
    // Please run additional performance benchmarks before rewriting these
    // files. In particular the changes to WTF's vector.h
    "skia/ext/",
    "third_party/blink/renderer/core/css/parser/css_parser_fast_paths.cc",
    "third_party/blink/renderer/core/layout/inline/line_breaker.cc",
    "third_party/blink/renderer/core/paint/box_border_painter.cc",
    "third_party/blink/renderer/platform/image-decoders/",
    "third_party/blink/renderer/platform/text/",
    "third_party/blink/renderer/platform/wtf/",
    "url/url_canon_host.cc",
    "url/url_canon_path.cc",

    // Exclude auto generated files. These are files that contain the string
    // "This file is auto-generated from".
    "gpu/GLES2/gl2chromium_autogen.h",
    "gpu/command_buffer/client/gles2_implementation_impl_autogen.h",
    "gpu/command_buffer/client/gles2_implementation_unittest_autogen.h",
    "gpu/command_buffer/client/raster_implementation_impl_autogen.h",
    "gpu/command_buffer/client/raster_implementation_unittest_autogen.h",
    "gpu/command_buffer/common/gles2_cmd_format_autogen.h",
    "gpu/command_buffer/service/context_state_impl_autogen.h",
    "gpu/command_buffer/service/gles2_cmd_decoder_autogen.h",
    "gpu/command_buffer/service/gles2_cmd_decoder_unittest_2_autogen.h",
    "gpu/command_buffer/service/raster_decoder_autogen.h",
    "gpu/config/gpu_control_list_testing_autogen.cc",
    "gpu/config/gpu_control_list_testing_autogen.h",
    "gpu/config/gpu_control_list_testing_entry_enums_autogen.h",
    "gpu/config/gpu_control_list_testing_exceptions_autogen.h",
    "gpu/ipc/common/vulkan_types.mojom",
    "gpu/ipc/common/vulkan_types_mojom_traits.cc",
    "gpu/ipc/common/vulkan_types_mojom_traits.h",
    "gpu/vulkan/vulkan_function_pointers.cc",
    "gpu/vulkan/vulkan_function_pointers.h",
    "ppapi/c/dev/ppb_opengles2ext_dev.h",
    "ppapi/c/ppb_opengles2.h",
    "ppapi/lib/gl/gles2/gles2.c",
    "ppapi/shared_impl/ppb_opengles2_shared.cc",
    "ui/gl/egl_bindings_autogen_mock.cc",
    "ui/gl/egl_bindings_autogen_mock.h",
    "ui/gl/gl_bindings_api_autogen_egl.h",
    "ui/gl/gl_bindings_api_autogen_gl.h",
    "ui/gl/gl_bindings_autogen_egl.cc",
    "ui/gl/gl_bindings_autogen_egl.h",
    "ui/gl/gl_bindings_autogen_gl.cc",
    "ui/gl/gl_bindings_autogen_gl.h",
    "ui/gl/gl_bindings_autogen_mock.cc",
    "ui/gl/gl_bindings_autogen_mock.h",
    "ui/gl/gl_enums_implementation_autogen.h",
    "ui/gl/gl_mock_autogen_egl.h",
    "ui/gl/gl_mock_autogen_gl.h",
    "ui/gl/gl_stub_autogen_gl.cc",
    "ui/gl/gl_stub_autogen_gl.h",

    // Exclude these generated files.
    //
    // An example of `spanify` picking them up can be seen at
    // https://crrev.com/c/6389460/1/third_party/blink/renderer/core/xml/xpath_grammar_generated.cc
    //
    // while a proper "rewrite" would require manipulating bison, e.g.
    // https://crrev.com/c/6357073
    "third_party/blink/renderer/core/xml/xpath_grammar_generated.h",
    "third_party/blink/renderer/core/xml/xpath_grammar_generated.cc",

    // Included inside a class declaration. Adding top-level #includes (e.g.,
    // for span.h, <vector>) here will cause compilation errors.
    "gpu/command_buffer/client/gles2_interface_autogen.h",
};

#endif  // TOOLS_CLANG_SPANIFY_SPANIFYMANUALPATHSTOIGNORE_H_
