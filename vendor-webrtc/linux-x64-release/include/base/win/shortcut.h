// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SHORTCUT_H_
#define BASE_WIN_SHORTCUT_H_

#include <guiddef.h>
#include <stdint.h>

#include "base/base_export.h"
#include "base/check.h"
#include "base/files/file_path.h"

namespace base {
namespace win {

enum class ShortcutOperation {
  // Create a new shortcut (overwriting if necessary).
  kCreateAlways = 0,
  // Overwrite an existing shortcut (fails if the shortcut doesn't exist).
  // If the arguments are not specified on the new shortcut, keep the old
  // shortcut's arguments.
  kReplaceExisting,
  // Update specified properties only on an existing shortcut.
  kUpdateExisting,
};

// Properties for shortcuts. Properties set will be applied to the shortcut on
// creation/update, others will be ignored.
// Callers are encouraged to use the setters provided which take care of
// setting |options| as desired.
struct BASE_EXPORT ShortcutProperties {
  using IndividualProperties = uint32_t;
  static constexpr IndividualProperties PROPERTIES_TARGET = 1U << 0;
  static constexpr IndividualProperties PROPERTIES_WORKING_DIR = 1U << 1;
  static constexpr IndividualProperties PROPERTIES_ARGUMENTS = 1U << 2;
  static constexpr IndividualProperties PROPERTIES_DESCRIPTION = 1U << 3;
  static constexpr IndividualProperties PROPERTIES_ICON = 1U << 4;
  static constexpr IndividualProperties PROPERTIES_APP_ID = 1U << 5;
  static constexpr IndividualProperties PROPERTIES_DUAL_MODE = 1U << 6;
  static constexpr IndividualProperties PROPERTIES_TOAST_ACTIVATOR_CLSID = 1U
                                                                           << 7;
  // Be sure to update the values below when adding a new property.
  static constexpr IndividualProperties PROPERTIES_ALL =
      PROPERTIES_TARGET | PROPERTIES_WORKING_DIR | PROPERTIES_ARGUMENTS |
      PROPERTIES_DESCRIPTION | PROPERTIES_ICON | PROPERTIES_APP_ID |
      PROPERTIES_DUAL_MODE | PROPERTIES_TOAST_ACTIVATOR_CLSID;

  ShortcutProperties();
  ShortcutProperties(const ShortcutProperties& other);
  ~ShortcutProperties();

  void set_target(const FilePath& target_in) {
    target = target_in;
    options |= PROPERTIES_TARGET;
  }

  void set_working_dir(const FilePath& working_dir_in) {
    working_dir = working_dir_in;
    options |= PROPERTIES_WORKING_DIR;
  }

  void set_arguments(const std::wstring& arguments_in) {
    arguments = arguments_in;
    options |= PROPERTIES_ARGUMENTS;
  }

  void set_description(const std::wstring& description_in);

  void set_icon(const FilePath& icon_in, int icon_index_in) {
    icon = icon_in;
    icon_index = icon_index_in;
    options |= PROPERTIES_ICON;
  }

  void set_app_id(const std::wstring& app_id_in) {
    app_id = app_id_in;
    options |= PROPERTIES_APP_ID;
  }

  void set_dual_mode(bool dual_mode_in) {
    dual_mode = dual_mode_in;
    options |= PROPERTIES_DUAL_MODE;
  }

  void set_toast_activator_clsid(const CLSID& toast_activator_clsid_in) {
    toast_activator_clsid = toast_activator_clsid_in;
    options |= PROPERTIES_TOAST_ACTIVATOR_CLSID;
  }

  // The target to launch from this shortcut. This is mandatory when creating
  // a shortcut.
  FilePath target;
  // The name of the working directory when launching the shortcut.
  FilePath working_dir;
  // The arguments to be applied to |target| when launching from this shortcut.
  std::wstring arguments;
  // The localized description of the shortcut.
  // The length of this string must be no larger than INFOTIPSIZE.
  std::wstring description;
  // The path to the icon (can be a dll or exe, in which case |icon_index| is
  // the resource id).
  FilePath icon;
  int icon_index = -1;
  // The app model id for the shortcut.
  std::wstring app_id;
  // Whether this is a dual mode shortcut (Windows).
  bool dual_mode = false;
  // The CLSID of the COM object registered with the OS via the shortcut. This
  // is for app activation via user interaction with a toast notification in the
  // Action Center. (Win10 version 1607, build 14393, and beyond).
  CLSID toast_activator_clsid;
  // Bitfield made of IndividualProperties. Properties set in |options| will be
  // set on the shortcut, others will be ignored.
  uint32_t options = 0U;
};

// This method creates (or updates) a shortcut link at `shortcut_path` using the
// information given through `properties`.
// Ensure you have initialized COM before calling into this function.
// `operation`: a choice from the ShortcutOperation enum.
// If `operation` is kReplaceExisting or kUpdateExisting and
// `shortcut_path` does not exist, this method is a no-op and returns false.
BASE_EXPORT bool CreateOrUpdateShortcutLink(
    const FilePath& shortcut_path,
    const ShortcutProperties& properties,
    ShortcutOperation operation);

// Resolves Windows shortcut (.LNK file).
// This methods tries to resolve selected properties of a shortcut .LNK file.
// The path of the shortcut to resolve is in |shortcut_path|. |options| is a bit
// field composed of ShortcutProperties::IndividualProperties, to specify which
// properties to read. It should be non-0. The resulting data are read into
// |properties|, which must not be NULL. Note: PROPERTIES_TARGET will retrieve
// the target path as stored in the shortcut but won't attempt to resolve that
// path so it may not be valid. The function returns true if all requested
// properties are successfully read. Otherwise some reads have failed and
// intermediate values written to |properties| should be ignored.
BASE_EXPORT bool ResolveShortcutProperties(const FilePath& shortcut_path,
                                           uint32_t options,
                                           ShortcutProperties* properties);

// Resolves Windows shortcut (.LNK file).
// This is a wrapper to ResolveShortcutProperties() to handle the common use
// case of resolving target and arguments. |target_path| and |args| are
// optional output variables that are ignored if NULL (but at least one must be
// non-NULL). The function returns true if all requested fields are found
// successfully. Callers can safely use the same variable for both
// |shortcut_path| and |target_path|.
BASE_EXPORT bool ResolveShortcut(const FilePath& shortcut_path,
                                 FilePath* target_path,
                                 std::wstring* args);

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SHORTCUT_H_
