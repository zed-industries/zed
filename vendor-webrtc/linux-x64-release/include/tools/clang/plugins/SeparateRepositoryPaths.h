// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_SEPARATEREPOSITORYPATHS_H_
#define TOOLS_CLANG_PLUGINS_SEPARATEREPOSITORYPATHS_H_

// Paths in separate repositories - i.e. in directories that
// 1. Contain a ".git" subdirectory
// 2. And hasn't been excluded via "third_party/" substring in their path
//    (see the isInThirdPartyLocation AST matcher in
//    RewriteRawPtrFields.cpp).
//
// The list below has been generated with:
//
//  $ find . -type d -name .git | \
//      sed -e 's/\.git$//g' | \
//      sed -e 's/\.\///g' | \
//      grep -v third_party | \
//      grep -v '^$' | \
//      sort | uniq > ~/scratch/git-paths
constexpr const char* const kSeparateRepositoryPaths[] = {
    "chrome/app/theme/default_100_percent/google_chrome/",
    "chrome/app/theme/default_200_percent/google_chrome/",
    "chrome/app/theme/google_chrome/",
    "chrome/browser/enterprise/connectors/internal/",
    "chrome/browser/google/linkdoctor_internal/",
    "chrome/browser/internal/",
    "chrome/browser/media/engagement_internal/",
    "chrome/browser/resources/chromeos/quickoffice/",
    "chrome/browser/resources/downloads/internal/",
    "chrome/browser/resources/preinstalled_web_apps/internal/",
    "chrome/browser/resources/settings/internal/",
    "chrome/browser/spellchecker/internal/",
    "chrome/installer/mac/internal/",
    "chromeos/assistant/internal/",
    "chrome/services/speech/internal/",
    "chrome/test/data/firefox3_profile/searchplugins/",
    "chrome/test/data/firefox3_searchplugins/",
    "chrome/test/data/gpu/vt/",
    "chrome/test/data/pdf_private/",
    "chrome/test/data/perf/canvas_bench/",
    "chrome/test/data/perf/frame_rate/content/",
    "chrome/test/data/perf/frame_rate/private/",
    "chrome/test/data/perf/private/",
    "chrome/test/data/xr/webvr_info/",
    "chrome/test/media_router/internal/",
    "chrome/test/python_tests/",
    "chrome/tools/memory/",
    "components/autofill/core/browser/form_parsing/internal_resources/",
    "components/crash/core/app/internal/",
    "components/metrics/internal/",
    "components/ntp_tiles/resources/internal/",
    "components/optimization_guide/internal/",
    "components/resources/default_100_percent/google_chrome/",
    "components/resources/default_200_percent/google_chrome/",
    "components/resources/default_300_percent/google_chrome/",
    "components/site_isolation/internal/",
    "components/vector_icons/google_chrome/",
    "content/test/data/plugin/",
    "docs/website/",
    "google_apis/internal/",
    "/internal/",  // Manually added '/' at the beginning for strictness.
    "media/cdm/api/",
    "native_client/",
    "remoting/host/installer/linux/internal/",
    "remoting/internal/",
    "remoting/test/internal/",
    "remoting/tools/internal/",
    "remoting/webapp/app_remoting/internal/",
    "tools/page_cycler/acid3/",
    "tools/perf/data/",
    "ui/file_manager/internal/",
    "ui/webui/internal/",
    "v8/",
    "webkit/data/bmp_decoder/",
    "webkit/data/ico_decoder/",
    "webkit/data/test_shell/plugins/",
};

#endif  // TOOLS_CLANG_PLUGINS_SEPARATEREPOSITORYPATHS_H_
