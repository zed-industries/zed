// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_REGISTRY_H_
#define BASE_WIN_REGISTRY_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/functional/callback_forward.h"
#include "base/types/expected.h"
#include "base/types/strong_alias.h"
#include "base/win/windows_types.h"

namespace base {
namespace win {

// Utility class to read, write and manipulate the Windows Registry.
// Registry vocabulary primer: a "key" is like a folder, in which there
// are "values", which are <name, data> pairs, with an associated data type.
//
// Note:
//  * ReadValue family of functions guarantee that the out-parameter
//    is not touched in case of failure.
//  * Functions returning LONG indicate success as ERROR_SUCCESS or an
//    error as a (non-zero) win32 error code.
class BASE_EXPORT RegKey {
 public:
  // Called from the MessageLoop when the key changes.
  using ChangeCallback = OnceCallback<void()>;

  RegKey();
  explicit RegKey(HKEY key);
  RegKey(HKEY rootkey, const wchar_t* subkey, REGSAM access);
  RegKey(RegKey&& other) noexcept;
  RegKey& operator=(RegKey&& other);

  RegKey(const RegKey&) = delete;
  RegKey& operator=(const RegKey&) = delete;

  ~RegKey();

  // Creates a new reg key, replacing `this` with a reference to the
  // newly-opened key. In case of error, `this` is unchanged.
  [[nodiscard]] LONG Create(HKEY rootkey, const wchar_t* subkey, REGSAM access);

  // Creates a new reg key, replacing `this` with a reference to the
  // newly-opened key. In case of error, `this` is unchanged.
  [[nodiscard]] LONG CreateWithDisposition(HKEY rootkey,
                                           const wchar_t* subkey,
                                           DWORD* disposition,
                                           REGSAM access);

  // Creates a subkey or opens it if it already exists. In case of error, `this`
  // is unchanged.
  [[nodiscard]] LONG CreateKey(const wchar_t* name, REGSAM access);

  // Opens an existing reg key, replacing `this` with a reference to the
  // newly-opened key. In case of error, `this` is unchanged.
  [[nodiscard]] LONG Open(HKEY rootkey, const wchar_t* subkey, REGSAM access);

  // Opens an existing reg key, given the relative key name.
  [[nodiscard]] LONG OpenKey(const wchar_t* relative_key_name, REGSAM access);

  // Closes this reg key.
  void Close();

  // Replaces the handle of the registry key and takes ownership of the handle.
  void Set(HKEY key);

  // Transfers ownership away from this object.
  HKEY Take();

  // Returns false if this key does not have the specified value, or if an error
  // occurrs while attempting to access it.
  bool HasValue(const wchar_t* value_name) const;

  // Returns the number of values for this key, or an error code if the number
  // cannot be determined.
  base::expected<DWORD, LONG> GetValueCount() const;

  // Determines the nth value's name.
  LONG GetValueNameAt(DWORD index, std::wstring* name) const;

  // True while the key is valid.
  bool Valid() const { return key_ != nullptr; }

  // Kills a key and, by default, everything that lives below it; please be
  // careful when using it. `recursive` = false may be used to prevent
  // recursion, in which case the key is only deleted if it has no subkeys.
  using RecursiveDelete = base::StrongAlias<class RecursiveDeleteTag, bool>;
  LONG DeleteKey(const wchar_t* name,
                 RecursiveDelete recursive = RecursiveDelete(true));

  // Deletes a single value within the key.
  LONG DeleteValue(const wchar_t* name);

  // Getters:

  // Reads a REG_DWORD (uint32_t) into |out_value|. If |name| is null or empty,
  // reads the key's default value, if any.
  LONG ReadValueDW(const wchar_t* name, DWORD* out_value) const;

  // Reads a REG_QWORD (int64_t) into |out_value|. If |name| is null or empty,
  // reads the key's default value, if any.
  LONG ReadInt64(const wchar_t* name, int64_t* out_value) const;

  // Reads a string into |out_value|. If |name| is null or empty, reads
  // the key's default value, if any.
  LONG ReadValue(const wchar_t* name, std::wstring* out_value) const;

  // Reads a REG_MULTI_SZ registry field into a vector of strings. Clears
  // |values| initially and adds further strings to the list. Returns
  // ERROR_CANTREAD if type is not REG_MULTI_SZ.
  LONG ReadValues(const wchar_t* name, std::vector<std::wstring>* values);

  // Reads raw data into |data|. If |name| is null or empty, reads the key's
  // default value, if any.
  LONG ReadValue(const wchar_t* name,
                 void* data,
                 DWORD* dsize,
                 DWORD* dtype) const;

  // Setters:

  // Sets a uint32_t value.
  LONG WriteValue(const wchar_t* name, DWORD in_value);

  // Sets a string value.
  LONG WriteValue(const wchar_t* name, const wchar_t* in_value);

  // Sets raw data, including type.
  LONG WriteValue(const wchar_t* name,
                  const void* data,
                  DWORD dsize,
                  DWORD dtype);

  // Starts watching the key to see if any of its values have changed.
  // The key must have been opened with the KEY_NOTIFY access privilege.
  // Returns true on success.
  // To stop watching, delete this RegKey object. To continue watching the
  // object after the callback is invoked, call StartWatching again.
  bool StartWatching(ChangeCallback callback);

  HKEY Handle() const { return key_; }

 private:
  class Watcher;

  // Opens the key `subkey` under `rootkey` with the given options and
  // access rights. `options` may be 0 or `REG_OPTION_OPEN_LINK`. Returns
  // ERROR_SUCCESS or a Windows error code.
  [[nodiscard]] LONG Open(HKEY rootkey,
                          const wchar_t* subkey,
                          DWORD options,
                          REGSAM access);

  // Returns true if the key is a symbolic link, false if it is not, or a
  // Windows error code in case of a failure to determine. `this` *MUST* have
  // been opened via at least `Open(..., REG_OPTION_OPEN_LINK,
  // REG_QUERY_VALUE);`.
  expected<bool, LONG> IsLink() const;

  // Deletes the key if it is a symbolic link. Returns ERROR_SUCCESS if the key
  // was a link and was deleted, a Windows error code if checking the key or
  // deleting it failed, or `nullopt` if the key exists and is not a symbolic
  // link.
  std::optional<LONG> DeleteIfLink();

  // Recursively deletes a key and all of its subkeys.
  static LONG RegDelRecurse(HKEY root_key, const wchar_t* name, REGSAM access);

  HKEY key_ = nullptr;  // The registry key being iterated.
  REGSAM wow64access_ = 0;
  std::unique_ptr<Watcher> key_watcher_;
};

// Iterates the entries found in a particular folder on the registry.
class BASE_EXPORT RegistryValueIterator {
 public:
  // Constructs a Registry Value Iterator with default WOW64 access.
  RegistryValueIterator(HKEY root_key, const wchar_t* folder_key);

  // Constructs a Registry Key Iterator with specific WOW64 access, one of
  // KEY_WOW64_32KEY or KEY_WOW64_64KEY, or 0.
  // Note: |wow64access| should be the same access used to open |root_key|
  // previously, or a predefined key (e.g. HKEY_LOCAL_MACHINE).
  // See http://msdn.microsoft.com/en-us/library/windows/desktop/aa384129.aspx.
  RegistryValueIterator(HKEY root_key,
                        const wchar_t* folder_key,
                        REGSAM wow64access);

  RegistryValueIterator(const RegistryValueIterator&) = delete;
  RegistryValueIterator& operator=(const RegistryValueIterator&) = delete;

  ~RegistryValueIterator();

  DWORD ValueCount() const;

  // True while the iterator is valid.
  bool Valid() const;

  // Advances to the next registry entry.
  void operator++();

  // TODO(crbug.com/329476354): Provide a wcstring_view instead of a pointer.
  const wchar_t* Name() const { return name_.c_str(); }
  // TODO(crbug.com/329476354): Provide a wcstring_view instead of a pointer.
  const wchar_t* Value() const { return value_.data(); }
  // ValueSize() is in bytes.
  DWORD ValueSize() const { return value_size_; }
  DWORD Type() const { return type_; }

  DWORD Index() const { return index_; }

 private:
  // Reads in the current values.
  bool Read();

  void Initialize(HKEY root_key, const wchar_t* folder_key, REGSAM wow64access);

  // The registry key being iterated.
  HKEY key_;

  // Current index of the iteration.
  DWORD index_;

  // Current values.
  std::wstring name_;
  // The vector always has a `0` at the end, after its `ValueSize() / 2u`
  // elements (since ValueSize() is in bytes, but the vector is of 2-byte
  // objects). This allows the value to always be read as a NUL-terminated
  // string, even if it's holding another type of data.
  std::vector<wchar_t> value_;
  DWORD value_size_;
  DWORD type_;
};

class BASE_EXPORT RegistryKeyIterator {
 public:
  // Constructs a Registry Key Iterator with default WOW64 access.
  RegistryKeyIterator(HKEY root_key, const wchar_t* folder_key);

  // Constructs a Registry Value Iterator with specific WOW64 access, one of
  // KEY_WOW64_32KEY or KEY_WOW64_64KEY, or 0.
  // Note: |wow64access| should be the same access used to open |root_key|
  // previously, or a predefined key (e.g. HKEY_LOCAL_MACHINE).
  // See http://msdn.microsoft.com/en-us/library/windows/desktop/aa384129.aspx.
  RegistryKeyIterator(HKEY root_key,
                      const wchar_t* folder_key,
                      REGSAM wow64access);

  RegistryKeyIterator(const RegistryKeyIterator&) = delete;
  RegistryKeyIterator& operator=(const RegistryKeyIterator&) = delete;

  ~RegistryKeyIterator();

  DWORD SubkeyCount() const;

  // True while the iterator is valid.
  bool Valid() const;

  // Advances to the next entry in the folder.
  void operator++();

  const wchar_t* Name() const { return name_; }

  DWORD Index() const { return index_; }

 private:
  // Reads in the current values.
  bool Read();

  void Initialize(HKEY root_key, const wchar_t* folder_key, REGSAM wow64access);

  // The registry key being iterated.
  HKEY key_;

  // Current index of the iteration.
  DWORD index_;

  wchar_t name_[MAX_PATH];
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_REGISTRY_H_
